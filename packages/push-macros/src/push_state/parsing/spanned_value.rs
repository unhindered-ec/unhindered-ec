use std::ops::{Deref, DerefMut};

use proc_macro2::Span;

#[derive(Clone)]
pub struct SpannedValue<T> {
    pub span: Span,
    pub value: T,
}

impl<T: Default> Default for SpannedValue<T> {
    fn default() -> Self {
        Self {
            span: Span::mixed_site(),
            value: Default::default(),
        }
    }
}

impl<T> Deref for SpannedValue<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for SpannedValue<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}
