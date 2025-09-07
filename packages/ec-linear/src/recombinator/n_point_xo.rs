use ec_core::operator::recombinator::Recombinator;
use rand::{Rng, seq::IteratorRandom};

use super::{
    crossover::Crossover,
    errors::{CrossoverGeneError, DifferentGenomeLength},
};
use crate::genome::Linear;

pub struct NPointXo<const N: usize>(());

impl<const N: usize> NPointXo<N> {
    #[must_use]
    pub const fn new() -> Option<Self> {
        if N == 0 {
            return None;
        }
        Some(Self(()))
    }
}

#[derive(Debug)]
enum CurrentParent {
    First,
    Second,
}

impl CurrentParent {
    const fn swap_inplace(&mut self) {
        *self = match *self {
            Self::First => Self::Second,
            Self::Second => Self::First,
        }
    }
}

impl<G, const N: usize> Recombinator<[G; 2]> for NPointXo<N>
where
    G: Crossover + Linear,
{
    type Output = G;
    type Error = CrossoverGeneError<G::SegmentCrossoverError>;

    fn recombine<R: Rng + ?Sized>(
        &self,
        [mut first_genome, mut second_genome]: [G; 2],
        rng: &mut R,
    ) -> Result<Self::Output, Self::Error> {
        let len = first_genome.size();
        if len != second_genome.size() {
            return Err(DifferentGenomeLength(len, second_genome.size()).into());
        }

        let mut crossover_points = [0; N];
        (0..len).choose_multiple_fill(rng, &mut crossover_points);
        crossover_points.sort_unstable();

        let mut current_parent = CurrentParent::First;
        for i in 0..const { N + 2 } {
            let start = i
                .checked_sub(1)
                .and_then(|i| crossover_points.get(i))
                .copied()
                .unwrap_or(0);

            let end = crossover_points.get(i).copied().unwrap_or(len);

            if matches!(current_parent, CurrentParent::Second) {
                first_genome
                    .crossover_segment(&mut second_genome, start..end)
                    .map_err(CrossoverGeneError::Crossover)?;
            }

            current_parent.swap_inplace();
        }

        Ok(first_genome)
    }
}

impl<G, const N: usize> Recombinator<(G, G)> for NPointXo<N>
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

#[cfg(test)]
mod tests {
    use ec_core::{operator::recombinator::Recombinator, population::Population};

    use crate::recombinator::n_point_xo::NPointXo;

    #[test]
    pub fn recombine_one_point() {
        const GENOME_SIZE: usize = 10;

        let first_parent = vec![0; GENOME_SIZE];
        let second_parent = vec![1; GENOME_SIZE];

        let crossover_operator = NPointXo::<1>::new().unwrap();

        let mut rng = rand::rng();

        let child = crossover_operator
            .recombine([first_parent, second_parent], &mut rng)
            .unwrap();

        // Confirm that the child has the same size as the parents
        assert_eq!(child.size(), GENOME_SIZE);

        // Splits the child up into segments of equal values
        let segments = child.chunk_by(|a, b| a == b);
        // With one crossover point there should be exactly two segments.
        assert_eq!(segments.count(), 2);
    }
}
