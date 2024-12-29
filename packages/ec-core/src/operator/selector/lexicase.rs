use std::{cmp::Ordering, mem::swap, ops::Not};

use rand::{prelude::SliceRandom, rngs::ThreadRng};

use super::{Selector, error::EmptyPopulation};
use crate::{individual::Individual, population::Population, test_results::TestResults};

#[derive(Debug)]
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
    R: Ord,
{
    type Error = EmptyPopulation;

    fn select<'pop>(
        &self,
        population: &'pop P,
        rng: &mut ThreadRng,
    ) -> Result<&'pop P::Individual, Self::Error> {
        // Candidate set is initially the whole population.
        // Shuffle the (indices of the) test cases.
        // For each test in turn:
        //   * Find the best score of any individual still in in the candidate set on
        //     that test case.
        //   * Remove any individual from the candidate set that is worse than that best
        //     score on that test case.
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
            let mut current_best_result = &winners[0].test_results().results[test_case_index];

            for c in &candidates[1..] {
                let this_result = &c.test_results().results[test_case_index];
                match this_result.cmp(current_best_result) {
                    // If `c` is strictly less (worse) than the current winner
                    // it's removed from consideration by not doing
                    // anything with it (i.e., not adding it to the
                    // set of `winners`).
                    Ordering::Less => {}
                    // If `c` is equal (on this test case) to the
                    // current winner, then it is added to the
                    // set of potential `winners`.
                    Ordering::Equal => winners.push(c),
                    // If `c` is greater (better) than the current winner
                    // then we clear the current set of winners (since they're
                    // "worse" than `c`), and add `c` to the set of `winners`.
                    // We also have to update `current_best_result` to now
                    // be `this_result` since it's the new best.
                    Ordering::Greater => {
                        winners.clear();
                        winners.push(c);
                        current_best_result = this_result;
                    }
                }
            }
            swap(&mut candidates, &mut winners);
        }

        candidates.shuffle(rng);
        candidates.first().copied().ok_or(EmptyPopulation)
    }
}

#[cfg(test)]
#[expect(
    clippy::unwrap_used,
    reason = "Panicking is the best way to deal with errors in unit tests"
)]
mod tests {
    use std::iter::once;

    use proptest::{
        collection,
        prelude::{Strategy, any},
        prop_assert, prop_assert_eq,
    };
    use test_strategy::proptest;

    use super::*;
    use crate::individual::ec::EcIndividual;

    #[test]
    fn empty_population() {
        let pop = population_from_single_scores([]);
        let mut rng = rand::thread_rng();
        let selector = Lexicase::new(0);
        assert!(matches!(
            selector.select(&pop, &mut rng),
            Err(EmptyPopulation)
        ));
    }

    // Generate a population from a vector of single scores, one per individual.
    // These have to be converted to iterators using `once` so they're "like"
    // arrays of scores.
    fn population_from_single_scores(
        scores: impl IntoIterator<Item = i32>,
    ) -> Vec<EcIndividual<usize, TestResults<i32>>> {
        // Mapping `once` here converts each single `i32` value into
        // an iterator over `i32`s.
        population_from_scores(scores.into_iter().map(once))
    }

    // Generate a population from a collection of score collections (e.g., arrays),
    // one per individual.
    fn population_from_scores(
        scores: impl IntoIterator<Item: IntoIterator<Item = i32>>,
    ) -> Vec<EcIndividual<usize, TestResults<i32>>> {
        scores
            .into_iter()
            .map(TestResults::<i32>::from)
            // We'll use the index as the "genome".
            .enumerate()
            .map(EcIndividual::from)
            .collect::<Vec<_>>()
    }

    #[test]
    fn single_best_single_error() {
        // 9 is the sole winner in this population
        let population = population_from_single_scores([5, 8, 9, 6, 3, 2, 0]);

        let lexicase = Lexicase::new(1);
        let mut rng = rand::thread_rng();

        assert_eq!(&2, lexicase.select(&population, &mut rng).unwrap().genome());
    }

    #[test]
    fn multiple_copies_best() {
        // The two 9s are the possible winners
        let population = population_from_single_scores([5, 8, 9, 6, 3, 2, 0, 9]);

        let lexicase = Lexicase::new(1);
        let mut rng = rand::thread_rng();

        let selected = *lexicase.select(&population, &mut rng).unwrap().genome();
        assert!(
            [2usize, 7usize].contains(&selected),
            "genome {selected} should have been 2 or 7"
        );
    }

    #[test]
    fn single_best_multiple_errors() {
        // [9, 8] is the sole winner
        let population =
            population_from_scores([[5, 3], [8, 2], [9, 8], [6, 2], [3, 8], [2, 8], [0, 6]]);

        let lexicase = Lexicase::new(2);
        let mut rng = rand::thread_rng();

        assert_eq!(&2, lexicase.select(&population, &mut rng).unwrap().genome());
    }

    #[test]
    fn multiple_best_multiple_errors() {
        let population = population_from_scores([
            [5, 3],
            [8, 2],
            [9, 8], // A possible winner
            [6, 2],
            [3, 8],
            [2, 8],
            [0, 6],
            [7, 9], // A possible winner
        ]);

        let lexicase = Lexicase::new(2);
        let mut rng = rand::thread_rng();

        let selected = *lexicase.select(&population, &mut rng).unwrap().genome();
        assert!(
            [2usize, 7usize].contains(&selected),
            "genome {selected} should have been 2 or 7"
        );
    }

    // A name for this type to simplify things in the `proptest` test
    // below.
    type TestIndividual = EcIndividual<String, TestResults<u16>>;

    // This test uses `proptest` to generate a random set of between 1 and 20
    // vectors of two scores, and then converts those into a vector of
    // `EcIndividual`s (i.e., a population). I then determine the largest of the
    // first values (i.e., the highest score on the first test case), and then the
    // largest of the second values. I make a new set of results that is those
    // highest values with one (randomly chosen by proptest) incremented by 1.
    // This new result is guaranteed to be "better" than any other in the list,
    // so it should be what is selected after I add it to the population.
    //
    // There are several helper functions that exist just to support the somewhat
    // complex logic of this test.
    #[proptest]
    fn selects_sole_best(
        #[strategy(1..=20usize)] pop_size: usize,
        #[strategy(make_pop(#pop_size))] mut population: Vec<TestIndividual>,
        #[strategy(0..=1usize)] score_to_increase: usize,
    ) {
        prop_assert!(pop_size > 0);
        prop_assert_eq!(pop_size, population.len());

        let (winning_label, winning_individual) =
            make_winning_individual(&population, score_to_increase);

        population.push(winning_individual);

        let num_test_cases = population[0].test_results.len();
        let lexicase = Lexicase::new(num_test_cases);
        let mut rng = rand::thread_rng();

        let selected = lexicase.select(&population, &mut rng).unwrap().genome();
        prop_assert_eq!(selected, &winning_label);
    }

    // A function that creates a `proptest` `Strategy`` that creates a random
    // population of individuals, each with two scores. The "genome" of each
    // individual is just a string version of their pair of scores, i.e.,
    // "[2, 3]".
    fn make_pop(pop_size: usize) -> impl Strategy<Value = Vec<TestIndividual>> {
        collection::vec([any::<u8>(), any::<u8>()], pop_size).prop_map(|pop_scores| {
            pop_scores
                .into_iter()
                .map(|scores| EcIndividual::new(format!("{scores:?}"), TestResults::from(scores)))
                .collect::<Vec<_>>()
        })
    }

    // Constructs what is guaranteed to be the "winning" individual
    // that lexicase selection will return regardless of the ordering
    // of the test cases. This is generated by finding the largest
    // values for each of the test cases, and incrementing one of
    // them (as determined by `score_to_increase`) by one. This
    // ensures that this individual will tie with the best on
    // one score, and win on the other.
    fn make_winning_individual(
        population: &[TestIndividual],
        score_to_increase: usize,
    ) -> (String, TestIndividual) {
        let first_max = largest_test_case_value(population, 0);
        let second_max = largest_test_case_value(population, 1);
        // After incrementing one of the values by 1, this is guaranteed to
        // be "better" than any other score, and will thus be the value
        // selected.
        let mut winning_scores = [first_max, second_max];
        winning_scores[score_to_increase] =
            winning_scores[score_to_increase].checked_add(1).unwrap();
        let winning_label = format!("{winning_scores:?}");
        let winning_individual: TestIndividual =
            EcIndividual::new(winning_label.clone(), TestResults::from(winning_scores));
        (winning_label, winning_individual)
    }

    // Get the largest test case value in the given population from the test case
    // specified by `index`.
    fn largest_test_case_value(population: &[TestIndividual], index: usize) -> u16 {
        population
            .iter()
            .map(|i| i.test_results.results[index])
            .max()
            .unwrap()
    }
}
