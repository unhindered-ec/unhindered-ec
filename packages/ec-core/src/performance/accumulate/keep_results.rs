use std::{convert::Infallible, slice::SliceIndex};

use super::strategy::AccumulateStrategy;
use crate::performance::accumulate::{
    combine::Combine,
    results::{IndexResults, IndividualResults},
};

// Could make this generic in a ´C: Container` struct to use Vec/VecDeque/...
// where Container has a Container<T> associated type (hkt).
#[derive(Debug, Clone, Copy)]
pub struct StoreResults;

impl<Item> AccumulateStrategy<Item> for StoreResults {
    type Error = Infallible;
    type State = Vec<Item>;

    fn initialize() -> Self::State {
        Vec::new()
    }

    fn accululate_into<I>(state: &mut Self::State, iter: I) -> Result<(), Self::Error>
    where
        I: Iterator<Item = Item>,
    {
        state.extend(iter);

        Ok(())
    }

    fn accumulate<I>(iter: I) -> Result<Self::State, Self::Error>
    where
        I: Iterator<Item = Item>,
    {
        Ok(iter.collect())
    }
}

impl<Item> IndividualResults<Item> for StoreResults {
    type Item = Item;

    fn len(state: &Self::State) -> usize {
        state.len()
    }

    fn results<'a>(state: &'a Self::State) -> impl Iterator<Item = &'a Self::Item>
    where
        Self::Item: 'a,
    {
        state.iter()
    }

    fn into_results(state: Self::State) -> impl Iterator<Item = Self::Item> {
        state.into_iter()
    }
}

impl<Item, Idx> IndexResults<Item, Idx> for StoreResults
where
    Idx: SliceIndex<[Item]>,
{
    type Output = Idx::Output;

    fn get<'a>(state: &'a Self::State, index: Idx) -> Option<&'a Self::Output>
    where
        Self::Item: 'a,
    {
        state.get(index)
    }
}

pub type KeepResults<T> = Combine<StoreResults, T>;
