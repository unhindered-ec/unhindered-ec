use rand::rngs::ThreadRng;
use rayon::prelude::{ParallelExtend, IntoParallelIterator, ParallelIterator};

use crate::individual::Individual;

pub struct Population<T> {
    pub individuals: Vec<Individual<T>>,
}

impl<T: Send> Population<T> {
    pub fn new(
            pop_size: usize,
            make_genome: impl Fn(&mut ThreadRng) -> T + Send + Sync + Clone, 
            compute_fitness: impl Fn(&T) -> i64 + Send + Sync + Clone) 
        -> Population<T>
    {
        let mut pop = Vec::with_capacity(pop_size);
        pop.par_extend((0..pop_size)
            .into_par_iter()
            .map_init(
                rand::thread_rng,
                |rng, _| {
                    Individual::new(make_genome.clone(), compute_fitness.clone(), rng)
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