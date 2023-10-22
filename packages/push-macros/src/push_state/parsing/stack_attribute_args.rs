use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Ident, Path, Token,
};

#[derive(Default)]
pub struct StackMarkerFlags {
    pub builder_name: Option<Ident>,
    pub instruction_name: Option<Path>,
    pub is_exec: bool,
}

syn::custom_keyword!(exec);
syn::custom_keyword!(builder_name);
syn::custom_keyword!(instruction_name);

pub enum StackMarkerFlagsKw {
    Exec(exec),
    BuilderName(builder_name, Token![=], Ident),
    InstructionName(instruction_name, Token![=], Path),
}

impl ToTokens for StackMarkerFlagsKw {
    fn to_tokens(&self, tokens: &mut TokenStream) {
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

impl Parse for StackMarkerFlagsKw {
    fn parse(input: ParseStream) -> syn::Result<Self> {
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

impl Parse for StackMarkerFlags {
    fn parse(input: ParseStream) -> syn::Result<Self> {
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
