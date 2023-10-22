use push_state::printing::generate_builder::generate_builder;
use quote::quote;
use syn::{spanned::Spanned, DeriveInput};

use crate::push_state::{parsing::parse_fields, printing::derive_has_stack::derive_has_stack};

mod push_state;

#[manyhow::manyhow(proc_macro_attribute)]
pub fn push_state(
    attrs: proc_macro2::TokenStream,
    tokens: proc_macro2::TokenStream,
) -> manyhow::Result {
    let macro_span = attrs.span();
    let macro_flags = syn::parse2(attrs)?;

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
            "This macro only supports structs with named fields.",
        )
        .into());
    };

    let (stacks, exec_stack, input_instructions) = parse_fields(fields, macro_span, &macro_flags)?;

    let has_stack_derives = macro_flags
        .has_stack
        .then(|| derive_has_stack(struct_ident, &stacks, &exec_stack));

    let builder = macro_flags
        .builder
        .then(|| {
            generate_builder(
                macro_span,
                struct_ident,
                struct_visibility,
                struct_generics,
                &stacks,
                &exec_stack,
                input_instructions,
            )
        })
        .transpose()?;

    Ok(quote! {
        #struct_defn
        #has_stack_derives
        #builder
    })
}
