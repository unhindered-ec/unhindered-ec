use anyhow::bail;
use rand::{rngs::ThreadRng, seq::SliceRandom, Rng};

pub trait Generator<T, Context> {
    fn generate(&mut self, context: &Context) -> T;
}

impl Generator<bool, f64> for ThreadRng {
    fn generate(&mut self, probability: &f64) -> bool {
        self.gen_bool(*probability)
    }
}

#[derive(Clone)]
pub struct CollectionContext<T> {
    collection: Vec<T>,
}

impl<T> CollectionContext<T> {
    /// # Errors
    /// This returns an error if the given collection of `T` is empty.
    /// We can't return randomly selected items from an empty collection.
    pub fn new(collection: Vec<T>) -> anyhow::Result<Self> {
        if collection.is_empty() {
            bail!("You must have a non-empty collection for a `CollectionContext`");
        }
        Ok(Self { collection })
    }
}

impl<T> Generator<T, CollectionContext<T>> for ThreadRng
where
    T: Clone,
{
    fn generate(&mut self, context: &CollectionContext<T>) -> T {
        #[allow(clippy::unwrap_used)]
        context.collection.choose(self).unwrap().clone()
    }
}
