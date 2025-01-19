use ec_core::operator::recombinator::Recombinator;
use rand::Rng;

use super::{crossover::Crossover, errors::UniformCrossoverError};
use crate::{genome::Linear, recombinator::errors::DifferentGenomeLength};

/// Recombinator for fixed-length linear genomes, like
/// [`Bitstring`](crate::genome::bitstring::Bitstring).
///
/// This recombinator works by having two parents and randomly choosing at each
/// gene which parent the child's gene will be from. This is in contrast to
/// [`TwoPointXo`](super::two_point_xo::TwoPointXo) which chooses a range in the
/// genomes and swaps the entire range.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
pub struct UniformXo;

impl<G> Recombinator<[G; 2]> for UniformXo
where
    G: Crossover + Linear,
{
    type Output = G;
    type Error = UniformCrossoverError<G::GeneCrossoverError>;

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
                    .map_err(UniformCrossoverError::Crossover)?;
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
