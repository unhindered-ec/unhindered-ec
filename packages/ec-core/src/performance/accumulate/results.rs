use super::strategy::AccumulateStrategy;

#[diagnostic::on_unimplemented(
    message = "tried to access individual results but `{Self}` does not provide individual results",
    label = "`IndividualResults<{Item}>` required here",
    note = "try wrapping your accumulator in an adapter that keeps individual results, such as \
            `KeepResults<{Self}>`"
)]
pub trait IndividualResults<Item>: AccumulateStrategy<Item> {
    type Item;

    fn len(state: &Self::State) -> usize;

    fn results<'a>(state: &'a Self::State) -> impl Iterator<Item = &'a Self::Item>
    where
        Self::Item: 'a;

    fn into_results(state: Self::State) -> impl Iterator<Item = Self::Item>;

    fn is_empty(state: &Self::State) -> bool {
        Self::len(state) == 0
    }
}

#[diagnostic::on_unimplemented(
    message = "tried to index results but `{Self}` does not provide indexing for results",
    label = "`IndexResults<{Item}, {Index}>` required here",
    note = "try wrapping your accumulator in an adapter that keeps individual results, such as \
            `KeepResults<{Self}>`"
)]
pub trait IndexResults<Item, Index = usize>: IndividualResults<Item> {
    type Output: ?Sized; // = Self::Item;

    fn get<'a>(state: &'a Self::State, index: Index) -> Option<&'a Self::Output>
    where
        Self::Item: 'a;
}
