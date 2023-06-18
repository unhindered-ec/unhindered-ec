use std::iter::repeat_with;

use ec_core::{generator::Generator, genome::Genome};
use rand::rngs::ThreadRng;

pub mod bitstring;
pub mod demo_scorers;
pub mod vector;

pub trait Linear: Genome {
    fn size(&self) -> usize;

    fn gene_mut(&mut self, index: usize) -> Option<&mut Self::Gene>;
}

pub struct LinearContext<C> {
    pub length: usize,
    pub element_context: C,
}

impl<T, C> Generator<Vec<T>> for LinearContext<C>
where
    C: Generator<T>,
{
    fn generate(&self, rng: &mut ThreadRng) -> anyhow::Result<Vec<T>> {
        repeat_with(|| self.element_context.generate(rng))
            .take(self.length)
            .collect()
    }
}
