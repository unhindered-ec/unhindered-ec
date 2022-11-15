#![allow(clippy::missing_panics_doc)]

use std::{borrow::Borrow, ops::Not};

use rand::rngs::ThreadRng;
use rayon::prelude::{IntoParallelIterator, ParallelExtend, ParallelIterator};

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
        run_tests: impl Fn(&H) -> R + Send + Sync,
    ) -> Self
    where
        G: Borrow<H>,
        H: ?Sized,
    {
        let mut individuals = Vec::with_capacity(pop_size);
        individuals.par_extend(
            (0..pop_size)
                .into_par_iter()
                .map_init(rand::thread_rng, |rng, _| {
                    Individual::new(&make_genome, &run_tests, rng)
                }),
        );
        Self { individuals }
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
        assert!(self.individuals.is_empty().not());
        #[allow(clippy::unwrap_used)]
        self.individuals.iter().max().unwrap()
    }
}
