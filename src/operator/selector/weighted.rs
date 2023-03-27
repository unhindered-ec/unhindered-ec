use std::ops::Not;

use rand::{rngs::ThreadRng, seq::SliceRandom};

use crate::population::Population;

use super::Selector;

// type PopIndividual<'pop, P> = &'pop <P as Population>::Individual;
// // TODO: Is there some way to have this `SelectionOperator` type in one
// //   place and re-use it since parts of this come up quite a lot. You can't
// //   use a type alias where a `trait` goes, though, so it can't just be
// //   with `type` like I do it here. Should it be a sub-trait of `Operator`
// //   that just specifies the relevant components?
// type SelectionOperator<'sel, P> =
//     &'sel (dyn for<'pop> Operator<&'pop P, Output = PopIndividual<'pop, P>> + Sync);

// TODO: When we remove the `Selector`, we can simplify this a lot, removing
//   the `'pop` lifetime and making it more generic.
pub struct Weighted<P: Population> {
    selectors: Vec<(Box<dyn Selector<P> + Send + Sync>, usize)>,
}

// impl<P: Population> Clone for Weighted<P> {
//     fn clone(&self) -> Self {
//         Self {
//             selectors: self.selectors.clone(),
//         }
//     }
// }

impl<P: Population> Weighted<P> {
    // Since we should never have an empty collection of weighted selectors,
    // the `new` implementation takes an initial selector so `selectors` is
    // guaranteed to never be empty.
    #[must_use]
    pub fn new<S>(selector: S, weight: usize) -> Self
    where
        S: Selector<P> + Send + Sync + 'static,
    {
        Self {
            selectors: vec![(Box::new(selector), weight)],
        }
    }

    #[must_use]
    pub fn with_selector<S>(mut self, selector: S, weight: usize) -> Self
    where
        S: Selector<P> + Send + Sync + 'static,
    {
        self.selectors.push((Box::new(selector), weight));
        self
    }
}

impl<P> Selector<P> for Weighted<P>
where
    P: Population,
{
    fn select<'pop>(&self, population: &'pop P, rng: &mut ThreadRng) -> &'pop P::Individual {
        assert!(
            self.selectors.is_empty().not(),
            "The collection of selectors should be non-empty"
        );
        #[allow(clippy::unwrap_used)]
        let (selector, _) = self.selectors.choose_weighted(rng, |(_, w)| *w).unwrap();
        selector.select(population, rng)
    }
}
