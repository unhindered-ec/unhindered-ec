use std::{ops::Not};

use rand::{rngs::ThreadRng, seq::SliceRandom};

use crate::{individual::ec::EcIndividual, population::VecPop};

use super::Selector;

pub struct Weighted<'a, G, R> {
    selectors: Vec<(&'a dyn Selector<G, R>, usize)>,
}

impl<'a, G, R> Weighted<'a, G, R> {
    // Since we should never have an empty collection of weighted selectors,
    // the `new` implementation takes an initial selector so `selectors` is
    // guaranteed to never be empty.
    #[must_use]
    pub fn new(selector: &'a dyn Selector<G, R>, weight: usize) -> Self {
        Self {
            selectors: vec![(selector, weight)],
        }
    }

    #[must_use]
    pub fn with_selector(mut self, selector: &'a dyn Selector<G, R>, weight: usize) -> Self {
        self.selectors.push((selector, weight));
        self
    }
}

impl<'a, G, R> Selector<G, R> for Weighted<'a, G, R> {
    fn select<'b>(
        &self,
        rng: &mut ThreadRng,
        population: &'b VecPop<EcIndividual<G, R>>,
    ) -> &'b EcIndividual<G, R> {
        assert!(
            self.selectors.is_empty().not(),
            "The collection of selectors should be non-empty"
        );
        #[allow(clippy::unwrap_used)]
        let (selector, _) = self.selectors.choose_weighted(rng, |(_, w)| *w).unwrap();
        selector.select(rng, population)
    }
}
