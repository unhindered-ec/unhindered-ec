use std::num::NonZeroUsize;

use rand::{
    distr::slice::{Choose, Empty},
    prelude::Distribution,
};

use crate::distributions::finite::Finite;

/// Uniform [`Distribution`] of a slice of options, cloning the chosen element,
/// borrowing from a collection of choices.
///
/// This [`Distribution`] borrows the collection.
///
/// Also see [`ChooseCloningOwning`](super::choose_cloning_owning::ChooseCloningOwning) for an
/// alternative that takes ownership of the collection instead.
///
///
/// # Example
/// ```
/// # use rand::{rng, distr::{Distribution, slice::Empty}};
/// # use ec_core::distributions::wrappers::choose_cloning::ChooseCloning;
/// #
/// # fn main() -> Result<(), Empty> {
/// let distribution = ChooseCloning::new(&[1, 2, 3])?;
///
/// let choice = distribution.sample(&mut rng());
/// # let _ = choice;
/// # Ok(())
/// # }
/// ```
#[derive(Copy, Clone, Debug)]
pub struct ChooseCloning<'a, T>(Choose<'a, T>);

impl<'a, T> ChooseCloning<'a, T> {
    /// Create a new [`ChooseCloning`] [`Distribution`], sampling uniformly and
    /// cloning from the given slice.
    ///
    /// # Example
    /// ```
    /// # use rand::distr::slice::Empty;
    /// # use ec_core::distributions::wrappers::choose_cloning::ChooseCloning;
    /// #
    /// # fn main() -> Result<(), Empty> {
    /// let distribution = ChooseCloning::new(&[1, 2, 3])?;
    /// # let _ = distribution;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    /// - [`Empty`] if the passed in slice is empty
    pub fn new(slice: &'a [T]) -> Result<Self, Empty> {
        Ok(Self(Choose::new(slice)?))
    }
}

impl<T> Finite for ChooseCloning<'_, T> {
    /// Sample space size / number of choices of this [`ChooseCloning`]
    /// [`Distribution`].
    ///
    /// # Example
    /// ```
    /// # use rand::distr::slice::Empty;
    /// # use ec_core::distributions::{wrappers::choose_cloning::ChooseCloning, finite::Finite};
    /// #
    /// # fn main() -> Result<(), Empty> {
    /// let distribution = ChooseCloning::new(&[1, 2, 3])?;
    /// assert_eq!(distribution.sample_space_size().get(), 3);
    /// # let _ = distribution;
    /// # Ok(())
    /// # }
    /// ```
    fn sample_space_size(&self) -> NonZeroUsize {
        self.0.num_choices()
    }
}

impl<T> Distribution<T> for ChooseCloning<'_, T>
where
    T: Clone,
{
    /// Sample a single value of this [`ChooseCloning`] [`Distribution`].
    ///
    /// This does the following
    /// 1. select a random element of the slice this [`Distribution`] was
    ///    constructed from
    /// 3. return a clone of that selected element
    ///
    /// # Example
    /// ```
    /// # use rand::{rng, distr::{Distribution, slice::Empty}};
    /// # use ec_core::distributions::wrappers::choose_cloning::ChooseCloning;
    /// #
    /// # fn main() -> Result<(), Empty> {
    /// let distribution = ChooseCloning::new(&[1, 2, 3])?;
    ///
    /// let choice = distribution.sample(&mut rng());
    /// # let _ = choice;
    /// # Ok(())
    /// # }
    /// ```
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> T {
        self.0.sample(rng).clone()
    }
}
