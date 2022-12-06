use std::ops::Not;

use rand::prelude::IteratorRandom;
use rand::rngs::ThreadRng;

use crate::{individual::Individual, population::VecPop};

use super::{SelectorI};

pub struct Random {}

impl<I: Individual> SelectorI<I> for Random {
    #[must_use]
    fn select<'a>(
        &self,
        rng: &mut ThreadRng,
        population: &'a VecPop<I>,
    ) -> &'a I {
        // The population should never be empty here.
        assert!(
            population.is_empty().not(),
            "The population should not be empty"
        );
        #[allow(clippy::unwrap_used)]
        population.iter().choose(rng).unwrap()
    }
}
