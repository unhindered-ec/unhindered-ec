use rand::rngs::ThreadRng;

use super::{Composable, Operator};

pub mod with_one_over_length;
pub mod with_rate;

pub trait Mutator<G> {
    fn mutate(&self, genome: G, rng: &mut ThreadRng) -> G;
}

pub struct Mutate<M> {
    mutator: M,
}

impl<M> Mutate<M> {
    pub const fn new(mutator: M) -> Self {
        Self { mutator }
    }
}

impl<M, G> Operator<G> for Mutate<M>
where
    M: Mutator<G>,
{
    type Output = G;

    fn apply(&self, genome: G, rng: &mut ThreadRng) -> Self::Output {
        self.mutator.mutate(genome, rng)
    }
}
impl<M> Composable for Mutate<M> {}
