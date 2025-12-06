use std::num::NonZeroUsize;

use rand::{
    distr::slice::{Choose, Empty},
    prelude::Distribution,
};

use crate::distributions::finite::Finite;

/// Generate a random element from an array of options, cloning the choosen
/// element.
#[derive(Copy, Clone, Debug)]
pub struct ChooseCloning<'a, T>(Choose<'a, T>);

impl<'a, T> ChooseCloning<'a, T> {
    /// Create a new `Slice` instance which samples uniformly from the slice.
    /// Returns `Err` if the slice is empty.
    ///
    /// # Errors
    /// - [`Empty`] if the passed in slice is empty
    pub fn new(slice: &'a [T]) -> Result<Self, Empty> {
        Ok(Self(Choose::new(slice)?))
    }
}

impl<T> Finite for ChooseCloning<'_, T> {
    fn sample_space_size(&self) -> NonZeroUsize {
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
