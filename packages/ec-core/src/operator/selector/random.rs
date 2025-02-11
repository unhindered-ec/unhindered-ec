use rand::{Rng, prelude::IndexedRandom};

use super::{Selector, error::EmptyPopulation};
use crate::population::Population;

#[derive(Debug)]
pub struct Random;

impl<P> Selector<P> for Random
where
    P: Population + AsRef<[P::Individual]>,
{
    type Error = EmptyPopulation;

    fn select<'pop, R: Rng + ?Sized>(
        &self,
        population: &'pop P,
        rng: &mut R,
    ) -> Result<&'pop P::Individual, Self::Error> {
        population.as_ref().choose(rng).ok_or(EmptyPopulation)
    }
}

#[cfg(test)]
mod tests {
    use test_strategy::proptest;

    use super::{Random, Selector};
    use crate::operator::selector::error::EmptyPopulation;

    #[test]
    fn empty_population() {
        let pop: Vec<i32> = Vec::new();
        let mut rng = rand::rng();
        assert!(matches!(
            Random.select(&pop, &mut rng),
            Err(EmptyPopulation)
        ));
    }

    #[proptest]
    fn test_random(#[map(|v: [i32;10]| v.into())] pop: Vec<i32>) {
        let mut rng = rand::rng();
        let selection = Random.select(&pop, &mut rng).unwrap();
        assert!(pop.contains(selection));
    }
}
