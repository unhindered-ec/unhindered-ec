use rand::rngs::ThreadRng;

use super::{Composable, Operator};
use crate::population::Population;

pub mod best;
pub mod error;
pub mod lexicase;
pub mod random;
pub mod tournament;
pub mod weighted;
pub mod worst;

pub trait Selector<P>
where
    P: Population,
{
    type Error;

    /// # Errors
    /// This will return an error if there's some problem selecting. That will
    /// usually be because the population is empty or not large enough for
    /// the desired selector.
    fn select<'pop>(
        &self,
        population: &'pop P,
        rng: &mut ThreadRng,
    ) -> Result<&'pop P::Individual, Self::Error>;
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
    type Error = S::Error;

    fn apply(&self, population: &'pop P, rng: &mut ThreadRng) -> Result<Self::Output, Self::Error> {
        self.selector.select(population, rng)
    }
}
impl<S> Composable for Select<S> {}

impl<S, P> Selector<P> for &S
where
    P: Population,
    S: Selector<P>,
{
    type Error = S::Error;

    fn select<'pop>(
        &self,
        population: &'pop P,
        rng: &mut ThreadRng,
    ) -> Result<&'pop P::Individual, Self::Error> {
        (*self).select(population, rng)
    }
}
