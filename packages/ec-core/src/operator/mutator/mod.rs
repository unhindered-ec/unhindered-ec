use rand::rngs::ThreadRng;

use super::{Composable, Operator};

/// Mutate a given genome of type `G` generating a new
/// genome of the same type.
///
/// # Examples
///
/// ```
/// # use rand::thread_rng;
/// # use rand::rngs::ThreadRng;
/// # use rand::Rng;
/// # use ec_core::operator::mutator::Mutator;
/// #
/// struct FlipOne;
///
/// impl Mutator<Vec<bool>> for FlipOne {
///     fn mutate(&self, mut genome: Vec<bool>, rng: &mut ThreadRng) -> anyhow::Result<Vec<bool>> {
///         let index = rng.gen_range(0..genome.len());
///         genome[index] = !genome[index];
///         Ok(genome)
///     }
/// }
///
/// let genome = vec![true, false, false, true];
/// let child_genome = FlipOne.mutate(genome.clone(), &mut thread_rng()).unwrap();
/// let num_diffs = genome
///     .iter()
///     .zip(child_genome.iter())
///     .filter(|(x, y)| x != y)
///     .count();
/// assert_eq!(num_diffs, 1);
/// ```
pub trait Mutator<G> {
    /// Mutate the given `genome`
    ///
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

    /// Apply this `Mutator` as an `Operator`
    fn apply(&self, genome: G, rng: &mut ThreadRng) -> Result<Self::Output, Self::Error> {
        self.mutator.mutate(genome, rng)
    }
}
impl<M> Composable for Mutate<M> {}

/// Implement `Mutator` for a reference to a `Mutator`
impl<M, G> Mutator<G> for &M
where
    M: Mutator<G>,
{
    fn mutate(&self, genome: G, rng: &mut ThreadRng) -> anyhow::Result<G> {
        (**self).mutate(genome, rng)
    }
}
