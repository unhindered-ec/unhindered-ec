use std::borrow::Borrow;

use rand::seq::SliceRandom;
use rand::{rngs::ThreadRng, Rng};
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

use crate::individual::Individual;
use crate::population::Population;

pub type Bitstring = Vec<bool>;

pub fn count_ones(bits: &[bool]) -> i64 {
    bits.iter().filter(|&&bit| bit).count() as i64
}

fn all_same(bits: &[bool]) -> bool {
    bits.iter().all(|&bit| bit == bits[0])
}

pub fn hiff(bits: &[bool]) -> i64 {
    if bits.len() < 2 {
        0
    } else {
        let half_len = bits.len() / 2;
        let mut score = hiff(&bits[..half_len]) + hiff(&bits[half_len..]);
        if all_same(bits) {
            score += bits.len() as i64;
        }
        score
    }
}

pub fn make_bitstring(len: usize, rng: &mut ThreadRng) -> Bitstring {
    (0..len).map(|_| rng.gen_bool(0.5)).collect()
}

pub fn uniform_xo(first_parent: &[bool], second_parent: &[bool], rng: &mut ThreadRng) -> Bitstring {
    // The two parents should have the same length.
    assert!(first_parent.len() == second_parent.len());
    let len = first_parent.len();
    (0..len).map(|i| 
        if rng.gen_bool(0.5) { 
            first_parent[i] 
        } else { 
            second_parent[i] 
        }).collect()
}

impl Individual<Bitstring> {
    pub fn new_bitstring<R>(bit_length: usize, compute_fitness: impl Fn(&R) -> i64 + Send + Sync, rng: &mut ThreadRng) -> Individual<Bitstring> 
    where
        Bitstring: Borrow<R>,
        R: ?Sized
    {
        Individual::new(
                |rng| make_bitstring(bit_length, rng), 
                compute_fitness,
                rng)
    }
}

// This has hiff cooked in and needs to be parameterized on the fitness calculator.
impl Individual<Bitstring> {
    pub fn uniform_xo(&self, other_parent: &Individual<Bitstring>, rng: &mut ThreadRng) -> Individual<Bitstring> {
        let genome = uniform_xo(&self.genome, &other_parent.genome, rng);
        let fitness = hiff(&genome);
        Individual { genome, fitness }
    }
}

// Todo: Change this to use the new parameterized constructor.
impl Population<Bitstring> {
    pub fn new_bitstring<R>(pop_size: usize, bit_length: usize, compute_fitness: impl Fn(&R) -> i64 + Send + Sync) -> Population<Bitstring>
    where
        Bitstring: Borrow<R>,
        R: ?Sized
    {
        Population::new(
            pop_size,
            |rng| make_bitstring(bit_length, rng),
            compute_fitness
        )
    }

    // This does uniform selection when it needs to take a parameterized selection function.
    pub fn next_generation(&self) -> Population<Bitstring> {
        let previous_individuals = &self.individuals;
        assert!(!previous_individuals.is_empty());
        let pop_size = previous_individuals.len();
        let individuals = (0..pop_size)
            .into_par_iter()
            .map_init(
                rand::thread_rng,
                |rng, _| {
                    // These `unwraps()` are OK because we know the previous population isn't empty
                    // thanks to the assertion a few lines up.
                    let first_parent = previous_individuals.choose(rng).unwrap();
                    let second_parent = previous_individuals.choose(rng).unwrap();
                    first_parent.uniform_xo(second_parent, rng)
                }
            ).collect();
        Population { individuals }
    }
}
