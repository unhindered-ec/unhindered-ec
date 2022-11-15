use std::{mem::swap, ops::Not};

use rand::{rngs::ThreadRng, seq::SliceRandom};

use crate::{individual::Individual, population::Population, test_results::TestResults};

// TODO: Is there a circumstance where selection should fail? If so, do we want to have
//  it return `Option<Individual>` or even `Result<Individual, Error>`? Not sure.
//  esitsu@Twitch suggested, for example, having a selector with a threshhold and then
//  a composite that keeps trying selectors until it finds one that works.
pub trait Selector<G, R>: Sync {
    fn select<'a>(
        &self,
        rng: &mut ThreadRng,
        population: &'a Population<G, R>,
    ) -> &'a Individual<G, R>;
}

pub struct Random {}

impl<G, R: Ord> Selector<G, R> for Random {
    #[must_use]
    fn select<'a>(
        &self,
        rng: &mut ThreadRng,
        population: &'a Population<G, R>,
    ) -> &'a Individual<G, R> {
        // The population should never be empty here.
        assert!(
            population.individuals.is_empty().not(),
            "The population should not be empty"
        );
        #[allow(clippy::unwrap_used)]
        population.individuals.choose(rng).unwrap()
    }
}

pub struct Best {}

impl<G: Eq, R: Ord> Selector<G, R> for Best {
    #[must_use]
    fn select<'a>(
        &self,
        _: &mut ThreadRng,
        population: &'a Population<G, R>,
    ) -> &'a Individual<G, R> {
        // The population should never be empty here.
        assert!(
            population.individuals.is_empty().not(),
            "The population should not be empty"
        );
        population.best_individual()
    }
}

pub struct Tournament {
    size: usize,
}

impl Tournament {
    #[must_use]
    pub const fn new(size: usize) -> Self {
        Self { size }
    }
}

impl<G: Eq, R: Ord> Selector<G, R> for Tournament {
    fn select<'a>(
        &self,
        rng: &mut ThreadRng,
        population: &'a Population<G, R>,
    ) -> &'a Individual<G, R> {
        assert!(population.individuals.len() >= self.size && self.size > 0);
        // Since we know that the population and tournament aren't empty, we
        // can safely unwrap() the `.max()` call.
        #[allow(clippy::unwrap_used)]
        population
            .individuals
            .choose_multiple(rng, self.size)
            .max()
            .unwrap()
    }
}

pub struct Lexicase {
    num_test_cases: usize,
}

impl Lexicase {
    #[must_use]
    pub const fn new(num_test_cases: usize) -> Self {
        Self { num_test_cases }
    }
}

impl<G, R: Ord> Selector<G, TestResults<R>> for Lexicase {
    fn select<'a>(
        &self,
        rng: &mut ThreadRng,
        population: &'a Population<G, TestResults<R>>,
    ) -> &'a Individual<G, TestResults<R>> {
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

        let mut candidates: Vec<_> = population.individuals.iter().collect();

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
                if c.test_results.results[test_case_index]
                    > winners[0].test_results.results[test_case_index]
                {
                    winners.clear();
                    winners.push(c);
                } else if c.test_results.results[test_case_index]
                    == winners[0].test_results.results[test_case_index]
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

pub struct Weighted<'a, G, R> {
    selectors: Vec<(&'a dyn Selector<G, R>, usize)>,
}

impl<'a, G, R> Weighted<'a, G, R> {
    // Since we should never have an empty collection of weighted selectors,
    // the `new` implementation takes an initial selector so `selectors` is
    // guaranteed to never be empty.
    #[must_use]
    pub fn new(selector: &'a dyn Selector<G, R>, weight: usize) -> Self {
        Self {
            selectors: vec![(selector, weight)],
        }
    }

    #[must_use]
    pub fn with_selector(mut self, selector: &'a dyn Selector<G, R>, weight: usize) -> Self {
        self.selectors.push((selector, weight));
        self
    }
}

impl<'a, G, R> Selector<G, R> for Weighted<'a, G, R> {
    fn select<'b>(
        &self,
        rng: &mut ThreadRng,
        population: &'b Population<G, R>,
    ) -> &'b Individual<G, R> {
        assert!(
            self.selectors.is_empty().not(),
            "The collection of selectors should be non-empty"
        );
        #[allow(clippy::unwrap_used)]
        let (selector, _) = self.selectors.choose_weighted(rng, |(_, w)| *w).unwrap();
        selector.select(rng, population)
    }
}
