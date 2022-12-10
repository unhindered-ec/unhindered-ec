use std::ops::Not;

use rand::rngs::ThreadRng;

use crate::population::Population;

use super::Selector;

pub struct Best;

impl<P> Selector<P> for Best
where
    P: Population,
    for<'pop> &'pop P: IntoIterator<Item = &'pop P::Individual>,
    P::Individual: Ord,
{
    #[must_use]
    fn select<'pop>(&self, _: &mut ThreadRng, population: &'pop P) -> &'pop P::Individual {
        // The population should never be empty here.
        assert!(
            population.is_empty().not(),
            "The population should not be empty"
        );
        #[allow(clippy::unwrap_used)]
        population.into_iter().max().unwrap()
    }
}
