use crate::push_vm::stack::StackError;

use super::TypedStack;

pub trait ExtendTail: TypedStack {
    /// # Errors
    /// - [`StackError::Overflow`] is returned when the stack has not enough capacity for [`Iter::len()`] new values remaining.
    fn extend_tail<Iter>(&mut self, iter: Iter) -> Result<(), StackError>
    where
        Iter: IntoIterator<Item = Self::Item>,
        Iter::IntoIter: DoubleEndedIterator + ExactSizeIterator;
}

pub trait ExtendHead: TypedStack {
    /// # Errors
    /// - [`StackError::Overflow`] is returned when the stack has not enough capacity for [`Iter::len()`] new values remaining.
    fn extend_head<Iter>(&mut self, iter: Iter) -> Result<(), StackError>
    where
        Iter: IntoIterator<Item = Self::Item>,
        Iter::IntoIter: DoubleEndedIterator + ExactSizeIterator;
}
