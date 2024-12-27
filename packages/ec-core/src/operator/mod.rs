//! Documentation on the `operator` module...
//!
//! # Wrappers
//!
//! Explain the use of wrappers, and why blanket
//! implementations weren't feasible.

use rand::Rng;

pub mod composable;
pub mod constant;
pub mod genome_extractor;
pub mod genome_scorer;
pub mod identity;
pub mod mutator;
pub mod recombinator;
pub mod selector;

pub use composable::Composable;

pub trait Operator<Input>: Composable {
    type Output;
    type Error;

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

#[cfg(feature = "erased")]
pub trait DynOperator<Input, Error = Box<dyn std::error::Error + Send + Sync>>: Composable {
    type Output;

    /// # Errors
    /// This will return an error if there's some problem applying the operator.
    /// Given how general this concept is, there's no good way of saying here
    /// what that might be.
    fn dyn_apply(&self, input: Input, rng: &mut dyn rand::RngCore) -> Result<Self::Output, Error>;
}

#[cfg(feature = "erased")]
impl<T, I, E> DynOperator<I, E> for T
where
    T: Operator<I, Error: Into<E>>,
{
    type Output = T::Output;

    fn dyn_apply(&self, input: I, rng: &mut dyn rand::RngCore) -> Result<Self::Output, E> {
        self.apply(input, rng).map_err(Into::into)
    }
}

#[cfg(feature = "erased")]
static_assertions::assert_obj_safe!(DynOperator<(), (), Output = ()>);

#[cfg(feature = "erased")]
#[ec_macros::dyn_ref_impls]
impl<I, E, O> Composable for &dyn DynOperator<I, E, Output = O> {}

#[cfg(feature = "erased")]
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
