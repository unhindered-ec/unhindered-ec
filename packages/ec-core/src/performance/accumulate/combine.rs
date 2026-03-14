use std::{convert::Infallible, marker::PhantomData};

use crate::performance::accumulate::{
    results::{IndexResults, IndividualResults},
    strategy::AccumulateStrategy,
    total::TotalResult,
};

#[derive(Debug, Clone, Copy)]
pub struct Combine<IndividualStrategy, TotalStrategy> {
    _p: PhantomData<(IndividualStrategy, TotalStrategy)>,
}

#[derive(Debug, Clone, Copy)]
pub enum CombineError<IndividualStrategyError, TotalStrategyError> {
    First(IndividualStrategyError),
    Second(TotalStrategyError),
}

impl<T, U> From<CombineError<T, U>> for Infallible
where
    T: Into<Self>,
    U: Into<Self>,
{
    fn from(val: CombineError<T, U>) -> Self {
        match val {
            CombineError::First(v) => v.into(),
            CombineError::Second(u) => u.into(),
        }
    }
}

impl<Item, IndividualStrategy, TotalStrategy> AccumulateStrategy<Item>
    for Combine<IndividualStrategy, TotalStrategy>
where
    IndividualStrategy: IndividualResults<Item, Item: Clone>,
    TotalStrategy: TotalResult<IndividualStrategy::Item>,
{
    type Error = CombineError<IndividualStrategy::Error, TotalStrategy::Error>;
    type State = (IndividualStrategy::State, TotalStrategy::State);

    fn initialize() -> Self::State {
        (
            IndividualStrategy::initialize(),
            TotalStrategy::initialize(),
        )
    }

    fn accululate_into<I>(state: &mut Self::State, iter: I) -> Result<(), Self::Error>
    where
        I: Iterator<Item = Item>,
    {
        let len = IndividualStrategy::len(&state.0);
        IndividualStrategy::accululate_into(&mut state.0, iter).map_err(CombineError::First)?;
        TotalStrategy::accululate_into(
            &mut state.1,
            IndividualStrategy::results(&state.0)
                .skip(len)
                .map(Clone::clone),
        )
        .map_err(CombineError::Second)?;

        Ok(())
    }

    fn accumulate<I>(iter: I) -> Result<Self::State, Self::Error>
    where
        I: Iterator<Item = Item>,
    {
        let first_state = IndividualStrategy::accumulate(iter).map_err(CombineError::First)?;
        let second_state =
            TotalStrategy::accumulate(IndividualStrategy::results(&first_state).map(Clone::clone))
                .map_err(CombineError::Second)?;

        Ok((first_state, second_state))
    }
}

impl<Item, IndividualStrategy, TotalStrategy> IndividualResults<Item>
    for Combine<IndividualStrategy, TotalStrategy>
where
    IndividualStrategy: IndividualResults<Item, Item: Clone>,
    TotalStrategy: TotalResult<IndividualStrategy::Item>,
{
    type Item = IndividualStrategy::Item;

    fn len(state: &Self::State) -> usize {
        IndividualStrategy::len(&state.0)
    }

    fn results<'a>(state: &'a Self::State) -> impl Iterator<Item = &'a Self::Item>
    where
        Self::Item: 'a,
    {
        IndividualStrategy::results(&state.0)
    }

    fn into_results(state: Self::State) -> impl Iterator<Item = Self::Item> {
        IndividualStrategy::into_results(state.0)
    }

    fn is_empty(state: &Self::State) -> bool {
        IndividualStrategy::is_empty(&state.0)
    }
}

impl<Item, IndividualStrategy, TotalStrategy, Idx> IndexResults<Item, Idx>
    for Combine<IndividualStrategy, TotalStrategy>
where
    IndividualStrategy: IndexResults<Item, Idx, Item: Clone>,
    TotalStrategy: TotalResult<IndividualStrategy::Item>,
{
    type Output = IndividualStrategy::Output;

    fn get<'a>(state: &'a Self::State, index: Idx) -> Option<&'a Self::Output>
    where
        Self::Item: 'a,
    {
        IndividualStrategy::get(&state.0, index)
    }
}

impl<Item, IndividualStrategy, TotalStrategy> TotalResult<Item>
    for Combine<IndividualStrategy, TotalStrategy>
where
    IndividualStrategy: IndividualResults<Item, Item: Clone>,
    TotalStrategy: TotalResult<IndividualStrategy::Item>,
{
    type TotalRef<'a>
        = TotalStrategy::TotalRef<'a>
    where
        Self::Total: 'a;

    type Total = TotalStrategy::Total;

    fn total(state: &Self::State) -> Self::TotalRef<'_> {
        TotalStrategy::total(&state.1)
    }

    fn into_total(state: Self::State) -> Self::Total {
        TotalStrategy::into_total(state.1)
    }
}
