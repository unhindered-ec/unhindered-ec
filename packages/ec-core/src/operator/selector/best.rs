use anyhow::{Context, Result};
use rand::rngs::ThreadRng;

use super::Selector;
use crate::population::Population;

pub struct Best;

impl<P> Selector<P> for Best
where
    P: Population,
    for<'pop> &'pop P: IntoIterator<Item = &'pop P::Individual>,
    P::Individual: Ord,
{
    fn select<'pop>(&self, population: &'pop P, _: &mut ThreadRng) -> Result<&'pop P::Individual> {
        population
            .into_iter()
            .max()
            .context("The population was empty")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::unwrap_used)]
    fn can_select_twice() {
        let pop = vec![5, 8, 9, 6, 3, 2, 0];
        let mut rng = rand::thread_rng();
        assert_eq!(&9, Best.select(&pop, &mut rng).unwrap());
        assert_eq!(&9, Best.select(&pop, &mut rng).unwrap());
    }
}
