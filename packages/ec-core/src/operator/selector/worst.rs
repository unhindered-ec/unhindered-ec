use rand::Rng;

use super::{Selector, error::EmptyPopulation};
use crate::population::Population;

/// Selector that selects the individual with the lowest value as specified by
/// the [`Ord`] relation on the individuals.
///
/// Also see [`Best`](ec_core::operator::selector::best::Best) for a selector
/// that selects the individual with the highest value instead.
///
/// # Example
/// ```
/// # use ec_core::operator::selector::{Selector, worst::Worst, error::EmptyPopulation};
/// let population = [2, 3, 5];
///
/// let worst = Worst;
/// let selected = worst.select(&population, &mut rand::rng())?;
///
/// assert_eq!(*selected, 2);
/// # Ok::<(), EmptyPopulation>(())
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Worst;

impl<P> Selector<P> for Worst
where
    P: Population,
    for<'pop> &'pop P: IntoIterator<Item = &'pop P::Individual>,
    P::Individual: Ord,
{
    type Error = EmptyPopulation;

    /// Selects the worst (as defined by the [`Ord`] relation) individual of the
    /// passed Population.
    ///
    /// # Example
    /// ```
    /// # use ec_core::operator::selector::{Selector, worst::Worst, error::EmptyPopulation};
    /// let population = [2, 3, 5];
    ///
    /// let worst = Worst;
    /// let selected = worst.select(&population, &mut rand::rng())?;
    ///
    /// assert_eq!(*selected, 2);
    /// # Ok::<(), EmptyPopulation>(())
    /// ```
    ///
    /// # Errors
    /// - [`EmptyPopulation`] if the population selected from is empty.
    fn select<'pop, R: Rng + ?Sized>(
        &self,
        population: &'pop P,
        _: &mut R,
    ) -> Result<&'pop P::Individual, Self::Error> {
        population.into_iter().min().ok_or(EmptyPopulation)
    }
}

#[cfg(test)]
mod tests {
    use test_strategy::proptest;

    use super::{Selector, Worst};
    use crate::operator::selector::error::EmptyPopulation;

    #[test]
    fn empty_population() {
        let pop: Vec<i32> = Vec::new();
        let mut rng = rand::rng();
        assert!(matches!(Worst.select(&pop, &mut rng), Err(EmptyPopulation)));
    }

    #[test]
    fn can_select_twice() {
        // Currently `.select()` can't take an array, so we need to make this a `Vec`.
        // Once we've generalized `.select()` appropriately we can change this to be
        // an array. See #259
        let pop = vec![5, 8, 9, 6, 3, 2, 10];
        let mut rng = rand::rng();
        assert_eq!(&2, Worst.select(&pop, &mut rng).unwrap());
        assert_eq!(&2, Worst.select(&pop, &mut rng).unwrap());
    }

    #[proptest]
    fn test_worst_select(#[map(|v: [i32;10]| v.into())] pop: Vec<i32>) {
        let mut rng = rand::rng();
        let smallest = pop.iter().min().unwrap();
        assert_eq!(smallest, Worst.select(&pop, &mut rng).unwrap());
    }
}
