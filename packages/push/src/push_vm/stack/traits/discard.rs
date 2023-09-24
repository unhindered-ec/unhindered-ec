use crate::{
    error::into_state::{State, StateMut},
    push_vm::stack::StackError,
    type_eq::TypeEq,
};

use super::has_stack::{HasStack, HasStackMut};

pub trait DiscardHead {
    /// # Errors
    /// - [`StackError::Underflow`] is returned when there is not at least one item on the Stack to discard
    fn discard_head(&mut self) -> Result<(), StackError> {
        self.discard_n_head(1)
    }

    /// # Errors
    /// - [`StackError::Underflow`] is returned when there are not at least [`MonotonicTuple::Length`] items on the Stack to discard.
    fn discard_n_head(&mut self, n: usize) -> Result<(), StackError>;
}

pub trait DiscardTail {
    /// # Errors
    /// - [`StackError::Underflow`] is returned when there is not at least one item on the Stack to discard
    fn discard_tail(&mut self) -> Result<(), StackError> {
        self.discard_n_tail(1)
    }

    /// # Errors
    /// - [`StackError::Underflow`] is returned when there are not at least [`MonotonicTuple::Length`] items on the Stack to discard.
    fn discard_n_tail(&mut self, n: usize) -> Result<(), StackError>;
}

pub trait DiscardHeadIn<Stack>: Sized {
    /// # Errors
    /// - [`StackError::Underflow`] is returned when there is not at least one item on the Stack to discard
    fn discard_head_in<U: TypeEq<This = Stack>>(self) -> Result<Self, StackError>;

    /// # Errors
    /// - [`StackError::Underflow`] is returned when there are not at least [`MonotonicTuple::Length`] items on the Stack to discard.
    fn discard_n_head_in<U: TypeEq<This = Stack>>(self, n: usize) -> Result<Self, StackError>;
}

pub trait DiscardTailIn<Stack>: Sized {
    /// # Errors
    /// - [`StackError::Underflow`] is returned when there is not at least one item on the Stack to discard
    fn discard_tail_in<U: TypeEq<This = Stack>>(self) -> Result<Self, StackError>;

    /// # Errors
    /// - [`StackError::Underflow`] is returned when there are not at least [`MonotonicTuple::Length`] items on the Stack to discard.
    fn discard_n_tail_in<U: TypeEq<This = Stack>>(self, n: usize) -> Result<Self, StackError>;
}

impl<T, Stack> DiscardHeadIn<Stack> for T
where
    T: StateMut,
    <T as State>::State: HasStackMut<Stack>,
    <<T as State>::State as HasStack<Stack>>::StackType: DiscardHead,
{
    fn discard_head_in<U: TypeEq<This = Stack>>(mut self) -> Result<Self, StackError> {
        self.state_mut().stack_mut::<U>().discard_head()?;

        Ok(self)
    }

    fn discard_n_head_in<U: TypeEq<This = Stack>>(mut self, n: usize) -> Result<Self, StackError> {
        self.state_mut().stack_mut::<U>().discard_n_head(n)?;

        Ok(self)
    }
}

impl<T, Stack> DiscardTailIn<Stack> for T
where
    T: StateMut,
    <T as State>::State: HasStackMut<Stack>,
    <<T as State>::State as HasStack<Stack>>::StackType: DiscardTail,
{
    fn discard_tail_in<U: TypeEq<This = Stack>>(mut self) -> Result<Self, StackError> {
        self.state_mut().stack_mut::<U>().discard_tail()?;

        Ok(self)
    }

    fn discard_n_tail_in<U: TypeEq<This = Stack>>(mut self, n: usize) -> Result<Self, StackError> {
        self.state_mut().stack_mut::<U>().discard_n_tail(n)?;

        Ok(self)
    }
}
