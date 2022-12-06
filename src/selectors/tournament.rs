use rand::prelude::IteratorRandom;
use rand::rngs::ThreadRng;

use crate::{individual::ec::EcIndividual, population::VecPop};

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

impl<G: Eq, R: Ord> Selector<G, R> for Tournament {
    fn select<'a>(
        &self,
        rng: &mut ThreadRng,
        population: &'a VecPop<EcIndividual<G, R>>,
    ) -> &'a EcIndividual<G, R> {
        assert!(population.size() >= self.size && self.size > 0);
        // Since we know that the population and tournament aren't empty, we
        // can safely unwrap() the `.max()` call.
        #[allow(clippy::unwrap_used)]
        population
            .iter()
            .choose_multiple(rng, self.size)
            .iter()
            .max()
            .unwrap()
    }
}