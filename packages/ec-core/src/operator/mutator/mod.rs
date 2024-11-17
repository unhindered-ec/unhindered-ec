use rand::rngs::ThreadRng;

use super::{Composable, Operator};

pub trait Mutator<G> {
    /// # Errors
    /// This can return an error if there is an error mutating the given
    /// genome.
    fn mutate(&self, genome: G, rng: &mut ThreadRng) -> anyhow::Result<G>;
}

/// A wrapper that converts a `Mutator` into an `Operator`
///
/// See [the `operator` module docs](crate::operator#wrappers) for more on the
/// design decisions behind using wrappers.
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
    type Error = anyhow::Error;

    fn apply(&self, genome: G, rng: &mut ThreadRng) -> Result<Self::Output, Self::Error> {
        self.mutator.mutate(genome, rng)
    }
}
impl<M> Composable for Mutate<M> {}
