use std::ops::Not;

use rand::{rngs::ThreadRng, seq::SliceRandom};

use crate::{population::VecPop};

use super::{Selector};

pub struct Weighted<'a, I> {
    selectors: Vec<(&'a dyn Selector<I>, usize)>,
}

impl<'a, I> Weighted<'a, I> {
    // Since we should never have an empty collection of weighted selectors,
    // the `new` implementation takes an initial selector so `selectors` is
    // guaranteed to never be empty.
    #[must_use]
    pub fn new(selector: &'a dyn Selector<I>, weight: usize) -> Self {
        Self {
            selectors: vec![(selector, weight)],
        }
    }

    #[must_use]
    pub fn with_selector(mut self, selector: &'a dyn Selector<I>, weight: usize) -> Self {
        self.selectors.push((selector, weight));
        self
    }
}

impl<'a, I> Selector<I> for Weighted<'a, I> {
    fn select<'b>(
        &self,
        rng: &mut ThreadRng,
        population: &'b VecPop<I>,
    ) -> &'b I {
        assert!(
            self.selectors.is_empty().not(),
            "The collection of selectors should be non-empty"
        );
        #[allow(clippy::unwrap_used)]
        let (selector, _) = self.selectors.choose_weighted(rng, |(_, w)| *w).unwrap();
        selector.select(rng, population)
    }
}
