use std::marker::PhantomData;

use crate::performance::accumulate::{
    results::{IndexResults, IndividualResults},
    strategy::AccumulateStrategy,
    total::TotalResult,
};

#[derive(Debug, Clone, Copy)]
pub struct Widen<Item, Strategy> {
    _p: PhantomData<(Item, Strategy)>,
}

impl<Strategy, NewItem, Item> AccumulateStrategy<Item> for Widen<NewItem, Strategy>
where
    Strategy: AccumulateStrategy<NewItem>,
    Item: Into<NewItem>,
{
    type Error = Strategy::Error;
    type State = Strategy::State;

    fn initialize() -> Self::State {
        Strategy::initialize()
    }

    fn accululate_into<I>(state: &mut Self::State, iter: I) -> Result<(), Self::Error>
    where
        I: Iterator<Item = Item>,
    {
        Strategy::accululate_into(state, iter.map(Into::into))
    }

    fn accumulate<I>(iter: I) -> Result<Self::State, Self::Error>
    where
        I: Iterator<Item = Item>,
    {
        Strategy::accumulate(iter.map(Into::into))
    }
}

impl<Strategy, NewItem, Item> IndividualResults<Item> for Widen<NewItem, Strategy>
where
    Strategy: IndividualResults<NewItem>,
    Item: Into<NewItem>,
{
    type Item = Strategy::Item;

    fn len(state: &Self::State) -> usize {
        Strategy::len(state)
    }

    fn results<'a>(state: &'a Self::State) -> impl Iterator<Item = &'a Self::Item>
    where
        Self::Item: 'a,
    {
        Strategy::results(state)
    }

    fn into_results(state: Self::State) -> impl Iterator<Item = Self::Item> {
        Strategy::into_results(state)
    }

    fn is_empty(state: &Self::State) -> bool {
        Strategy::is_empty(state)
    }
}

impl<Strategy, NewItem, Item, Idx> IndexResults<Item, Idx> for Widen<NewItem, Strategy>
where
    Strategy: IndexResults<NewItem, Idx>,
    Item: Into<NewItem>,
{
    type Output = Strategy::Output;

    fn get<'a>(state: &'a Self::State, index: Idx) -> Option<&'a Self::Output>
    where
        Self::Item: 'a,
    {
        Strategy::get(state, index)
    }
}

impl<Strategy, NewItem, Item> TotalResult<Item> for Widen<NewItem, Strategy>
where
    Strategy: TotalResult<NewItem>,
    Item: Into<NewItem>,
{
    type TotalRef<'a>
        = Strategy::TotalRef<'a>
    where
        Self::Total: 'a;
    type Total = Strategy::Total;

    fn total(state: &Self::State) -> Self::TotalRef<'_> {
        Strategy::total(state)
    }

    fn into_total(state: Self::State) -> Self::Total {
        Strategy::into_total(state)
    }
}
