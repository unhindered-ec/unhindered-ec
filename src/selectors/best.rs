use std::ops::Not;

use rand::rngs::ThreadRng;

use crate::{individual::ec::EcIndividual, population::VecPop};

use super::Selector;

pub struct Best {}

impl<G: Eq, R: Ord> Selector<G, R> for Best {
    #[must_use]
    fn select<'a>(
        &self,
        _: &mut ThreadRng,
        population: &'a VecPop<EcIndividual<G, R>>,
    ) -> &'a EcIndividual<G, R> {
        // The population should never be empty here.
        assert!(
            population.is_empty().not(),
            "The population should not be empty"
        );
        population.best_individual()
    }
}
