use anyhow::{Context, Result};
use rand::{prelude::IndexedRandom, rngs::ThreadRng};

use super::Selector;
use crate::population::Population;

pub struct Random;

impl<P> Selector<P> for Random
where
    P: Population + AsRef<[P::Individual]>,
{
    fn select<'pop>(
        &self,
        population: &'pop P,
        rng: &mut ThreadRng,
    ) -> Result<&'pop P::Individual> {
        population
            .as_ref()
            .choose(rng)
            .context("The population was empty")
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
    use test_strategy::proptest;

    use super::{Random, Selector};

    #[proptest]
    fn test_random(#[map(|v: [i32;10]| v.into())] pop: Vec<i32>) {
        let mut rng = rand::thread_rng();
        let selection = Random.select(&pop, &mut rng).unwrap();
        assert!(pop.contains(selection));
    }
}
