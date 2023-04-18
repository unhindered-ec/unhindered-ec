use anyhow::Result;
use rand::rngs::ThreadRng;

use super::{Composable, Operator};

/// Recombine (usually two or more) genomes into a new
/// genome.
///
/// GS: a generic for the _group_ of input genomes.
/// Output: An associated type indicating what the result
///   type of the recombination action will be.
///
/// Typically `Output` will be the same as the type of the
/// individual genomes contained in `GS`, but that isn't captured
/// (or required) here.
pub trait Recombinator<GS> {
    type Output;

    /// # Errors
    /// This will return an error if there's some problem with the recombination.
    fn recombine(&self, genomes: GS, rng: &mut ThreadRng) -> Result<Self::Output>;
}

pub struct Recombine<R> {
    recombinator: R,
}

impl<R> Recombine<R> {
    pub const fn new(recombinator: R) -> Self {
        Self { recombinator }
    }
}

impl<R, G> Operator<G> for Recombine<R>
where
    R: Recombinator<G>,
{
    type Output = R::Output;

    fn apply(&self, genomes: G, rng: &mut ThreadRng) -> Result<Self::Output> {
        self.recombinator.recombine(genomes, rng)
    }
}
impl<R> Composable for Recombine<R> {}
