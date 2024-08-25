use std::error::Error;

use miette::Diagnostic;
use rand::{
    rngs::ThreadRng,
    seq::{IndexedRandom, WeightError},
};

use super::{error::SelectionError, Selector};
use crate::population::Population;

pub struct Weighted<P: Population> {
    selectors: Vec<(
        Box<dyn Selector<P, Error = Box<dyn Error>> + Send + Sync>,
        usize,
    )>,
}

#[derive(Debug, thiserror::Error, Diagnostic)]
pub enum WeightedError {
    #[error(transparent)]
    #[diagnostic(transparent)]
    Selector(#[from] SelectionError),

    #[error(transparent)]
    #[diagnostic(help = "Ensure that the weights are all non-negative and add to more than zero")]
    Weight(#[from] WeightError),

    #[error(transparent)]
    Other(Box<dyn Error>),
}

impl<P: Population> Weighted<P> {
    // Since we should never have an empty collection of weighted selectors,
    // the `new` implementation takes an initial selector so `selectors` is
    // guaranteed to never be empty.
    #[must_use]
    pub fn new<S>(selector: S, weight: usize) -> Self
    where
        S: Selector<P, Error = std::boxed::Box<(dyn std::error::Error + 'static)>>
            + Send
            + Sync
            + 'static,
    {
        Self {
            selectors: vec![(Box::new(selector), weight)],
        }
    }

    #[must_use]
    pub fn with_selector<S>(mut self, selector: S, weight: usize) -> Self
    where
        S: Selector<P, Error = std::boxed::Box<(dyn std::error::Error + 'static)>>
            + Send
            + Sync
            + 'static,
    {
        self.selectors.push((Box::new(selector), weight));
        self
    }
}

impl<P> Selector<P> for Weighted<P>
where
    P: Population,
{
    type Error = WeightedError;

    fn select<'pop>(
        &self,
        population: &'pop P,
        rng: &mut ThreadRng,
    ) -> Result<&'pop P::Individual, Self::Error> {
        let (selector, _) = self.selectors.choose_weighted(rng, |(_, w)| *w)?;
        selector
            .select(population, rng)
            .map_err(WeightedError::Other)
    }
}

impl<P, S> Selector<P> for Box<S>
where
    P: Population,
    S: Selector<P>,
    S::Error: Error + 'static,
{
    type Error = Box<dyn Error>;

    fn select<'pop>(
        &self,
        population: &'pop P,
        rng: &mut ThreadRng,
    ) -> Result<&'pop <P as Population>::Individual, Self::Error> {
        (**self)
            .select(population, rng)
            .map_err(|e| Box::new(e).into())
    }
}

#[cfg(test)]
#[rustversion::attr(before(1.81), allow(clippy::unwrap_used))]
#[rustversion::attr(
    since(1.81),
    expect(
        clippy::unwrap_used,
        reason = "Panicking is the best way to deal with errors in unit tests"
    )
)]
mod tests {
    use itertools::Itertools;
    use test_strategy::proptest;

    use super::Weighted;
    use crate::operator::selector::{best::Best, worst::Worst, Selector};

    #[proptest]
    fn test_random(#[map(|v: [i32;10]| v.into())] pop: Vec<i32>) {
        let mut rng = rand::thread_rng();
        // We'll make a selector that has a 50/50 chance of choosing the highest
        // or lowest value.
        let weighted = Weighted::new(Box::new(Best), 1).with_selector(Box::new(Worst), 1);
        let selection = weighted.select(&pop, &mut rng).unwrap();
        let extremes: [&i32; 2] = pop.iter().minmax().into_option().unwrap().into();
        assert!(extremes.contains(&selection));
    }
}
