use rand::rngs::ThreadRng;

use super::{Composable, Operator};

/// Recombine (usually two or more) genomes into a new
/// genome of the same type
///
/// GS: a generic for the _group_ of input genomes.
/// Output: An associated type indicating what the result
///   type of the recombination action will be.
///
/// Typically `Output` will be the same as the type of the
/// individual genomes contained in `GS`, but that isn't captured
/// (or required) here.
///
/// # Examples
///
/// ```
/// # use rand::thread_rng;
/// # use rand::rngs::ThreadRng;
/// # use rand::Rng;
/// # use ec_core::operator::recombinator::Recombinator;
/// #
/// struct SwapOne;
/// type Parents = (Vec<u8>, Vec<u8>);
///
/// impl Recombinator<Parents> for SwapOne {
///     type Output = Vec<u8>;
///
///     fn recombine(
///         &self,
///         (mut first_parent, second_parent): Parents,
///         rng: &mut ThreadRng,
///     ) -> anyhow::Result<Vec<u8>> {
///         let index = rng.gen_range(0..first_parent.len());
///         first_parent[index] = second_parent[index];
///         Ok(first_parent)
///     }
/// }
///
/// let first_parent = vec![0, 0, 0, 0];
/// let second_parent = vec![1, 1, 1, 1];
/// let child = SwapOne
///     .recombine((first_parent, second_parent), &mut thread_rng())
///     .unwrap();
/// let num_zeros = child.iter().filter(|&&x| x == 0).count();
/// let num_ones = child.iter().filter(|&&x| x == 1).count();
/// assert_eq!(num_zeros, 3);
/// assert_eq!(num_ones, 1);
/// ```
pub trait Recombinator<GS> {
    type Output;

    /// # Errors
    /// This will return an error if there's some problem with the
    /// recombination.
    fn recombine(&self, genomes: GS, rng: &mut ThreadRng) -> anyhow::Result<Self::Output>;
}

/// A wrapper that converts a `Mutator` into an `Operator`
///
/// See [the `operator` module docs](crate::operator#wrappers) for more on
/// the design decisions behind using wrappers.
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
    type Error = anyhow::Error;

    fn apply(&self, genomes: G, rng: &mut ThreadRng) -> Result<Self::Output, Self::Error> {
        self.recombinator.recombine(genomes, rng)
    }
}
impl<R> Composable for Recombine<R> {}

/// Implement `Recombinator` for a reference to a `Recombinator`
impl<R, GS> Recombinator<GS> for &R
where
    R: Recombinator<GS>,
{
    type Output = R::Output;

    fn recombine(&self, genomes: GS, rng: &mut ThreadRng) -> anyhow::Result<Self::Output> {
        (**self).recombine(genomes, rng)
    }
}
