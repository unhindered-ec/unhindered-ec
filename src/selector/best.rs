use std::ops::Not;

use rand::rngs::ThreadRng;

use crate::population::VecPop;

use super::Selector;

pub struct Best {}

impl<I: Ord> Selector<I> for Best {
    #[must_use]
    fn select<'pop>(&self, _: &mut ThreadRng, population: &'pop VecPop<I>) -> &'pop I {
        // The population should never be empty here.
        assert!(
            population.is_empty().not(),
            "The population should not be empty"
        );
        population.best_individual()
    }
}
