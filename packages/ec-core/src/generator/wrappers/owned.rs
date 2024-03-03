use std::{borrow::Borrow, marker::PhantomData, num::NonZeroUsize};

use rand::{distributions::Uniform, prelude::Distribution};

use crate::generator::wrappers::slice_cloning::EmptySlice;

#[derive(Debug, PartialEq)]
pub struct OneOfCloning<T, U> {
    collection: T,
    range: Uniform<usize>,
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
    /// # Errors
    /// - [`EmptySlice`] if an empty collection is passed in, since then no
    ///   element can be selected from there
    pub fn new(val: T) -> Result<Self, EmptySlice> {
        match val.borrow().len() {
            0 => Err(EmptySlice),
            len => Ok(Self {
                collection: val,
                range: Uniform::new(0, len),
                _p: PhantomData,
            }),
        }
    }

    pub fn choices(&self) -> NonZeroUsize {
        // Safety: at construction time, it was ensured that the slice was
        // non-empty, as such the len is > 0.
        unsafe { NonZeroUsize::new_unchecked(self.collection.borrow().len()) }
    }
}

impl<'a, T, U> Distribution<U> for OneOfCloning<T, U>
where
    T: Borrow<[U]>,
    U: 'a + Clone,
{
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> U {
        let idx = self.range.sample(rng);

        // Safety: at construction time, it was ensured that the slice was
        // non-empty, and that the `Uniform` range produces values in range
        // for the slice
        let slice = self.collection.borrow();
        unsafe { slice.get_unchecked(idx) }.clone()
    }
}
