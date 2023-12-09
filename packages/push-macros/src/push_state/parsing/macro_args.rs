use quote::ToTokens;
use syn::{parse::Parse, punctuated::Punctuated, Token};

use super::FlagsValue;

pub struct PushStateFlags {
    pub builder: bool,
    pub has_stack: bool,
}

impl Default for PushStateFlags {
    fn default() -> Self {
        Self {
            builder: false,
            has_stack: true,
        }
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

impl Parse for PushStateFlagsKw {
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

impl Parse for PushStateFlags {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let default_flags = PushStateFlags::default();
        let parsed_flags_list =
            Punctuated::<FlagsValue<PushStateFlagsKw>, Token![,]>::parse_terminated(input)?;

        let mut builder_flag_set = false;
        let mut has_stack_flag_set = false;

        let mut current_flags = PushStateFlags::default();
        for flag in parsed_flags_list {
            let (flag, set_to) = flag.seperate();

            match flag {
                PushStateFlagsKw::Builder(_) if default_flags.builder == set_to => {
                    return Err(syn::Error::new_spanned(flag, "Redundant flag, this is set by default. Maybe you meant to use !flag to disable it?"));
                }
                PushStateFlagsKw::Builder(_) if builder_flag_set => {
                    return Err(syn::Error::new_spanned(flag, "Flag already set."));
                }
                PushStateFlagsKw::Builder(_) => {
                    builder_flag_set = true;
                    current_flags.builder = set_to;
                }
                PushStateFlagsKw::HasStack(_) if default_flags.has_stack == set_to => {
                    return Err(syn::Error::new_spanned(flag, "Redundant flag, this is set by default. Maybe you meant to use !flag to disable it?"));
                }
                PushStateFlagsKw::HasStack(_) if has_stack_flag_set => {
                    return Err(syn::Error::new_spanned(flag, "Flag already set."));
                }
                PushStateFlagsKw::HasStack(_) => {
                    has_stack_flag_set = true;
                    current_flags.has_stack = set_to;
                }
            }
        }

        Ok(current_flags)
    }
}
