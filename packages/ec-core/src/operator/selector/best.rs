use rand::Rng;

use super::{Selector, error::EmptyPopulation};
use crate::population::Population;

/// Selector that selects the individual with the highest value as specified by
/// the [`Ord`] relation on the individuals.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Best;

impl<P> Selector<P> for Best
where
    P: Population,
    for<'pop> &'pop P: IntoIterator<Item = &'pop P::Individual>,
    P::Individual: Ord,
{
    type Error = EmptyPopulation;

    fn select<'pop, R: Rng + ?Sized>(
        &self,
        population: &'pop P,
        _: &mut R,
    ) -> Result<&'pop P::Individual, Self::Error> {
        population.into_iter().max().ok_or(EmptyPopulation)
    }
}

#[cfg(test)]
mod tests {
    use test_strategy::proptest;

    use super::{Best, Selector};
    use crate::operator::selector::error::EmptyPopulation;

    #[test]
    fn empty_population() {
        let pop: Vec<i32> = Vec::new();
        let mut rng = rand::rng();
        assert!(matches!(Best.select(&pop, &mut rng), Err(EmptyPopulation)));
    }

    #[test]
    fn can_select_twice() {
        let pop = [5, 8, 9, 6, 3, 2, 0];
        let mut rng = rand::rng();
        assert_eq!(&9, Best.select(&pop, &mut rng).unwrap());
        assert_eq!(&9, Best.select(&pop, &mut rng).unwrap());
    }

    #[proptest]
    fn test_best_select(#[map(|v: [i32;10]| v.into())] pop: Vec<i32>) {
        let mut rng = rand::rng();
        let largest = pop.iter().max().unwrap();
        assert_eq!(largest, Best.select(&pop, &mut rng).unwrap());
    }
}
