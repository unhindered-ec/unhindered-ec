// pub type Selector<G, R> = dyn Fn(&Population<G, R>) -> &Individual<G, R> + Sync + Send;
// pub type WeightedSelector<'a, G, R> = (&'a dyn SelectorTrait<G, R>, usize);

use std::mem::swap;

use rand::{rngs::ThreadRng, seq::SliceRandom};

use crate::{population::Population, individual::Individual, test_results::TestResults};

pub trait Selector<G, R>: Sync {
    fn select<'a>(&self, rng: &mut ThreadRng, population: &'a Population<G, R>) -> &'a Individual<G, R>;
}

pub struct Lexicase {
    num_test_cases: usize
}

impl Lexicase {
    #[must_use]
    pub const fn new(num_test_cases: usize) -> Self {
        Self { 
            num_test_cases
        }
    }
}

impl<G, R: Ord> Selector<G, TestResults<R>> for Lexicase {
    fn select<'a>(&self, rng: &mut ThreadRng, population: &'a Population<G, TestResults<R>>) -> &'a Individual<G, TestResults<R>> {
        // Candidate set is initially the whole population.
        // Shuffle the (indices of the) test cases.
        // For each test in turn:
        //   * Find the best score of any individual still in
        //     in the candidate set on that test case.
        //   * Remove any individual from the candidate set that
        //     is worse than that best score on that test case.
        // Go until you get to a single individual or you run
        // out of test cases.

        // TODO: Compute these bits once when the population is initially constructed
        //   and then just look it up when necessary instead of recomputing it
        //   for every selection.
        // let first_individual = &population.individuals[0];
        // let first_results = &first_individual.test_results.results;
        // let num_results = first_results.len();
        let mut case_indices: Vec<usize> = (0..self.num_test_cases).collect();
        case_indices.shuffle(rng);

        let mut candidates: Vec<_> = population.individuals
            .iter()
            .collect();

        let mut winners = Vec::with_capacity(candidates.len());
        for test_case_index in case_indices {
            assert!(!candidates.is_empty(), "The set of lexicase candidates shouldn't be empty");
            if candidates.len() == 1 {
                break;
            }
            winners.clear();
            winners.push(candidates[0]);
            for c in &candidates[1..] {
                // I find the `if-else` to be easier to read than Clippy's preferred
                // use of `match`.
                #[allow(clippy::comparison_chain)]
                if c.test_results.results[test_case_index] > winners[0].test_results.results[test_case_index] {
                    winners.clear();
                    winners.push(c);
                } else if c.test_results.results[test_case_index] == winners[0].test_results.results[test_case_index] {
                    winners.push(c);
                }
            }
            swap(&mut candidates, &mut winners);
        }

        candidates.shuffle(rng);
        candidates[0]
    }
}