use quote::quote;
use syn::TypeReference;

mod modification;
use modification::*;

wrap_modification!(Pointer; ty ; &#ty);
wrap_modification!(MutPointer; ty ; &mut #ty);
wrap_modification!(Arc; ty ; std::sync::Arc<#ty>);
wrap_modification!(Rc; ty ; std::rc::Rc<#ty>);
wrap_modification!(Ref; ty ; std::cell::Ref<'_,#ty>);
wrap_modification!(RefMut; ty ; std::cell::RefMut<'_,#ty>);
wrap_modification!(BoxMod; ty ; Box<#ty>);

static PRELIMINARY_MODIFICATIONS: fn() -> Vec<Box<dyn Modification>> = || {
    vec![
        Box::new(Passthrough),
        Box::new(AddAutoTraits(syn::parse_quote!(Send))),
        Box::new(AddAutoTraits(syn::parse_quote!(Sync))),
        Box::new(AddAutoTraits(syn::parse_quote!(Send + Sync))),
    ]
};

static MUT_ALTERNATIVES: fn() -> Vec<Box<dyn Modification>> =
    || vec![Box::new(MutPointer), Box::new(RefMut), Box::new(BoxMod)];

static SHARED_ALTERNATIVES: fn() -> Vec<Box<dyn Modification>> = || {
    vec![
        Box::new(Pointer),
        Box::new(MutPointer),
        Box::new(RefMut),
        Box::new(BoxMod),
        Box::new(Arc),
        Box::new(Rc),
        Box::new(Ref),
    ]
};

pub fn dyn_ref_impls(a: proc_macro2::TokenStream, tokens: syn::ItemImpl) -> manyhow::Result {
    manyhow::ensure!(a.is_empty(), a, "Expected no inputs");

    match tokens.trait_ {
        Some((None, _, _)) => {}
        _ => manyhow::bail!(tokens, "Only non-negative trait impls are supported"),
    }

    let ty = tokens.self_ty.clone();

    let (modifications, ty) = match *ty {
        syn::Type::Reference(TypeReference {
            mutability: Some(_),
            elem,
            ..
        }) => (MUT_ALTERNATIVES, *elem),
        syn::Type::Reference(TypeReference {
            mutability: None,
            elem,
            ..
        }) => (SHARED_ALTERNATIVES, *elem),
        _ => manyhow::bail!(
            ty,
            "Only reference types (&, &mut) are supported at the moment"
        ),
    };

    let prelim_mods = PRELIMINARY_MODIFICATIONS();
    let modifications = modifications();

    let out = modifications
        .iter()
        .flat_map(|m| prelim_mods.iter().map(|s1m| m.apply(s1m.apply(ty.clone()))))
        .map(|t| {
            let mut out = tokens.clone();
            *out.self_ty = t;
            out
        });

    Ok(quote! {#(#out)*})
}
