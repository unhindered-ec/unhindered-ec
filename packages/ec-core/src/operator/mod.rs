// TODO: Documentation on the `operator` module...
//
// # Wrappers
//
// Explain the use of wrappers, and why blanket
// implementations weren't feasible.

use rand::Rng;

#[cfg(feature = "erased")]
mod erased;
#[cfg(feature = "erased")]
pub use erased::*;

pub mod composable;
pub mod constant;
pub mod genome_extractor;
pub mod genome_scorer;
pub mod identity;
pub mod mutator;
pub mod recombinator;
pub mod selector;

#[doc(inline)]
pub use composable::Composable;

/// An operator in a data pipeline
///
/// For more information on how data pipelines work in general in this crate,
/// see the [module-level documentation](super::operator)
///
/// For concrete operators that you might wish to use can take a look at
/// [`Map`](composable::Map) and [`Then`](composable::Then) among
/// others.
///
/// # Example
/// This example shows how to implement a [`Operator`], not how you use
/// predefined ones. For that, take a look at the [module-level
/// documentation](super::operator)
///
/// ```
/// # use ec_core::operator::{Operator, Composable};
/// # use rand::Rng;
/// #
/// # #[allow(dead_code)]
/// struct Apply<F>(F);
///
/// impl<F> Composable for Apply<F> {}
///
/// impl<In, F, Out> Operator<In> for Apply<F>
/// where
///     F: Fn(In) -> Out,
/// {
///     type Output = Out;
///     type Error = std::convert::Infallible;
///
///     fn apply<R: Rng + ?Sized>(
///         &self,
///         input: In,
///         _rng: &mut R,
///     ) -> Result<Self::Output, Self::Error> {
///         Ok((self.0)(input))
///     }
/// }
/// ```
///
/// # [dyn-compatibility](https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility)
/// This trait is **not** dyn-compatible. As such please
/// try to avoid the need for trait objects whenever you can.
///
/// If you can't get around the usage of trait objects, you can
/// use the [`DynOperator`] trait, which is available if you compile
/// this crate with the `erased` feature.
///
/// Please see its documentation for further details on its usage.
pub trait Operator<Input>: Composable {
    /// The output type of this operator
    type Output;
    /// The error type of this operator. May be
    /// [`Infallible`](std::convert::Infallible) if the operator does not error.
    type Error;

    /// Apply this operator to an input
    ///
    /// This also takes an rng that is passed along to customize random number
    /// generation behavior and avoid re-creating RNGs in each operator.
    ///
    /// # Example
    /// ```
    /// # use ec_core::operator::{constant::Constant, Operator};
    /// # use rand::rng;
    /// #
    /// let my_constant_operator: Constant<_> = Constant::new(5);
    ///
    /// let Ok(sample_value) = my_constant_operator.apply((), &mut rng());
    /// assert_eq!(sample_value, 5);
    /// ```
    ///
    /// # Errors
    /// This will return an error if there's some problem applying the operator.
    /// Given how general this concept is, there's no good way of saying here
    /// what that might be.
    fn apply<R: Rng + ?Sized>(
        &self,
        input: Input,
        rng: &mut R,
    ) -> Result<Self::Output, Self::Error>;
}
