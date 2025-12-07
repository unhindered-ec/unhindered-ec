mod sample_distinct_uniform;

use std::iter::once;

use ec_core::operator::recombinator::Recombinator;
use rand::Rng;

use super::{
    crossover::Crossover,
    errors::{DifferentGenomeLength, NPointCrossoverError},
};
use crate::{
    genome::Linear,
    recombinator::{
        errors::GenomeLengthTooShort,
        n_point_xo::sample_distinct_uniform::sample_distinct_uniform_sorted,
    },
};

/// Recombinator for fixed-length linear genomes, like
/// [`Bitstring`](crate::genome::bitstring::Bitstring).
///
/// This recombinator works by having two parents and choosing `N` points, where
/// the genomes will be swapped every other pair `(n, n+1)`. This is in contrast
/// to for example [`UniformXo`](super::uniform_xo::UniformXo) which randomly
/// chooses which gene to keep at each position.
///
/// # Example
/// ```
/// # use ec_core::operator::recombinator::Recombinator;
/// # use ec_linear::{
/// #     recombinator::n_point_xo::NPointXo,
/// #     genome::bitstring::Bitstring,
/// # };
/// # use rand::rng;
/// #
/// # let mut rng = rng();
/// #
/// let parent_1 = Bitstring::random(10, &mut rng);
/// let parent_2 = Bitstring::random(10, &mut rng);
///
/// let child = NPointXo::<4>::new().recombine([parent_1, parent_2], &mut rng)?;
/// # let _ = child;
/// #
/// # Ok::<(),Box<dyn std::error::Error>>(())
/// ```
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct NPointXo<const N: usize>(());

impl<const N: usize> NPointXo<N> {
    /// Create a new [`NPointXo`] recombinator with `N` crossover points
    ///
    /// This ensures at compile time that at least one croosover point is
    /// specified.
    ///
    /// # Example
    /// ```
    /// # use ec_linear::recombinator::n_point_xo::NPointXo;
    /// #
    /// let my_recombinator = NPointXo::<4>::new();
    /// # let _ = my_recombinator;
    /// ```
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
    /// Create a new [`NPointXo`] recombinator with `N` crossover points
    ///
    /// This ensures at compile time that at least one croosover point is
    /// specified.
    ///
    /// # Example
    /// ```
    /// # use ec_linear::recombinator::n_point_xo::NPointXo;
    /// #
    /// let my_recombinator = NPointXo::<4>::default();
    /// # let _ = my_recombinator;
    /// ```
    fn default() -> Self {
        Self::new()
    }
}

impl<G, const N: usize> Recombinator<[G; 2]> for NPointXo<N>
where
    G: Crossover + Linear,
{
    type Output = G;
    type Error = NPointCrossoverError<G::SegmentCrossoverError>;

    /// Apply this crossover operator to the genomes `first_genome` and
    /// `second_genome`
    ///
    /// # Example
    /// ```
    /// # use ec_core::operator::recombinator::Recombinator;
    /// # use ec_linear::{
    /// #     recombinator::n_point_xo::NPointXo,
    /// #     genome::bitstring::Bitstring,
    /// # };
    /// # use rand::rng;
    /// #
    /// # let mut rng = rng();
    /// #
    /// let parent_1 = Bitstring::random(10, &mut rng);
    /// let parent_2 = Bitstring::random(10, &mut rng);
    ///
    /// let child = NPointXo::<4>::new().recombine([parent_1, parent_2], &mut rng)?;
    /// # let _ = child;
    /// #
    /// # Ok::<(),Box<dyn std::error::Error>>(())
    /// ```
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

        let crossover_points = sample_distinct_uniform_sorted::<_, N>(1, len, rng);

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
                .map_err(NPointCrossoverError::Crossover)?;
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

    /// Apply this crossover operator to the genomes `first_genome` and
    /// `second_genome`
    ///
    /// # Example
    /// ```
    /// # use ec_core::operator::recombinator::Recombinator;
    /// # use ec_linear::{
    /// #     recombinator::n_point_xo::NPointXo,
    /// #     genome::bitstring::Bitstring,
    /// # };
    /// # use rand::rng;
    /// #
    /// # let mut rng = rng();
    /// #
    /// let parent_1 = Bitstring::random(10, &mut rng);
    /// let parent_2 = Bitstring::random(10, &mut rng);
    ///
    /// let child = NPointXo::<4>::new().recombine((parent_1, parent_2), &mut rng)?;
    /// # let _ = child;
    /// #
    /// # Ok::<(),Box<dyn std::error::Error>>(())
    /// ```
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

        let crossover_operator = NPointXo::<1>::new();

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

        let crossover_operator = NPointXo::<4>::new();

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
