use rand::rngs::ThreadRng;

use super::{Selector, error::EmptyPopulation};
use crate::population::Population;

#[derive(Debug)]
pub struct Best;

impl<P> Selector<P> for Best
where
    P: Population,
    for<'pop> &'pop P: IntoIterator<Item = &'pop P::Individual>,
    P::Individual: Ord,
{
    type Error = EmptyPopulation;

    fn select<'pop>(
        &self,
        population: &'pop P,
        _: &mut ThreadRng,
    ) -> Result<&'pop P::Individual, Self::Error> {
        population.into_iter().max().ok_or(EmptyPopulation)
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

    use super::{Best, Selector};
    use crate::operator::selector::error::EmptyPopulation;

    #[test]
    fn empty_population() {
        let pop: Vec<i32> = Vec::new();
        let mut rng = rand::thread_rng();
        assert!(matches!(Best.select(&pop, &mut rng), Err(EmptyPopulation)));
    }

    #[test]
    fn can_select_twice() {
        // Currently `.select()` can't take an array, so we need to make this a `Vec`.
        // Once we've generalized `.select()` appropriately we can change this to be
        // an array. See #259
        let pop = vec![5, 8, 9, 6, 3, 2, 0];
        let mut rng = rand::thread_rng();
        assert_eq!(&9, Best.select(&pop, &mut rng).unwrap());
        assert_eq!(&9, Best.select(&pop, &mut rng).unwrap());
    }

    #[proptest]
    fn test_best_select(#[map(|v: [i32;10]| v.into())] pop: Vec<i32>) {
        let mut rng = rand::thread_rng();
        let largest = pop.iter().max().unwrap();
        assert_eq!(largest, Best.select(&pop, &mut rng).unwrap());
    }
}
