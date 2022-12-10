use std::ops::Not;

use rand::{rngs::ThreadRng, seq::SliceRandom};

use crate::population::Population;

use super::Selector;

pub struct Weighted<'sel, P> {
    selectors: Vec<(&'sel dyn Selector<P>, usize)>,
}

impl<'sel, P> Clone for Weighted<'sel, P> {
    fn clone(&self) -> Self {
        Self { selectors: self.selectors.clone() }
    }
}

impl<'sel, P> Weighted<'sel, P> {
    // Since we should never have an empty collection of weighted selectors,
    // the `new` implementation takes an initial selector so `selectors` is
    // guaranteed to never be empty.
    #[must_use]
    pub fn new(selector: &'sel dyn Selector<P>, weight: usize) -> Self {
        Self {
            selectors: vec![(selector, weight)],
        }
    }

    #[must_use]
    pub fn with_selector(mut self, selector: &'sel dyn Selector<P>, weight: usize) -> Self {
        self.selectors.push((selector, weight));
        self
    }
}

impl<'sel, P: Population> Selector<P> for Weighted<'sel, P> {
    fn select<'pop>(&self, rng: &mut ThreadRng, population: &'pop P) -> &'pop P::Individual {
        assert!(
            self.selectors.is_empty().not(),
            "The collection of selectors should be non-empty"
        );
        #[allow(clippy::unwrap_used)]
        let (selector, _) = self.selectors.choose_weighted(rng, |(_, w)| *w).unwrap();
        selector.select(rng, population)
    }
}
