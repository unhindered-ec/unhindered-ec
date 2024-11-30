use rand::rngs::ThreadRng;

use super::{Composable, Operator};

/// Mutate a genome of type `G` generating a new
/// genome of the same type (`G`).
///
/// Implementations of this trait are typically representation dependent,
/// so see crates like [`ec_linear`](../../../ec_linear/mutator/index.html) for
/// examples of mutators on linear genomes.
///
/// # See also
///
/// See [`Mutate`] for a wrapper that converts a `Mutator` into an [`Operator`],
/// allowing mutators to be used in chains of operators.
///
/// # Examples
///
/// In this example, we define a `Mutator` that flips one random bit in a
/// `Genome<bool>`. We then use this `Mutator` to mutate a genome, and check
/// that exactly one bit has changed.
///
/// ```
/// # use rand::{rngs::ThreadRng, thread_rng, Rng};
/// # use ec_core::operator::mutator::Mutator;
/// #
/// type Genome<T> = [T; 4];
///
/// struct FlipOne;
///
/// impl Mutator<Genome<bool>> for FlipOne {
///     fn mutate(
///         &self,
///         mut genome: Genome<bool>,
///         rng: &mut ThreadRng,
///     ) -> anyhow::Result<Genome<bool>> {
///         let index = rng.gen_range(0..genome.len());
///         genome[index] = !genome[index];
///         Ok(genome)
///     }
/// }
///
/// let genome = [true, false, false, true];
/// let child_genome = FlipOne.mutate(genome.clone(), &mut thread_rng()).unwrap();
/// let num_diffs = genome
///     .iter()
///     .zip(child_genome.iter()) // Pair up corresponding elements from the two genomes
///     .filter(|(x, y)| x != y) // Filter out pairs where the elements are the same
///     .count();
/// assert_eq!(num_diffs, 1);
/// ```
pub trait Mutator<G> {
    /// Mutate the given `genome` returning a new genome of the same type (`G`)
    ///
    /// # Errors
    ///
    /// This will return an error if there is an error mutating the given
    /// genome. This will usually be because the given `genome` is invalid in
    /// some way, thus making the mutation impossible.
    fn mutate(&self, genome: G, rng: &mut ThreadRng) -> anyhow::Result<G>;
}

/// A wrapper that converts a [`Mutator`] into an [`Operator`].
///
/// This allows the inclusion of `Mutator`s in chains of operators.
///
/// # See also
///
/// See [`Mutator`] for the details of that trait.
///
/// See [the `operator` module docs](crate::operator#wrappers) for more on the
/// design decisions behind using wrappers to convert things like a [`Mutator`]
/// into an [`Operator`].
///
/// # Examples
///
/// Here we illustrate the implementation of a simple [`Mutator`],
/// `FlipFirst`, which flips the first `bool` in a `Genome<bool>`. We then wrap
/// that in a [`Mutate`] to create an operator. Then calling [`Operator::apply`]
/// on the operator is the same as calling [`Mutator::mutate`] directly on the
/// mutator.
///
/// ```
/// # use rand::{rngs::ThreadRng, thread_rng};
/// #
/// # use ec_core::operator::{
/// #     mutator::{Mutate, Mutator},
/// #     Operator,
/// # };
/// #
/// type Genome<T> = [T; 4];
///
/// // A simple mutator that flips the first `bool` in a `Genome<bool>`.
/// struct FlipFirst;
///
/// impl Mutator<Genome<bool>> for FlipFirst {
///     fn mutate(
///         &self,
///         mut genome: Genome<bool>,
///         _: &mut ThreadRng,
///     ) -> anyhow::Result<Genome<bool>> {
///         genome[0] = !genome[0];
///         Ok(genome)
///     }
/// }
///
/// let genome = [true, false, false, true];
/// let mutator_result = FlipFirst.mutate(genome.clone(), &mut thread_rng()).unwrap();
///
/// // Create a `Mutate` operator from the `FlipFirst` mutator
/// let mutate = Mutate::new(FlipFirst);
/// let operator_result = mutate.apply(genome.clone(), &mut thread_rng()).unwrap();
///
/// assert_eq!(mutator_result, operator_result);
/// ```
///
/// Because [`Mutate`] and [`Operator`] are [`Composable`], we can chain them
/// using the [`Composable::then()`] method, and then [`Operator::apply`] the
/// resulting chain to an input. In the examples below we apply the chain to a
/// genome that is a vector of `false`. The `FlipOne` mutator will flip one of
/// these to `true`, and the `CountTrue` operator will count the number of
/// `true` values in the resulting genome, which should then be 1.
///
/// ```
/// # use std::convert::Infallible;
/// # use ec_core::operator::{
/// #     mutator::{Mutate, Mutator},
/// #     Composable, Operator,
/// # };
/// # use rand::{rngs::ThreadRng, thread_rng, Rng};
/// #
/// type Genome<T> = [T; 4];
///
/// // A simple mutator that flips a random `bool` in a `Genome<bool>`.
/// struct FlipOne;
///
/// impl Mutator<Genome<bool>> for FlipOne {
///     fn mutate(
///         &self,
///         mut genome: Genome<bool>,
///         rng: &mut ThreadRng,
///     ) -> anyhow::Result<Genome<bool>> {
///         let index = rng.gen_range(0..genome.len());
///         genome[index] = !genome[index];
///         Ok(genome)
///     }
/// }
///
/// // A simple `Operator` that takes a `Genome<bool>` and returns the number
/// // of `true` values in the genome.
/// struct CountTrue;
///
/// impl Operator<Genome<bool>> for CountTrue {
///     type Output = usize;
///     type Error = Infallible;
///
///     fn apply(
///         &self,
///         genome: Genome<bool>,
///         _: &mut ThreadRng,
///     ) -> Result<Self::Output, Self::Error> {
///         Ok(genome.iter().filter(|&&x| x).count())
///     }
/// }
/// impl Composable for CountTrue {}
///
/// // If we flip exactly one of these, we should have exactly one `true`.
/// let genome = [false, false, false, false];
/// // Wrap the mutator in a `Mutate` operator so we can chain it with `CountTrue`
/// let operator = Mutate::new(FlipOne);
/// let chain = operator.then(CountTrue);
/// assert_eq!(chain.apply(genome, &mut thread_rng())?, 1);
/// # Ok::<(), anyhow::Error>(())
/// ```
///
/// We can also pass a _reference_ to a [`Mutator`] (i.e., `&Mutator`) to
/// [`Mutate`], allowing us to pass references to mutators into chains of
/// operators without requiring or giving up ownership of the mutator.
///
/// ```
/// # use ec_core::operator::{
/// #     mutator::{Mutate, Mutator},
/// #     Composable, Operator,
/// # };
/// # use rand::{rngs::ThreadRng, thread_rng, Rng};
/// #
/// # type Genome<T> = [T; 4];
/// #
/// # // A simple mutator that flips a random `bool` in a `Genome<bool>`.
/// # struct FlipOne;
/// #
/// # impl Mutator<Genome<bool>> for FlipOne {
/// #    fn mutate(&self, mut genome: Genome<bool>, rng: &mut ThreadRng) -> anyhow::Result<Genome<bool>> {
/// #        let index = rng.gen_range(0..genome.len());
/// #        genome[index] = !genome[index];
/// #        Ok(genome)
/// #    }
/// # }
/// #
/// # // A simple `Operator` that takes a `Genome<bool>` and returns the number
/// # // of `true` values in the genome.
/// # struct CountTrue;
/// #
/// # impl Operator<Genome<bool>> for CountTrue {
/// #    type Output = usize;
/// #    type Error = anyhow::Error;
/// #
/// #    fn apply(&self, genome: Genome<bool>, _: &mut ThreadRng) -> Result<Self::Output, Self::Error> {
/// #        Ok(genome.iter().filter(|&&x| x).count())
/// #    }
/// # }
/// # impl Composable for CountTrue {}
/// #
/// // If we flip exactly one of these, we should have exactly one `true`.
/// let genome = [false, false, false, false];
/// // Wrap a reference to the mutator in a `Mutate` operator so we can chain it with `CountTrue`
/// let mutate = Mutate::new(&FlipOne);
/// let chain = mutate.then(CountTrue);
/// assert_eq!(chain.apply(genome, &mut thread_rng())?, 1);
/// # Ok::<(), anyhow::Error>(())
/// ```
pub struct Mutate<M> {
    /// The wrapped [`Mutator`] that this [`Mutate`] will apply
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

/// Implement [`Mutator`] for a reference to a [`Mutator`].
/// This allows us to wrap a reference to a [`Mutator`] in a [`Mutate`] operator
/// and use it in a chain of operators.
impl<M, G> Mutator<G> for &M
where
    M: Mutator<G>,
{
    fn mutate(&self, genome: G, rng: &mut ThreadRng) -> anyhow::Result<G> {
        (**self).mutate(genome, rng)
    }
}

#[expect(
    clippy::unwrap_used,
    reason = "Panicking is the best way to deal with errors in unit tests"
)]
#[cfg(test)]
mod tests {
    use rand::{Rng, rngs::ThreadRng, thread_rng};

    use super::Mutator;
    use crate::operator::{Composable, Operator, mutator::Mutate};

    type Genome<T> = [T; 4];

    struct FlipOne;

    impl Mutator<Genome<bool>> for FlipOne {
        fn mutate(
            &self,
            mut genome: Genome<bool>,
            rng: &mut ThreadRng,
        ) -> anyhow::Result<Genome<bool>> {
            let index = rng.gen_range(0..genome.len());
            genome[index] = !genome[index];
            Ok(genome)
        }
    }

    // A simple `Operator` that takes a `Genome<bool>` and returns the number
    // of `true` values in the genome.
    struct CountTrue;

    impl Operator<Genome<bool>> for CountTrue {
        type Output = usize;
        type Error = anyhow::Error;

        fn apply(
            &self,
            genome: Genome<bool>,
            _: &mut ThreadRng,
        ) -> Result<Self::Output, Self::Error> {
            Ok(genome.iter().filter(|&&x| x).count())
        }
    }
    impl Composable for CountTrue {}

    fn count_differences(genome: &[bool], child_genome: &[bool]) -> usize {
        genome
            .iter()
            // Pair up corresponding elements from the two genomes
            .zip(child_genome.iter())
            // Filter out pairs where the elements are the same
            .filter(|(x, y)| x != y)
            .count()
    }

    #[test]
    fn flip_one() {
        let genome = [true, false, false, true];
        let child_genome = FlipOne.mutate(genome, &mut thread_rng()).unwrap();
        assert_eq!(count_differences(&genome, &child_genome), 1);
    }

    #[test]
    fn can_wrap_mutator() {
        let genome = [true, false, false, true];
        let mutator = FlipOne;
        // Wrap the mutator in a `Mutate` operator
        let operator = Mutate::new(mutator);
        let child_genome = operator.apply(genome, &mut thread_rng()).unwrap();
        assert_eq!(count_differences(&genome, &child_genome), 1);
    }

    #[test]
    fn can_wrap_mutator_reference() {
        let genome = [true, false, false, true];
        let mutator = FlipOne;
        // Wrap a reference to the mutator in a `Mutate` to make it an `Operator`.
        let operator = Mutate::new(&mutator);
        let child_genome = operator.apply(genome, &mut thread_rng()).unwrap();
        assert_eq!(count_differences(&genome, &child_genome), 1);
    }

    #[test]
    fn can_chain_mutator_and_operator() {
        // If we flip exactly one of these, we should have exactly one `true`.
        let genome = [false, false, false, false];
        let mutator = FlipOne;
        let operator = Mutate::new(mutator);
        let count_true = CountTrue;
        let chain = operator.then(count_true);
        assert_eq!(chain.apply(genome, &mut thread_rng()).unwrap(), 1);
    }

    #[test]
    fn can_chain_with_mutator_reference() {
        // If we flip exactly one of these, we should have exactly one `true`.
        let genome = [false, false, false, false];
        let mutator = FlipOne;
        // Wrap a reference to the mutator in a `Mutate` to make it an `Operator`.
        let operator = Mutate::new(&mutator);
        let count_true = CountTrue;
        let chain = operator.then(count_true);
        assert_eq!(chain.apply(genome, &mut thread_rng()).unwrap(), 1);
    }
}
