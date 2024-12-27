use std::error::Error as StdError;

use rand::{Rng, RngCore};

use super::{Composable, Operator};

pub trait DynOperator<Input, Error = Box<dyn StdError + Send + Sync>>: Composable {
    type Output;

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
