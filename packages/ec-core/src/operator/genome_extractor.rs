use std::convert::Infallible;

use super::{Composable, Operator};
use crate::individual::Individual;

/// An [`Operator`] that returns a _cloned_ copy of the given [`Individual`]'s
/// genome.
///
/// This copy of the genome is cloned so that subsequent stages of a pipeline
/// (like mutation) can take ownership and mutate as appropriate without
/// affecting the parent individual.
///
/// # Examples
///
/// In this example we chain `GenomeExtractor` with a simple mutator to
/// extract the genome from an individual and mutate it.
///
/// ```
/// # use rand::{rngs::ThreadRng, thread_rng, Rng};
/// # use ec_core::{
/// #     individual::ec::EcIndividual,
/// #     operator::{
/// #         genome_extractor::GenomeExtractor,
/// #         mutator::{Mutate, Mutator},
/// #         Composable, Operator,
/// #     },
/// # };
/// // Flip a single bit in an array of booleans.
/// struct FlipOne;
///
/// impl Mutator<[bool; 4]> for FlipOne {
///     fn mutate(&self, mut genome: [bool; 4], rng: &mut ThreadRng) -> anyhow::Result<[bool; 4]> {
///         let index = rng.gen_range(0..genome.len());
///         genome[index] = !genome[index];
///         Ok(genome)
///     }
/// }
///
/// let genome = [true, false, false, true];
/// let individual = EcIndividual::new(genome, ());
///
/// let mutate = Mutate::new(FlipOne);
/// let chain = GenomeExtractor.then(mutate);
///
/// let result = chain.apply(&individual, &mut thread_rng()).unwrap();
///
/// // Count the number of genome values that have been changed, which should be 1.
/// let num_different = result
///     .iter()
///     .zip(genome.iter())
///     .filter(|(x, y)| x != y)
///     .count();
///
/// assert_eq!(num_different, 1);
/// ```
pub struct GenomeExtractor;

impl<I> Operator<&I> for GenomeExtractor
where
    I: Individual,
    <I as Individual>::Genome: Clone,
{
    type Output = I::Genome;
    type Error = Infallible;

    fn apply(
        &self,
        individual: &I,
        _: &mut rand::rngs::ThreadRng,
    ) -> Result<Self::Output, Self::Error> {
        Ok(individual.genome().clone())
    }
}

impl Composable for GenomeExtractor {}

#[expect(clippy::unwrap_used, reason = "panicking is appropriate in tests")]
#[cfg(test)]
mod tests {
    use rand::{Rng, rngs::ThreadRng, thread_rng};

    use super::GenomeExtractor;
    use crate::{
        individual::ec::EcIndividual,
        operator::{
            Composable, Operator,
            mutator::{Mutate, Mutator},
        },
    };

    struct FlipOne;

    impl Mutator<[bool; 4]> for FlipOne {
        fn mutate(&self, mut genome: [bool; 4], rng: &mut ThreadRng) -> anyhow::Result<[bool; 4]> {
            let index = rng.gen_range(0..genome.len());
            genome[index] = !genome[index];
            Ok(genome)
        }
    }

    #[test]
    fn can_chain_extractor() {
        let genome = [true, false, false, true];
        let individual = EcIndividual::new(genome, ());

        let mutate = Mutate::new(FlipOne);
        let chain = GenomeExtractor.then(mutate);

        let result = chain.apply(&individual, &mut thread_rng()).unwrap();

        let num_same = result
            .iter()
            .zip(genome.iter())
            .filter(|(x, y)| x == y)
            .count();

        assert_eq!(num_same, 3);
    }
}
