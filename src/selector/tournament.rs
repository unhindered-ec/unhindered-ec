use rand::prelude::SliceRandom;
use rand::rngs::ThreadRng;
// use rand::seq::IteratorRandom;

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
    P: Population,
    P::Individual: Ord
{
    fn select<'pop>(&self, rng: &mut ThreadRng, population: &'pop P) -> &'pop P::Individual {
        assert!(population.size() >= self.size && self.size > 0);
        // Since we know that the population and tournament aren't empty, we
        // can safely unwrap() the `.max()` call.

        // 11.0µs, 11.1µs, 11.6µs
        // #[allow(clippy::unwrap_used)]
        // population
        //     .iter()
        //     .choose_multiple(rng, self.size)
        //     .iter()
        //     .max()
        //     .unwrap()

        // 102ns, 199ns, 1.23µs
        #[allow(clippy::unwrap_used)]
        population
            .iter()
            .as_slice()
            .choose_multiple(rng, self.size)
            .max()
            .unwrap()

        // 106ns, 203ns, 1.19µs
        // #[allow(clippy::unwrap_used)]
        // population
        //     .slice()
        //     .choose_multiple(rng, self.size)
        //     .max()
        //     .unwrap()

        // 101ns, 201ns, 1.20µs
        // #[allow(clippy::unwrap_used)]
        // population
        //     .individuals()
        //     .choose_multiple(rng, self.size)
        //     .max()
        //     .unwrap()
    }
}
