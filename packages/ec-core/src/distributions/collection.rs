use std::collections::{LinkedList, VecDeque};

use rand::prelude::Distribution;

/// [`Distribution`] adapter that turns a [`Distribution`] of elements `T` into
/// a [`Distribution`] of collections of elements (i.e. `Vec<T>`) of a given
/// length.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
pub struct Collection<C> {
    pub item_distribution: C,
    pub length: usize,
}

impl<C> From<(C, usize)> for Collection<C> {
    fn from((item_distribution, length): (C, usize)) -> Self {
        Self::new(item_distribution, length)
    }
}

impl<C> Collection<C> {
    pub const fn new(item_distribution: C, length: usize) -> Self {
        Self {
            item_distribution,
            length,
        }
    }
}

pub trait ConvertToCollectionDistribution {
    fn into_collection(self, length: usize) -> Collection<Self>
    where
        Self: Sized;

    fn to_collection(&self, length: usize) -> Collection<&Self>;
}

impl<C> ConvertToCollectionDistribution for C
where
    C: ?Sized,
{
    fn into_collection(self, length: usize) -> Collection<Self>
    where
        Self: Sized,
    {
        Collection::new(self, length)
    }

    fn to_collection(&self, length: usize) -> Collection<&Self> {
        Collection::new(self, length)
    }
}

impl<T, C> Distribution<Vec<T>> for Collection<C>
where
    C: Distribution<T>,
{
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Vec<T> {
        (&self.item_distribution)
            .sample_iter(rng)
            .take(self.length)
            .collect()
    }
}

impl<T, C> Distribution<LinkedList<T>> for Collection<C>
where
    C: Distribution<T>,
{
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> LinkedList<T> {
        (&self.item_distribution)
            .sample_iter(rng)
            .take(self.length)
            .collect()
    }
}

impl<T, C> Distribution<VecDeque<T>> for Collection<C>
where
    C: Distribution<T>,
{
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> VecDeque<T> {
        (&self.item_distribution)
            .sample_iter(rng)
            .take(self.length)
            .collect()
    }
}
