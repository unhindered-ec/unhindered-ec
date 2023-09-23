use crate::{push_vm::stack::StackError, tuples::MonotonicTuple};

use super::get::{GetHead, GetTail};

pub trait PopHead: GetHead {
    /// # Errors
    /// - [`StackError::Underflow`] is returned when there is not at least one item on the Stack to return.
    fn pop_head(&mut self) -> Result<Self::Item, StackError> {
        Ok(self.pop_n_head::<(_,)>()?.0)
    }

    /// # Errors
    /// - [`StackError::Underflow`] is returned when there are not at least [`MonotonicTuple::Length`] items on the Stack to return.
    fn pop_n_head<Tuple: MonotonicTuple<Item = Self::Item>>(&mut self)
        -> Result<Tuple, StackError>;
}

pub trait PopTail: GetTail {
    /// # Errors
    /// - [`StackError::Underflow`] is returned when there is not at least one item on the Stack to return.
    fn pop_tail(&mut self) -> Result<Self::Item, StackError> {
        Ok(self.pop_n_tail::<(_,)>()?.0)
    }

    /// # Errors
    /// - [`StackError::Underflow`] is returned when there are not at least [`MonotonicTuple::Length`] items on the Stack to return.
    fn pop_n_tail<Tuple: MonotonicTuple<Item = Self::Item>>(&mut self)
        -> Result<Tuple, StackError>;
}
