#![allow(clippy::missing_panics_doc)]

use std::borrow::Borrow;
use rand::seq::SliceRandom;

use rand::rngs::ThreadRng;
use rayon::prelude::{ParallelExtend, IntoParallelIterator, ParallelIterator};

use crate::individual::Individual;

pub struct Population<G, R> {
    pub individuals: Vec<Individual<G, R>>,
}

impl<G: Send, R: Send> Population<G, R> {
    /*
     * See the lengthy comment in `individual.rs` on why we need the
     * whole `Borrow<H>` business.
     */
    pub fn new<H>(
            pop_size: usize,
            make_genome: impl Fn(&mut ThreadRng) -> G + Send + Sync, 
            run_tests: impl Fn(&H) -> R + Send + Sync) 
        -> Self
    where
        G: Borrow<H>,
        H: ?Sized
    {
        let mut individuals = Vec::with_capacity(pop_size);
        individuals.par_extend((0..pop_size)
            .into_par_iter()
            .map_init(
                rand::thread_rng,
                |rng, _| {
                    Individual::new(&make_genome, &run_tests, rng)
                })
        );
        Self {
            individuals,
        }
    }
}

impl<G, R> Population<G, R> {
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.individuals.is_empty()
    }

    #[must_use]
    pub fn size(&self) -> usize {
        self.individuals.len()
    }
}

impl<G: Eq, R: Ord> Population<G, R> {
    /// # Panics
    /// 
    /// Will panic if the population is empty.
    #[must_use]
    pub fn best_individual(&self) -> &Individual<G, R> {
        assert!(!self.individuals.is_empty());
        #[allow(clippy::unwrap_used)]
        self
            .individuals
            .iter()
            .max()
            .unwrap()
    }
}

impl<G, R> Population<G, R> {
    #[must_use]
    pub fn random(&self) -> Option<&Individual<G, R>> {
        self.individuals.choose(&mut rand::thread_rng())
    }
}

impl<G: Eq, R: Ord> Population<G, R> {
    pub fn make_tournament_selector(tournament_size: usize) -> impl Fn(&Self) -> &Individual<G, R> {
        move |pop: &Self| {
            pop.tournament(tournament_size)
        }
    }

    #[must_use]
    pub fn tournament(&self, tournament_size: usize) -> &Individual<G, R> {
        assert!(self.individuals.len()>=tournament_size && tournament_size>0);
        // Since we know that the population and tournament aren't empty, we
        // can safely unwrap() the `.max()` call.
        #[allow(clippy::unwrap_used)]
        self.individuals
            .choose_multiple(&mut rand::thread_rng(), tournament_size)
            .max()
            .unwrap()
    }
}
