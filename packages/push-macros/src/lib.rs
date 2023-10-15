#![warn(clippy::nursery)]
#![warn(clippy::pedantic)]

mod parse;
mod splice_iter;

use manyhow::manyhow;
use proc_macro2::{Span, TokenStream as TokenStream2};
use proc_macro_assertions::{prelude::ProvideBounds, store::DefaultStore};
use quote::quote;
use syn::GenericArgument;

use crate::parse::{StackWrapperDeclaration, WrapperTransactionArgs};

#[manyhow(proc_macro)]
pub fn wrapper_transaction(input_tokens: TokenStream2) -> manyhow::Result<TokenStream2> {
    let default_readonly_wrapper =
        syn::parse_quote!(::push::push_vm::stack::wrappers::readonly::ReadOnly);

    let args: WrapperTransactionArgs = syn::parse2(input_tokens)?;
    let WrapperTransactionArgs {
        state,
        stacks,
        struct_ident,
    } = &args;

    let state_var_ident = syn::Ident::new("state_val", Span::mixed_site());
    let mut assertion_store = DefaultStore::new();

    assertion_store.assert(args.provide_bounds(&state_var_ident));

    let stacks_decl = stacks.iter().enumerate().map(|(i, stack_wrapper_decl)| {
        let StackWrapperDeclaration {
            stack,
            wrapper_type,
            ..
        } = stack_wrapper_decl;
        let wrapper_type = wrapper_type.as_ref().unwrap_or(&default_readonly_wrapper);
        let stack_ident = syn::Ident::new(&format!("s_{i}"), Span::mixed_site());

        let binding = GenericArgument::Type(stack.clone());
        let arguments = stack_wrapper_decl.generic_arguments_with_type(&binding);

        quote! {
            #stack_ident: #wrapper_type<#(#arguments),*>
        }
    });

    let trait_impls = stacks.iter().enumerate().map(|(i, stack_wrapper_decl)| {
        let StackWrapperDeclaration {
            mutable,
            stack,
            wrapper_type,
            ..
        } = stack_wrapper_decl;
        let wrapper_type = wrapper_type.as_ref().unwrap_or(&default_readonly_wrapper);
        let stack_ident = syn::Ident::new(&format!("s_{i}"), Span::mixed_site());

        let mutable_impl = mutable.then(|| {
            quote! {
                impl HasWrapperMut<#stack> for #struct_ident {
                    fn get_mut(&mut self) -> &mut Option<Self::Wrapper> {
                        &mut self.#stack_ident
                    }
                }
            }
        });

        let binding = GenericArgument::Type(syn::parse_quote!(Self));
        let arguments = stack_wrapper_decl.generic_arguments_with_type(&binding);

        quote! {
            impl HasWrapper<#stack> for #struct_ident {
                type Wrapper = #wrapper_type<#(#arguments),*>;

                fn get(&self) -> &mut Option<Self::Wrapper> {
                    &self.#stack_ident
                }
            }

            #mutable_impl
        }
    });

    // Add imports for asserts if required
    if !stacks.is_empty() {
        assertion_store.add_extra_items(quote! {
            use ::push::push_vm::stack::traits::has_stack::HasStack;
        });
    }

    if stacks
        .iter()
        .any(|StackWrapperDeclaration { mutable, .. }| *mutable)
    {
        assertion_store.add_extra_items(quote! {
            use ::push::push_vm::stack::traits::has_stack::HasStackMut;
            use ::push::error::into_state::StateMut;
        });
    }

    assertion_store.add_extra_items(quote! {
        use ::push::error::into_state::State;
    });

    Ok(quote! {
        {
            // This is needed as the rvalue could in theory be any expression here
            let #state_var_ident = #state;

            struct #struct_ident {
                #(#stacks_decl),*
            }

            #(#trait_impls)*
            #assertion_store

            #state.new_transaction::<#struct_ident>()

        }
    })
}
