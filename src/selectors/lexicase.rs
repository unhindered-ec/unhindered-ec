use std::{ops::Not, mem::swap};

use rand::prelude::SliceRandom;
use rand::rngs::ThreadRng;

use crate::{test_results::TestResults, population::VecPop, individual::Individual};

use super::SelectorI;

pub struct Lexicase {
    num_test_cases: usize,
}

impl Lexicase {
    #[must_use]
    pub const fn new(num_test_cases: usize) -> Self {
        Self { num_test_cases }
    }
}

impl<R, I> SelectorI<I> for Lexicase 
where
    R: PartialEq + PartialOrd,
    I: Individual<TestResults = TestResults<R>>,
{
    fn select<'a>(
        &self,
        rng: &mut ThreadRng,
        population: &'a VecPop<I>,
    ) -> &'a I {
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

        let mut candidates: Vec<_> = population.iter().collect();

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
