use std::iter::repeat_with;

use ec_core::{generator::Generator, genome::Genome};
use rand::rngs::ThreadRng;

pub mod bitstring;
pub mod demo_scorers;

pub trait LinearGenome: Genome {
    fn size(&self) -> usize;

    fn gene_mut(&mut self, index: usize) -> Option<&mut Self::Gene>;
}

pub struct LinearContext<C> {
    pub length: usize,
    pub element_context: C,
}

impl<T, C> Generator<Vec<T>, LinearContext<C>> for ThreadRng
where
    Self: Generator<T, C>,
{
    fn generate(&mut self, context: &LinearContext<C>) -> Vec<T> {
        repeat_with(|| self.generate(&context.element_context))
            .take(context.length)
            .collect()
    }
}
