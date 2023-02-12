use rand::prelude::SliceRandom;
use rand::rngs::ThreadRng;

use crate::operator::{Operator, Composable};
use crate::population::Population;

use super::Selector;

pub struct Tournament {
    size: usize,
}

impl Tournament {
    #[must_use]
    pub const fn new(size: usize) -> Self {
        Self { size }
    }
}

impl<P> Selector<P> for Tournament
where
    P: Population + AsRef<[P::Individual]>,
    P::Individual: Ord,
{
    fn select<'pop>(&self, rng: &mut ThreadRng, population: &'pop P) -> &'pop P::Individual {
        self.apply(population, rng)
    }
}

impl<'pop, P> Operator<&'pop P> for Tournament
where
    P: Population + AsRef<[P::Individual]>,
    P::Individual: Ord,
{
    type Output = &'pop P::Individual;

    fn apply(&self, population: &'pop P, rng: &mut ThreadRng) -> Self::Output {
        assert!(population.size() >= self.size && self.size > 0);
        // Since we know that the population and tournament aren't empty, we
        // can safely unwrap() the `.max()` call.

        #[allow(clippy::unwrap_used)]
        population
            .as_ref()
            .choose_multiple(rng, self.size)
            .max()
            .unwrap()
    }
}
impl Composable for Tournament {}
