use ident_case_conversions::CaseConversions;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{ext::IdentExt, Generics, Ident, Visibility};

use crate::{
    doctest_tokenstream::{doctest, Import},
    push_state::parsing::{
        stack_attribute_args::StackMarkerFlags, ExecStackInput, InputInstructionsInput, StacksInput,
    },
};

macro_rules! derived_ident {
    ($($tok: expr),*) => {
        {
            #[allow(unused_imports)]
            use syn::ext::IdentExt;
            syn::Ident::new_raw(
                &[$(format!(
                    "{}",
                    derived_ident!(@handle_seprate $tok)
                )),*].concat(),
                proc_macro2::Span::mixed_site()
            )
        }
    };
    (@handle_seprate $lit: literal) => {
        $lit
    };
    (@handle_seprate $lit: expr) => {
        $lit.unraw()
    }
}

pub fn generate_builder(
    macro_span: Span,
    struct_ident: &Ident,
    struct_visibility: &Visibility,
    struct_generics: &Generics,
    stacks: &StacksInput,
    exec_stack: &ExecStackInput,
    input_instructions: InputInstructionsInput,
) -> syn::Result<TokenStream> {
    let Some((exec_stack_ident, _, _)) = exec_stack else {
        return Err(syn::Error::new(
            macro_span,
            "Need to declare exactly one exec stack using #[stack(exec)] to use the builder \
             feature.",
        ));
    };

    let utilities_mod_ident = struct_ident.unraw().to_snake_case();
    let import_utilities_path = syn::UseTree::Path(syn::UsePath {
        ident: utilities_mod_ident.clone(),
        colon2_token: syn::token::PathSep::default(),
        tree: Box::new(syn::UseTree::Path(syn::UsePath {
            ident: syn::parse_quote!(imports),
            colon2_token: syn::token::PathSep::default(),
            tree: Box::new(syn::UseTree::Glob(syn::UseGlob {
                star_token: syn::token::Star::default(),
            })),
        })),
    });

    let builder_name = derived_ident!(struct_ident, "Builder");

    let fields = stacks.keys().collect::<Vec<_>>();

    // Generic bounds for stacks, like `Int: StackState, Bool: StackState`
    let stack_generics = fields
        .iter()
        .map(|i| {
            i.unraw()
                .to_pascal_case_spanned(proc_macro2::Span::mixed_site())
        })
        .collect::<Vec<_>>();
    let stack_generics_with_state_bounds = stack_generics
        .iter()
        .map(|g| quote! {#g: #utilities_mod_ident::StackState})
        .collect::<Vec<_>>();
    let stack_generics_with_dataless_bounds = stack_generics
        .iter()
        .map(|g| quote! {#g: #utilities_mod_ident::Dataless})
        .collect::<Vec<_>>();

    // List with length |stacks| of ()
    let default_states = stack_generics
        .iter()
        .map(|_| quote! {()})
        .collect::<Vec<_>>();

    let with_size_repeated = stack_generics
        .iter()
        .map(|_| {
            quote! {
                #utilities_mod_ident::WithSize
            }
        })
        .collect::<Vec<_>>();

    let (impl_generics, type_generics, where_clause) = struct_generics.split_for_impl();

    let with_inputs_impl = input_instructions.map(|input_instructions_field| {
        let with_inputs = stacks.iter().map(
            |(
                field,
                (
                    StackMarkerFlags {
                        builder_name,
                        instruction_name,
                        ..
                    },
                    ty,
                ),
            )| {
                let stack_ident = builder_name
                    .as_ref()
                    .unwrap_or(field)
                    .unraw()
                    .to_snake_case();
                let instruction_path = instruction_name.as_ref().cloned().unwrap_or_else(|| {
                    let instruction_fn_name =
                        derived_ident!("push_", field.unraw().to_snake_case());
                    syn::parse_quote!(::push::instruction::PushInstruction::#instruction_fn_name)
                });

                let fn_ident = derived_ident!("with_", stack_ident, "_input");

                quote! {
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
                    pub fn #fn_ident(
                            mut self,
                            input_name: &str,
                            input_value: <#ty as ::push::push_vm::stack::StackType>::Type
                    ) -> Self {
                        self.partial_state.#input_instructions_field.insert(
                            ::push::instruction::variable_name::VariableName::from(input_name),
                            #instruction_path(input_value),
                        );
                        self
                    }
                }
            },
        );

        quote! {
            impl<__Exec: #utilities_mod_ident::StackState, #(#stack_generics_with_state_bounds),*>
                #builder_name<__Exec, #(#stack_generics),*>
            {
                #(#with_inputs)*
            }
        }
    });

    let with_values_impl = stacks
        .iter()
        .map(
            |(
                field,
                (
                    StackMarkerFlags {
                        builder_name: builder_methods_name,
                        sample_values,
                        ignore_doctests,
                        ..
                    },
                    ty,
                ),
            )| {
                let stack_ident = builder_methods_name
                    .as_ref()
                    .unwrap_or(field)
                    .unraw()
                    .to_snake_case()
                    .unraw();

                // Where bounds where the current stack is required to be SizeSet
                //  and every other stack can be in any state
                let where_bounds = stacks.keys().map(|ident| {
                    let generic_name = ident
                        .unraw()
                        .to_pascal_case_spanned(proc_macro2::Span::mixed_site());
                    if ident == field {
                        quote! {#generic_name: #utilities_mod_ident::SizeSet}
                    } else {
                        quote! {#generic_name: #utilities_mod_ident::StackState}
                    }
                });

                // Type list where the current stack is a certain value
                // and all others are the corresponding generics
                let stack_generics_or_type = stacks.keys().map(|ident| {
                    if ident == field {
                        quote! {#utilities_mod_ident::WithSizeAndData}
                    } else {
                        let generic_name = ident
                            .unraw()
                            .to_pascal_case_spanned(proc_macro2::Span::mixed_site());
                        quote! {#generic_name}
                    }
                });

                let fn_ident = derived_ident!("with_", stack_ident, "_values").unraw();
                let doctest_tokenstream = sample_values.as_ref().map(|sample_values| {
                    let max_stack_size = ((sample_values.len() / 100) + 1) * 100;
                    let number_values = sample_values.len();
                    let first_value = sample_values.iter().next().unwrap();
                    let imports = [
                        Import::Path(
                            Some(syn::token::PathSep::default()),
                            syn::parse_quote!(push::push_vm::{stack::{Stack, StackError, StackType}, HasStack}),
                        ),
                        Import::SuperRelativePath(import_utilities_path.clone()),
                    ];

                    let outtro = "# Ok::<(), StackError>(())";

                    let doctest_code = quote! {
                        let mut state = #struct_ident::builder()
                            .with_max_stack_size(#max_stack_size)
                            .with_no_program()
                            .#fn_ident([#sample_values])?
                            .build();
                        let int_stack: &Stack<<#ty as StackType>::Type> =
                            state.stack::<<#ty as StackType>::Type>();
                        assert_eq!(int_stack.size(), #number_values);
                        assert_eq!(int_stack.top()?, &#first_value);
                    };

                    let ignore_attr = (
                        (!matches!(struct_visibility, syn::Visibility::Public(_)))
                            || **ignore_doctests
                    )
                        .then_some("ignore");

                    doctest(Some(&imports), None::<&str>, doctest_code, Some(outtro), ignore_attr)
                });

                let example_section = sample_values.is_some().then_some(quote! {
                        ///
                        /// # Examples
                        ///
                        #doctest_tokenstream
                });

                quote! {
                    impl<
                        __Exec: #utilities_mod_ident::StackState,
                         #(#where_bounds),*
                    >
                        #builder_name<__Exec, #(#stack_generics),*>
                    {
                        /// Adds the given sequence of values to the
                        /// stack for the state you're building.
                        ///
                        /// The first value in `values` will be the new top of the
                        /// stack. If the stack was initially empty, the last value
                        /// in `values` will be the new bottom of the stack.
                        ///
                        /// # Arguments
                        ///
                        /// * `values` - A `Vec` holding the values to add to the stack
                        #example_section
                        #[must_use]
                        pub fn #fn_ident<T>(
                            mut self,
                            values: T
                        ) -> ::std::result::Result<
                            #builder_name<__Exec, #(#stack_generics_or_type),*>,
                            ::push::push_vm::stack::StackError
                        >
                        where
                            T: ::std::iter::IntoIterator<
                                Item = <#ty as ::push::push_vm::stack::StackType>::Type
                            >,
                            <T as ::std::iter::IntoIterator>::IntoIter:
                                ::std::iter::DoubleEndedIterator +
                                ::std::iter::ExactSizeIterator,
                        {
                            self.partial_state.#field.try_extend(values)?;

                            ::std::result::Result::Ok(#builder_name {
                                partial_state: self.partial_state,
                                _p: ::std::marker::PhantomData,
                            })
                        }
                    }
                }
            },
        )
        .collect::<proc_macro2::TokenStream>();

    let set_max_size_impl = stacks
        .iter()
        .map(
            |(
                field,
                (
                    StackMarkerFlags {
                        builder_name: builder_methods_name,
                        ..
                    },
                    _,
                ),
            )| {
                let stack_ident = builder_methods_name
                    .as_ref()
                    .unwrap_or(field)
                    .unraw()
                    .to_snake_case();

                let fn_ident = derived_ident!("with_", stack_ident, "_max_size");

                // Where bounds where the current stack is required
                // to be SizeSet and every other stack can be in any state
                let where_bounds = stacks.keys().map(|ident| {
                    let generic_name = ident
                        .unraw()
                        .to_pascal_case_spanned(proc_macro2::Span::mixed_site());

                    if ident == field {
                        quote! {#generic_name: #utilities_mod_ident::Dataless}
                    } else {
                        quote! {#generic_name: #utilities_mod_ident::StackState}
                    }
                });

                // Type list where the current stack is a certain
                // value and all others are the corresponding generics
                let stack_generics_or_type = stacks.keys().map(|ident| {
                    if ident == field {
                        quote! {#utilities_mod_ident::WithSize}
                    } else {
                        let generic_name = ident
                            .unraw()
                            .to_pascal_case_spanned(proc_macro2::Span::mixed_site());
                        quote! {#generic_name}
                    }
                });

                quote! {
                    impl<
                        __Exec: #utilities_mod_ident::StackState,
                        #(#where_bounds),*
                    >
                        #builder_name<__Exec, #(#stack_generics),*>
                    {
                        /// Sets the maximum stack size for the stack in this state.
                        ///
                        /// # Arguments
                        ///
                        /// * `max_stack_size` - A `usize` specifying the maximum stack size
                        #[must_use]
                        pub fn #fn_ident(
                            mut self,
                            max_stack_size: usize
                        ) -> #builder_name<__Exec, #(#stack_generics_or_type),*>  {
                            self.partial_state.#field.set_max_stack_size(max_stack_size);

                            #builder_name {
                                partial_state: self.partial_state,
                                _p: ::std::marker::PhantomData,
                            }
                        }
                    }
                }
            },
        )
        .collect::<proc_macro2::TokenStream>();

    let with_max_stack_size_examples = stacks
        .iter()
        .filter_map(|(_, (StackMarkerFlags { ignore_doctests, .. }, ty))|
            (!**ignore_doctests).then_some(ty)
        )
        .chain(exec_stack.iter().filter_map(|(_, StackMarkerFlags { ignore_doctests, .. }, ty)|
            (!**ignore_doctests).then_some(ty)
        ))
        .next()
        .map(|ty| (false, ty))
        .or_else(|| stacks
            .iter()
            .map(|(_, (_, ty))| ty)
            .chain(exec_stack.iter()
            .map(|(_, _, ty)| ty))
            .next()
            .map(|ty| (true, ty)))
        .map(|(ignore, ty)| {
            let imports = [
                Import::Path(
                    Some(syn::token::PathSep::default()),
                    syn::parse_quote!(push::push_vm::{stack::{Stack, StackError, StackType}, HasStack}),
                ),
                Import::SuperRelativePath(import_utilities_path.clone()),
            ];

            let outtro = "# Ok::<(), StackError>(())";

            let doctest_code = quote! {
                let mut state = #struct_ident::builder()
                    .with_max_stack_size(100)
                    .with_no_program()
                    .build();
                let stack: &Stack<<#ty as StackType>::Type> =
                    state.stack::<<#ty as StackType>::Type>();
                assert_eq!(stack.max_stack_size(), 100);
            };

            let ignore_attr = (
                (!matches!(struct_visibility, syn::Visibility::Public(_)))
                || ignore
            ).then_some("ignore");

            let doctest_tokenstream = doctest(
                Some(&imports),
                None::<&str>,
                doctest_code,
                Some(outtro),
                ignore_attr
            );

            quote! {
                    ///
                    /// # Examples
                    ///
                    #doctest_tokenstream
            }
        });

    Ok(quote! {
        impl #impl_generics #struct_ident #type_generics #where_clause {
            #[must_use]
            #struct_visibility fn builder() -> #builder_name<(),#(#default_states),*>{
                #builder_name::<(),#(#default_states),*>::default()
            }
        }

        #[doc(hidden)]
        #struct_visibility mod #utilities_mod_ident {
            #struct_visibility mod imports {
                pub use super::super::*;
            }

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

        #struct_visibility struct #builder_name<
            __Exec: #utilities_mod_ident::StackState,
            #(#stack_generics_with_state_bounds),*
        > {
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

        impl<
            __Exec: #utilities_mod_ident::Dataless,
            #(#stack_generics_with_dataless_bounds),*
        > #builder_name<__Exec, #(#stack_generics),*> {
            /// Sets the maximum stack size for all the stacks in this state.
            ///
            /// # Arguments
            ///
            /// * `max_stack_size` - A `usize` specifying the maximum stack size
            #with_max_stack_size_examples
            #[must_use]
            pub fn with_max_stack_size(
                mut self,
                max_size: usize,
            ) -> #builder_name<#utilities_mod_ident::WithSize, #(#with_size_repeated),*> {
                self.partial_state
                    .#exec_stack_ident
                    .set_max_stack_size(max_size);

                #(
                    self.partial_state.#fields.set_max_stack_size(max_size);
                )*

                #builder_name {
                    partial_state: self.partial_state,
                    _p: ::std::marker::PhantomData,
                }
            }
        }

        impl<
            #(#stack_generics_with_state_bounds),*
        > #builder_name<
            #utilities_mod_ident::WithSize,
            #(#stack_generics),*
        > {
            /// Sets the program you wish to execute.
            /// Note that the program will be executed in ascending order.
            ///
            /// # Arguments
            /// - `program` - The program you wish to execute
            #[must_use]
            pub fn with_program<P, I>(mut self, program: P)
                -> ::std::result::Result<#builder_name<#utilities_mod_ident::WithSizeAndData, #(#stack_generics),*>, ::push::push_vm::stack::StackError>
            where
                P: ::std::iter::IntoIterator<Item = I>,
                <P as ::std::iter::IntoIterator>::IntoIter: ::std::iter::DoubleEndedIterator + ::std::iter::ExactSizeIterator,
                I: ::std::convert::Into<::push::push_vm::program::PushProgram>
            {
                self.partial_state.#exec_stack_ident.try_extend(::std::iter::IntoIterator::into_iter(program).map(::std::convert::Into::into))?;
                ::std::result::Result::Ok(#builder_name {
                    partial_state: self.partial_state,
                    _p: ::std::marker::PhantomData,
                })
            }

            /// Explicitly sets this state as having no program.
            #[must_use]
            pub fn with_no_program(mut self)
                -> #builder_name<#utilities_mod_ident::WithSizeAndData, #(#stack_generics),*>
            {
                #builder_name {
                    partial_state: self.partial_state,
                    _p: ::std::marker::PhantomData,
                }
            }

        }

        impl<
            #(#stack_generics_with_state_bounds),*
        > #builder_name<
            #utilities_mod_ident::WithSizeAndData,
            #(#stack_generics),*
        > {
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
}
