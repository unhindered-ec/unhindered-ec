use std::ops::Not;

use rand::{rngs::ThreadRng, seq::SliceRandom};

use crate::population::VecPop;

use super::Selector;

pub struct Weighted<'sel, I> {
    selectors: Vec<(&'sel dyn Selector<I>, usize)>,
}

impl<'sel, I> Weighted<'sel, I> {
    // Since we should never have an empty collection of weighted selectors,
    // the `new` implementation takes an initial selector so `selectors` is
    // guaranteed to never be empty.
    #[must_use]
    pub fn new(selector: &'sel dyn Selector<I>, weight: usize) -> Self {
        Self {
            selectors: vec![(selector, weight)],
        }
    }

    #[must_use]
    pub fn with_selector(mut self, selector: &'sel dyn Selector<I>, weight: usize) -> Self {
        self.selectors.push((selector, weight));
        self
    }
}

impl<'sel, I> Selector<I> for Weighted<'sel, I> {
    fn select<'pop>(&self, rng: &mut ThreadRng, population: &'pop VecPop<I>) -> &'pop I {
        assert!(
            self.selectors.is_empty().not(),
            "The collection of selectors should be non-empty"
        );
        #[allow(clippy::unwrap_used)]
        let (selector, _) = self.selectors.choose_weighted(rng, |(_, w)| *w).unwrap();
        selector.select(rng, population)
    }
}
