use std::{borrow::Borrow, marker::PhantomData, num::NonZeroUsize};

use rand::{distributions::Uniform, prelude::Distribution};

use crate::distributions::{choices::ChoicesDistribution, wrappers::slice_cloning::EmptySlice};

/// Generate a random element from a collection of options, cloning the chosen
/// element. The [`OneOfCloning`] struct takes ownership of the collection; the
/// [`SliceCloning`](super::slice_cloning::SliceCloning) struct allows one to
/// borrow the collection.
#[derive(Debug, PartialEq, Eq)]
pub struct OneOfCloning<T, U> {
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

impl<T, U> OneOfCloning<T, U>
where
    T: Borrow<[U]>,
{
    /// Create a new [`OneOfCloning`] distribution, which selects a
    /// value from a collection and then returns a new value by cloning the
    /// selected value.
    ///
    /// ```
    /// # use rand::distributions::Distribution;
    /// # use ec_core::distributions::{
    /// #    choices::ChoicesDistribution,
    /// #    wrappers::{
    /// #       owned::OneOfCloning,
    /// #       slice_cloning::EmptySlice,
    /// #    },
    /// # };
    /// #
    /// let options = [1, 2, 3];
    /// let distr = OneOfCloning::new(options)?;
    /// assert_eq!(options.len(), distr.num_choices().get());
    ///
    /// let val = distr.sample(&mut rand::thread_rng());
    /// assert!(options.contains(&val));
    ///
    /// # Ok::<(), EmptySlice>(())
    ///  ```
    ///
    /// # Errors
    /// - [`EmptySlice`] if an empty collection is passed in, since then no
    ///   element can be selected from there
    pub fn new(collection: T) -> Result<Self, EmptySlice> {
        let num_choices = NonZeroUsize::new(collection.borrow().len()).ok_or(EmptySlice)?;

        Ok(Self {
            collection,
            // This error can actually never occur since it's checked above, but erroring is the
            // easiest option here.
            range: Uniform::new(0, num_choices.get()).map_err(|_| EmptySlice)?,
            num_choices,
            _p: PhantomData,
        })
    }
}
impl<T, U> ChoicesDistribution for OneOfCloning<T, U> {
    fn num_choices(&self) -> NonZeroUsize {
        self.num_choices
    }
}

impl<'a, T, U> Distribution<U> for OneOfCloning<T, U>
where
    T: Borrow<[U]>,
    U: 'a + Clone,
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
