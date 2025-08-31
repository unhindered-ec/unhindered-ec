use ec_core::operator::recombinator::Recombinator;
use rand::Rng;

use super::{
    crossover::Crossover,
    errors::{CrossoverGeneError, DifferentGenomeLength},
};
use crate::genome::Linear;

pub struct UniformXo;

impl<G> Recombinator<[G; 2]> for UniformXo
where
    G: Crossover + Linear,
{
    type Output = G;
    type Error = CrossoverGeneError<G::GeneCrossoverError>;

    fn recombine<R: Rng + ?Sized>(
        &self,
        [mut first_genome, mut second_genome]: [G; 2],
        rng: &mut R,
    ) -> Result<Self::Output, Self::Error> {
        let len = first_genome.size();
        if len != second_genome.size() {
            return Err(DifferentGenomeLength(len, second_genome.size()).into());
        }
        for i in 0..len {
            if rng.random::<bool>() {
                first_genome
                    .crossover_gene(&mut second_genome, i)
                    .map_err(CrossoverGeneError::Crossover)?;
            }
        }

        Ok(first_genome)
    }
}

impl<G> Recombinator<(G, G)> for UniformXo
where
    G: Crossover + Linear,
{
    type Output = G;
    type Error = <Self as Recombinator<[G; 2]>>::Error;

    fn recombine<R: Rng + ?Sized>(
        &self,
        genomes: (G, G),
        rng: &mut R,
    ) -> Result<Self::Output, Self::Error> {
        self.recombine(<[G; 2]>::from(genomes), rng)
    }
}
