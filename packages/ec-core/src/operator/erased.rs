use std::error::Error as StdError;

use rand::{Rng, RngCore};

use super::{Composable, Operator};

/// Erased
/// ([dyn-compatible](https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility))
/// version of the [`Operator`] trait
///
/// # How does this work?
///
/// The [`erased` pattern](https://quinedot.github.io/rust-learning/dyn-trait-erased.html)
/// (archived [here](https://web.archive.org/web/20250719135019/https://quinedot.github.io/rust-learning/dyn-trait-erased.html))
/// in rust aids in type-erasure for traits
/// that aren't themselves dyn-compatible by declaring a dyn-compatible
/// extension trait wrapper for the original trait and blanket-implementing
/// that for all types which implement the original trait.
///
/// In this case, the trait [`DynOperator`] can be seen as a dyn-compatible
/// version of the [`Operator`] trait, and any [`T: Operator`](Operator) can
/// also be interpreted as [`T: DynOperator`](DynOperator)
///
/// This allows you to use [`dyn DynOperator<I>`](DynOperator) trait objects to
/// perform type erasure on types implementing the [`Operator`] trait.
///
/// # When to use it?
///
/// The original trait most of the time has a reason for not beeing
/// dyn-compatible. As such, usually the erased variants of traits come with
/// performance tradeoffs, and [`DynOperator`] is of course no exception either,
/// since it introduces additonal indirection and vtable-lookups.
///
/// Please prefer the [`Operator`] trait whenever possible.
///
/// # How to use it?
///
/// tl;dr: use [`dyn DynOperator<>`](DynOperator) instead of [`dyn
/// Operator<I>`](Operator) and still use all the usual [`Operator`] methods
/// elsewhere.
///
/// This trait tries to provide some useful ergonomics to ease the interaction
/// with existing [`Operator`] code.
/// For example, many common pointer types in Rust pointing to a [`dyn
/// DynOperator<I>`](DynOperator) also implement the [`Operator`] trait
/// themselves, so you most likely do not need to interact with this trait
/// directly.
///
/// For example: `Box<dyn DynOperator<()>>` implements
/// [`Operator<()>`](Operator) and as such you can directly call
/// [`.apply()`](Operator::apply) on it and do not need to use
/// [`DynOperator::dyn_apply`].
///
/// This also means, any `Box<dyn DynOperator<()>>` can be passed to generic
/// functions expecting an [`Operator`], like `fn foo(t: impl Operator<()>);`.
pub trait DynOperator<Input, Error = Box<dyn StdError + Send + Sync>>: Composable {
    type Output;

    /// Apply this type erased operator to an input
    ///
    /// This also takes an rng that is passed along to customize random number
    /// generation behavior and avoid re-creating RNGs in each operator.
    ///
    /// It is recommended to not use this method directly and instead call the
    /// normal [`Operator::apply`] implemented on various container types
    /// containing `dyn DynOperator<_>`'s.
    ///
    /// If you want to call this method directly, make sure to only call it on a
    /// `&dyn DynOperator<_>` (i.e. dereference a box first) else you will
    /// introduce another layer of indirection because of the implementations on
    /// the various container types, and additionally type inference would
    /// require additional type annotations (this is usually a sign you are
    /// doing something wrong).
    ///
    /// # Example
    /// ```
    /// # use ec_core::operator::{constant::Constant, DynOperator};
    /// # use rand::rng;
    /// #
    /// let my_constant_operator: Constant<i32> = 5.into();
    /// let my_erased_operator: Box<dyn DynOperator<(), Output = i32>> = Box::new(my_constant_operator);
    ///
    /// let sample_value = (*my_erased_operator).dyn_apply((), &mut rng())?;
    /// assert_eq!(sample_value, 5);
    /// # Ok::<(), Box<dyn std::error::Error + Send + Sync>>(())
    /// ```
    ///
    /// # Errors
    /// This will return an error if there's some problem applying the operator.
    /// Given how general this concept is, there's no good way of saying here
    /// what that might be.
    fn dyn_apply(&self, input: Input, rng: &mut dyn RngCore) -> Result<Self::Output, Error>;
}

impl<T, I, E> DynOperator<I, E> for T
where
    T: Operator<I, Error: Into<E>>,
{
    type Output = T::Output;

    fn dyn_apply(&self, input: I, rng: &mut dyn RngCore) -> Result<Self::Output, E> {
        self.apply(input, rng).map_err(Into::into)
    }
}

static_assertions::assert_obj_safe!(DynOperator<(), (), Output = ()>);

#[ec_macros::dyn_ref_impls]
impl<I, E, O> Composable for &dyn DynOperator<I, E, Output = O> {}

#[ec_macros::dyn_ref_impls]
impl<I, E, O> Operator<I> for &dyn DynOperator<I, E, Output = O> {
    type Error = E;
    type Output = O;

    fn apply<R: Rng + ?Sized>(
        &self,
        input: I,
        mut rng: &mut R,
    ) -> Result<Self::Output, Self::Error> {
        (**self).dyn_apply(input, &mut rng)
    }
}
