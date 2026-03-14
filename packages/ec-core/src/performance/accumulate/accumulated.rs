use std::{convert::Infallible, fmt::Display, ops::Index};

use crate::performance::accumulate::{
    default::DefaultAccumulateStrategy,
    results::{IndexResults, IndividualResults},
    strategy::AccumulateStrategy,
    total::TotalResult,
};

#[derive(Debug)]
pub struct Accumulated<
    Item,
    Strategy: AccumulateStrategy<Item> = <Item as DefaultAccumulateStrategy>::Strategy,
> {
    state: Strategy::State,
}

impl<Item, Strategy> Clone for Accumulated<Item, Strategy>
where
    Strategy: AccumulateStrategy<Item, State: Clone>,
{
    fn clone(&self) -> Self {
        Self {
            state: self.state.clone(),
        }
    }
}

impl<Item, Strategy> Copy for Accumulated<Item, Strategy> where
    Strategy: AccumulateStrategy<Item, State: Copy>
{
}

impl<Item, Strategy> Accumulated<Item, Strategy>
where
    Strategy: AccumulateStrategy<Item>,
{
    #[must_use]
    pub fn new() -> Self {
        Self {
            state: Strategy::initialize(),
        }
    }

    pub(crate) const fn from_state(state: Strategy::State) -> Self {
        Self { state }
    }

    pub fn total(&self) -> Strategy::TotalRef<'_>
    where
        Strategy: TotalResult<Item>,
    {
        Strategy::total(&self.state)
    }

    pub fn into_total(self) -> Strategy::Total
    where
        Strategy: TotalResult<Item>,
    {
        Strategy::into_total(self.state)
    }

    pub fn len(&self) -> usize
    where
        Strategy: IndividualResults<Item>,
    {
        Strategy::len(&self.state)
    }

    pub fn is_empty(&self) -> bool
    where
        Strategy: IndividualResults<Item>,
    {
        Strategy::is_empty(&self.state)
    }

    pub fn results(&self) -> impl Iterator<Item = &Strategy::Item>
    where
        Strategy: IndividualResults<Item>,
    {
        Strategy::results(&self.state)
    }

    pub fn into_results(self) -> impl Iterator<Item = Strategy::Item>
    where
        Strategy: IndividualResults<Item>,
    {
        Strategy::into_results(self.state)
    }

    pub fn get<Index>(&self, index: Index) -> Option<&Strategy::Output>
    where
        Strategy: IndexResults<Item, Index>,
    {
        Strategy::get(&self.state, index)
    }
}

impl<Item, Strategy> Ord for Accumulated<Item, Strategy>
where
    Strategy: for<'a> TotalResult<Item, TotalRef<'a>: Ord>,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.total().cmp(&other.total())
    }
}

impl<Item, Strategy> PartialOrd for Accumulated<Item, Strategy>
where
    Strategy: for<'a> TotalResult<Item, TotalRef<'a>: PartialOrd>,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.total().partial_cmp(&other.total())
    }
}

impl<Item, Strategy> PartialEq for Accumulated<Item, Strategy>
where
    Strategy: for<'a> TotalResult<Item, TotalRef<'a>: PartialEq>,
{
    fn eq(&self, other: &Self) -> bool {
        self.total().eq(&other.total())
    }
}

impl<Item, Strategy> Eq for Accumulated<Item, Strategy> where
    Strategy: for<'a> TotalResult<Item, TotalRef<'a>: Eq>
{
}

// currently not possible, should be viable in q3-q4 this year once the
// next trait solver and ATPIT lands more widely
//
// Alternatively, we could introduce another associated type on the
// IndividualResults trait which would make the implementation more inconvenient
// but make it possible to write these two impls now.
//
// impl<Item, Strategy> IntoIterator for Accumulated<Item, Strategy>
// where
//     Strategy: IndividualResults<Item>,
// {
//     type Item = <Strategy as IndividualResults<Item>>::Item;

//     // unstable for now, see https://github.com/rust-lang/rust/issues/63063
//     type IntoIter = impl Iterator<Item = Self::Item>;

//     fn into_iter(self) -> Self::IntoIter {
//         self.into_results()
//     }
// }
//
// impl<'a, Item, Strategy> IntoIterator for &'a Accumulated<Item, Strategy>
// where
//     Strategy: IndividualResults<Item>,
// {
//     type Item = &'a <Strategy as IndividualResults<Item>>::Item;

//     // unstable for now, see https://github.com/rust-lang/rust/issues/63063
//     type IntoIter = impl Iterator<Item = &'a Self::Item>;

//     fn into_iter(self) -> Self::IntoIter {
//         self.results()
//     }
// }
//

impl<Item, Strategy, Idx> Index<Idx> for Accumulated<Item, Strategy>
where
    Strategy: IndexResults<Item, Idx>,
{
    type Output = Strategy::Output;

    fn index(&self, index: Idx) -> &Self::Output {
        #[expect(
            clippy::panic,
            reason = "This is intended behaviour via the interface of this method"
        )]
        self.get(index)
            .unwrap_or_else(|| panic!("Index out of bounds"))
    }
}

impl<Item, Strategy> Default for Accumulated<Item, Strategy>
where
    Strategy: AccumulateStrategy<Item>,
{
    fn default() -> Self {
        Self::new()
    }
}
impl<V, U, Strategy> FromIterator<V> for Accumulated<U, Strategy>
where
    Strategy: AccumulateStrategy<U, Error: Into<Infallible>>,
    V: Into<U>,
{
    fn from_iter<T: IntoIterator<Item = V>>(iter: T) -> Self {
        let Ok(res) = Strategy::accumulate(iter.into_iter().map(Into::into)).map_err(Into::into);

        Self::from_state(res)
    }
}

impl<Item, Strategy> Display for Accumulated<Item, Strategy>
where
    Strategy: TotalResult<Item>,
    for<'a> Strategy::TotalRef<'a>: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.total().fmt(f)
    }
}
