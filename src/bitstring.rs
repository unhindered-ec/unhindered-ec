use rand::{rngs::ThreadRng, Rng};
use rayon::prelude::{ParallelExtend, IntoParallelIterator, ParallelIterator};

use crate::individual::Individual;
use crate::population::Population;

pub type Bitstring = Vec<bool>;

pub fn count_ones(bits: &[bool]) -> f64 {
    bits.iter().filter(|&&bit| bit).count() as f64
}

impl Individual<Bitstring> {
    pub fn new(bit_length: usize, rng: &mut ThreadRng) -> Individual<Bitstring> {
        let mut bits = Vec::with_capacity(bit_length);
        for _ in 0..bit_length {
            bits.push(rng.gen_bool(0.5));
        }
        let fitness = count_ones(&bits);
        Individual {
            genome: bits,
            fitness,
        }
    }
}

impl Population<Bitstring> {
    pub fn new(pop_size: usize, bit_length: usize) -> Population<Bitstring> {
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
