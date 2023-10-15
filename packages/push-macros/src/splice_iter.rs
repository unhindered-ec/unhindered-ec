pub struct SpliceIter<Iter, Cond, GetItem>
where
    Iter: Iterator,
    Cond: FnMut(Option<&Iter::Item>) -> bool,
    GetItem: FnOnce() -> Iter::Item,
{
    base: Iter,
    get_item: Option<GetItem>,
    cond_closure: Cond,
}

impl<Iter, Cond, GetItem> Iterator for SpliceIter<Iter, Cond, GetItem>
where
    Iter: Iterator,
    Cond: FnMut(Option<&Iter::Item>) -> bool,
    GetItem: FnOnce() -> Iter::Item,
{
    type Item = Iter::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let mut peekable_iter = (&mut self.base).peekable();
        let item = peekable_iter.peek();
        let cond_closure = &mut self.cond_closure;
        if self.get_item.is_some() && cond_closure(item) {
            // Safety: We checked that get_item is some in the line above
            let get_item = unsafe { std::mem::take(&mut self.get_item).unwrap_unchecked() };
            Some(get_item())
        } else {
            self.base.next()
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (min, max) = self.base.size_hint();
        (min, max.map(|m| m + 1))
    }
}

pub trait SpliceOne<Iter>
where
    Iter: Iterator,
{
    fn splice_one_with<Cond, GetItem>(
        self,
        cond: Cond,
        get_item: GetItem,
    ) -> SpliceIter<Iter, Cond, GetItem>
    where
        Cond: FnMut(Option<&Iter::Item>) -> bool,
        GetItem: FnOnce() -> Iter::Item;
}

impl<Iter> SpliceOne<Iter> for Iter
where
    Iter: Iterator,
{
    fn splice_one_with<'a, Cond, GetItem>(
        self,
        cond: Cond,
        get_item: GetItem,
    ) -> SpliceIter<Iter, Cond, GetItem>
    where
        Cond: FnMut(Option<&Iter::Item>) -> bool,
        GetItem: FnOnce() -> Iter::Item,
    {
        SpliceIter {
            base: self,
            get_item: Some(get_item),
            cond_closure: cond,
        }
    }
}
