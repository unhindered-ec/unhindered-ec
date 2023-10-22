use std::collections::BTreeMap;

use proc_macro2::Span;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    spanned::Spanned,
    Field, Ident, Token, Type,
};

use self::{macro_args::PushStateFlags, stack_attribute_args::StackMarkerFlags};

pub mod macro_args;
pub mod stack_attribute_args;

pub enum FlagsValue<T> {
    Set(T),
    Unset(T),
}

impl<T> FlagsValue<T> {
    fn inverted(self) -> Self {
        match self {
            Self::Set(v) => Self::Unset(v),
            Self::Unset(v) => Self::Set(v),
        }
    }

    pub fn seperate(self) -> (T, bool) {
        match self {
            FlagsValue::Set(v) => (v, true),
            FlagsValue::Unset(v) => (v, false),
        }
    }
}

impl<T: Parse> Parse for FlagsValue<T> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(if input.peek(Token![!]) {
            input.parse::<Token![!]>()?;
            Self::parse(input)?.inverted()
        } else {
            FlagsValue::Set(input.parse()?)
        })
    }
}

pub type StacksInput = BTreeMap<Ident, (StackMarkerFlags, Type)>;
pub type ExecStackInput = Option<(Ident, StackMarkerFlags, Type)>;
pub type InputInstructionsInput = Option<Ident>;

pub fn parse_fields(
    fields: &mut Punctuated<Field, Token![,]>,
    macro_span: Span,
    PushStateFlags {
        builder: generate_builder,
        has_stack: derive_has_stack,
    }: &PushStateFlags,
) -> syn::Result<(StacksInput, ExecStackInput, InputInstructionsInput)> {
    let mut stacks: BTreeMap<Ident, (StackMarkerFlags, Type)> = BTreeMap::new();
    let mut exec_stack: Option<(Ident, StackMarkerFlags, Type)> = None;

    let mut input_instructions: Option<Ident> = None;

    for Field {
        attrs, ident, ty, ..
    } in fields
    {
        let mut matching_attrs: Vec<syn::Attribute> = Vec::new();
        let ident = ident.clone().ok_or_else(|| {
            syn::Error::new(
                macro_span,
                "This macro only supports struct with named fields.",
            )
        })?;
        let mut i = 0;
        // Consume elements that are for our macro from the input to not return them
        loop {
            let Some(to_compare) = attrs.get(i) else {
                break;
            };
            if to_compare.meta.path() == &syn::parse_quote!(stack) {
                matching_attrs.push(attrs.remove(i));
                continue;
            }
            if to_compare.meta.path() == &syn::parse_quote!(input_instructions) {
                if input_instructions.is_some() {
                    return Err(syn::Error::new_spanned(
                        to_compare,
                        "Only one input instructions field is supported",
                    ));
                }
                if !matches!(to_compare.meta, syn::Meta::Path(_)) {
                    return Err(syn::Error::new_spanned(
                        to_compare,
                        "This attribute does not support any arguments",
                    ));
                }
                attrs.remove(i);
                input_instructions = Some(ident.clone());
                continue;
            }
            i += 1;
        }
        if matching_attrs.is_empty() {
            continue;
        }

        let mut empty_attr: Option<syn::Attribute> = None;
        let mut nonempty_attr = false;

        let mut stack_marker_flags = StackMarkerFlags::default();

        for attr in matching_attrs {
            let attr_span = attr.span();
            match attr.meta {
                syn::Meta::Path(_) => empty_attr = Some(attr),
                syn::Meta::List(l) => {
                    nonempty_attr = true;
                    if !matches!(l.delimiter, syn::MacroDelimiter::Paren(_)) {
                        return Err(syn::Error::new_spanned(
                            l,
                            "Only parenthecies supported in this position (#[stack(...)])",
                        ));
                    }

                    let marker_flags: StackMarkerFlags = syn::parse2(l.tokens)?;
                    if marker_flags.is_exec {
                        if !generate_builder && !derive_has_stack {
                            return Err(syn::Error::new(attr_span, "Unknown flag exec. Maybe you meant to enable the builder or has_stack feature of the push_state macro?"));
                        }
                        if stack_marker_flags.is_exec {
                            return Err(syn::Error::new(attr_span, "Redundant exec flag"));
                        } else if stack_marker_flags.builder_name.is_some() {
                            return Err(syn::Error::new(
                                attr_span,
                                "Builder name cannot be set for exec stacks",
                            ));
                        } else {
                            stack_marker_flags.is_exec = marker_flags.is_exec;
                        }
                    }
                    if let Some(builder_name) = marker_flags.builder_name {
                        if !generate_builder {
                            return Err(syn::Error::new(attr_span, "Unknown flag generate_builder. Maybe you meant to enable the builder feature of the push_state macro?"));
                        }
                        if stack_marker_flags.builder_name.is_some() {
                            return Err(syn::Error::new(
                                attr_span,
                                "Builder name already set explicitly",
                            ));
                        } else if stack_marker_flags.is_exec {
                            return Err(syn::Error::new(
                                attr_span,
                                "Builder name cannot be set for exec stacks",
                            ));
                        } else {
                            stack_marker_flags.builder_name = Some(builder_name);
                        }
                    }
                }
                syn::Meta::NameValue(n) => {
                    return Err(syn::Error::new_spanned(n, "This kind of attribute meta is not supported, did you mean to use a list? (#[stack(builder_name = ...)])"));
                }
            }
        }

        if let Some(empty_attr) = empty_attr {
            if nonempty_attr {
                return Err(syn::Error::new_spanned(empty_attr, "Redundant attribute"));
            }
        }

        if stack_marker_flags.is_exec {
            if exec_stack.is_some() {
                return Err(syn::Error::new_spanned(
                    ident,
                    "Only one exec stack supported",
                ));
            } else {
                exec_stack = Some((ident, stack_marker_flags, ty.clone()))
            }
        } else {
            stacks.insert(ident, (stack_marker_flags, ty.clone()));
        }
    }
    Ok((stacks, exec_stack, input_instructions))
}
