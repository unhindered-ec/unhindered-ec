#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

use std::{borrow::Borrow, iter::from_fn};
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

impl<T> Population<T> {
    // TODO: This iterator is sequential, and we might want it to become parallel,
    // if that is feasible. I'll need to better understand how to implement Rayon's
    // parallel iterator trait.
    pub fn selection_iter<'a>(&'a self, selectors: &'a Vec<&'a Selector<T>>) -> impl Iterator<Item = &'a Individual<T>> {
        // TODO: We might want or need to move this somewhere else if we convert
        // to parallel iterators.
        // println!("We're calling selection_iter");
        from_fn(move || {
            let mut rng = rand::thread_rng();
            let s = selectors.choose(&mut rng)?;
            s(self)
        })
    }
}

impl<T> Population<T> {
    #[must_use]
    pub fn best_score(&self) -> Option<&Individual<T>> {
        assert!(!self.individuals.is_empty());
        self.individuals.iter().max_by_key(|ind| ind.score)
    }

    pub fn random(&self) -> Option<&Individual<T>> {
        self.individuals.choose(&mut rand::thread_rng())
    }
}


