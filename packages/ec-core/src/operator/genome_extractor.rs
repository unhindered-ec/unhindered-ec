use super::{Composable, Operator};
use crate::individual::Individual;

pub struct GenomeExtractor;

/// Returns a _cloned_ copy of the given individual's genome.
///
/// This copy of the genome is cloned so that subsequent stages of a pipeline
/// (like mutation) can take ownership and mutate as appropriate without
/// affecting the parent individual.
impl<I> Operator<&I> for GenomeExtractor
where
    I: Individual,
    <I as Individual>::Genome: Clone,
{
    type Output = I::Genome;
    type Error = anyhow::Error;

    fn apply(
        &self,
        individual: &I,
        _: &mut rand::rngs::ThreadRng,
    ) -> Result<Self::Output, Self::Error> {
        Ok(individual.genome().clone())
    }
}
impl Composable for GenomeExtractor {}
