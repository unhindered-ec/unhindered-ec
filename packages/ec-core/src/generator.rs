use anyhow::{bail, Context};
use rand::{rngs::ThreadRng, seq::SliceRandom, Rng};

pub trait Generator<T> {
    /// # Errors
    ///
    /// This returns an `anyhow::Error` if the implementation of `generate`
    /// returns some sort of error. An example would be choosing a random
    /// item from a collection; this fails if the collection is empty.  
    fn generate(&self, rng: &mut ThreadRng) -> anyhow::Result<T>;
}

impl Generator<bool> for f64 {
    fn generate(&self, rng: &mut ThreadRng) -> anyhow::Result<bool> {
        Ok(rng.gen_bool(*self))
    }
}

impl<const N: usize, T> Generator<T> for [T; N]
where
    T: Clone,
{
    fn generate(&self, rng: &mut ThreadRng) -> anyhow::Result<T> {
        Ok(self
            .choose(rng)
            .context("`generate` called with an empty array of options to choose from")?
            .clone())
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

impl<T> Generator<T> for CollectionContext<T>
where
    T: Clone,
{
    fn generate(&self, rng: &mut ThreadRng) -> anyhow::Result<T> {
        Ok(self
            .collection
            .choose(rng)
            .context("`generate` called with an empty collection of options to choose from")?
            .clone())
    }
}
