use std::{borrow::Borrow, marker::PhantomData, num::NonZeroUsize};

use rand::{
    distr::{Uniform, slice::Empty},
    prelude::Distribution,
};

use crate::distributions::finite::Finite;

/// Uniform [`Distribution`] of a collection of options, cloning the chosen
/// element.
///
/// This [`Distribution`] takes ownership of the collection.
///
/// Also see [`ChooseCloning`](super::choose_cloning::ChooseCloning) for an
/// alternative that borrows the collection instead.
///
/// # Example
/// ```
/// # use rand::{rng, distr::{Distribution, slice::Empty}};
/// # use ec_core::distributions::wrappers::choose_cloning_owning::ChooseCloningOwning;
/// #
/// # fn main() -> Result<(), Empty> {
/// let my_collection = [0, 1, 2, 3];
/// let my_distribution = ChooseCloningOwning::new(my_collection)?;
///
/// let my_sample = my_distribution.sample(&mut rng());
/// # let _ = my_sample;
/// # Ok(())
/// # }
/// ```
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct ChooseCloningOwning<T, U> {
    // It is really important here that the fields `collection`, `range` and `num_choices` are
    // never modified, since they all contain information about the length of the collection
    // which need to be in sync for no panics to occur.
    //
    // Therefore, these fields may *never* be pub and no methods may be introduced which can modify
    // fields without keeping this contract.
    //
    // Currently these fields are *never* modified at all.
    collection: T,
    range: Uniform<usize>,
    // we store the NonZeroUsize in the struct here, since we need to check this invariant in the
    // new anyways. As such it would make little sense to recompute that every time.
    num_choices: NonZeroUsize,
    _p: PhantomData<U>,
}

impl<T, U> ChooseCloningOwning<T, U>
where
    T: Borrow<[U]>,
{
    /// Create a new [`ChooseCloningOwning`] distribution, by moving the
    /// provided collection
    ///
    /// # Example
    ///
    /// ```
    /// # use rand::distr::slice::Empty;
    /// # use ec_core::distributions::wrappers::choose_cloning_owning::ChooseCloningOwning;
    /// #
    /// let options = [1, 2, 3];
    /// let distr = ChooseCloningOwning::new(options)?;
    /// # let _ = distr;
    /// # Ok::<(), Empty>(())
    ///  ```
    ///
    /// # Errors
    /// - [`Empty`] if an empty collection is passed in, since then no
    ///   element can be selected from there
    pub fn new(collection: T) -> Result<Self, Empty> {
        let num_choices = NonZeroUsize::new(collection.borrow().len()).ok_or(Empty)?;

        Ok(Self {
            collection,
            // This error can actually never occur since it's checked above, but erroring is the
            // easiest option here.
            range: Uniform::new(0, num_choices.get()).map_err(|_| Empty)?,
            num_choices,
            _p: PhantomData,
        })
    }
}
impl<T, U> Finite for ChooseCloningOwning<T, U> {
    /// Sample space size / number of choices of this [`ChooseCloningOwning`]
    /// [`Distribution`].
    ///
    /// # Example
    /// ```
    /// # use rand::distr::slice::Empty;
    /// # use ec_core::distributions::{wrappers::choose_cloning_owning::ChooseCloningOwning, finite::Finite};
    /// #
    /// # fn main() -> Result<(), Empty> {
    /// let distribution = ChooseCloningOwning::new([1, 2, 3])?;
    /// assert_eq!(distribution.sample_space_size().get(), 3);
    /// # let _ = distribution;
    /// # Ok(())
    /// # }
    /// ```
    fn sample_space_size(&self) -> NonZeroUsize {
        self.num_choices
    }
}

impl<T, U> Distribution<U> for ChooseCloningOwning<T, U>
where
    T: Borrow<[U]>,
    U: Clone,
{
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> U {
        let idx = self.range.sample(rng);

        let slice = self.collection.borrow();

        debug_assert!(
            slice.len() >= idx,
            "This should never happen since the new method provides these guarantees"
        );

        // FIXME: Check the performance of this, and if neccessary replace with
        // let val = unsafe { slice.get_unchecked(idx) }

        #[expect(
            clippy::unwrap_used,
            reason = "At construction time, it was ensured that the slice was non-empty, and that \
                      the `Uniform` range produces values in range for the slice - this should \
                      never occur"
        )]
        let val = slice.get(idx).unwrap();

        val.clone()
    }
}
