use rand::rngs::ThreadRng;

use super::{Composable, Operator};

pub mod two_point_xo;
pub mod uniform_xo;

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
