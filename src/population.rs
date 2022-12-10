#![allow(clippy::missing_panics_doc)]

use std::borrow::Borrow;

use rand::rngs::ThreadRng;
use rayon::prelude::{
    FromParallelIterator, IntoParallelIterator, ParallelExtend, ParallelIterator,
};

use crate::individual::Generate;

pub trait Population {
    type Individual;

    fn is_empty(&self) -> bool {
        self.size() == 0
    }

    fn size(&self) -> usize;
}

impl<I> Population for Vec<I> {
    type Individual = I;

    fn size(&self) -> usize {
        self.len()
    }
}

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

impl<I> Population for VecPop<I> {
    type Individual = I;

    #[must_use]
    fn size(&self) -> usize {
        self.individuals.len()
    }
}

impl<I> VecPop<I> {
    #[deprecated = "After we trait-ify `Population` we should see if we want/need this."]
    #[must_use]
    pub fn slice(&self) -> &[I] {
        &self.individuals
    }

    #[must_use]
    #[deprecated = "We'd rather not expose the details of the implementation like this."]
    pub const fn individuals(&self) -> &Vec<I> {
        &self.individuals
    }
}

impl<'pop, I> IntoIterator for &'pop VecPop<I> {
    type Item = &'pop I;
    type IntoIter = std::slice::Iter<'pop, I>;

    fn into_iter(self) -> Self::IntoIter {
        self.individuals.iter()
    }
}

impl<I> AsRef<[I]> for VecPop<I> {
    fn as_ref(&self) -> &[I] {
        &self.individuals
    }
}

impl<I> FromIterator<I> for VecPop<I> {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = I>,
    {
        let individuals = iter.into_iter().collect();
        Self { individuals }
    }
}

impl<I: Send> FromParallelIterator<I> for VecPop<I> {
    fn from_par_iter<T>(par_iter: T) -> Self
    where
        T: IntoParallelIterator<Item = I>,
    {
        let individuals = par_iter.into_par_iter().collect();
        Self { individuals }
    }
}

#[cfg(test)]
mod vec_pop_tests {
    use std::ops::Not;

    use rand::RngCore;

    use crate::individual::{ec::EcIndividual, Individual};

    use super::*;

    #[test]
    fn new_works() {
        let vec_pop =
            VecPop::<EcIndividual<_, _>>::generate(10, |rng| rng.next_u32() % 20, |g| (*g) + 100);
        assert!(vec_pop.is_empty().not());
        assert_eq!(10, vec_pop.size());
        #[allow(clippy::unwrap_used)] // The population shouldn't be empty
        let ind = vec_pop.into_iter().next().unwrap();
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
        let pop_inds: Vec<EcIndividual<String, Vec<i32>>> = vec_pop.into_iter().cloned().collect();
        assert_eq!(inds, pop_inds);
    }
}
