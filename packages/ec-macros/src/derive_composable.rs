use quote::quote;

pub fn derive_composable(input: syn::DeriveInput) -> proc_macro2::TokenStream {
    let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();

    let ty = input.ident;

    quote! {
        #[automatically_derived]
        impl #impl_generics ::ec_core::operator::Composable for #ty #type_generics #where_clause {}
    }
}
