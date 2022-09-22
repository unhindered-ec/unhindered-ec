#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

use std::borrow::Borrow;
use rand::seq::SliceRandom;

use rand::rngs::ThreadRng;
use rayon::prelude::{ParallelExtend, IntoParallelIterator, ParallelIterator};

use crate::individual::Individual;

pub struct Population<T> {
    pub individuals: Vec<Individual<T>>,
}

impl<T: Send> Population<T> {
    /*
     * See the lengthy comment in `individual.rs` on why we need the
     * whole `Borrow<R>` business.
     */
    pub fn new<R>(
            pop_size: usize,
            make_genome: impl Fn(&mut ThreadRng) -> T + Send + Sync, 
            compute_score: impl Fn(&R) -> i64 + Send + Sync) 
        -> Self
    where
        T: Borrow<R>,
        R: ?Sized
    {
        let mut individuals = Vec::with_capacity(pop_size);
        individuals.par_extend((0..pop_size)
            .into_par_iter()
            .map_init(
                rand::thread_rng,
                |rng, _| {
                    Individual::new(&make_genome, &compute_score, rng)
                })
        );
        Self {
            individuals,
        }
    }
}

pub type Selector<T> = dyn Fn(&Population<T>) -> Option<&Individual<T>> + Sync + Send;

// TODO: Should this just become part of the `Population` type?
//   We could provide a set of selectors in the constructor (or
//   a builder) for `Population`, and then just have a `get_parent()`
//   method there.
// TODO: Extend this to a WeightedParentSelector that is essentially
//   a wrapper around `rand::distributions::WeightedChoice` so we can
//   provide weights on the different selectors.
pub struct ParentSelector<'a, T> {
    population: &'a Population<T>,
    selectors: &'a Vec<&'a Selector<T>>,
}

impl<'a, T> ParentSelector<'a, T> {
    fn new(population: &'a Population<T>, selectors: &'a Vec<&'a Selector<T>>) -> Self {
        ParentSelector {
            population,
            selectors
        }
    }

    pub fn get(&self, rng: &mut ThreadRng) -> Option<&'a Individual<T>> {
        let s = self.selectors.choose(rng)?;
        s(self.population)
    }
}

impl<T> Population<T> {
    // TODO: This iterator is sequential, and we might want it to become parallel,
    // if that is feasible. I'll need to better understand how to implement Rayon's
    // parallel iterator trait.
    #[must_use]
    pub fn parent_selector<'a>(&'a self, selectors: &'a Vec<&'a Selector<T>>) -> ParentSelector<T> {
        // TODO: We might want or need to move this somewhere else if we convert
        // to parallel iterators.
        ParentSelector::new(self, selectors)
    }
}

impl<T> Population<T> {
    /// # Panics
    ///
    /// This will panic if the population is empty.
    #[must_use]
    pub fn best_score(&self) -> Option<&Individual<T>> {
        assert!(!self.individuals.is_empty());
        self.individuals.iter().max_by_key(|ind| ind.score)
    }

    #[must_use]
    pub fn random(&self) -> Option<&Individual<T>> {
        self.individuals.choose(&mut rand::thread_rng())
    }

    pub fn make_tournament_selector(tournament_size: usize) -> impl Fn(&Self) -> Option<&Individual<T>> {
        move |pop: &Self| {
            pop.tournament(tournament_size)
        }
    }

    #[must_use]
    pub fn tournament(&self, tournament_size: usize) -> Option<&Individual<T>> {
        self.individuals
            .choose_multiple(&mut rand::thread_rng(), tournament_size)
            .max_by_key(|ind| ind.score)
    }
}


