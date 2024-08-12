use anyhow::{Context, Result};
use rand::rngs::ThreadRng;

use super::Selector;
use crate::population::Population;

pub struct Worst;

impl<P> Selector<P> for Worst
where
    P: Population,
    for<'pop> &'pop P: IntoIterator<Item = &'pop P::Individual>,
    P::Individual: Ord,
{
    fn select<'pop>(&self, population: &'pop P, _: &mut ThreadRng) -> Result<&'pop P::Individual> {
        population
            .into_iter()
            .min()
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

    use super::{Selector, Worst};

    #[test]
    fn can_select_twice() {
        let pop = vec![5, 8, 9, 6, 3, 2, 10];
        let mut rng = rand::thread_rng();
        assert_eq!(&2, Worst.select(&pop, &mut rng).unwrap());
        assert_eq!(&2, Worst.select(&pop, &mut rng).unwrap());
    }

    #[proptest]
    fn test_worst_select(#[any] values: [i32; 10]) {
        let pop: Vec<i32> = values.into();
        let mut rng = rand::thread_rng();
        let smallest = pop.iter().min().unwrap();
        assert_eq!(smallest, Worst.select(&pop, &mut rng).unwrap());
    }
}
