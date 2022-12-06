use std::ops::Not;

use rand::rngs::ThreadRng;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

use crate::{individual::ec::EcIndividual, population::VecPop, selectors::Selector};

pub trait ChildMaker<G, R>: Sync {
    fn make_child(&self, rng: &mut ThreadRng, generation: &Generation<G, R>) -> EcIndividual<G, R>;
}

// TODO: Extend the vector of Selectors to a WeightedParentSelector that is essentially
//   a wrapper around `rand::distributions::WeightedChoice` so we can
//   provide weights on the different selectors.
// TODO: Should the `scorer` be inside the generation so we don't have to keep
//   capturing it and passing it around?
// TODO: Should there actually be a `Run` type (or a `RunParams` type) that
//   holds all this stuff and is used to make them available to types like
//   `Generation` and `Population`?
// TODO: Maybe go from `&Selector` to `Arc<dyn Selector>`, etc. This would
//  require changing all the lifetime references to be `Arc`s (so both
//  `weighted_selectors` and `child_maker`). It would be good to benchmark
//  both versions to see what the costs are.
pub struct Generation<'a, G, R> {
    pub population: VecPop<EcIndividual<G, R>>,
    selector: &'a dyn Selector<G, R>,
    child_maker: &'a dyn ChildMaker<G, R>,
}

impl<'a, G: Eq, R: Ord> Generation<'a, G, R> {
    /// # Panics
    ///
    /// This can panic if the population is empty or the weighted set of
    /// selectors is empty.
    pub fn new(
        population: VecPop<EcIndividual<G, R>>,
        selector: &'a dyn Selector<G, R>,
        child_maker: &'a dyn ChildMaker<G, R>,
    ) -> Self {
        assert!(population.is_empty().not());
        Self {
            population,
            selector,
            child_maker,
        }
    }

    #[must_use]
    pub fn best_individual(&self) -> &EcIndividual<G, R> {
        self.population.best_individual()
    }

    /// # Panics
    ///
    /// This can panic if the set of selectors is empty.
    pub fn get_parent(&self, rng: &mut ThreadRng) -> &EcIndividual<G, R> {
        // The set of selectors should be non-empty, and if it is, then we
        // should be able to safely unwrap the `choose()` call.
        #[allow(clippy::unwrap_used)]
        self.selector.select(rng, &self.population)
    }
}

impl<'a, G: Send + Sync, R: Send + Sync> Generation<'a, G, R> {
    /// Make the next generation using a Rayon parallel iterator.
    #[must_use]
    pub fn par_next(&self) -> Self {
        let pop_size = self.population.size();
        let population = (0..pop_size)
            .into_par_iter()
            // "Convert" the individual number (which we never use) into
            // the current `Generation` object so the `make_child` closure
            // will have access to the selectors and population.
            .map(|_| self)
            .map_init(rand::thread_rng, |rng, _| {
                self.child_maker.make_child(rng, self)
            })
            .collect();
        Self {
            population,
            selector: self.selector,
            child_maker: self.child_maker,
        }
    }
}

impl<'a, G, R> Generation<'a, G, R> {
    /// Make the next generation serially.
    #[must_use]
    pub fn next(&self) -> Self {
        let pop_size = self.population.size();
        let mut rng = rand::thread_rng();
        let population = (0..pop_size)
            .map(|_| self.child_maker.make_child(&mut rng, self))
            .collect();
        Self {
            population,
            selector: self.selector,
            child_maker: self.child_maker,
        }
    }
}
