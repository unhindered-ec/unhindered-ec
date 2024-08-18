use anyhow::{Context, Result};
use rand::{rngs::ThreadRng, seq::IndexedRandom};

use super::Selector;
use crate::population::Population;

pub struct Weighted<P: Population> {
    selectors: Vec<(Box<dyn Selector<P> + Send + Sync>, usize)>,
}

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
    fn select<'pop>(
        &self,
        population: &'pop P,
        rng: &mut ThreadRng,
    ) -> Result<&'pop P::Individual> {
        let (selector, _) = self
            .selectors
            .choose_weighted(rng, |(_, w)| *w)
            .context("The set of selectors was empty")?;
        selector.select(population, rng)
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
    fn test_random(#[any] values: [i32; 10]) {
        let pop: Vec<i32> = values.into();
        let mut rng = rand::thread_rng();
        // We'll make a selector that has a 50/50 chance of choosing the highest
        // or lowest value.
        let weighted = Weighted::new(Best, 1).with_selector(Worst, 1);
        let selection = weighted.select(&pop, &mut rng).unwrap();
        let extremes: [&i32; 2] = pop.iter().minmax().into_option().unwrap().into();
        assert!(extremes.contains(&selection));
    }
}
