use std::convert::Infallible;

use rand::Rng;

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
/// # use rand::{rng, Rng};
/// # use ec_core::{
/// #     individual::ec::EcIndividual,
/// #     operator::{
/// #         genome_extractor::GenomeExtractor,
/// #         mutator::{Mutate, Mutator},
/// #         Composable, Operator,
/// #     },
/// # };
/// # use std::convert::Infallible;
/// #
/// type Genome<T> = [T; 4];
/// // Flip a single bit in an array of booleans.
/// struct FlipOne;
///
/// impl Mutator<Genome<bool>> for FlipOne {
///     type Error = Infallible;
///
///     fn mutate<R: Rng + ?Sized>(
///         &self,
///         mut genome: Genome<bool>,
///         rng: &mut R,
///     ) -> Result<Genome<bool>, Self::Error> {
///         let index = rng.random_range(0..genome.len());
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
/// let result = chain.apply(&individual, &mut rng())?;
///
/// // Count the number of genome values that have been changed, which should be 1.
/// let num_different = result
///     .iter()
///     .zip(genome.iter())
///     .filter(|(x, y)| x != y)
///     .count();
///
/// assert_eq!(num_different, 1);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[derive(Debug, Composable, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct GenomeExtractor;

impl<I> Operator<&I> for GenomeExtractor
where
    I: Individual,
    <I as Individual>::Genome: Clone,
{
    type Output = I::Genome;
    type Error = Infallible;

    /// Apply the [`GenomeExtractor`] as an [`Operator`], extracting the Genome
    /// from the [`Individual`].
    fn apply<R: Rng + ?Sized>(
        &self,
        individual: &I,
        _: &mut R,
    ) -> Result<Self::Output, Self::Error> {
        Ok(individual.genome().clone())
    }
}

#[cfg(test)]
mod tests {
    use std::convert::Infallible;

    use rand::{Rng, rng};

    use super::GenomeExtractor;
    use crate::{
        individual::ec::EcIndividual,
        operator::{
            Composable, Operator,
            mutator::{Mutate, Mutator},
        },
    };

    type Genome<T> = [T; 4];
    struct FlipOne;

    impl Mutator<Genome<bool>> for FlipOne {
        type Error = Infallible;

        fn mutate<R: Rng + ?Sized>(
            &self,
            mut genome: Genome<bool>,
            rng: &mut R,
        ) -> Result<Genome<bool>, Self::Error> {
            let index = rng.random_range(0..genome.len());
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

        let result = chain.apply(&individual, &mut rng()).unwrap();

        let num_same = result
            .iter()
            .zip(genome.iter())
            .filter(|(x, y)| x == y)
            .count();

        assert_eq!(num_same, 3);
    }
}
