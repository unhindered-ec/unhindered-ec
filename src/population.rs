use rayon::prelude::{ParallelExtend, IntoParallelIterator, ParallelIterator};

use crate::individual::Individual;

pub struct Population {
    pub individuals: Vec<Individual>,
}

impl Population {
    pub fn new(pop_size: usize, bit_length: usize) -> Population {
        let mut pop = Vec::with_capacity(pop_size);
        // Using rayon's `par_extend` speeds up the population construction by a factor of 2
        // according to the Criterion benchmark results.
        pop.par_extend((0..pop_size).into_par_iter().map(|_| {
            let mut rng = rand::thread_rng();
            Individual::new(bit_length, &mut rng)
        }));
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
