use std::ops::Not;

use rand::rngs::ThreadRng;
use rayon::prelude::{IntoParallelIterator, ParallelIterator, FromParallelIterator};

use crate::{
    child_maker::ChildMaker, population::Population, selector::Selector,
};

// TODO: Should there actually be a `Run` type (or a `RunParams` type) that
//   holds all this stuff and is used to make them available to types like
//   `Generation` and `Population`?
// TODO: Maybe go from `&Selector` to `Arc<dyn Selector>`, etc. This would
//  require changing all the lifetime references to be `Arc`s (so both
//  `weighted_selectors` and `child_maker`). It would be good to benchmark
//  both versions to see what the costs are.
pub struct Generation<'a, P: Population> {
    population: P,
    selector: &'a dyn Selector<P>,
    child_maker: &'a (dyn ChildMaker<P> + Sync + Send),
}

impl<'a, P: Population> Generation<'a, P> {
    #[must_use]
    pub fn selector(&self) -> &'a dyn Selector<P> {
        self.selector
    }

    pub const fn population(&self) -> &P {
        &self.population
    }
}

impl<'a, P: Population> Generation<'a, P> {
    /// # Panics
    ///
    /// This can panic if the population is empty or the weighted set of
    /// selectors is empty.
    pub fn new(
        population: P,
        selector: &'a dyn Selector<P>,
        child_maker: &'a (dyn ChildMaker<P> + Sync + Send),
    ) -> Self {
        assert!(population.is_empty().not());
        Self {
            population,
            selector,
            child_maker,
        }
    }

    /// # Panics
    ///
    /// This can panic if the set of selectors is empty.
    pub fn get_parent(&self, rng: &mut ThreadRng) -> &P::Individual {
        // The set of selectors should be non-empty, and if it is, then we
        // should be able to safely unwrap the `choose()` call.
        #[allow(clippy::unwrap_used)]
        self.selector.select(rng, &self.population)
    }
}

impl<'a, P> Generation<'a, P>
where
    P: Population + FromParallelIterator<P::Individual> + Sync,
    P::Individual: Send
{
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
                self.child_maker
                    .make_child(rng, &self.population, self.selector)
            })
            .collect();
        Self {
            population,
            selector: self.selector,
            child_maker: self.child_maker,
        }
    }
}

impl<'a, P> Generation<'a, P> 
where
    P: Population + FromIterator<P::Individual>,
{
    /// Make the next generation serially.
    #[must_use]
    pub fn next(&self) -> Self {
        let pop_size = self.population.size();
        let mut rng = rand::thread_rng();
        let population = (0..pop_size)
            .map(|_| {
                self.child_maker
                    .make_child(&mut rng, &self.population, self.selector)
            })
            .collect();
        Self {
            population,
            selector: self.selector,
            child_maker: self.child_maker,
        }
    }
}
