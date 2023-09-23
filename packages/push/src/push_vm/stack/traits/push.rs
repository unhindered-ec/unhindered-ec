use crate::{push_vm::stack::StackError, tuples::MonotonicTuple};

use super::{
    extend::{ExtendHead, ExtendTail},
    TypedStack,
};

pub trait PushHead: TypedStack {
    /// # Errors
    /// - [`StackError::Overflow`] is returned when the stack has not enough capacity for one new value remaining.
    fn push_head(&mut self, value: Self::Item) -> Result<(), StackError> {
        self.push_n_head((value,))
    }

    /// # Errors
    /// - [`StackError::Overflow`] is returned when the stack has not enough capacity for n new values remaining.
    fn push_n_head<Tuple: MonotonicTuple<Item = Self::Item>>(
        &mut self,
        value: Tuple,
    ) -> Result<(), StackError>;
}

pub trait PushTail: TypedStack {
    /// # Errors
    /// - [`StackError::Overflow`] is returned when the stack has not enough capacity for one new value remaining.
    fn push_tail(&mut self, value: Self::Item) -> Result<(), StackError> {
        self.push_n_tail((value,))
    }

    /// # Errors
    /// - [`StackError::Overflow`] is returned when the stack has not enough capacity for n new values remaining.
    fn push_n_tail<Tuple: MonotonicTuple<Item = Self::Item>>(
        &mut self,
        value: Tuple,
    ) -> Result<(), StackError>;
}

// Blanket impl - Is this a good idea even when specialization is not stable yet?
impl<T> PushHead for T
where
    T: ExtendHead,
{
    fn push_n_head<Tuple: MonotonicTuple<Item = Self::Item>>(
        &mut self,
        value: Tuple,
    ) -> Result<(), StackError> {
        self.extend_head(value.into_iterator())
    }
}

impl<T> PushTail for T
where
    T: ExtendTail,
{
    fn push_n_tail<Tuple: MonotonicTuple<Item = Self::Item>>(
        &mut self,
        value: Tuple,
    ) -> Result<(), StackError> {
        self.extend_tail(value.into_iterator())
    }
}
