use std::{convert::Infallible, iter::Sum as StdSum, ops::AddAssign};

use super::{strategy::AccumulateStrategy, total::TotalResult};

#[derive(Debug, Clone, Copy)]
pub struct Sum;

impl<T> AccumulateStrategy<T> for Sum
where
    T: AddAssign + StdSum + Default,
{
    type Error = Infallible;
    type State = T;

    fn initialize() -> Self::State {
        T::default()
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
        *state += iter.sum::<T>();

        Ok(())
    }
}

impl<T> TotalResult<T> for Sum
where
    T: AddAssign + StdSum + Default,
{
    type TotalRef<'a>
        = &'a T
    where
        T: 'a;
    type Total = T;

    fn total(state: &Self::State) -> Self::TotalRef<'_> {
        state
    }

    fn into_total(state: Self::State) -> Self::Total {
        state
    }
}
