use std::collections::BTreeMap;

use ident_case_conversions::CaseConversions;
use kw::StackMarkerFlagsKw;
use quote::quote;
use syn::{ext::IdentExt, punctuated::Punctuated, spanned::Spanned, DeriveInput, Token};

struct PushStateFlags {
    builder: bool,
    has_stack: bool,
}

impl Default for PushStateFlags {
    fn default() -> Self {
        Self {
            builder: false,
            has_stack: true,
        }
    }
}

#[derive(Default)]
struct StackMarkerFlags {
    builder_name: Option<syn::Ident>,
    instruction_name: Option<syn::Path>,
    is_exec: bool,
}

mod kw {
    use quote::ToTokens;
    use syn::Token;

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

    impl<T: syn::parse::Parse> syn::parse::Parse for FlagsValue<T> {
        fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
            Ok(if input.peek(Token![!]) {
                input.parse::<Token![!]>()?;
                Self::parse(input)?.inverted()
            } else {
                FlagsValue::Set(input.parse()?)
            })
        }
    }

    syn::custom_keyword!(builder);
    syn::custom_keyword!(has_stack);

    pub enum PushStateFlagsKw {
        Builder(builder),
        HasStack(has_stack),
    }

    impl ToTokens for PushStateFlagsKw {
        fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
            match self {
                Self::Builder(t) => t.to_tokens(tokens),
                Self::HasStack(t) => t.to_tokens(tokens),
            }
        }
    }

    impl syn::parse::Parse for PushStateFlagsKw {
        fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
            Ok(if input.peek(builder) {
                PushStateFlagsKw::Builder(input.parse()?)
            } else if input.peek(has_stack) {
                PushStateFlagsKw::HasStack(input.parse()?)
            } else {
                return Err(input.error("Expected flag"));
            })
        }
    }

    syn::custom_keyword!(exec);
    syn::custom_keyword!(builder_name);
    syn::custom_keyword!(instruction_name);

    pub enum StackMarkerFlagsKw {
        Exec(exec),
        BuilderName(builder_name, syn::Token![=], syn::Ident),
        InstructionName(instruction_name, syn::Token![=], syn::Path),
    }

    impl quote::ToTokens for StackMarkerFlagsKw {
        fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
            match self {
                Self::Exec(v) => v.to_tokens(tokens),
                Self::BuilderName(v, w, x) => {
                    v.to_tokens(tokens);
                    w.to_tokens(tokens);
                    x.to_tokens(tokens);
                }
                Self::InstructionName(v, w, x) => {
                    v.to_tokens(tokens);
                    w.to_tokens(tokens);
                    x.to_tokens(tokens);
                }
            }
        }
    }

    impl syn::parse::Parse for StackMarkerFlagsKw {
        fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
            Ok(if input.peek(exec) {
                StackMarkerFlagsKw::Exec(input.parse()?)
            } else if input.peek(builder_name) {
                StackMarkerFlagsKw::BuilderName(input.parse()?, input.parse()?, input.parse()?)
            } else if input.peek(instruction_name) {
                StackMarkerFlagsKw::InstructionName(input.parse()?, input.parse()?, input.parse()?)
            } else {
                return Err(input.error("Expected flag"));
            })
        }
    }
}

impl syn::parse::Parse for PushStateFlags {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let default_flags = PushStateFlags::default();
        let parsed_flags_list =
            Punctuated::<kw::FlagsValue<kw::PushStateFlagsKw>, Token![,]>::parse_terminated(input)?;

        let mut builder_flag_set = false;
        let mut has_stack_flag_set = false;

        let mut current_flags = PushStateFlags::default();
        for flag in parsed_flags_list {
            let (flag, set_to) = flag.seperate();

            match flag {
                kw::PushStateFlagsKw::Builder(_) if default_flags.builder == set_to => {
                    return Err(syn::Error::new_spanned(flag, "Redundant flag, this is set by default. Maybe you meant to use !flag to disable it?"));
                }
                kw::PushStateFlagsKw::Builder(_) if builder_flag_set => {
                    return Err(syn::Error::new_spanned(flag, "Flag already set."));
                }
                kw::PushStateFlagsKw::Builder(_) => {
                    builder_flag_set = true;
                    current_flags.builder = set_to;
                }
                kw::PushStateFlagsKw::HasStack(_) if default_flags.has_stack == set_to => {
                    return Err(syn::Error::new_spanned(flag, "Redundant flag, this is set by default. Maybe you meant to use !flag to disable it?"));
                }
                kw::PushStateFlagsKw::HasStack(_) if has_stack_flag_set => {
                    return Err(syn::Error::new_spanned(flag, "Flag already set."));
                }
                kw::PushStateFlagsKw::HasStack(_) => {
                    has_stack_flag_set = true;
                    current_flags.has_stack = set_to;
                }
            }
        }

        Ok(current_flags)
    }
}

impl syn::parse::Parse for StackMarkerFlags {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let parsed_flags_list =
            Punctuated::<StackMarkerFlagsKw, Token![,]>::parse_terminated(input)?;

        let mut exec_flag_set = false;
        let mut builder_name_flag_set = false;
        let mut instruction_name_flag_set = false;

        let mut current_flags = Self::default();
        for flag in parsed_flags_list {
            match flag {
                StackMarkerFlagsKw::Exec(_) if exec_flag_set => {
                    return Err(syn::Error::new_spanned(flag, "Flag already set."));
                }
                StackMarkerFlagsKw::Exec(_) => {
                    exec_flag_set = true;
                    current_flags.is_exec = true;
                }
                StackMarkerFlagsKw::BuilderName(_, _, _) if builder_name_flag_set => {
                    return Err(syn::Error::new_spanned(flag, "Property already set."));
                }
                StackMarkerFlagsKw::BuilderName(_, _, v) => {
                    builder_name_flag_set = true;
                    current_flags.builder_name = Some(v);
                }
                StackMarkerFlagsKw::InstructionName(_, _, _) if instruction_name_flag_set => {
                    return Err(syn::Error::new_spanned(flag, "Property already set."));
                }
                StackMarkerFlagsKw::InstructionName(_, _, v) => {
                    instruction_name_flag_set = true;
                    current_flags.instruction_name = Some(v);
                }
            }
        }

        Ok(current_flags)
    }
}

#[manyhow::manyhow(proc_macro_attribute)]
pub fn push_state(
    attrs: proc_macro2::TokenStream,
    tokens: proc_macro2::TokenStream,
) -> manyhow::Result {
    let macro_span = attrs.span();
    let PushStateFlags {
        builder: generate_builder,
        has_stack: derive_has_stack,
    } = syn::parse2(attrs)?;

    let mut struct_defn: DeriveInput = syn::parse2(tokens)?;

    let DeriveInput {
        data:
            syn::Data::Struct(syn::DataStruct {
                fields: syn::Fields::Named(syn::FieldsNamed { named: fields, .. }),
                ..
            }),
        ident: struct_ident,
        generics: struct_generics,
        vis: struct_visibility,
        ..
    } = &mut struct_defn
    else {
        return Err(syn::Error::new(
            macro_span,
            "This macro only supports struct with named fields.",
        )
        .into());
    };

    let mut stacks: BTreeMap<syn::Ident, (StackMarkerFlags, syn::Type)> = BTreeMap::new();
    let mut exec_stack: Option<(syn::Ident, StackMarkerFlags, syn::Type)> = None;

    let mut input_instructions: Option<syn::Ident> = None;

    for syn::Field {
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
                    )
                    .into());
                }
                if !matches!(to_compare.meta, syn::Meta::Path(_)) {
                    return Err(syn::Error::new_spanned(
                        to_compare,
                        "This attribute does not support any arguments",
                    )
                    .into());
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
                        )
                        .into());
                    }

                    let marker_flags: StackMarkerFlags = syn::parse2(l.tokens)?;
                    if marker_flags.is_exec {
                        if !generate_builder && !derive_has_stack {
                            return Err(syn::Error::new(attr_span, "Unknown flag exec. Maybe you meant to enable the builder or has_stack feature of the push_state macro?").into());
                        }
                        if stack_marker_flags.is_exec {
                            return Err(syn::Error::new(attr_span, "Redundant exec flag").into());
                        } else if stack_marker_flags.builder_name.is_some() {
                            return Err(syn::Error::new(
                                attr_span,
                                "Builder name cannot be set for exec stacks",
                            )
                            .into());
                        } else {
                            stack_marker_flags.is_exec = marker_flags.is_exec;
                        }
                    }
                    if let Some(builder_name) = marker_flags.builder_name {
                        if !generate_builder {
                            return Err(syn::Error::new(attr_span, "Unknown flag generate_builder. Maybe you meant to enable the builder feature of the push_state macro?").into());
                        }
                        if stack_marker_flags.builder_name.is_some() {
                            return Err(syn::Error::new(
                                attr_span,
                                "Builder name already set explicitly",
                            )
                            .into());
                        } else if stack_marker_flags.is_exec {
                            return Err(syn::Error::new(
                                attr_span,
                                "Builder name cannot be set for exec stacks",
                            )
                            .into());
                        } else {
                            stack_marker_flags.builder_name = Some(builder_name);
                        }
                    }
                }
                syn::Meta::NameValue(n) => {
                    return Err(syn::Error::new_spanned(n, "This kind of attribute meta is not supported, did you mean to use a list? (#[stack(builder_name = ...)])").into());
                }
            }
        }

        if let Some(empty_attr) = empty_attr {
            if nonempty_attr {
                return Err(syn::Error::new_spanned(empty_attr, "Redundant attribute").into());
            }
        }

        if stack_marker_flags.is_exec {
            if exec_stack.is_some() {
                return Err(syn::Error::new_spanned(ident, "Only one exec stack supported").into());
            } else {
                exec_stack = Some((ident, stack_marker_flags, ty.clone()))
            }
        } else {
            stacks.insert(ident, (stack_marker_flags, ty.clone()));
        }
    }

    let has_stack_derives = derive_has_stack.then(|| {
        // let mut stacks_to_derive_for = stacks
        let stacks_to_derive_for = stacks
            .iter()
            .map(|(ident, (_, ty))| (ident, ty))
            .collect::<Vec<_>>();
        // if let Some((ident, _, ty)) = &exec_stack {
        //     stacks_to_derive_for.push((ident, ty));
        // }

        stacks_to_derive_for
            .into_iter()
            .map(|(ident, ty)| {
                quote! {
                    #[automatically_derived]
                    impl ::push::push_vm::stack::HasStack<<#ty as ::push::push_vm::stack::StackType>::Type> for #struct_ident {
                        fn stack<U: ::push::push_vm::stack::TypeEq<This = <#ty as ::push::push_vm::stack::StackType>::Type>>(&self) -> &#ty {
                            &self.#ident
                        }

                        fn stack_mut<U: ::push::push_vm::stack::TypeEq<This = <#ty as ::push::push_vm::stack::StackType>::Type>>(&mut self) -> &mut #ty {
                            &mut self.#ident
                        }
                    }
                }
            })
            .collect::<proc_macro2::TokenStream>()
    });

    let builder = generate_builder
        .then(|| -> manyhow::Result {
            let Some((exec_stack_ident, _, _)) = exec_stack else {
                return Err(syn::Error::new(macro_span, "Need to declare exactly one exec stack using #[stack(exec)] to use the builder feature.").into())
            };

            let struct_ident_unrawed = struct_ident.unraw();
            let struct_ident_unrawed_snake_case = struct_ident_unrawed.to_snake_case().unraw();
            let utilities_mod_ident = syn::Ident::new_raw(&format!("{struct_ident_unrawed_snake_case}_builder"), proc_macro2::Span::mixed_site());

            let builder_name = syn::Ident::new_raw(&format!("{struct_ident_unrawed}Builder"), proc_macro2::Span::mixed_site());

            let stack_generics = stacks.keys().map(|i| i.unraw().to_pascal_case_spanned(proc_macro2::Span::mixed_site())).collect::<Vec<_>>();

            let stack_generics_with_state_bounds = stack_generics.iter().map(|g| quote!{#g: #utilities_mod_ident::StackState}).collect::<Vec<_>>();

            let stack_generics_with_dataless_bounds = stack_generics.iter().map(|g| quote!{#g: #utilities_mod_ident::Dataless}).collect::<Vec<_>>();

            let default_states = stack_generics.iter().map(|_| quote!{()}).collect::<Vec<_>>();

            let with_size_repeated = stack_generics.iter().map(|_| quote!{
                #utilities_mod_ident::WithSize
            }).collect::<Vec<_>>();

            let (impl_generics, type_generics, where_clause) = struct_generics.split_for_impl();

            let fields = stacks.keys().collect::<Vec<_>>();

            let with_inputs_impl = input_instructions.map(|input_instructions_field| {
                let with_inputs = stacks.iter().map(|(field, (StackMarkerFlags{builder_name, instruction_name, ..}, ty))| {
                    let stack_ident = builder_name.as_ref().unwrap_or(field).unraw().to_snake_case().unraw();
                    let instruction_path = instruction_name.clone().unwrap_or_else(|| {
                        let snake_case_field = field.unraw().to_snake_case().unraw();

                        let instruction_fn_name = syn::Ident::new(&format!("push_{snake_case_field}"), proc_macro2::Span::mixed_site());

                        syn::parse_quote!(::push::instruction::PushInstruction::#instruction_fn_name)
                    });

                    let fn_ident = syn::Ident::new(&format!("with_{stack_ident}_input"), proc_macro2::Span::mixed_site());

                    quote!{
                        /// Adds a input instruction to the current current state's set
                        /// of instructions. The name for the input must have been included
                        /// in the `Inputs` provided when the `Builder` was initially constructed.
                        /// Here you provide the name and the boolean value for that
                        /// input variable. That will create a new `PushInstruction::push_[type]()`
                        /// instruction that will push the specified value onto the stack
                        /// when performed.
                        ///
                        /// # Panics
                        /// This panics if the `input_name` provided isn't included in the set of
                        /// names in the `Inputs` object used in the construction of the `Builder`.
                        #[must_use]
                        pub fn #fn_ident(mut self, input_name: &str, input_value: <#ty as ::push::push_vm::stack::StackType>::Type) -> Self {
                            self.partial_state.#input_instructions_field.insert(
                                ::push::instruction::VariableName::from(input_name),
                                #instruction_path(input_value),
                            );
                            self
                        }
                    }
                });

                quote!{
                    impl<__Exec: #utilities_mod_ident::StackState, #(#stack_generics_with_state_bounds),*> #builder_name<__Exec, #(#stack_generics),*> {
                        #(#with_inputs)*
                    }
                }
            });

            let with_values_impl = stacks.iter().map(|(field, (StackMarkerFlags { builder_name: builder_methods_name, .. }, ty))| {
                let stack_ident = builder_methods_name.as_ref().unwrap_or(field).unraw().to_snake_case().unraw();

                let fn_ident = syn::Ident::new(&format!("with_{stack_ident}_values"), proc_macro2::Span::mixed_site());

                let where_bounds = stacks.keys().map(|ident| {
                    let generic_name = ident.unraw().to_pascal_case_spanned(proc_macro2::Span::mixed_site()).unraw();
                    if ident == field {
                        quote!{#generic_name: #utilities_mod_ident::SizeSet}
                    } else {
                        quote!{#generic_name: #utilities_mod_ident::StackState}
                    }
                });

                let stack_generics_or_type = stacks.keys().map(|ident| {
                    if ident == field {
                        quote!{#utilities_mod_ident::WithSizeAndData}
                    } else {
                        let generic_name = ident.unraw().to_pascal_case_spanned(proc_macro2::Span::mixed_site()).unraw();
                        quote!{#generic_name}
                    }
                    
                });

                quote!{
                    impl<__Exec: #utilities_mod_ident::StackState, #(#where_bounds),*> #builder_name<__Exec, #(#stack_generics),*> {
                        /// Adds the given sequence of values to the stack for the state you're building.
                        ///
                        /// The first value in `values` will be the new top of the
                        /// stack. If the stack was initially empty, the last value
                        /// in `values` will be the new bottom of the stack.
                        ///
                        /// # Arguments
                        ///
                        /// * `values` - A `Vec` holding the values to add to the stack
                        ///
                        /// # Examples
                        ///
                        /// ```ignore
                        /// use push::push_vm::push_state::{ Stack, HasStack, PushState, Builder };
                        /// let mut state = Builder::new(PushState::default())
                        ///     .with_int_values(vec![5, 8, 9])
                        ///     .build();
                        /// let int_stack: &Stack<PushInteger> = state.stack();
                        /// assert_eq!(int_stack.size(), 3);
                        /// // Now the top of the stack is 5, followed by 8, then 9 at the bottom.
                        /// assert_eq!(int_stack.top().unwrap(), &5);
                        /// ```
                        #[must_use]
                        pub fn #fn_ident(mut self, values: Vec<<#ty as ::push::push_vm::stack::StackType>::Type>) -> #builder_name<__Exec, #(#stack_generics_or_type),*> {
                            self.partial_state.#field.extend(values);

                            #builder_name {
                                partial_state: self.partial_state,
                                _p: ::std::marker::PhantomData,
                            }
                        }
                    }
                }
            }).collect::<proc_macro2::TokenStream>();

            let set_max_size_impl = stacks.iter().map(|(field, (StackMarkerFlags { builder_name: builder_methods_name, .. }, _))| {
                let stack_ident = builder_methods_name.as_ref().unwrap_or(field).unraw().to_snake_case().unraw();

                let fn_ident = syn::Ident::new(&format!("with_{stack_ident}_max_size"), proc_macro2::Span::mixed_site());

                let where_bounds = stacks.keys().map(|ident| {
                    let generic_name = ident.unraw().to_pascal_case_spanned(proc_macro2::Span::mixed_site()).unraw();
                    if ident == field {
                        quote!{#generic_name: #utilities_mod_ident::Dataless}
                    } else {
                        quote!{#generic_name: #utilities_mod_ident::StackState}
                    }
                });

                let stack_generics_or_type = stacks.keys().map(|ident| {
                    if ident == field {
                        quote!{#utilities_mod_ident::WithSize}
                    } else {
                        let generic_name = ident.unraw().to_pascal_case_spanned(proc_macro2::Span::mixed_site()).unraw();
                        quote!{#generic_name}
                    }
                    
                });

                quote!{
                    impl<__Exec: #utilities_mod_ident::StackState, #(#where_bounds),*> #builder_name<__Exec, #(#stack_generics),*> {
                        /// Sets the maximum stack size for the stack in this state.
                        ///
                        /// # Arguments
                        ///
                        /// * `max_stack_size` - A `usize` specifying the maximum stack size
                        #[must_use]
                        pub fn #fn_ident(mut self, max_stack_size: usize) ->#builder_name<__Exec, #(#stack_generics_or_type),*>  {
                            self.partial_state.#field.set_max_stack_size(max_stack_size);

                            #builder_name {
                                partial_state: self.partial_state,
                                _p: ::std::marker::PhantomData,
                            }
                        }
                    }
                }
            }).collect::<proc_macro2::TokenStream>();


            Ok(quote!{
                impl #impl_generics #struct_ident #type_generics #where_clause {
                    #[must_use]
                    #struct_visibility fn builder() -> #builder_name<(),#(#default_states),*>{
                        #builder_name::<(),#(#default_states),*>::default()
                    }
                }
                
                #struct_visibility mod #utilities_mod_ident {
                    mod sealed {
                        pub trait SealedMarker {}
                    }

                    pub trait StackState: sealed::SealedMarker {}
                    pub trait Dataless: StackState {}
                    pub trait SizeSet: StackState {}

                    impl sealed::SealedMarker for () {}
                    impl StackState for () {}
                    impl Dataless for () {}

                    pub struct WithSize;
                    impl sealed::SealedMarker for WithSize {}
                    impl StackState for WithSize {}
                    impl Dataless for WithSize {}
                    impl SizeSet for WithSize {}

                    pub struct WithSizeAndData;
                    impl sealed::SealedMarker for WithSizeAndData {}
                    impl StackState for WithSizeAndData {}
                    impl SizeSet for WithSizeAndData {}
                }

                #struct_visibility struct #builder_name<__Exec: #utilities_mod_ident::StackState, #(#stack_generics_with_state_bounds),*> {
                    partial_state: #struct_ident,
                    _p: std::marker::PhantomData<(__Exec, #(#stack_generics),*)>
                }

                impl ::std::default::Default for #builder_name<(), #(#default_states),*> {
                    fn default() -> Self {
                        #builder_name {
                            partial_state: ::std::default::Default::default(),
                            _p: ::std::marker::PhantomData,
                        }
                    }
                }

                impl<__Exec: #utilities_mod_ident::Dataless, #(#stack_generics_with_dataless_bounds),*> #builder_name<__Exec, #(#stack_generics),*> {
                    /// Sets the maximum stack size for all the stacks in this state.
                    ///
                    /// # Arguments
                    ///
                    /// * `max_stack_size` - A `usize` specifying the maximum stack size
                    ///
                    /// # Examples
                    ///
                    /// ```ignore
                    /// use push::push_vm::HasStack;
                    /// use push::push_vm::push_state::{ Stack, HasStack, PushState, Builder };
                    /// let mut state = Builder::new(PushState::default())
                    ///     .with_max_stack_size(100)
                    ///     .build();
                    /// let bool_stack: &Stack<bool> = state.stack();
                    /// assert_eq!(bool_stack.max_stack_size, 100);
                    /// ```
                    #[must_use]
                    pub fn with_max_stack_size(
                        mut self,
                        max_size: usize,
                    ) -> #builder_name<#utilities_mod_ident::WithSize, #(#with_size_repeated),*> {
                        self.partial_state
                            .exec
                            .reserve(max_size - self.partial_state.exec().len());

                        #(
                            self.partial_state.#fields.set_max_stack_size(max_size);
                        )*

                        #builder_name {
                            partial_state: self.partial_state,
                            _p: ::std::marker::PhantomData,
                        }
                    }
                }

                impl<#(#stack_generics_with_state_bounds),*> #builder_name<#utilities_mod_ident::WithSize, #(#stack_generics),*> {
                    /// Sets the program you wish to execute.
                    /// Note that the program will be executed in ascending order.
                    ///
                    /// # Arguments
                    /// - `program` - The program you wish to execute
                    #[must_use]
                    pub fn with_program<P>(mut self, program: P) -> #builder_name<#utilities_mod_ident::WithSizeAndData, #(#stack_generics),*>
                    where
                        P: ::std::iter::IntoIterator<Item = ::push::instruction::PushInstruction>,
                        <P as ::std::iter::IntoIterator>::IntoIter: ::std::iter::DoubleEndedIterator,
                    {
                        self.partial_state.#exec_stack_ident =program.into_iter().rev().collect();
                        #builder_name {
                            partial_state: self.partial_state,
                            _p: ::std::marker::PhantomData,
                        }
                    }
                }

                impl<#(#stack_generics_with_state_bounds),*> #builder_name<#utilities_mod_ident::WithSizeAndData, #(#stack_generics),*> {
                    /// Finalize the build process, returning the fully constructed `PushState`
                    /// value. For this to successfully build, all the input variables has to
                    /// have been given values. Thus every input variable provided
                    /// in the `Inputs` used when constructing the `Builder` must have had a
                    /// corresponding `with_X_input()` call that specified the value for that
                    /// variable.
                    ///
                    /// # Panics
                    /// Panics if one or more of the variables provided in the `Inputs` wasn't
                    /// then given a value during the build process.
                    /*
                     * Note that the `with_x_input()` functions ensure that the instruction for
                     * that input variable will be in the same position in `self.input_instructions`
                     * as the name is in `self.inputs.input_names`. This allows us to zip together
                     * those two lists and know that we'll be pairing up instructions with the appropriate
                     * names.
                     */
                    #[must_use]
                    pub fn build(self) -> #struct_ident {
                        self.partial_state
                    }
                }

                #with_inputs_impl
                #with_values_impl
                #set_max_size_impl

            })

        })
        .transpose()?;

    Ok(quote! {
        #struct_defn
        #has_stack_derives
        #builder
    })
}
