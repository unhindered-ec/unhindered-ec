use syn::{Token, TypeParamBound, punctuated::Punctuated};

pub trait Modification {
    fn apply(&self, ty: syn::Type) -> syn::Type;
}

#[derive(Clone)]
pub struct Passthrough;

impl Modification for Passthrough {
    fn apply(&self, ty: syn::Type) -> syn::Type {
        ty
    }
}

#[derive(Clone)]
pub struct AddAutoTraits(pub Punctuated<TypeParamBound, Token![+]>);
impl Modification for AddAutoTraits {
    fn apply(&self, ty: syn::Type) -> syn::Type {
        let bounds = &self.0;
        syn::parse_quote!((#ty + #bounds))
    }
}

macro_rules! wrap_modification {
    ($i: ident; $tp:ident; $($t:tt)*) => {
        #[derive(Clone)]
        struct $i;
        impl Modification for $i {
            fn apply(&self, $tp: syn::Type) -> syn::Type {
                syn::parse_quote!($($t)*)
            }
        }
    }
}
pub(crate) use wrap_modification;
