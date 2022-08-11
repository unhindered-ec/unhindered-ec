use std::borrow::Borrow;

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
            compute_fitness: impl Fn(&R) -> i64 + Send + Sync) 
        -> Population<T>
    where
        T: Borrow<R>,
        R: ?Sized
    {
        let mut pop = Vec::with_capacity(pop_size);
        pop.par_extend((0..pop_size)
            .into_par_iter()
            .map_init(
                rand::thread_rng,
                |rng, _| {
                    Individual::new(&make_genome, &compute_fitness, rng)
                })
        );
        // let mut rng = rand::thread_rng();
        // for _ in 0..pop_size {
        //     let ind = Individual::new(bit_length, &mut rng);
        //     pop.push(ind);
        // }
        Population {
            individuals: pop,
        }
    }
}

impl<T> Population<T> {
    pub fn best_fitness(&self) -> &Individual<T> {
        assert!(!self.individuals.is_empty());
        self.individuals.iter().max_by_key(
                |ind| ind.fitness
            ).unwrap()
    }
}