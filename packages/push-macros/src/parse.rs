use proc_macro_assertions::{
    generatable::StaticTid,
    maybe_borrowed::FromMaybeBorrowed,
    prelude::{
        Assert, AssertableWithBounds, InsertIntoTemplate, ProvideBounds, ResolvedBounds, Trait,
    },
    raw_assert::iter::AssertCollection,
};
use syn::{punctuated::Punctuated, GenericArgument, Token};

use crate::splice_iter::SpliceOne;

pub struct StackWrapperDeclaration {
    pub mutable: bool,
    pub stack: syn::Type,
    pub wrapper_type: Option<syn::Path>,
    pub arguments: Option<Punctuated<syn::GenericArgument, Token![,]>>,
}

impl StackWrapperDeclaration {
    pub fn generic_arguments_with_type<'a>(
        &'a self,
        ty: &'a GenericArgument,
    ) -> impl Iterator<Item = &'a syn::GenericArgument> {
        self.arguments
            .as_ref()
            .map(|arguments| arguments.iter())
            .into_iter()
            .flatten()
            .splice_one_with(
                |next| !matches!(next, Some(GenericArgument::Lifetime(_))),
                || ty,
            )
    }
}

impl syn::parse::Parse for StackWrapperDeclaration {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mutable = input.peek(Token![mut]);
        if mutable {
            input.parse::<Token![mut]>()?;
        }
        let stack = input.parse()?;
        let has_wrapper_specified = input.peek(Token![:]);
        if has_wrapper_specified {
            input.parse::<Token![:]>()?;
        }

        if mutable && !has_wrapper_specified {
            return Err(syn::Error::new_spanned(
                stack,
                "Expected wrapper type to be specified",
            ));
        }

        let wrapper_type = has_wrapper_specified.then(|| input.parse()).transpose()?;

        let arguments = has_wrapper_specified
            .then(|| {
                input
                    .peek(Token![<])
                    .then(|| {
                        input.parse::<Token![<]>()?;
                        let arguments =
                    Punctuated::<syn::GenericArgument, Token![,]>::parse_separated_nonempty(input)?;
                        input.parse::<Token![>]>()?;
                        Ok::<_, syn::Error>(arguments)
                    })
                    .transpose()
            })
            .transpose()?
            .flatten();

        Ok(Self {
            mutable,
            stack,
            wrapper_type,
            arguments,
        })
    }
}

impl<'a> AssertableWithBounds<'a, &'a syn::Ident> for StackWrapperDeclaration {
    type Output = (
        Assert<'a, Trait<'a>, StaticTid<syn::Ident>>,
        Option<Assert<'a, Trait<'a>, StaticTid<syn::Ident>>>,
    );

    fn do_assert(&self, bounds: &'a syn::Ident) -> Self::Output {
        let stack_ty = &self.stack;
        let stack_assert = Trait::from_owned(syn::parse_quote!(HasStack<#stack_ty>))
            .test::<StaticTid<syn::Ident>>(bounds);
        let stack_mut_assert = self.mutable.then(|| {
            Trait::from_owned(syn::parse_quote!(HasStackMut<#stack_ty>))
                .test::<StaticTid<syn::Ident>>(bounds)
        });

        (stack_assert, stack_mut_assert)
    }
}
pub struct WrapperTransactionArgs {
    pub state: syn::Expr,
    pub stacks: Punctuated<StackWrapperDeclaration, Token![,]>,
    pub struct_ident: syn::Ident,
}

type CollectionIter<'a> =
    Box<dyn Iterator<Item = ResolvedBounds<'a, &'a syn::Ident, &'a StackWrapperDeclaration>> + 'a>;

impl<'a> AssertableWithBounds<'a, &'a syn::Ident> for &'a WrapperTransactionArgs {
    // This would be a whole bunch easer if return positon impl T in traits was a thing
    type Output = (
        AssertCollection<'a, CollectionIter<'a>>,
        Assert<'a, Trait<'a>, StaticTid<syn::Ident>>,
        Option<Assert<'a, Trait<'a>, StaticTid<syn::Ident>>>,
    );

    fn do_assert(&self, bounds: &'a syn::Ident) -> Self::Output {
        let boxed_iter: CollectionIter<'a> =
            Box::new(self.stacks.iter().map(|stack| stack.provide_bounds(bounds)));

        let state_assert =
            Trait::from_owned(syn::parse_quote!(State)).test::<StaticTid<syn::Ident>>(bounds);

        let any_mut = self.stacks.iter().any(|stack| stack.mutable);

        let state_mut_assert = any_mut.then(|| {
            Trait::from_owned(syn::parse_quote!(StateMut)).test::<StaticTid<syn::Ident>>(bounds)
        });

        (boxed_iter.into(), state_assert, state_mut_assert)
    }
}

mod kw {
    use proc_macro2::Span;
    use quote::ToTokens;

    syn::custom_keyword!(stacks);
    syn::custom_keyword!(struct_ident);

    pub enum Keywords {
        Stacks(stacks),
        StructIdent(struct_ident),
    }

    impl syn::parse::Parse for Keywords {
        fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
            if input.peek(stacks) {
                Ok(Self::Stacks(input.parse()?))
            } else if input.peek(struct_ident) {
                Ok(Self::StructIdent(input.parse()?))
            } else {
                Err(syn::Error::new(Span::mixed_site(), "no keyword found"))
            }
        }
    }

    impl ToTokens for Keywords {
        fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
            match self {
                Self::Stacks(s) => s.to_tokens(tokens),
                Self::StructIdent(s) => s.to_tokens(tokens),
            }
        }
    }
}

impl syn::parse::Parse for WrapperTransactionArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let state = input.parse()?;
        input.parse::<Token![,]>()?;
        let mut stacks = None;
        let mut struct_ident = None;

        while let Some(kw) = input.parse::<kw::Keywords>().map(Some).or_else(|e| {
            if e.to_string() == "no keyword found" {
                Ok(None)
            } else {
                Err(e)
            }
        })? {
            input.parse::<Token![=]>()?;
            match kw {
                kw::Keywords::Stacks(_) if stacks.is_none() => {
                    let stacks_tokens;
                    syn::braced!(stacks_tokens in input);

                    stacks = Some(
                        Punctuated::<StackWrapperDeclaration, Token![,]>::parse_terminated(
                            &stacks_tokens,
                        )?,
                    );
                }
                kw::Keywords::StructIdent(_) if struct_ident.is_none() => {
                    struct_ident = Some(input.parse()?);
                }
                kw => Err(syn::Error::new_spanned(kw, "Duplicate argument"))?,
            }
        }

        let Some(stacks) = stacks else {
            return Err(input.error("Expected `stacks` argument"));
        };

        Ok(Self {
            state,
            stacks,
            struct_ident: struct_ident.unwrap_or_else(|| {
                syn::Ident::new("WrapperTransaction", proc_macro2::Span::call_site())
            }),
        })
    }
}
