use anyhow::{ensure, Result};
use rand::{rngs::ThreadRng, Rng};

use ec_core::operator::recombinator::Recombinator;

pub struct UniformXo;

impl<T: Clone> Recombinator<[Vec<T>; 2]> for UniformXo {
    type Output = Vec<T>;

    fn recombine(
        &self,
        [first_genome, second_genome]: [Vec<T>; 2],
        rng: &mut ThreadRng,
    ) -> Result<Self::Output> {
        ensure!(
            first_genome.len() == second_genome.len(),
            "Attempted to perform UniformXo on genomes of different length: {} and {}",
            first_genome.len(),
            second_genome.len()
        );
        let len = first_genome.len();
        Ok((0..len)
            .map(|pos| {
                if rng.gen_bool(0.5) {
                    first_genome[pos].clone()
                } else {
                    second_genome[pos].clone()
                }
            })
            .collect())
    }
}

impl<T: Clone> Recombinator<(Vec<T>, Vec<T>)> for UniformXo {
    type Output = Vec<T>;

    fn recombine(
        &self,
        (first_genome, second_genome): (Vec<T>, Vec<T>),
        rng: &mut ThreadRng,
    ) -> Result<Self::Output> {
        self.recombine([first_genome, second_genome], rng)
    }
}
