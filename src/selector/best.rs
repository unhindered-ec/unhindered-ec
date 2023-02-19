use std::ops::Not;

use rand::rngs::ThreadRng;

use crate::{
    operator::{Composable, Operator},
    population::Population,
};

use super::Selector;

pub struct Best;

impl<P> Selector<P> for Best
where
    P: Population,
    for<'pop> &'pop P: IntoIterator<Item = &'pop P::Individual>,
    P::Individual: Ord,
{
    #[must_use]
    fn select<'pop>(&self, rng: &mut ThreadRng, population: &'pop P) -> &'pop P::Individual {
        self.apply(population, rng)
    }
}

impl<'pop, P> Operator<&'pop P> for Best
where
    P: Population,
    &'pop P: IntoIterator<Item = &'pop P::Individual>,
    P::Individual: Ord,
{
    type Output = &'pop P::Individual;

    fn apply(&self, population: &'pop P, _: &mut ThreadRng) -> Self::Output {
        // The population should never be empty here.
        assert!(
            population.is_empty().not(),
            "The population should not be empty"
        );
        #[allow(clippy::unwrap_used)]
        population.into_iter().max().unwrap()
    }
}
impl Composable for Best {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_select_twice() {
        let pop = vec![5, 8, 9, 6, 3, 2, 0];
        let mut rng = rand::thread_rng();
        assert_eq!(&9, Best.select(&mut rng, &pop));
        assert_eq!(&9, Best.select(&mut rng, &pop));
    }
}
