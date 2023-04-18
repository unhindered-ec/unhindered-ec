use anyhow::{ensure, Result};
use rand::{rngs::ThreadRng, Rng};

use ec_core::operator::recombinator::Recombinator;

use crate::genome::LinearGenome;

use super::crossover::Crossover;

pub struct UniformXo;

// TODO: We should get rid of the `Vec<T>` versions when 
//   we've completed the migration to a struct-based `Bitstring`.
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

impl<G> Recombinator<[G; 2]> for UniformXo
where
    G: Crossover,
{
    type Output = G;

    fn recombine(
        &self,
        [mut first_genome, mut second_genome]: [G; 2],
        rng: &mut ThreadRng,
    ) -> Result<Self::Output> {
        ensure!(
            first_genome.size() == second_genome.size(),
            "Attempted to perform UniformXo on genomes of different length: {} and {}",
            first_genome.size(),
            second_genome.size()
        );
        let len = first_genome.size();
        for i in 0..len {
            if rng.gen_bool(0.5) {
                first_genome.crossover_gene(&mut second_genome, i)?;
            }
        }

        Ok(first_genome)
    }
}

impl<G> Recombinator<(G, G)> for UniformXo 
where
    G: Crossover
{
    type Output = G;

    fn recombine(
        &self,
        (first_genome, second_genome): (G, G),
        rng: &mut ThreadRng,
    ) -> Result<Self::Output> {
        self.recombine([first_genome, second_genome], rng)
    }
}

