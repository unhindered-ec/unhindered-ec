use std::num::NonZeroUsize;

use miette::Diagnostic;
use rand::{distr::slice::Choose, prelude::Distribution};

use crate::distributions::choices::ChoicesDistribution;

/// Generate a random element from an array of options, cloning the choosen
/// element.
#[derive(Debug, Clone, Copy)]
pub struct ChooseCloning<'a, T>(Choose<'a, T>);

#[derive(Debug, Clone, Copy, thiserror::Error, Diagnostic)]
#[error("Tried to create a `distributions::Slice` with an empty slice")]
#[diagnostic(help = "Ensure your slice has at least length one.")]
pub struct EmptySlice;

impl<'a, T> ChooseCloning<'a, T> {
    /// Create a new `Slice` instance which samples uniformly from the slice.
    /// Returns `Err` if the slice is empty.
    ///
    /// # Errors
    /// - [`EmptySlice`] if the passed in slice is empty
    pub fn new(slice: &'a [T]) -> Result<Self, EmptySlice> {
        Ok(Self(Choose::new(slice).map_err(|_| EmptySlice)?))
    }
}

impl<T> ChoicesDistribution for ChooseCloning<'_, T> {
    fn num_choices(&self) -> NonZeroUsize {
        self.0.num_choices()
    }
}

impl<T> Distribution<T> for ChooseCloning<'_, T>
where
    T: Clone,
{
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> T {
        self.0.sample(rng).clone()
    }
}
