#![allow(clippy::missing_panics_doc)]

use std::{borrow::Borrow, ops::Not, slice::Iter};

use rand::rngs::ThreadRng;
use rayon::prelude::{IntoParallelIterator, ParallelExtend, ParallelIterator, FromParallelIterator};

use crate::individual::Generate;

pub struct VecPop<I> {
    individuals: Vec<I>,
}

impl<I: Generate + Send> VecPop<I> {
    /*
     * See the lengthy comment in `individual.rs` on why we need the
     * whole `Borrow<H>` business.
     */
    pub fn generate<H>(
        pop_size: usize,
        make_genome: impl Fn(&mut ThreadRng) -> I::Genome + Send + Sync,
        run_tests: impl Fn(&H) -> I::TestResults + Send + Sync,
    ) -> Self
    where
        I::Genome: Borrow<H>,
        H: ?Sized,
    {
        let mut individuals = Vec::with_capacity(pop_size);
        individuals.par_extend(
            (0..pop_size)
                .into_par_iter()
                .map_init(rand::thread_rng, |rng, _| {
                    I::generate(&make_genome, &run_tests, rng)
                }),
        );
        Self { individuals }
    }
}

impl<I> VecPop<I> {
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.individuals.is_empty()
    }

    #[must_use]
    pub fn size(&self) -> usize {
        self.individuals.len()
    }

    pub fn iter(&self) -> Iter<I> {
        self.individuals.iter()
    }
}

impl<I: Ord> VecPop<I> {
    /// # Panics
    ///
    /// Will panic if the population is empty.
    #[must_use]
    pub fn best_individual(&self) -> &I {
        assert!(self.individuals.is_empty().not());
        #[allow(clippy::unwrap_used)]
        self.individuals.iter().max().unwrap()
    }
}

impl<I> FromIterator<I> for VecPop<I> {
    fn from_iter<T>(iter: T) -> Self 
    where
        T: IntoIterator<Item = I>
    {
        let individuals = iter.into_iter().collect();
        Self { individuals }
    }
}

impl<I: Send> FromParallelIterator<I> for VecPop<I> {
    fn from_par_iter<T>(par_iter: T) -> Self
    where
        T: IntoParallelIterator<Item = I>
    {
        let individuals = par_iter.into_par_iter().collect();
        Self { individuals }
    }
}

#[cfg(test)]
mod vec_pop_tests {
    use rand::RngCore;

    use crate::individual::{ec::EcIndividual, Individual};

    use super::*;

    #[test]
    fn new_works() {
        let vec_pop = VecPop::<EcIndividual<_, _>>::generate(
            10, 
            |rng| rng.next_u32() % 20,
            |g| (*g)+100
        );
        assert!(vec_pop.is_empty().not());
        assert_eq!(10, vec_pop.size());
        let ind = vec_pop.iter().next().unwrap();
        assert!(*ind.genome() < 20);
        assert!(100 <= *ind.test_results() && *ind.test_results() < 120);
    }

    #[test]
    fn from_iter() {
        let first_ind = EcIndividual::new("First".to_string(), vec![5, 8, 9]);
        let second_ind = EcIndividual::new("Second".to_string(), vec![3, 2, 0]);
        let third_ind = EcIndividual::new("Third".to_string(), vec![6, 3, 2]);
        let inds = vec![first_ind, second_ind, third_ind];
        let vec_pop = VecPop::from_iter(inds.clone());
        assert!(vec_pop.is_empty().not());
        assert_eq!(3, vec_pop.size());
        let pop_inds: Vec<EcIndividual<String, Vec<i32>>> = vec_pop.iter().cloned().collect();
        assert_eq!(inds, pop_inds);
    }
}