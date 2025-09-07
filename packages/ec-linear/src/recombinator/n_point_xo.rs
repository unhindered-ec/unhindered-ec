use std::iter::once;

use ec_core::operator::recombinator::Recombinator;
use itertools::Itertools;
use rand::{Rng, seq::IteratorRandom};

use super::{
    crossover::Crossover,
    errors::{CrossoverGeneError, DifferentGenomeLength},
};
use crate::genome::Linear;

// TODO: Assert that len >= N + 1

pub struct NPointXoChunks<const N: usize>(());

impl<const N: usize> NPointXoChunks<N> {
    #[must_use]
    pub const fn new() -> Option<Self> {
        if N == 0 {
            return None;
        }
        Some(Self(()))
    }
}

impl<G, const N: usize> Recombinator<[G; 2]> for NPointXoChunks<N>
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
        (1..len).choose_multiple_fill(rng, &mut crossover_points);
        crossover_points.sort_unstable();

        let segments = once(0).chain(crossover_points).chain(once(len)).chunks(2);
        for mut segment in &segments {
            let Some(start) = segment.next() else { break };
            let Some(end) = segment.next() else { break };
            // let start = segment.next().unwrap();
            // let end = segment.next().unwrap();
            first_genome
                .crossover_segment(&mut second_genome, start..end)
                .map_err(CrossoverGeneError::Crossover)?;
        }

        Ok(first_genome)
    }
}

pub struct NPointXoMultiSwap<const N: usize>(());

impl<const N: usize> NPointXoMultiSwap<N> {
    #[must_use]
    pub const fn new() -> Option<Self> {
        if N == 0 {
            return None;
        }
        Some(Self(()))
    }
}

impl<G, const N: usize> Recombinator<[G; 2]> for NPointXoMultiSwap<N>
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
        (1..len).choose_multiple_fill(rng, &mut crossover_points);
        crossover_points.sort_unstable();

        for crossover_point in crossover_points {
            first_genome
                .crossover_segment(&mut second_genome, crossover_point..len)
                .map_err(CrossoverGeneError::Crossover)?;
        }

        Ok(first_genome)
    }
}

pub struct NPointXoWindows<const N: usize>(());

impl<const N: usize> NPointXoWindows<N> {
    #[must_use]
    pub const fn new() -> Option<Self> {
        if N == 0 {
            return None;
        }
        Some(Self(()))
    }
}

impl<G, const N: usize> Recombinator<[G; 2]> for NPointXoWindows<N>
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
        (1..len).choose_multiple_fill(rng, &mut crossover_points);
        crossover_points.sort_unstable();

        #[expect(clippy::unwrap_used, reason = "frogs")]
        let a = [0, *crossover_points.first().unwrap()];
        #[expect(clippy::unwrap_used, reason = "frogs")]
        let z = [*crossover_points.last().unwrap(), len];
        let iter = once(a.as_slice())
            .chain(crossover_points.windows(2))
            .chain(once(z.as_slice()))
            .step_by(2);

        for point in iter {
            first_genome
                .crossover_segment(&mut second_genome, point[0]..point[1])
                .map_err(CrossoverGeneError::Crossover)?;
        }
        Ok(first_genome)
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

pub struct NPointXoPrimitive<const N: usize>(());

impl<const N: usize> NPointXoPrimitive<N> {
    #[must_use]
    pub const fn new() -> Option<Self> {
        if N == 0 {
            return None;
        }
        Some(Self(()))
    }
}

impl<G, const N: usize> Recombinator<[G; 2]> for NPointXoPrimitive<N>
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
        (1..len).choose_multiple_fill(rng, &mut crossover_points);
        crossover_points.sort_unstable();

        // let mut current_parent = CurrentParent::First;
        for i in (0..const { N + 2 }).step_by(2) {
            let start = i
                .checked_sub(1)
                .and_then(|i| crossover_points.get(i))
                .copied()
                .unwrap_or(0);

            let end = crossover_points.get(i).copied().unwrap_or(len);

            // if matches!(current_parent, CurrentParent::Second) {
            first_genome
                .crossover_segment(&mut second_genome, start..end)
                .map_err(CrossoverGeneError::Crossover)?;
            // }

            // current_parent.swap_inplace();
        }

        Ok(first_genome)
    }
}

// impl<G, const N: usize> Recombinator<(G, G)> for NPointXo<N>
// where
//     G: Crossover + Linear,
// {
//     type Output = G;
//     type Error = <Self as Recombinator<[G; 2]>>::Error;

//     fn recombine<R: Rng + ?Sized>(
//         &self,
//         genomes: (G, G),
//         rng: &mut R,
//     ) -> Result<Self::Output, Self::Error> {
//         self.recombine(<[G; 2]>::from(genomes), rng)
//     }
// }

// #[cfg(test)]
// mod tests {
//     use ec_core::{operator::recombinator::Recombinator,
// population::Population};

//     use crate::recombinator::n_point_xo::NPointXo;

//     #[test]
//     pub fn recombine_one_point() {
//         const GENOME_SIZE: usize = 10;

//         let first_parent = vec![0; GENOME_SIZE];
//         let second_parent = vec![1; GENOME_SIZE];

//         let crossover_operator = NPointXo::<1>::new().unwrap();

//         let mut rng = rand::rng();

//         let child = crossover_operator
//             .recombine([first_parent, second_parent], &mut rng)
//             .unwrap();

//         // Confirm that the child has the same size as the parents
//         assert_eq!(child.size(), GENOME_SIZE);

//         // Splits the child up into segments of equal values
//         let segments = child.chunk_by(|a, b| a == b);
//         // With one crossover point there should be exactly two segments.
//         assert_eq!(segments.count(), 2);
//     }
// }
