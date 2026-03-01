use std::{convert::Infallible, ops::Not};

use ec_core::operator::mutator::Mutator;
use rand::Rng;

use crate::genome::Linear;

/// Linear, boolish mutator which inverts each gene in a genome with the given
/// rate (probability).
///
/// # Example
/// ```
/// # use ec_linear::{
/// #     mutator::with_rate::WithRate,
/// #     genome::bitstring::Bitstring
/// # };
/// # use ec_core::operator::mutator::Mutator;
/// #
/// let mut rng = rand::rng();
///
/// let genome = Bitstring::random(100, &mut rng);
///
/// let mutator = WithRate::new(1.0);
/// let Ok(mutated_genome) = mutator.mutate(genome.clone(), &mut rng);
///
/// assert_ne!(genome, mutated_genome);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct WithRate {
    mutation_rate: f32,
}

impl WithRate {
    /// Create a new [`WithRate`] mutator from the given mutation rate
    /// (probability)
    ///
    /// # Example
    /// ```
    /// # use ec_linear::mutator::with_rate::WithRate;
    /// let mutator = WithRate::new(0.5);
    /// # let _ = mutator;
    /// ```
    #[must_use]
    pub const fn new(mutation_rate: f32) -> Self {
        Self { mutation_rate }
    }
}

// TODO: We should change this so that it mutates `genome` "in place".
//   We own `genome`, so there's no need to make a new one every time.
//   See the `Crossover` trait for the key idea.
impl<T> Mutator<T> for WithRate
where
    T: Linear + FromIterator<T::Gene> + IntoIterator<Item = T::Gene>,
    T::Gene: Not<Output = T::Gene>,
{
    type Error = Infallible;

    /// Apply the mutator to the genome `genome`.
    ///
    /// # Example
    /// ```
    /// # use ec_linear::{
    /// #     mutator::with_rate::WithRate,
    /// #     genome::bitstring::Bitstring
    /// # };
    /// # use ec_core::operator::mutator::Mutator;
    /// #
    /// let mut rng = rand::rng();
    ///
    /// let genome = Bitstring::random(100, &mut rng);
    ///
    /// let mutator = WithRate::new(1.0);
    /// let Ok(mutated_genome) = mutator.mutate(genome.clone(), &mut rng);
    ///
    /// assert_ne!(genome, mutated_genome);
    /// ```
    fn mutate<R: Rng + ?Sized>(&self, genome: T, rng: &mut R) -> Result<T, Self::Error> {
        Ok(genome
            .into_iter()
            .map(|bit| {
                let r: f32 = rng.random();
                if r < self.mutation_rate { !bit } else { bit }
            })
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use std::iter::zip;

    use ec_core::operator::mutator::Mutator;

    use crate::{genome::bitstring::Bitstring, mutator::with_rate::WithRate};

    #[test]
    #[ignore = "This test is stochastic, so I'm going to ignore it most of the time."]
    fn mutate_using_generator_with_rate_does_not_change_much() {
        let mutator = WithRate {
            mutation_rate: 0.05,
        };

        let mut rng = rand::rng();
        let num_bits = 100;

        let parent_bits = Bitstring::random(num_bits, &mut rng);
        let child_bits = mutator.mutate(parent_bits.clone(), &mut rng).unwrap();

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

    #[test]
    #[ignore = "This test is stochastic, so I'm going to ignore it most of the time."]
    fn mutate_bitstring_with_rate_does_not_change_much() {
        let mutator = WithRate {
            mutation_rate: 0.05,
        };

        let mut rng = rand::rng();
        let num_bits = 100;
        let parent_bits = Bitstring::random(num_bits, &mut rng);
        let child_bits = mutator.mutate(parent_bits.clone(), &mut rng).unwrap();

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
