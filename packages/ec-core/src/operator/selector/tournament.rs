use anyhow::{ensure, Context, Result};
use rand::{prelude::IndexedRandom, rngs::ThreadRng};

use super::Selector;
use crate::population::Population;

pub struct Tournament {
    size: usize,
}

impl Tournament {
    #[must_use]
    pub const fn new(size: usize) -> Self {
        Self { size }
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
            population.size() >= self.size,
            "The population had size {} and we wanted a tournament of size {}",
            population.size(),
            self.size
        );
        population
            .as_ref()
            .choose_multiple(rng, self.size)
            .max()
            .with_context(|| format!("The tournament was empty; should have been {}", self.size))
    }
}

#[cfg(test)]
#[rustversion::attr(before(1.81), allow(clippy::arithmetic_side_effects))]
#[rustversion::attr(
    since(1.81),
    expect(
        clippy::arithmetic_side_effects,
        reason = "The tradeoff safety <> ease of writing arguably lies on the ease of writing \
                  side for test code."
    )
)]
#[rustversion::attr(before(1.81), allow(clippy::unwrap_used))]
#[rustversion::attr(
    since(1.81),
    expect(
        clippy::unwrap_used,
        reason = "Panicking is the best way to deal with errors in unit tests"
    )
)]
mod tests {
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
        let selector = Tournament::new(1);
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
        let selector = Tournament::new(2);
        let winner = selector.select(&population, &mut rng).unwrap();
        assert_eq!(winner.test_results, &x.max(y));
    }

    #[proptest]
    fn tournament_size_2_pop_size_3(
        #[strategy(-1000..1000)] x: i32,
        #[strategy(-1000..1000)] y: i32,
        #[strategy(-1000..1000)] z: i32,
    ) {
        let mut rng = thread_rng();
        let scores = &[3 * x, 3 * y + 1, 3 * z + 2];
        let population = scores
            .iter()
            .enumerate()
            .map(|(genome, score)| EcIndividual::new(genome, score))
            .collect::<Vec<_>>();
        let selector = Tournament::new(2);
        let winner = selector.select(&population, &mut rng).unwrap();
        assert!(scores.contains(winner.test_results));
        assert!(winner.test_results > scores.iter().min().unwrap());
    }
}
