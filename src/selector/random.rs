use std::ops::Not;

use rand::prelude::IteratorRandom;
use rand::rngs::ThreadRng;

use crate::population::Population;

use super::Selector;

pub struct Random;

impl<P: Population> Selector<P> for Random {
    #[must_use]
    fn select<'pop>(&self, rng: &mut ThreadRng, population: &'pop P) -> &'pop P::Individual {
        // The population should never be empty here.
        assert!(
            population.is_empty().not(),
            "The population should not be empty"
        );
        #[allow(clippy::unwrap_used)]
        population.iter().choose(rng).unwrap()
    }
}
