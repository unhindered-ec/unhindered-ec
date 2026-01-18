use rand::{Rng, prelude::IndexedRandom};

use super::{Selector, error::EmptyPopulation};
use crate::population::Population;

/// Selector that selects a random individual.
///
/// # Example
/// ```
/// # use ec_core::operator::selector::{Selector, random::Random, error::EmptyPopulation};
/// let population = [2, 3, 5];
///
/// let random = Random;
/// let selected = random.select(&population, &mut rand::rng())?;
///
/// assert!(population.contains(selected));
/// # Ok::<(), EmptyPopulation>(())
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Random;

impl<P> Selector<P> for Random
where
    P: Population + AsRef<[P::Individual]>,
{
    type Error = EmptyPopulation;

    /// Select an Individual from the given Population using this selector.
    ///
    /// # Example
    /// ```
    /// # use ec_core::operator::selector::{Selector, random::Random, error::EmptyPopulation};
    /// let population = [2, 3, 5];
    ///
    /// let random = Random;
    /// let selected = random.select(&population, &mut rand::rng())?;
    ///
    /// assert!(population.contains(selected));
    /// # Ok::<(), EmptyPopulation>(())
    /// ```
    ///
    /// # Errors
    /// - [`EmptyPopulation`] if the population selected from is empty.
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
