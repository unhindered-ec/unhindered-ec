use ec_core::operator::recombinator::Recombinator;
use rand::Rng;

use super::{
    crossover::Crossover,
    errors::{CrossoverGeneError, DifferentGenomeLength},
};

pub struct UniformXo;

// TODO: We should get rid of the `Vec<T>` versions when
//   we've completed the migration to a struct-based `Bitstring`.
impl<T: Clone> Recombinator<[Vec<T>; 2]> for UniformXo {
    type Output = Vec<T>;
    type Error = DifferentGenomeLength;

    fn recombine<R: Rng + ?Sized>(
        &self,
        [first_genome, second_genome]: [Vec<T>; 2],
        rng: &mut R,
    ) -> Result<Self::Output, Self::Error> {
        let len = first_genome.len();
        if len != second_genome.len() {
            return Err(DifferentGenomeLength(len, second_genome.len()));
        }
        Ok((0..len)
            .map(|pos| {
                if rng.random::<bool>() {
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
    type Error = <Self as Recombinator<[Vec<T>; 2]>>::Error;

    fn recombine<R: Rng + ?Sized>(
        &self,
        genomes: (Vec<T>, Vec<T>),
        rng: &mut R,
    ) -> Result<Self::Output, Self::Error> {
        self.recombine(<[Vec<T>; 2]>::from(genomes), rng)
    }
}

impl<G> Recombinator<[G; 2]> for UniformXo
where
    G: Crossover,
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
    G: Crossover,
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
