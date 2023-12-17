use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{
    bracketed,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    spanned::Spanned,
    Expr, Ident, Path, Token,
};

use super::spanned_value::SpannedValue;

#[derive(Default)]
pub struct StackMarkerFlags {
    pub builder_name: SpannedValue<Option<Ident>>,
    pub instruction_name: SpannedValue<Option<Path>>,
    pub is_exec: SpannedValue<bool>,
    pub sample_values: SpannedValue<Option<Punctuated<Expr, Token![,]>>>,
    pub ignore_doctests: SpannedValue<bool>,
}

syn::custom_keyword!(exec);
syn::custom_keyword!(builder_name);
syn::custom_keyword!(instruction_name);
syn::custom_keyword!(sample_values);
syn::custom_keyword!(ignore_doctests);

/// Any option passed to a field inside a struct to be used inside the macro,
/// for example the `exec` in #[stack(exec)]
pub enum StackFieldOption {
    /// `exec` option, this determines which stack is the exec stack
    Exec(exec),
    /// `ignore_doctests` option, this forces the doctests outputed for
    /// this stack to be annotated with `ignore`.
    IgnoreDoctests(ignore_doctests),
    /// `builder_name = name` option,
    /// this can be used to change the name of the stack as it is used inside
    /// the builder, like the `int` in `with_int_values`
    /// (or if you set the builder name to `number` then
    BuilderName(builder_name, Token![=], Ident),
    /// `instruction_name = some::path` option,
    /// this can be used to change the instruction that is used
    /// to set input values, for example in the `with_int_input` method
    InstructionName(instruction_name, Token![=], Path),
    /// `sample_values = comma, seperated, values` option, those are sample
    /// values used within doctests for example
    SampleValues(
        sample_values,
        Token![=],
        syn::token::Bracket,
        Punctuated<Expr, Token![,]>,
    ),
}

impl ToTokens for StackFieldOption {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Exec(v) => v.to_tokens(tokens),
            Self::IgnoreDoctests(v) => v.to_tokens(tokens),
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
            Self::SampleValues(v, w, b, y) => {
                v.to_tokens(tokens);
                w.to_tokens(tokens);
                b.surround(tokens, |t| y.to_tokens(t));
                y.to_tokens(tokens);
            }
        }
    }
}

impl Parse for StackFieldOption {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(if input.peek(exec) {
            StackFieldOption::Exec(input.parse()?)
        } else if input.peek(ignore_doctests) {
            StackFieldOption::IgnoreDoctests(input.parse()?)
        } else if input.peek(builder_name) {
            StackFieldOption::BuilderName(input.parse()?, input.parse()?, input.parse()?)
        } else if input.peek(instruction_name) {
            StackFieldOption::InstructionName(input.parse()?, input.parse()?, input.parse()?)
        } else if input.peek(sample_values) {
            let kw = input.parse()?;
            let eq_sign = input.parse()?;
            let stream;
            let bracket_token = bracketed!(stream in input);
            let exprs = Punctuated::<Expr, Token![,]>::parse_terminated(&stream)?;

            StackFieldOption::SampleValues(kw, eq_sign, bracket_token, exprs)
        } else {
            return Err(input.error("Expected flag"));
        })
    }
}

// #[stack(exec, builder_name = something, sample_values = [4, 2, 1])]

impl Parse for StackMarkerFlags {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let parsed_flags_list = Punctuated::<StackFieldOption, Token![,]>::parse_terminated(input)?;

        let mut exec_flag_set = false;
        let mut ignore_doctests_flag_set = false;
        let mut builder_name_flag_set = false;
        let mut instruction_name_flag_set = false;
        let mut sample_values_flag_set = false;

        let mut current_flags = Self::default();
        for flag in parsed_flags_list {
            let flag_span = flag.span();
            match flag {
                StackFieldOption::Exec(_) if exec_flag_set => {
                    return Err(syn::Error::new_spanned(flag, "Flag already set."));
                }
                StackFieldOption::Exec(_) => {
                    exec_flag_set = true;
                    current_flags.is_exec = SpannedValue {
                        value: true,
                        span: flag_span,
                    };
                }
                StackFieldOption::IgnoreDoctests(_) if ignore_doctests_flag_set => {
                    return Err(syn::Error::new_spanned(flag, "Flag already set."));
                }
                StackFieldOption::IgnoreDoctests(_) => {
                    ignore_doctests_flag_set = true;
                    current_flags.ignore_doctests = SpannedValue {
                        value: true,
                        span: flag_span,
                    };
                }
                StackFieldOption::BuilderName(_, _, _) if builder_name_flag_set => {
                    return Err(syn::Error::new_spanned(flag, "Property already set."));
                }
                StackFieldOption::BuilderName(_, _, v) => {
                    builder_name_flag_set = true;
                    current_flags.builder_name = SpannedValue {
                        value: Some(v),
                        span: flag_span,
                    };
                }
                StackFieldOption::InstructionName(_, _, _) if instruction_name_flag_set => {
                    return Err(syn::Error::new_spanned(flag, "Property already set."));
                }
                StackFieldOption::InstructionName(_, _, v) => {
                    instruction_name_flag_set = true;
                    current_flags.instruction_name = SpannedValue {
                        value: Some(v),
                        span: flag_span,
                    };
                }
                StackFieldOption::SampleValues(_, _, _, _) if sample_values_flag_set => {
                    return Err(syn::Error::new_spanned(flag, "Property already set."));
                }
                StackFieldOption::SampleValues(_, _, _, v) => {
                    if v.is_empty() {
                        return Err(syn::Error::new(
                            flag_span,
                            "Expected at least one sample value!",
                        ));
                    }
                    sample_values_flag_set = true;
                    current_flags.sample_values = SpannedValue {
                        value: Some(v),
                        span: flag_span,
                    }
                }
            }
        }

        Ok(current_flags)
    }
}
