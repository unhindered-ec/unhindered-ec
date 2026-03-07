use std::{convert::Infallible, ops::Not};

use ec_core::operator::mutator::Mutator;
use miette::Diagnostic;
use num_traits::ToPrimitive;
use rand::Rng;

use super::with_rate::WithRate;
use crate::genome::Linear;

/// Linear, boolish mutator which inverts each gene in the genome with a rate
/// (probability) of `1 / genome.size()`.
///
/// # Example
///
/// ```
/// # use ec_linear::{
/// #     mutator::with_one_over_length::WithOneOverLength,
/// #     genome::bitstring::Bitstring
/// # };
/// # use ec_core::operator::mutator::Mutator;
/// #
/// let mut rng = rand::rng();
///
/// let genome = Bitstring::random(100, &mut rng);
///
/// let mutated_genome = WithOneOverLength.mutate(genome.clone(), &mut rng)?;
/// # let _ = mutated_genome;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
pub struct WithOneOverLength;

/// Error that occurs when trying to use the [`WithOneOverLength`] mutator on a
/// genome whose size is not representable by a [`f32`].
#[derive(Debug, thiserror::Error, Diagnostic)]
#[error("Failed to convert genome size {0} to an f32 value to be used as the mutation rate")]
#[diagnostic(help = "Make sure the genome size is representable by an f32 when using this mutator")]
pub struct GenomeSizeConversionError(usize);

impl<T> Mutator<T> for WithOneOverLength
where
    T: Linear + FromIterator<T::Gene> + IntoIterator<Item = T::Gene>,
    T::Gene: Not<Output = T::Gene>,
{
    type Error = GenomeSizeConversionError;

    /// Apply the mutator to the genome `genome`.
    ///
    /// # Example
    ///
    /// ```
    /// # use ec_linear::{
    /// #     mutator::with_one_over_length::WithOneOverLength,
    /// #     genome::bitstring::Bitstring
    /// # };
    /// # use ec_core::operator::mutator::Mutator;
    /// #
    /// let mut rng = rand::rng();
    ///
    /// let genome = Bitstring::random(100, &mut rng);
    ///
    /// let mutated_genome = WithOneOverLength.mutate(genome.clone(), &mut rng)?;
    /// # let _ = mutated_genome;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    fn mutate<R: Rng + ?Sized>(&self, genome: T, rng: &mut R) -> Result<T, Self::Error> {
        let genome_length = genome
            .size()
            .to_f32()
            .ok_or_else(|| GenomeSizeConversionError(genome.size()))?;
        let mutation_rate = 1.0 / genome_length;
        let mutator = WithRate::new(mutation_rate);
        mutator
            .mutate(genome, rng)
            // This can't happen, as the only error is the conversion error
            // since `WithRate::mutator` can't fail.
            .map_err(|_: Infallible| unreachable!())
    }
}

#[cfg(test)]
mod tests {
    use std::iter::zip;

    use ec_core::operator::mutator::Mutator;

    use crate::{genome::bitstring::Bitstring, mutator::with_one_over_length::WithOneOverLength};

    #[test]
    #[ignore = "This test is stochastic, so I'm going to ignore it most of the time."]
    fn mutate_one_over_does_not_change_much() {
        let mut rng = rand::rng();
        let num_bits = 100;
        let parent_bits = Bitstring::random(num_bits, &mut rng);

        let child_bits = WithOneOverLength
            .mutate(parent_bits.clone(), &mut rng)
            .unwrap();

        let num_differences = zip(parent_bits, child_bits)
            .filter(|(p, c)| *p != *c)
            .count();
        println!("Num differences = {num_differences}");
        assert!(
            0 < num_differences,
            "We're expecting at least one difference"
        );
        assert!(
            num_differences < num_bits / 10,
            "We're not expecting lots of differences, and got {num_differences}."
        );
    }
}
