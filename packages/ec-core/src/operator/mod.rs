//! Documentation on the `operator` module...
//!
//! # Wrappers
//!
//! Explain the use of wrappers, and why blanket
//! implementations weren't feasible.

#[cfg(feature = "erased")]
use std::{
    cell::{Ref, RefMut},
    rc::Rc,
    sync::Arc,
};

use rand::Rng;
#[cfg(feature = "erased")]
use rand::RngCore;

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
    fn dyn_apply(&self, input: Input, rng: &mut dyn RngCore) -> Result<Self::Output, Error>;
}

#[cfg(feature = "erased")]
impl<T, I, E> DynOperator<I, E> for T
where
    T: Operator<I, Error: Into<E>>,
{
    type Output = T::Output;

    fn dyn_apply(&self, input: I, rng: &mut dyn RngCore) -> Result<Self::Output, E> {
        self.apply(input, rng).map_err(Into::into)
    }
}

#[cfg(feature = "erased")]
static_assertions::assert_obj_safe!(DynOperator<(), (), Output = ()>);

#[cfg(feature = "erased")]
macro_rules! dyn_operator_impl {
    ($t: ty) => {
        #[cfg(feature = "erased")]
        impl<I, E, O> Operator<I> for $t
        {
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

        #[cfg(feature = "erased")]
        impl<I,E,O> Composable for $t {}
    };
    ($($t: ty),+ $(,)?) => {
        $(dyn_operator_impl!($t);)+
    }
}

#[cfg(feature = "erased")]
// TODO: Create a macro to do this in a nicer way without needing to manually
// repeat all the pointer types everywhere we provide a type erased trait
dyn_operator_impl!(
    &dyn DynOperator<I, E, Output = O>,
    &(dyn DynOperator<I, E, Output = O> + Send),
    &(dyn DynOperator<I, E, Output = O> + Sync),
    &(dyn DynOperator<I, E, Output = O> + Send + Sync),
    &mut dyn DynOperator<I, E, Output = O>,
    &mut (dyn DynOperator<I, E, Output = O> + Send),
    &mut (dyn DynOperator<I, E, Output = O> + Sync),
    &mut (dyn DynOperator<I, E, Output = O> + Send + Sync),
    Box<dyn DynOperator<I, E, Output = O>>,
    Box<dyn DynOperator<I, E, Output = O> + Send>,
    Box<dyn DynOperator<I, E, Output = O> + Sync>,
    Box<dyn DynOperator<I, E, Output = O> + Send + Sync>,
    Arc<dyn DynOperator<I, E, Output = O>>,
    Arc<dyn DynOperator<I, E, Output = O> + Send>,
    Arc<dyn DynOperator<I, E, Output = O> + Sync>,
    Arc<dyn DynOperator<I, E, Output = O> + Send + Sync>,
    Rc<dyn DynOperator<I, E, Output = O>>,
    Rc<dyn DynOperator<I, E, Output = O> + Send>,
    Rc<dyn DynOperator<I, E, Output = O> + Sync>,
    Rc<dyn DynOperator<I, E, Output = O> + Send + Sync>,
    Ref<'_, dyn DynOperator<I, E, Output = O>>,
    Ref<'_, dyn DynOperator<I, E, Output = O> + Send>,
    Ref<'_, dyn DynOperator<I, E, Output = O> + Sync>,
    Ref<'_, dyn DynOperator<I, E, Output = O> + Send + Sync>,
    RefMut<'_, dyn DynOperator<I, E, Output = O>>,
    RefMut<'_, dyn DynOperator<I, E, Output = O> + Send>,
    RefMut<'_, dyn DynOperator<I, E, Output = O> + Sync>,
    RefMut<'_, dyn DynOperator<I, E, Output = O> + Send + Sync>,
);
