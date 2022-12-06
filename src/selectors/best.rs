use std::ops::Not;

use rand::rngs::ThreadRng;

use crate::{population::VecPop};

use super::{Selector};

pub struct Best {}

impl<I: Ord> Selector<I> for Best {
    #[must_use]
    fn select<'a>(
        &self,
        _: &mut ThreadRng,
        population: &'a VecPop<I>,
    ) -> &'a I {
        // The population should never be empty here.
        assert!(
            population.is_empty().not(),
            "The population should not be empty"
        );
        population.best_individual()
    }
}
