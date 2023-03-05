use crate::individual::Individual;

use super::{Operator, Composable};

pub struct GenomeExtractor {}

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

    fn apply(&self, individual: &I, _: &mut rand::rngs::ThreadRng) -> Self::Output {
        individual.genome().clone()
    }
}
impl Composable for GenomeExtractor {}