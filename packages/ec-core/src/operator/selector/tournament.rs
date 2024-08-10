use std::num::NonZeroUsize;

use anyhow::{ensure, Context, Result};
use rand::{prelude::IndexedRandom, rngs::ThreadRng};

use super::Selector;
use crate::population::Population;

pub struct Tournament {
    size: NonZeroUsize,
}

impl Tournament {
    /// Construct a tournament selector with the given tournament size. This
    /// will select `size` individuals from the population, randomly
    /// without replacement, and return the "best" from that set.
    #[must_use]
    pub const fn new(size: NonZeroUsize) -> Self {
        Self { size }
    }

    /// Construct a tournament selector with the given _constant_
    /// tournament size, allowing for compile-time checks that the
    /// tournament size `N` is greater than zero. This selector
    /// will select `size` individuals from the population, randomly
    /// without replacement, and return the "best" from that set.
    ///
    /// # Examples
    ///
    /// Create a tournament selector with tournament size 3:
    /// ```
    /// # use ec_core::operator::selector::tournament::Tournament;
    /// let selector = Tournament::of_size::<3>();
    /// ```
    #[must_use]
    pub const fn of_size<const N: usize>() -> Self {
        Self::new(
            const {
                match NonZeroUsize::new(N) {
                    Some(x) => x,
                    None => panic!("only positive tournament sizes are permitted"),
                }
            },
        )
    }

    /// Construct a binary tournament selector, i.e., a tournament
    /// selector that selects two random individuals from the population
    /// and returns the "better" of the two.
    #[must_use]
    pub const fn binary() -> Self {
        Self::of_size::<2>()
    }
}

impl<P> Selector<P> for Tournament
where
    P: Population + AsRef<[P::Individual]>,
    P::Individual: Ord,
{
    fn select<'pop>(
        &self,
        population: &'pop P,
        rng: &mut ThreadRng,
    ) -> Result<&'pop P::Individual> {
        ensure!(
            population.size() >= self.size.into(),
            "The population had size {} and we wanted a tournament of size {}",
            population.size(),
            self.size
        );
        population
            .as_ref()
            .choose_multiple(rng, self.size.into())
            .max()
            .with_context(|| format!("The tournament was empty; should have been {}", self.size))
    }
}

#[cfg(test)]
#[rustversion::attr(before(1.81), allow(clippy::unwrap_used))]
#[rustversion::attr(
    since(1.81),
    expect(
        clippy::unwrap_used,
        reason = "`max()` and `min()` can fail if the list of individuals is empty, but we know \
                  that can't happen so we'll unwrap"
    )
)]
mod tests {
    use std::{num::NonZeroUsize, ops::Not};

    use rand::thread_rng;
    use test_strategy::proptest;

    use super::Tournament;
    use crate::{individual::ec::EcIndividual, operator::selector::Selector};

    #[test]
    fn tournament_size_1() {
        let mut rng = thread_rng();
        let scores = &[5, 8, 9];
        let population = scores
            .iter()
            .enumerate()
            .map(|(genome, score)| EcIndividual::new(genome, score))
            .collect::<Vec<_>>();
        let selector = Tournament::new(NonZeroUsize::MIN);
        let winner = selector.select(&population, &mut rng).unwrap();
        assert!(scores.contains(winner.test_results));
    }

    #[proptest]
    fn tournament_size_2_pop_size_2(#[any] x: i32, #[any] y: i32) {
        let mut rng = thread_rng();
        let scores = &[x, y];
        let population = scores
            .iter()
            .enumerate()
            .map(|(genome, score)| EcIndividual::new(genome, score))
            .collect::<Vec<_>>();
        let selector = Tournament::binary();
        let winner = selector.select(&population, &mut rng).unwrap();
        assert_eq!(winner.test_results, &x.max(y));
    }

    #[proptest]
    fn tournament_size_2_pop_size_3(
        #[any] x: i32,
        #[filter(|v| *v != #x)] y: i32,
        #[filter(|v| [#x, #y].contains(v).not())] z: i32,
    ) {
        let mut rng = thread_rng();
        // We know from the filters that all the scores are unique, so the selected
        // score should always be better than the smallest score.
        let scores = &[x, y, z];
        let population = scores
            .iter()
            .enumerate()
            .map(|(genome, score)| EcIndividual::new(genome, score))
            .collect::<Vec<_>>();
        let selector = Tournament::binary();
        let selected = selector.select(&population, &mut rng).unwrap();
        assert!(scores.contains(selected.test_results));
        assert!(selected.test_results > scores.iter().min().unwrap());
    }
}
