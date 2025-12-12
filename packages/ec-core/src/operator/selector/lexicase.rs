use std::cmp::Ordering;

use miette::Diagnostic;
use rand::{
    Rng,
    distr::uniform::{UniformSampler, UniformUsize},
    seq::IndexedRandom,
};

use super::{Selector, error::EmptyPopulation};
use crate::{individual::Individual, population::Population, test_results::TestResults};

/// Lexicase selector.
///
/// This selector works by selecting a random test case and throwing out all
/// individuals that don't have the best score there, repeating that until only
/// one individual is left or all test cases have been compared.
/// In case more than one individual is left, one is selected randomly.
///
/// In case there are no test cases (i.e `num_test_cases` is zero) this devolves
/// to a random selection.
///
/// # Example
/// ```
/// # use ec_core::{
/// #     individual::ec::EcIndividual,
/// #     operator::selector::{Selector, lexicase::Lexicase},
/// #     test_results::TestResults
/// # };
/// let population = [
///     EcIndividual::new(100, TestResults::<i32>::from_iter([4, 10])),
///     EcIndividual::new(100, TestResults::<i32>::from_iter([2, 10])),
///     EcIndividual::new(100, TestResults::<i32>::from_iter([4, 5])),
///     EcIndividual::new(100, TestResults::<i32>::from_iter([2, 5])),
/// ];
///
/// let lexicase = Lexicase::new(2);
///
/// let selected = lexicase.select(&population, &mut rand::rng())?;
///
/// assert_eq!(
///     selected,
///     &EcIndividual::new(100, TestResults::from_iter([4, 10]))
/// );
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Lexicase {
    num_test_cases: usize,
}

impl Lexicase {
    /// Create a new Lexicase selector using the given number of test cases.
    ///
    /// Make sure your population you are intending to select from actually has
    /// this number of test cases, or else selection might error.
    ///
    /// # Example
    /// ```
    /// # use ec_core::{
    /// #     individual::ec::EcIndividual,
    /// #     operator::selector::{Selector, lexicase::Lexicase},
    /// #     test_results::TestResults
    /// # };
    /// let population = [
    ///     EcIndividual::new(100, TestResults::<i32>::from_iter([4, 10])),
    ///     EcIndividual::new(100, TestResults::<i32>::from_iter([2, 10])),
    ///     EcIndividual::new(100, TestResults::<i32>::from_iter([4, 5])),
    ///     EcIndividual::new(100, TestResults::<i32>::from_iter([2, 5])),
    /// ];
    ///
    /// let lexicase = Lexicase::new(2);
    ///
    /// # let selected = lexicase.select(&population, &mut rand::rng())?;
    /// #
    /// # assert_eq!(
    /// #     selected,
    /// #     &EcIndividual::new(100, TestResults::from_iter([4, 10]))
    /// # );
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[must_use]
    pub const fn new(num_test_cases: usize) -> Self {
        Self { num_test_cases }
    }
}

/// Errors that can occur during [`Lexicase`] selection:
/// - [`EmptyPopulation`] when trying to select from an empty population, or
/// - `MissingTestCase` when using a lexicase selector with a higher number of
///   test cases set than every individual in the population actually provides.
#[derive(Debug, thiserror::Error, Diagnostic, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LexicaseError {
    #[error(transparent)]
    #[diagnostic(transparent)]
    EmptyPopulation(#[from] EmptyPopulation),
    #[error(
        "Expected {total_cases} test cases but couldn't access test case result at index \
         {current_index}"
    )]
    #[diagnostic(help(
        "Make sure the configured lexicase test case count ({total_cases}) matches your actual \
         test case count"
    ))]
    MissingTestCase {
        total_cases: usize,
        current_index: usize,
    },
}

impl<P, Res> Selector<P> for Lexicase
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
    P::Individual: Individual<TestResults = TestResults<Res>>,
    Res: Ord,
{
    type Error = LexicaseError;

    /// Select an Individual from the given Population using this selector.
    ///
    /// # Example
    /// ```
    /// # use ec_core::{
    /// #     individual::ec::EcIndividual,
    /// #     operator::selector::{Selector, lexicase::Lexicase},
    /// #     test_results::TestResults
    /// # };
    /// let population = [
    ///     EcIndividual::new(100, TestResults::<i32>::from_iter([4, 10])),
    ///     EcIndividual::new(100, TestResults::<i32>::from_iter([2, 10])),
    ///     EcIndividual::new(100, TestResults::<i32>::from_iter([4, 5])),
    ///     EcIndividual::new(100, TestResults::<i32>::from_iter([2, 5])),
    /// ];
    ///
    /// let lexicase = Lexicase::new(2);
    ///
    /// let selected = lexicase.select(&population, &mut rand::rng())?;
    ///
    /// assert_eq!(
    ///     selected,
    ///     &EcIndividual::new(100, TestResults::from_iter([4, 10]))
    /// );
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// # Errors
    /// - [`LexicaseError::EmptyPopulation`] if trying to select from an empty
    ///   population
    /// - [`LexicaseError::MissingTestCase`] if passing a population where an
    ///   individual has fewer test cases than the given `num_test_cases` from
    ///   this selector.
    fn select<'pop, R: Rng + ?Sized>(
        &self,
        population: &'pop P,
        rng: &mut R,
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

        let mut candidates: Vec<_> = population.into_iter().collect();
        let mut winners = Vec::with_capacity(candidates.len());

        while let Some(test_case_index) = UniformUsize::sample_single(0, case_indices.len(), rng)
            .ok()
            .map(|idx| case_indices.swap_remove(idx))
        {
            let (&initial_winner, remaining) = candidates.split_first().ok_or(EmptyPopulation)?;

            if remaining.is_empty() {
                break;
            }

            winners.clear();
            winners.push(initial_winner);

            let mut current_best_result = initial_winner
                .test_results()
                .get(test_case_index)
                .ok_or(LexicaseError::MissingTestCase {
                    total_cases: self.num_test_cases,
                    current_index: test_case_index,
                })?;

            for c in remaining {
                let this_result = c.test_results().get(test_case_index).ok_or(
                    LexicaseError::MissingTestCase {
                        total_cases: self.num_test_cases,
                        current_index: test_case_index,
                    },
                )?;

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
            std::mem::swap(&mut candidates, &mut winners);
        }

        candidates
            .choose(rng)
            .copied()
            .ok_or(EmptyPopulation)
            .map_err(Into::into)
    }
}

#[cfg(test)]
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
        let mut rng = rand::rng();
        let selector = Lexicase::new(0);
        assert!(matches!(
            selector.select(&pop, &mut rng),
            Err(LexicaseError::EmptyPopulation(EmptyPopulation))
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
            .map(TestResults::<i32>::from_iter)
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
        let mut rng = rand::rng();

        assert_eq!(&2, lexicase.select(&population, &mut rng).unwrap().genome());
    }

    #[test]
    fn multiple_copies_best() {
        // The two 9s are the possible winners
        let population = population_from_single_scores([5, 8, 9, 6, 3, 2, 0, 9]);

        let lexicase = Lexicase::new(1);
        let mut rng = rand::rng();

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
        let mut rng = rand::rng();

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
        let mut rng = rand::rng();

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
        let mut rng = rand::rng();

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
                .map(|scores| {
                    EcIndividual::new(format!("{scores:?}"), TestResults::from_iter(scores))
                })
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
        let winning_individual: TestIndividual = EcIndividual::new(
            winning_label.clone(),
            TestResults::from_iter(winning_scores),
        );
        (winning_label, winning_individual)
    }

    // Get the largest test case value in the given population from the test case
    // specified by `index`.
    fn largest_test_case_value(population: &[TestIndividual], index: usize) -> u16 {
        population
            .iter()
            .map(|i| i.test_results[index])
            .max()
            .unwrap()
    }
}
