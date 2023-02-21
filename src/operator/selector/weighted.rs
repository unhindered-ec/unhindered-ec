use std::ops::Not;

use rand::{rngs::ThreadRng, seq::SliceRandom};

use crate::{
    operator::{Composable, Operator},
    population::Population,
};

type PopIndividual<'pop, P> = &'pop <P as Population>::Individual;
// TODO: Is there some way to have this `SelectionOperator` type in one
//   place and re-use it since parts of this come up quite a lot. You can't
//   use a type alias where a `trait` goes, though, so it can't just be
//   with `type` like I do it here. Should it be a sub-trait of `Operator`
//   that just specifies the relevant components?
type SelectionOperator<'sel, P> =
    &'sel (dyn for<'pop> Operator<&'pop P, Output = PopIndividual<'pop, P>> + Sync);

// TODO: When we remove the `Selector`, we can simplify this a lot, removing
//   the `'pop` lifetime and making it more generic.
pub struct Weighted<'sel, P: Population> {
    selectors: Vec<(SelectionOperator<'sel, P>, usize)>,
}

impl<'sel, P: Population> Clone for Weighted<'sel, P> {
    fn clone(&self) -> Self {
        Self {
            selectors: self.selectors.clone(),
        }
    }
}

impl<'sel, P: Population> Weighted<'sel, P> {
    // Since we should never have an empty collection of weighted selectors,
    // the `new` implementation takes an initial selector so `selectors` is
    // guaranteed to never be empty.
    #[must_use]
    pub fn new(selector: SelectionOperator<'sel, P>, weight: usize) -> Self {
        Self {
            selectors: vec![(selector, weight)],
        }
    }

    #[must_use]
    pub fn with_selector(mut self, selector: SelectionOperator<'sel, P>, weight: usize) -> Self {
        self.selectors.push((selector, weight));
        self
    }
}

impl<'sel, 'pop, P> Operator<&'pop P> for Weighted<'sel, P>
where
    P: Population,
{
    type Output = &'pop P::Individual;

    fn apply(&self, population: &'pop P, rng: &mut ThreadRng) -> Self::Output {
        assert!(
            self.selectors.is_empty().not(),
            "The collection of selectors should be non-empty"
        );
        #[allow(clippy::unwrap_used)]
        let (selector, _) = self.selectors.choose_weighted(rng, |(_, w)| *w).unwrap();
        selector.apply(population, rng)
    }
}
impl<'sel, P: Population> Composable for Weighted<'sel, P> {}
