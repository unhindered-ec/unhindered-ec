use ec_core::operator::recombinator::Recombinator;
use rand::Rng;

use super::{
    crossover::Crossover,
    errors::{DifferentGenomeLength, NPointCrossoverError},
};
use crate::{genome::Linear, recombinator::errors::GenomeLengthTooShort};

/// Recombinator for fixed-length linear genomes, like
/// [`Bitstring`](crate::genome::bitstring::Bitstring)
///
/// This recombinator works by having two parents and randomly choosing a range
/// in the genes to swap. This is in contrast to
/// [`UniformXo`](super::uniform_xo::UniformXo) which randomly chooses which
/// gene to keep at each position.
///
/// # Example
/// ```
/// # use ec_core::operator::recombinator::Recombinator;
/// # use ec_linear::{
/// #     recombinator::two_point_xo::TwoPointXo,
/// #     genome::bitstring::Bitstring,
/// # };
/// # use rand::rng;
/// #
/// # let mut rng = rng();
/// #
/// let parent_1 = Bitstring::random(10, &mut rng);
/// let parent_2 = Bitstring::random(10, &mut rng);
///
/// let child = TwoPointXo.recombine([parent_1, parent_2], &mut rng)?;
/// # let _ = child;
/// #
/// # Ok::<(),Box<dyn std::error::Error>>(())
/// ```
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
pub struct TwoPointXo;

// TODO: Note that `TwoPointXo` doesn't strictly need
//   the two vectors to have the same length, but the
//   swapped regions need to "make sense" for both parent
//   genomes. So we could just make sure that both
//   `first` and `second` are < the minimum length
//   and we'd be OK. I'm not sure that's really how we'd
//   want to implement `TwoPointXo` for differing lengths,
//   though. I suspect it would make more sense to ensure
//   that the length of the swapped region is less than the
//   the length of the shorter genome, but not require that
//   they line up. That's really sounding like a different
//   operator than this one, though.
impl<G> Recombinator<[G; 2]> for TwoPointXo
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
    /// #     recombinator::two_point_xo::TwoPointXo,
    /// #     genome::bitstring::Bitstring,
    /// # };
    /// # use rand::rng;
    /// #
    /// # let mut rng = rng();
    /// #
    /// let parent_1 = Bitstring::random(10, &mut rng);
    /// let parent_2 = Bitstring::random(10, &mut rng);
    ///
    /// let child = TwoPointXo.recombine([parent_1, parent_2], &mut rng)?;
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
        if len < 3 {
            return Err(GenomeLengthTooShort {
                min_size: 3,
                genome_length: len,
            }
            .into());
        }

        // One unrolled step of floyds sampling algorithm (see NPointXo for more
        // information on that) to generate 2 distinct random, sorted numbers.
        #[expect(
            clippy::arithmetic_side_effects,
            reason = "We check that len >= 3 above, as such len - 1 >= 2 => won't wrap."
        )]
        let mut first = rng.random_range(1..len - 1);
        let mut second = rng.random_range(1..len);
        #[expect(
            clippy::arithmetic_side_effects,
            reason = "We check that len >= 3 above, as such len - 1 >= 2 => won't wrap."
        )]
        if first == second {
            second = len - 1;
        } else if second < first {
            (first, second) = (second, first);
        }

        first_genome
            .crossover_segment(&mut second_genome, first..second)
            .map_err(NPointCrossoverError::Crossover)?;

        Ok(first_genome)
    }
}

impl<G> Recombinator<(G, G)> for TwoPointXo
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
    /// #     recombinator::two_point_xo::TwoPointXo,
    /// #     genome::bitstring::Bitstring,
    /// # };
    /// # use rand::rng;
    /// #
    /// # let mut rng = rng();
    /// #
    /// let parent_1 = Bitstring::random(10, &mut rng);
    /// let parent_2 = Bitstring::random(10, &mut rng);
    ///
    /// let child = TwoPointXo.recombine((parent_1, parent_2), &mut rng)?;
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
