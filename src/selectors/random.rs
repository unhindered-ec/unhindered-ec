use std::ops::Not;

use rand::prelude::IteratorRandom;
use rand::rngs::ThreadRng;

use crate::{individual::ec::EcIndividual, population::VecPop};

use super::Selector;

pub struct Random {}

impl<G, R: Ord> Selector<G, R> for Random {
    #[must_use]
    fn select<'a>(
        &self,
        rng: &mut ThreadRng,
        population: &'a VecPop<EcIndividual<G, R>>,
    ) -> &'a EcIndividual<G, R> {
        // The population should never be empty here.
        assert!(
            population.is_empty().not(),
            "The population should not be empty"
        );
        #[allow(clippy::unwrap_used)]
        population.iter().choose(rng).unwrap()
    }
}
