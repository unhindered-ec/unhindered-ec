use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::push_state::parsing::{ExecStackInput, StacksInput};

pub fn derive_has_stack(
    struct_ident: &Ident,
    stacks: &StacksInput,
    exec_stack: &ExecStackInput,
) -> TokenStream {
    stacks
        .iter()
        .map(|(ident, (_, ty))| (ident, ty))
        .chain(exec_stack.as_ref().map(|(ident, _, ty)| (ident, ty)))
        .map(|(ident, ty)| {
            quote! {
                #[automatically_derived]
                impl
                    ::push::push_vm::stack::HasStack<
                        <#ty as ::push::push_vm::stack::StackType>::Type
                    >
                for
                    #struct_ident
                {
                    fn stack<
                        U: ::push::push_vm::stack::TypeEq<
                            This = <#ty as ::push::push_vm::stack::StackType>::Type>
                    >(&self) -> &#ty {
                        &self.#ident
                    }

                    fn stack_mut<
                        U: ::push::push_vm::stack::TypeEq<
                            This = <#ty as ::push::push_vm::stack::StackType>::Type>
                    >(&mut self) -> &mut #ty {
                        &mut self.#ident
                    }
                }
            }
        })
        .collect::<proc_macro2::TokenStream>()
}
