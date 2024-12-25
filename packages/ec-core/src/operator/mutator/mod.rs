#[cfg(feature = "erased")]
use std::{
    cell::{Ref, RefMut},
    rc::Rc,
    sync::Arc,
};

use rand::Rng;
#[cfg(feature = "erased")]
use rand::RngCore;

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
/// # use rand::{rng, Rng};
/// # use ec_core::operator::mutator::Mutator;
/// # use std::convert::Infallible;
/// #
/// type Genome<T> = [T; 4];
///
/// struct FlipOne;
///
/// impl Mutator<Genome<bool>> for FlipOne {
///     type Error = Infallible;
///
///     fn mutate<R: Rng + ?Sized>(
///         &self,
///         mut genome: Genome<bool>,
///         rng: &mut R,
///     ) -> Result<Genome<bool>, Self::Error> {
///         let index = rng.random_range(0..genome.len());
///         genome[index] = !genome[index];
///         Ok(genome)
///     }
/// }
///
/// let genome = [true, false, false, true];
/// let child_genome = FlipOne.mutate(genome.clone(), &mut rng()).unwrap();
/// let num_diffs = genome
///     .iter()
///     .zip(child_genome.iter()) // Pair up corresponding elements from the two genomes
///     .filter(|(x, y)| x != y) // Filter out pairs where the elements are the same
///     .count();
/// assert_eq!(num_diffs, 1);
/// ```
pub trait Mutator<G> {
    type Error;

    /// Mutate the given `genome` returning a new genome of the same type (`G`)
    ///
    /// # Errors
    ///
    /// This will return an error if there is an error mutating the given
    /// genome. This will usually be because the given `genome` is invalid in
    /// some way, thus making the mutation impossible.
    fn mutate<R: Rng + ?Sized>(&self, genome: G, rng: &mut R) -> Result<G, Self::Error>;
}

#[cfg(feature = "erased")]
pub trait DynMutator<G, E = Box<dyn std::error::Error + Send + Sync>> {
    /// # Errors
    ///
    /// This will return an error if there is an error mutating the given
    /// genome. This will usually be because the given `genome` is invalid in
    /// some way, thus making the mutation impossible.
    fn dyn_mutate(&self, genome: G, rng: &mut dyn RngCore) -> Result<G, E>;
}

#[cfg(feature = "erased")]
static_assertions::assert_obj_safe!(DynMutator<()>);

#[cfg(feature = "erased")]
impl<T, G, E> DynMutator<G, E> for T
where
    T: Mutator<G, Error: Into<E>>,
{
    fn dyn_mutate(&self, genome: G, rng: &mut dyn RngCore) -> Result<G, E> {
        self.mutate(genome, rng).map_err(Into::into)
    }
}

#[cfg(feature = "erased")]
macro_rules! dyn_mutator_impl {
    ($t: ty) => {
        #[cfg(feature = "erased")]
        impl<G, E> Mutator<G> for $t
        {
            type Error = E;

            fn mutate<R: Rng + ?Sized>(&self, genome: G, mut rng: &mut R) -> Result<G, E> {
                (**self).dyn_mutate(genome, &mut rng)
            }
        }
    };
    ($($t: ty),+ $(,)?) => {
        $(dyn_mutator_impl!($t);)+
    }
}

#[cfg(feature = "erased")]
// TODO: Create a macro to do this in a nicer way without needing to manually
// repeat all the pointer types everywhere we provide a type erased trait
dyn_mutator_impl!(
    &dyn DynMutator<G, E>,
    &(dyn DynMutator<G, E> + Send),
    &(dyn DynMutator<G, E> + Sync),
    &(dyn DynMutator<G, E> + Send + Sync),
    &mut dyn DynMutator<G, E>,
    &mut (dyn DynMutator<G, E> + Send),
    &mut (dyn DynMutator<G, E> + Sync),
    &mut (dyn DynMutator<G, E> + Send + Sync),
    Box<dyn DynMutator<G, E>>,
    Box<dyn DynMutator<G, E> + Send>,
    Box<dyn DynMutator<G, E> + Sync>,
    Box<dyn DynMutator<G, E> + Send + Sync>,
    Arc<dyn DynMutator<G, E>>,
    Arc<dyn DynMutator<G, E> + Send>,
    Arc<dyn DynMutator<G, E> + Sync>,
    Arc<dyn DynMutator<G, E> + Send + Sync>,
    Rc<dyn DynMutator<G, E>>,
    Rc<dyn DynMutator<G, E> + Send>,
    Rc<dyn DynMutator<G, E> + Sync>,
    Rc<dyn DynMutator<G, E> + Send + Sync>,
    Ref<'_, dyn DynMutator<G, E>>,
    Ref<'_, dyn DynMutator<G, E> + Send>,
    Ref<'_, dyn DynMutator<G, E> + Sync>,
    Ref<'_, dyn DynMutator<G, E> + Send + Sync>,
    RefMut<'_, dyn DynMutator<G, E>>,
    RefMut<'_, dyn DynMutator<G, E> + Send>,
    RefMut<'_, dyn DynMutator<G, E> + Sync>,
    RefMut<'_, dyn DynMutator<G, E> + Send + Sync>,
);

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
/// # use rand::{Rng, rng};
/// #
/// # use ec_core::operator::{
/// #     mutator::{Mutate, Mutator},
/// #     Operator,
/// # };
/// # use std::convert::Infallible;
/// #
/// type Genome<T> = [T; 4];
///
/// // A simple mutator that flips the first `bool` in a `Genome<bool>`.
/// struct FlipFirst;
///
/// impl Mutator<Genome<bool>> for FlipFirst {
///     type Error = Infallible;
///
///     fn mutate<R: Rng + ?Sized>(
///         &self,
///         mut genome: Genome<bool>,
///         _: &mut R,
///     ) -> Result<Genome<bool>, Self::Error> {
///         genome[0] = !genome[0];
///         Ok(genome)
///     }
/// }
///
/// let genome = [true, false, false, true];
/// let mutator_result = FlipFirst.mutate(genome.clone(), &mut rng()).unwrap();
///
/// // Create a `Mutate` operator from the `FlipFirst` mutator
/// let mutate = Mutate::new(FlipFirst);
/// let operator_result = mutate.apply(genome.clone(), &mut rng()).unwrap();
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
/// # use rand::{Rng, rng};
/// #
/// type Genome<T> = [T; 4];
///
/// // A simple mutator that flips a random `bool` in a `Genome<bool>`.
/// struct FlipOne;
///
/// impl Mutator<Genome<bool>> for FlipOne {
///     type Error = Infallible;
///
///     fn mutate<R: Rng + ?Sized>(
///         &self,
///         mut genome: Genome<bool>,
///         rng: &mut R,
///     ) -> Result<Genome<bool>, Self::Error> {
///         let index = rng.random_range(0..genome.len());
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
///     fn apply<R: Rng + ?Sized>(
///         &self,
///         genome: Genome<bool>,
///         _: &mut R,
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
/// assert_eq!(chain.apply(genome, &mut rng())?, 1);
/// # Ok::<(), Box<dyn std::error::Error>>(())
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
/// # use rand::{Rng, rng};
/// # use std::convert::Infallible;
/// #
/// # type Genome<T> = [T; 4];
/// #
/// # // A simple mutator that flips a random `bool` in a `Genome<bool>`.
/// # struct FlipOne;
/// #
/// # impl Mutator<Genome<bool>> for FlipOne {
/// #    type Error = Infallible;
/// #
/// #    fn mutate<R: Rng + ?Sized>(&self, mut genome: Genome<bool>, rng: &mut R) -> Result<Genome<bool>, Self::Error> {
/// #        let index = rng.random_range(0..genome.len());
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
/// #    type Error = Infallible;
/// #
/// #    fn apply<R: Rng + ?Sized>(&self, genome: Genome<bool>, _: &mut R) -> Result<Self::Output, Self::Error> {
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
/// assert_eq!(chain.apply(genome, &mut rng())?, 1);
/// # Ok::<(), Box<dyn std::error::Error>>(())
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
    type Error = M::Error;

    /// Apply this `Mutator` as an `Operator`
    fn apply<R: Rng + ?Sized>(&self, genome: G, rng: &mut R) -> Result<Self::Output, Self::Error> {
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
    type Error = M::Error;

    fn mutate<R: Rng + ?Sized>(&self, genome: G, rng: &mut R) -> Result<G, Self::Error> {
        (**self).mutate(genome, rng)
    }
}

impl<M, G> Mutator<G> for &mut M
where
    M: Mutator<G>,
{
    type Error = M::Error;

    fn mutate<R: Rng + ?Sized>(&self, genome: G, rng: &mut R) -> Result<G, Self::Error> {
        (**self).mutate(genome, rng)
    }
}

#[cfg(test)]
mod tests {
    use std::convert::Infallible;

    use rand::{Rng, rng};

    use super::Mutator;
    use crate::operator::{Composable, Operator, mutator::Mutate};

    type Genome<T> = [T; 4];

    struct FlipOne;

    impl Mutator<Genome<bool>> for FlipOne {
        type Error = Infallible;

        fn mutate<R: Rng + ?Sized>(
            &self,
            mut genome: Genome<bool>,
            rng: &mut R,
        ) -> Result<Genome<bool>, Self::Error> {
            let index = rng.random_range(0..genome.len());
            genome[index] = !genome[index];
            Ok(genome)
        }
    }

    // A simple `Operator` that takes a `Genome<bool>` and returns the number
    // of `true` values in the genome.
    struct CountTrue;

    impl Operator<Genome<bool>> for CountTrue {
        type Output = usize;
        type Error = Infallible;

        fn apply<R: Rng + ?Sized>(
            &self,
            genome: Genome<bool>,
            _: &mut R,
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
        let child_genome = FlipOne.mutate(genome, &mut rng()).unwrap();
        assert_eq!(count_differences(&genome, &child_genome), 1);
    }

    #[test]
    fn can_wrap_mutator() {
        let genome = [true, false, false, true];
        let mutator = FlipOne;
        // Wrap the mutator in a `Mutate` operator
        let operator = Mutate::new(mutator);
        let child_genome = operator.apply(genome, &mut rng()).unwrap();
        assert_eq!(count_differences(&genome, &child_genome), 1);
    }

    #[test]
    fn can_wrap_mutator_reference() {
        let genome = [true, false, false, true];
        let mutator = FlipOne;
        // Wrap a reference to the mutator in a `Mutate` to make it an `Operator`.
        let operator = Mutate::new(&mutator);
        let child_genome = operator.apply(genome, &mut rng()).unwrap();
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
        assert_eq!(chain.apply(genome, &mut rng()).unwrap(), 1);
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
        assert_eq!(chain.apply(genome, &mut rng()).unwrap(), 1);
    }
}
