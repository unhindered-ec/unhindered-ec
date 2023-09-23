use crate::{push_vm::stack::StackError, tuples::MonotonicTuple};

use super::TypedStack;

pub trait GetHead: TypedStack {
    /// # Errors
    /// - [`StackError::Underflow`] is returned when there is not at least one item on the Stack to return.
    fn head(&self) -> Result<&Self::Item, StackError> {
        Ok(self.get_n_head::<(_,)>()?.0)
    }

    /// # Errors
    /// - [`StackError::Underflow`] is returned when there are not at least [`MonotonicTuple::Length`] items on the Stack to return.
    fn get_n_head<'a, Tuple: MonotonicTuple<Item = &'a Self::Item>>(
        &'a self,
    ) -> Result<Tuple, StackError>;
}

pub trait GetTail: TypedStack {
    /// # Errors
    /// - [`StackError::Underflow`] is returned when there is not at least one item on the Stack to return.
    fn tail(&self) -> Result<&Self::Item, StackError> {
        Ok(self.get_n_tail::<(_,)>()?.0)
    }

    /// # Errors
    /// - [`StackError::Underflow`] is returned when there are not at least [`MonotonicTuple::Length`] items on the Stack to return.
    fn get_n_tail<'a, Tuple: MonotonicTuple<Item = &'a Self::Item>>(
        &'a self,
    ) -> Result<Tuple, StackError>;
}
