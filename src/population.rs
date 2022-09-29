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
            compute_score: impl Fn(&R) -> Vec<i64> + Send + Sync) 
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

impl<T> Population<T> {
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.individuals.is_empty()
    }

    #[must_use]
    pub fn size(&self) -> usize {
        self.individuals.len()
    }

    /// # Panics
    /// 
    /// Will panic if the population is empty.
    #[must_use]
    pub fn best_individual(&self) -> &Individual<T> {
        assert!(!self.individuals.is_empty());
        #[allow(clippy::unwrap_used)]
        self
            .individuals
            .iter()
            .max_by_key(
                |ind| ind.total_score
            )
            .unwrap()
    }
}

impl<T> Population<T> {
    /// # Panics
    ///
    /// This will panic if the population is empty.
    #[must_use]
    pub fn best_score(&self) -> Option<&Individual<T>> {
        assert!(!self.individuals.is_empty());
        self.individuals.iter().max_by_key(|ind| ind.total_score)
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
            .max_by_key(|ind| ind.total_score)
    }
}


