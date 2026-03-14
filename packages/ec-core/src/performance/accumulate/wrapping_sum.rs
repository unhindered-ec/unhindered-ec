use std::{convert::Infallible, iter::Sum as StdSum, num::Wrapping, ops::AddAssign};

use super::{strategy::AccumulateStrategy, total::TotalResult};

#[derive(Debug, Clone, Copy)]
pub struct WrappingSum;

impl<T> AccumulateStrategy<T> for WrappingSum
where
    Wrapping<T>: AddAssign + StdSum,
    T: Default,
{
    type Error = Infallible;
    type State = Wrapping<T>;

    fn initialize() -> Self::State {
        Wrapping(T::default())
    }

    #[expect(clippy::allow_attributes, reason = "Inside macro")]
    #[allow(
        clippy::arithmetic_side_effects,
        reason = "This accumulation strategy explicitly uses the default sum behaviour"
    )]
    fn accululate_into<I>(state: &mut Self::State, iter: I) -> Result<(), Self::Error>
    where
        I: Iterator<Item = T>,
    {
        *state += iter.map(|x| Wrapping(x)).sum();

        Ok(())
    }
}

impl<T> TotalResult<T> for WrappingSum
where
    Wrapping<T>: AddAssign + StdSum,
    T: Default,
{
    type TotalRef<'a>
        = &'a T
    where
        T: 'a;
    type Total = T;

    fn total(state: &Self::State) -> Self::TotalRef<'_> {
        &state.0
    }

    fn into_total(state: Self::State) -> Self::Total {
        state.0
    }
}
