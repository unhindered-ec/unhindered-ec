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

/// Recombine (usually two or more) genomes into a new
/// genome of the same type,
///
/// - `GS` represents the type for the _group_ of input genomes. This is
///   typically a tuple, an array, or some other collection of genomes that will
///   be recombined.
/// - `Output`: An associated type indicating what the result type of the
///   recombination action will be.
///
/// Typically `Output` will be the same as the type of the
/// individual genomes contained in `GS`, but that isn't captured
/// (or required) here.
///
/// Implementations of this trait are typically representation dependent,
/// so see crates like [`ec_linear`](../../../ec_linear/recombinator/index.html)
/// for examples of recombinators on linear genomes.
///
/// # See also
///
/// See [`Recombine`] for a wrapper that converts a `Recombinator` into an
/// [`Operator`], allowing recombinators to be used in chains of operators.
///
/// # Examples
///
/// In this example, we define a `Recombinator` that swaps one randomly
/// chosen element from the first parent with the corresponding element
/// from the second parent. We then use this recombinator to create a
/// new genome from two parents, confirming that the child genome has
/// all but one element from the first parent and one element from the
/// second parent.
///
/// ```
/// # use rand::{rng, Rng};
/// # use ec_core::operator::recombinator::Recombinator;
/// # use std::convert::Infallible;
/// #
/// struct SwapOne;
/// type Genome<T> = [T; 4];
/// type Parents<T> = (Genome<T>, Genome<T>);
///
/// impl<T: Copy> Recombinator<Parents<T>> for SwapOne {
///     type Output = Genome<T>;
///     type Error = Infallible;
///
///     fn recombine<R: Rng + ?Sized>(
///         &self,
///         (mut first_parent, second_parent): Parents<T>,
///         rng: &mut R,
///     ) -> Result<Genome<T>, Self::Error> {
///         let index = rng.random_range(0..first_parent.len());
///         first_parent[index] = second_parent[index];
///         Ok(first_parent)
///     }
/// }
///
/// // Swpapping one element from the first parent with the second parent
/// // should result in a child with three zeros and a single one.
/// let first_parent = [0, 0, 0, 0];
/// let second_parent = [1, 1, 1, 1];
/// let child = SwapOne.recombine((first_parent, second_parent), &mut rng())?;
/// let num_zeros = child.iter().filter(|&&x| x == 0).count();
/// let num_ones = child.iter().filter(|&&x| x == 1).count();
/// assert_eq!(num_zeros, 3);
/// assert_eq!(num_ones, 1);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub trait Recombinator<GS> {
    /// The type of the output genome after recombination. This is typically
    /// the same as the type of genomes in the input group of type `GS`, but
    /// that isn't strictly required here.
    type Output;

    /// The type of the error that can happen using this recombinator
    type Error;

    /// Recombine the given `genomes` returning a new genome of type `Output`.
    ///
    /// # Errors
    ///
    /// This will return an error if there is an error recombining the given
    /// parent genomes. This will usually be because the given `genomes` are
    /// invalid in some way, thus making recombination impossible.
    fn recombine<R: Rng + ?Sized>(
        &self,
        genomes: GS,
        rng: &mut R,
    ) -> Result<Self::Output, Self::Error>;
}

#[cfg(feature = "erased")]
pub trait DynRecombinator<GS, E = Box<dyn std::error::Error + Send + Sync>> {
    type Output;

    /// Recombine the given `genomes` returning a new genome of type `Output`.
    ///
    /// # Errors
    ///
    /// This will return an error if there is an error recombining the given
    /// parent genomes. This will usually be because the given `genomes` are
    /// invalid in some way, thus making recombination impossible.
    fn dyn_recombine(&self, genomes: GS, rng: &mut dyn RngCore) -> Result<Self::Output, E>;
}

#[cfg(feature = "erased")]
static_assertions::assert_obj_safe!(DynRecombinator<(), Output = ()>);

#[cfg(feature = "erased")]
impl<T, GS, E> DynRecombinator<GS, E> for T
where
    T: Recombinator<GS, Error: Into<E>>,
{
    type Output = T::Output;

    fn dyn_recombine(&self, genomes: GS, rng: &mut dyn RngCore) -> Result<Self::Output, E> {
        self.recombine(genomes, rng).map_err(Into::into)
    }
}

#[cfg(feature = "erased")]
macro_rules! dyn_recombinator_impl {
    ($t: ty) => {
        #[cfg(feature = "erased")]
        impl<GS, O, E> Recombinator<GS> for $t
        {
            type Error = E;
            type Output = O;

            fn recombine<R: Rng + ?Sized>(
                &self,
                genomes: GS,
                mut rng: &mut R
            ) -> Result<Self::Output, Self::Error> {
                (**self).dyn_recombine(genomes, &mut rng)
            }
        }
    };
    ($($t: ty),+ $(,)?) => {
        $(dyn_recombinator_impl!($t);)+
    }
}

#[cfg(feature = "erased")]
// TODO: Create a macro to do this in a nicer way without needing to manually
// repeat all the pointer types everywhere we provide a type erased trait
dyn_recombinator_impl!(
    &dyn DynRecombinator<GS, E, Output = O>,
    &(dyn DynRecombinator<GS, E, Output = O> + Send),
    &(dyn DynRecombinator<GS, E, Output = O> + Sync),
    &(dyn DynRecombinator<GS, E, Output = O> + Send + Sync),
    &mut dyn DynRecombinator<GS, E, Output = O>,
    &mut (dyn DynRecombinator<GS, E, Output = O> + Send),
    &mut (dyn DynRecombinator<GS, E, Output = O> + Sync),
    &mut (dyn DynRecombinator<GS, E, Output = O> + Send + Sync),
    Box<dyn DynRecombinator<GS, E, Output = O>>,
    Box<dyn DynRecombinator<GS, E, Output = O> + Send>,
    Box<dyn DynRecombinator<GS, E, Output = O> + Sync>,
    Box<dyn DynRecombinator<GS, E, Output = O> + Send + Sync>,
    Arc<dyn DynRecombinator<GS, E, Output = O>>,
    Arc<dyn DynRecombinator<GS, E, Output = O> + Send>,
    Arc<dyn DynRecombinator<GS, E, Output = O> + Sync>,
    Arc<dyn DynRecombinator<GS, E, Output = O> + Send + Sync>,
    Rc<dyn DynRecombinator<GS, E, Output = O>>,
    Rc<dyn DynRecombinator<GS, E, Output = O> + Send>,
    Rc<dyn DynRecombinator<GS, E, Output = O> + Sync>,
    Rc<dyn DynRecombinator<GS, E, Output = O> + Send + Sync>,
    Ref<'_, dyn DynRecombinator<GS, E, Output = O>>,
    Ref<'_, dyn DynRecombinator<GS, E, Output = O> + Send>,
    Ref<'_, dyn DynRecombinator<GS, E, Output = O> + Sync>,
    Ref<'_, dyn DynRecombinator<GS, E, Output = O> + Send + Sync>,
    RefMut<'_, dyn DynRecombinator<GS, E, Output = O>>,
    RefMut<'_, dyn DynRecombinator<GS, E, Output = O> + Send>,
    RefMut<'_, dyn DynRecombinator<GS, E, Output = O> + Sync>,
    RefMut<'_, dyn DynRecombinator<GS, E, Output = O> + Send + Sync>,
);

/// A wrapper that converts a `Recombinator` into an `Operator`,
///
/// This allows the inclusion of `Recombinator`s in chains of operators.
///
/// # See also
///
/// See [`Recombinator`] for the details of that trait.
///
/// See [the `operator` module docs](crate::operator#wrappers) for more on
/// the design decisions behind using wrappers to convert things like a
/// [`Recombinator`] into an [`Operator`].
///
/// # Examples
///
/// Here we illustrate the implementation of `SwapFirst`, a simple
/// [`Recombinator`] that acts on a pair of vectors, replacing the first element
/// from the first vector with the first element from the the second vector. We
/// then wrap that in a [`Recombine`] to create an [`Operator`]. Calling
/// [`Operator::apply`] on the resulting operator should then be the same as
/// calling [`Recombinator::recombine`] directly on the recombinator.
///
/// ```
/// # use rand::{Rng, rng};
/// # use ec_core::operator::{Operator, recombinator::{Recombinator, Recombine}};
/// # use std::convert::Infallible;
/// #
/// // A simple `Recombinator` that swaps one element from the second parent
/// // into the corresponding position in the first parent.
/// struct SwapFirst;
///
/// type Genome<T> = [T; 4];
/// type Parents<T> = (Genome<T>, Genome<T>);
/// impl<T: Copy> Recombinator<Parents<T>> for SwapFirst {
///     type Output = Genome<T>;
///     type Error = Infallible;
///
///     fn recombine<R: Rng + ?Sized>(
///         &self,
///         (mut first_parent, second_parent): Parents<T>,
///         _: &mut R,
///     ) -> Result<Genome<T>, Self::Error> {
///         first_parent[0] = second_parent[0];
///         Ok(first_parent)
///     }
/// }
///
/// let first_parent = [0, 0, 0, 0];
/// let second_parent = [1, 1, 1, 1];
///
/// let recombinator = SwapFirst;
/// let recombinator_result =
///     recombinator.recombine((first_parent.clone(), second_parent.clone()), &mut rng())?;
///
/// // Wrap the recombinator in a `Recombine` to make it an `Operator`.
/// let recombine = Recombine::new(recombinator);
/// let operator_result = recombine.apply((first_parent, second_parent), &mut rng())?;
///
/// assert_eq!(recombinator_result, operator_result);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// Because [`Recombine`] and [`Operator`] are both [`Composable`], you can
/// chain them together using the [`Composable::then()`] method, and then
/// [`Operator::apply()`] the resulting chain to an input. In the examples below
/// we apply the chain to a pair of genomes, one all `true` and one all
/// `false`. The chain will first swap apply `SwapOne` to swap a randomly chosen
/// element (a `false`) from the second parent into the first parent, and then
/// `CountTrue` will count the number of `true` values in the resulting genome,
/// which should be 3.
///
/// ```
/// # use std::convert::Infallible;
/// # use rand::{rng, Rng};
/// #
/// # use ec_core::operator::{recombinator::{Recombinator, Recombine}, Composable, Operator};
/// #
/// // A simple `Recombinator` that swaps one element from the second parent
/// // into the corresponding position in the first parent.
/// struct SwapOne;
///
/// type Genome<T> = [T; 4];
/// type Parents<T> = (Genome<T>, Genome<T>);
/// impl<T: Copy> Recombinator<Parents<T>> for SwapOne {
///     type Output = Genome<T>;
///     type Error = Infallible;
///
///     fn recombine<R: Rng + ?Sized>(
///         &self,
///         (mut first_parent, second_parent): Parents<T>,
///         rng: &mut R,
///     ) -> Result<Genome<T>, Self::Error> {
///         let index = rng.random_range(0..first_parent.len());
///         first_parent[index] = second_parent[index];
///         Ok(first_parent)
///     }
/// }
///
/// // A simple `Operator` that takes a `Vec<bool>` and returns the number
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
/// // If we swap in exactly one value from the `second_parent` we
/// // should have 3 `true` values in the resulting genome.
/// let first_parent = [true, true, true, true];
/// let second_parent = [false, false, false, false];
///
/// let recombinator = SwapOne;
/// let recombine = Recombine::new(recombinator);
/// let count_true = CountTrue;
/// let chain = recombine.then(count_true);
///
/// let count = chain.apply((first_parent, second_parent), &mut rng())?;
/// assert_eq!(count, 3);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// We can also pass a _reference_ to a [`Recombinator`] (i.e.,
/// `&Recombinator`) to a [`Recombine`], allowing us to pass references to
/// recombinators into chains of operators without requiring or giving up
/// ownership of the recombinator.
///
/// ```
/// # use rand::{ rng, Rng};
/// # use std::convert::Infallible;
/// #
/// # use ec_core::operator::{recombinator::{Recombinator, Recombine}, Composable, Operator};
/// #
/// # // A simple `Recombinator` that swaps one element from the second parent
/// # // into the corresponding position in the first parent.
/// # struct SwapOne;
/// #
/// # type Genome<T> = [T; 4];
/// # type Parents<T> = (Genome<T>, Genome<T>);
/// #
/// # impl<T: Copy> Recombinator<Parents<T>> for SwapOne {
/// #     type Output = Genome<T>;
/// #     type Error = Infallible;
/// #
/// #     fn recombine<R: Rng + ?Sized>(
/// #         &self,
/// #         (mut first_parent, second_parent): Parents<T>,
/// #         rng: &mut R,
/// #     ) -> Result<Genome<T>, Self::Error> {
/// #         let index = rng.random_range(0..first_parent.len());
/// #         first_parent[index] = second_parent[index];
/// #         Ok(first_parent)
/// #     }
/// # }
/// #
/// # // A simple `Operator` that takes a `Vec<bool>` and returns the number
/// # // of `true` values in the genome.
/// # struct CountTrue;
/// #
/// # impl Operator<Genome<bool>> for CountTrue {
/// #     type Output = usize;
/// #     type Error = Infallible;
/// #
/// #     fn apply<R: Rng + ?Sized>(&self, genome: Genome<bool>, _: &mut R) -> Result<Self::Output, Self::Error> {
/// #         Ok(genome.iter().filter(|&&x| x).count())
/// #     }
/// # }
/// # impl Composable for CountTrue {}
/// #
/// # // If we swap in exactly one value from the `second_parent` we
/// # // should have 3 `true` values in the resulting genome.
/// let first_parent = [true, true, true, true];
/// let second_parent = [false, false, false, false];
///
/// let recombine = Recombine::new(&SwapOne);
/// let chain = recombine.then(CountTrue);
///
/// let count = chain
///     .apply((first_parent, second_parent), &mut rng())?;
/// assert_eq!(count, 3);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub struct Recombine<R> {
    /// The wrapped [`Recombinator`] that this [`Recombine`] will apply
    recombinator: R,
}

impl<R> Recombine<R> {
    pub const fn new(recombinator: R) -> Self {
        Self { recombinator }
    }
}

impl<Rec, G> Operator<G> for Recombine<Rec>
where
    Rec: Recombinator<G>,
{
    type Output = Rec::Output;
    type Error = Rec::Error;

    /// Apply the wrapped [`Recombinator`] as an [`Operator`] to the given
    /// genomes.
    fn apply<R: Rng + ?Sized>(&self, genomes: G, rng: &mut R) -> Result<Self::Output, Self::Error> {
        self.recombinator.recombine(genomes, rng)
    }
}
impl<R> Composable for Recombine<R> {}

/// Implement [`Recombinator`] for a reference to a [`Recombinator`].
/// This allows us to wrap a reference to a [`Recombinator`] in a [`Recombine`]
/// operator, allowing recombinators to be used in chains of operators.
impl<Rec, GS> Recombinator<GS> for &Rec
where
    Rec: Recombinator<GS>,
{
    type Output = Rec::Output;
    type Error = Rec::Error;

    fn recombine<R: Rng + ?Sized>(
        &self,
        genomes: GS,
        rng: &mut R,
    ) -> Result<Self::Output, Self::Error> {
        (**self).recombine(genomes, rng)
    }
}

#[cfg(test)]
mod tests {
    use std::convert::Infallible;

    use rand::{Rng, rng};

    use super::{Recombinator, Recombine};
    use crate::operator::{Composable, Operator};

    type Genome<T> = [T; 4];
    type Parents<T> = (Genome<T>, Genome<T>);

    // A simple `Recombinator` that swaps one element from the second parent
    // into the corresponding position in the first parent.
    struct SwapOne;

    impl<T: Copy> Recombinator<Parents<T>> for SwapOne {
        type Output = Genome<T>;
        type Error = Infallible;

        fn recombine<R: Rng + ?Sized>(
            &self,
            (mut first_parent, second_parent): Parents<T>,
            rng: &mut R,
        ) -> Result<Genome<T>, Self::Error> {
            let index = rng.random_range(0..first_parent.len());
            first_parent[index] = second_parent[index];
            Ok(first_parent)
        }
    }

    // A simple `Operator` that takes a `Vec<bool>` and returns the number
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

    #[test]
    fn swap_one() {
        let first_parent = [0, 0, 0, 0];
        let second_parent = [1, 1, 1, 1];
        let child = SwapOne
            .recombine((first_parent, second_parent), &mut rng())
            .unwrap();
        // `child` should be all zeros except for the one place where
        // a one was swapped in from the second parent.
        let num_zeros = child.iter().filter(|&&x| x == 0).count();
        let num_ones = child.iter().filter(|&&x| x == 1).count();
        assert_eq!(num_zeros, 3);
        assert_eq!(num_ones, 1);
    }

    #[test]
    fn can_wrap_recombinator() {
        let first_parent = [0, 0, 0, 0];
        let second_parent = [1, 1, 1, 1];

        let recombinator = SwapOne;
        // Wrap the recombinator in a `Recombine` to make it an `Operator`.
        let recombine = Recombine::new(recombinator);

        let child = recombine
            .apply((first_parent, second_parent), &mut rng())
            .unwrap();
        let num_zeros = child.iter().filter(|&&x| x == 0).count();
        let num_ones = child.iter().filter(|&&x| x == 1).count();
        assert_eq!(num_zeros, 3);
        assert_eq!(num_ones, 1);
    }

    #[test]
    fn can_wrap_recombinator_reference() {
        let first_parent = [0, 0, 0, 0];
        let second_parent = [1, 1, 1, 1];

        let recombinator = SwapOne;
        // Wrap a reference to the recombinator in a `Recombine` to make it an
        // `Operator`.
        let recombine = Recombine::new(&recombinator);

        let child = recombine
            .apply((first_parent, second_parent), &mut rng())
            .unwrap();
        let num_zeros = child.iter().filter(|&&x| x == 0).count();
        let num_ones = child.iter().filter(|&&x| x == 1).count();
        assert_eq!(num_zeros, 3);
        assert_eq!(num_ones, 1);
    }

    #[test]
    fn can_chain_recombinator_and_operator() {
        // If we swap in exactly one value from the `second_parent` we
        // should have 3 `true` values in the resulting genome.
        let first_parent = [true, true, true, true];
        let second_parent = [false, false, false, false];

        let recombinator = SwapOne;
        let recombine = Recombine::new(recombinator);
        let count_true = CountTrue;
        let chain = recombine.then(count_true);

        let count = chain
            .apply((first_parent, second_parent), &mut rng())
            .unwrap();
        assert_eq!(count, 3);
    }

    #[test]
    fn can_chain_with_recombinator_reference() {
        // If we swap in exactly one value from the `second_parent` we
        // should have 3 `true` values in the resulting genome.
        let first_parent = [true, true, true, true];
        let second_parent = [false, false, false, false];

        let recombinator = SwapOne;
        // Wrap a reference to the recombinator in a `Recombine` to make it an
        // `Operator`.
        let recombine = Recombine::new(&recombinator);
        let count_true = CountTrue;
        let chain = recombine.then(count_true);

        let count = chain
            .apply((first_parent, second_parent), &mut rng())
            .unwrap();
        assert_eq!(count, 3);
    }
}
