use std::{borrow::Borrow, convert::Infallible};

use rand::{distributions::Uniform, prelude::Distribution};

use crate::generator::wrappers::slice_cloning::EmptySlice;

pub struct OneOfCloning<T> {
    collection: T,
    range: Uniform<usize>,
}

impl<T> OneOfCloning<T> {
    pub fn new<U>(val: T) -> Result<Self, EmptySlice>
    where
        T: Borrow<[U]>,
    {
        match val.borrow().len() {
            0 => Err(EmptySlice),
            len => Ok(Self {
                collection: val,
                range: Uniform::new(0, len),
            }),
        }
    }
}

impl<'a, T, U> Distribution<U> for OneOfCloning<T>
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
