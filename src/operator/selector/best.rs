use std::ops::Not;

use rand::rngs::ThreadRng;

use crate::population::Population;

use super::Selector;

pub struct Best;

impl<P> Selector<P> for Best
where
    P: Population,
    for<'pop> &'pop P: IntoIterator<Item = &'pop P::Individual>,
    P::Individual: Ord,
{
    fn select<'pop>(&self, population: &'pop P, _: &mut ThreadRng) -> &'pop P::Individual {
        // The population should never be empty here.
        assert!(
            population.is_empty().not(),
            "The population should not be empty"
        );
        #[allow(clippy::unwrap_used)]
        population.into_iter().max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_select_twice() {
        let pop = vec![5, 8, 9, 6, 3, 2, 0];
        let mut rng = rand::thread_rng();
        assert_eq!(&9, Best.select(&pop, &mut rng));
        assert_eq!(&9, Best.select(&pop, &mut rng));
    }
}
