use rand::prelude::SliceRandom;
use rand::rngs::ThreadRng;

use crate::operator::{Composable, Operator};
use crate::population::Population;

pub struct Tournament {
    size: usize,
}

impl Tournament {
    #[must_use]
    pub const fn new(size: usize) -> Self {
        Self { size }
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
