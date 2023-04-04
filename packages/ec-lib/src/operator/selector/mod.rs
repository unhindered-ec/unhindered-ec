use anyhow::Result;
use rand::rngs::ThreadRng;

use crate::population::Population;

use super::{Composable, Operator};

pub mod best;
pub mod lexicase;
pub mod random;
pub mod tournament;
pub mod weighted;

pub trait Selector<P>
where
    P: Population,
{
    /// # Errors
    /// This will return an error if there's some problem selecting. That will usually
    /// be because the population is empty or not large enough for the desired selector.
    fn select<'pop>(&self, population: &'pop P, rng: &mut ThreadRng)
        -> Result<&'pop P::Individual>;
}

#[derive(Clone)]
pub struct Select<S> {
    selector: S,
}

impl<S> Select<S> {
    pub const fn new(selector: S) -> Self {
        Self { selector }
    }
}

impl<'pop, P, S> Operator<&'pop P> for Select<S>
where
    P: Population,
    S: Selector<P>,
{
    type Output = &'pop P::Individual;

    fn apply(&self, population: &'pop P, rng: &mut ThreadRng) -> Result<Self::Output> {
        self.selector.select(population, rng)
    }
}
impl<S> Composable for Select<S> {}

impl<T, P> Selector<P> for &T
where
    P: Population,
    T: Selector<P>,
{
    fn select<'pop>(
        &self,
        population: &'pop P,
        rng: &mut ThreadRng,
    ) -> Result<&'pop P::Individual> {
        (*self).select(population, rng)
    }
}
