mod sample_distinct_uniform;

use std::iter::once;

use ec_core::operator::recombinator::Recombinator;
use rand::Rng;

use super::{
    crossover::Crossover,
    errors::{CrossoverGeneError, DifferentGenomeLength},
};
use crate::{
    genome::Linear,
    recombinator::{
        errors::GenomeLengthTooShort,
        n_point_xo::sample_distinct_uniform::{
            sample_distinct_uniform_sorted_inplace,
            sample_distinct_uniform_sorted_inplace_start_at_0,
            sample_distinct_uniform_sorted_inplace_start_end,
        },
    },
};

// TODO: Assert that len >= N + 1

pub struct NPointXo<const N: usize>(());

impl<const N: usize> NPointXo<N> {
    #[must_use]
    pub const fn new() -> Self {
        const {
            assert!(
                N >= 1,
                "Need at least one crossover point but got less than 1 points."
            );
        }
        Self(())
    }
}

impl<const N: usize> Default for NPointXo<N> {
    fn default() -> Self {
        Self::new()
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
        // This should be checked in the constructor but since it is const and as such
        // no runtime cost just check again here just to make sure
        const {
            assert!(
                N >= 1,
                "Need at least one crossover point but got less than 1 points."
            );
        }
        // Since N >= 1 (as checked in the constructor, invariant!) we know that len >=
        // N + 1 <=> len  >= 2;
        if len < const { N + 1 } {
            return Err(GenomeLengthTooShort {
                min_size: const { N + 1 },
                genome_length: len,
            }
            .into());
        }

        #[expect(
            clippy::arithmetic_side_effects,
            reason = "Per above invariant len >= 2, as such len - 1 >= 1 and can't wrap"
        )]
        let crossover_points = sample_distinct_uniform_sorted_inplace::<_, N>(len - 1, rng);

        #[expect(
            clippy::unwrap_used,
            reason = "Per above invariant N >= 1 and as such the array of size N always has at \
                      least one (the last) element."
        )]
        let last_crossover_pair = [*crossover_points.last().unwrap(), len];
        let iter = crossover_points
            .windows(2)
            .chain(once(last_crossover_pair.as_slice()))
            .step_by(2);

        for point in iter {
            first_genome
                .crossover_segment(&mut second_genome, point[0]..point[1])
                .map_err(CrossoverGeneError::Crossover)?;
        }
        Ok(first_genome)
    }
}

pub struct NPointXoZero<const N: usize>(());

impl<const N: usize> NPointXoZero<N> {
    #[must_use]
    pub const fn new() -> Self {
        const {
            assert!(
                N >= 1,
                "Need at least one crossover point but got less than 1 points."
            );
        }
        Self(())
    }
}

impl<const N: usize> Default for NPointXoZero<N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<G, const N: usize> Recombinator<[G; 2]> for NPointXoZero<N>
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
        // This should be checked in the constructor but since it is const and as such
        // no runtime cost just check again here just to make sure
        const {
            assert!(
                N >= 1,
                "Need at least one crossover point but got less than 1 points."
            );
        }
        // Since N >= 1 (as checked in the constructor, invariant!) we know that len >=
        // N + 1 <=> len  >= 2;
        if len < const { N + 1 } {
            return Err(GenomeLengthTooShort {
                min_size: const { N + 1 },
                genome_length: len,
            }
            .into());
        }

        #[expect(
            clippy::arithmetic_side_effects,
            reason = "Per above invariant len >= 2, as such len - 1 >= 1 and can't wrap"
        )]
        let crossover_points =
            sample_distinct_uniform_sorted_inplace_start_at_0::<_, N>(len - 1, rng);

        #[expect(
            clippy::unwrap_used,
            reason = "Per above invariant N >= 1 and as such the array of size N always has at \
                      least one (the last) element."
        )]
        #[expect(
            clippy::arithmetic_side_effects,
            reason = "Per above invariant len >= 2, as such len - 1 >= 1 and can't wrap"
        )]
        let last_crossover_pair = [*crossover_points.last().unwrap(), len - 1];
        // Invariant: \forall (p, q) in Iterator . p < len && q < len
        // <= \forall p \in sample_distinct_uniform_sorted_inplace_start_at_0(len - 1) .
        //        p < len
        //   \and len - 1  < len
        let iter = crossover_points
            .windows(2)
            .chain(once(last_crossover_pair.as_slice()))
            .step_by(2);

        for point in iter {
            #[expect(
                clippy::arithmetic_side_effects,
                reason = "Per above invariant is point[0] & point[1] < len, as such x < len  => x \
                          + 1 <= len and since len and points are of the same data type we know \
                          that length is representable wrt. to that datatype and as such x + 1 is \
                          also representable without wrapping wrt. to the same datatype"
            )]
            first_genome
                .crossover_segment(&mut second_genome, (point[0] + 1)..(point[1] + 1))
                .map_err(CrossoverGeneError::Crossover)?;
        }
        Ok(first_genome)
    }
}

pub struct NPointXoStartEnd<const N: usize>(());

impl<const N: usize> NPointXoStartEnd<N> {
    #[must_use]
    pub const fn new() -> Self {
        const {
            assert!(
                N >= 1,
                "Need at least one crossover point but got less than 1 points."
            );
        }
        Self(())
    }
}

impl<const N: usize> Default for NPointXoStartEnd<N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<G, const N: usize> Recombinator<[G; 2]> for NPointXoStartEnd<N>
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
        // This should be checked in the constructor but since it is const and as such
        // no runtime cost just check again here just to make sure
        const {
            assert!(
                N >= 1,
                "Need at least one crossover point but got less than 1 points."
            );
        }
        // Since N >= 1 (as checked in the constructor, invariant!) we know that len >=
        // N + 1 <=> len  >= 2;
        if len < const { N + 1 } {
            return Err(GenomeLengthTooShort {
                min_size: const { N + 1 },
                genome_length: len,
            }
            .into());
        }

        let crossover_points =
            sample_distinct_uniform_sorted_inplace_start_end::<_, N>(1, len, rng);

        #[expect(
            clippy::unwrap_used,
            reason = "Per above invariant N >= 1 and as such the array of size N always has at \
                      least one (the last) element."
        )]
        let last_crossover_pair = [*crossover_points.last().unwrap(), len];
        let iter = crossover_points
            .windows(2)
            .chain(once(last_crossover_pair.as_slice()))
            .step_by(2);

        for point in iter {
            first_genome
                .crossover_segment(&mut second_genome, point[0]..point[1])
                .map_err(CrossoverGeneError::Crossover)?;
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

    use crate::recombinator::n_point_xo::{NPointXo, NPointXoStartEnd};

    #[test]
    pub fn recombine_one_point() {
        const GENOME_SIZE: usize = 10;

        let first_parent = vec![0; GENOME_SIZE];
        let second_parent = vec![1; GENOME_SIZE];

        let crossover_operator = NPointXo::<0>::new();

        let mut rng = rand::rng();

        let child = crossover_operator
            .recombine([first_parent, second_parent], &mut rng)
            .unwrap();

        // Confirm that the child has the same size as the parents
        assert_eq!(child.size(), GENOME_SIZE);

        // Splits the child up into segments of equal values
        let mut segments = child.chunk_by(|a, b| a == b);
        // With one crossover point there should be exactly two segments.
        assert_eq!(segments.clone().count(), 2);
        assert_eq!(segments.next().unwrap().iter().next().unwrap(), &0);
        assert_eq!(segments.next().unwrap().iter().next().unwrap(), &1);
    }

    #[test]
    pub fn recombine_two_point() {
        const GENOME_SIZE: usize = 10;

        let first_parent = vec![0; GENOME_SIZE];
        let second_parent = vec![1; GENOME_SIZE];

        let crossover_operator = NPointXo::<2>::new();

        let mut rng = rand::rng();

        let child = crossover_operator
            .recombine([first_parent, second_parent], &mut rng)
            .unwrap();

        // Confirm that the child has the same size as the parents
        assert_eq!(child.size(), GENOME_SIZE);

        // Splits the child up into segments of equal values
        let mut segments = child.chunk_by(|a, b| a == b);
        // With one crossover point there should be exactly two segments.
        assert_eq!(segments.clone().count(), 3);
        assert_eq!(segments.next().unwrap().iter().next().unwrap(), &0);
        assert_eq!(segments.next().unwrap().iter().next().unwrap(), &1);
        assert_eq!(segments.next().unwrap().iter().next().unwrap(), &0);
    }

    #[test]
    pub fn recombine_four_point() {
        const GENOME_SIZE: usize = 6;

        let first_parent = vec![0; GENOME_SIZE];
        let second_parent = vec![1; GENOME_SIZE];

        let crossover_operator = NPointXoStartEnd::<4>::new();

        let mut rng = rand::rng();

        let child = crossover_operator
            .recombine([first_parent, second_parent], &mut rng)
            .unwrap();

        // Confirm that the child has the same size as the parents
        assert_eq!(child.size(), GENOME_SIZE);

        // Splits the child up into segments of equal values
        let mut segments = child.chunk_by(|a, b| a == b);
        // With one crossover point there should be exactly two segments.
        assert_eq!(segments.clone().count(), 5);
        assert_eq!(segments.next().unwrap().iter().next().unwrap(), &0);
        assert_eq!(segments.next().unwrap().iter().next().unwrap(), &1);
        assert_eq!(segments.next().unwrap().iter().next().unwrap(), &0);
        assert_eq!(segments.next().unwrap().iter().next().unwrap(), &1);
        assert_eq!(segments.next().unwrap().iter().next().unwrap(), &0);
    }
}
