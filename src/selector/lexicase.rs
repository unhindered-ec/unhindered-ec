use std::{mem::swap, ops::Not};

use rand::prelude::SliceRandom;
use rand::rngs::ThreadRng;

use crate::{individual::Individual, population::Population, test_results::TestResults};

use super::Selector;

pub struct Lexicase {
    num_test_cases: usize,
}

impl Lexicase {
    #[must_use]
    pub const fn new(num_test_cases: usize) -> Self {
        Self { num_test_cases }
    }
}

impl<P, R> Selector<P> for Lexicase
where
    P: Population,
    // TODO: We don't really use the iterator here as we immediately
    //   `.collect()` to get a `Vec`. Maybe the constraint should be
    //   more specific to our needs, like a `Into<Vec>` constraint
    //   that says that our population needs to be convertible into
    //   a `Vec` of individuals.
    //   The concern (from esitsu@Twitch) is that the current setup
    //   will work with populations that are "bare" `Vec`s, where if
    //   we add this alternative constraint we won't be able to use
    //   bare `Vec`s and will be forced to wrap them like we currently
    //   do with `VecPop`.
    for<'pop> &'pop P: IntoIterator<Item = &'pop P::Individual>,
    P::Individual: Individual<TestResults = TestResults<R>>,
    R: Ord
{
    fn select<'pop>(&self, rng: &mut ThreadRng, population: &'pop P) -> &'pop P::Individual {
        // Candidate set is initially the whole population.
        // Shuffle the (indices of the) test cases.
        // For each test in turn:
        //   * Find the best score of any individual still in
        //     in the candidate set on that test case.
        //   * Remove any individual from the candidate set that
        //     is worse than that best score on that test case.
        // Go until you get to a single individual or you run
        // out of test cases.
        let mut case_indices: Vec<usize> = (0..self.num_test_cases).collect();
        case_indices.shuffle(rng);

        let mut candidates: Vec<_> = population.into_iter().collect();

        let mut winners = Vec::with_capacity(candidates.len());
        for test_case_index in case_indices {
            assert!(
                candidates.is_empty().not(),
                "The set of lexicase candidates shouldn't be empty"
            );
            if candidates.len() == 1 {
                break;
            }
            winners.clear();
            winners.push(candidates[0]);
            for c in &candidates[1..] {
                // I find the `if-else` to be easier to read than Clippy's preferred
                // use of `match`.
                #[allow(clippy::comparison_chain)]
                if c.test_results().results[test_case_index]
                    > winners[0].test_results().results[test_case_index]
                {
                    winners.clear();
                    winners.push(c);
                } else if c.test_results().results[test_case_index]
                    == winners[0].test_results().results[test_case_index]
                {
                    winners.push(c);
                }
            }
            swap(&mut candidates, &mut winners);
        }

        candidates.shuffle(rng);
        candidates[0]
    }
}
