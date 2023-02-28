use rand::rngs::ThreadRng;

use super::{Operator, Composable};

pub mod mutate_with_one_over_length;
pub mod mutate_with_rate;
pub mod two_point_xo;
pub mod uniform_xo;

// TODO: Move mutations into `operator::mutator` and
//   crossovers into `operator::crossover`, and simplify
//   the type names to not repeat "mutate_with" and "xo"
//   everywhere.

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
    M: Mutator<G>
{
    type Output = G;

    fn apply(&self, genome: G, rng: &mut ThreadRng) -> Self::Output {
        self.mutator.mutate(genome, rng)
    }
}
impl<M> Composable for Mutate<M> {}

pub trait Recombinator<G> {
    type Output;

    fn recombine(&self, genomes: G, rng: &mut ThreadRng) -> Self::Output;
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

    fn apply(&self, genomes: G, rng: &mut ThreadRng) -> Self::Output {
        self.recombinator.recombine(genomes, rng)
    }
}
impl<R> Composable for Recombine<R> {}