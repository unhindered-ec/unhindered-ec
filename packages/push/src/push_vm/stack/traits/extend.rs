use crate::{error::into_state::IntoState, push_vm::stack::StackError, type_eq::TypeEq};

use super::{
    has_stack::{HasStack, HasStackMut},
    TypedStack,
};

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

pub trait ExtendHeadIn<Stack, State>: Sized
where
    State: HasStackMut<Stack>,
    <State as HasStack<Stack>>::StackType: ExtendHead + TypedStack,
{
    /// # Errors
    /// - [`StackError::Overflow`] is returned when the stack has not enough capacity for [`Iter::len()`] new values remaining.
    fn extend_head_in<U: TypeEq<This = Stack>, Iter>(self, iter: Iter) -> Result<Self, StackError>
    where
        Iter: IntoIterator<Item = <<State as HasStack<Stack>>::StackType as TypedStack>::Item>,
        Iter::IntoIter: DoubleEndedIterator + ExactSizeIterator;
}

pub trait ExtendTailIn<Stack, State>: Sized
where
    State: HasStackMut<Stack>,
    <State as HasStack<Stack>>::StackType: ExtendTail + TypedStack,
{
    /// # Errors
    /// - [`StackError::Overflow`] is returned when the stack has not enough capacity for [`Iter::len()`] new values remaining.
    fn extend_tail_in<U: TypeEq<This = Stack>, Iter>(self, iter: Iter) -> Result<Self, StackError>
    where
        Iter: IntoIterator<Item = <<State as HasStack<Stack>>::StackType as TypedStack>::Item>,
        Iter::IntoIter: DoubleEndedIterator + ExactSizeIterator;
}

impl<WithState, State, Stack> ExtendHeadIn<Stack, State> for WithState
where
    WithState: IntoState<State>,
    State: HasStackMut<Stack>,
    <State as HasStack<Stack>>::StackType: ExtendHead + TypedStack,
{
    fn extend_head_in<U: TypeEq<This = Stack>, Iter>(self, iter: Iter) -> Result<Self, StackError>
    where
        Iter: IntoIterator<Item = <<State as HasStack<Stack>>::StackType as TypedStack>::Item>,
        Iter::IntoIter: DoubleEndedIterator + ExactSizeIterator,
    {
        self.into_state().stack_mut::<U>().extend_head(iter)?;

        Ok(self)
    }
}

impl<WithState, State, Stack> ExtendTailIn<Stack, State> for WithState
where
    WithState: IntoState<State>,
    State: HasStackMut<Stack>,
    <State as HasStack<Stack>>::StackType: ExtendTail + TypedStack,
{
    fn extend_tail_in<U: TypeEq<This = Stack>, Iter>(
        mut self,
        iter: Iter,
    ) -> Result<Self, StackError>
    where
        Iter: IntoIterator<Item = <<State as HasStack<Stack>>::StackType as TypedStack>::Item>,
        Iter::IntoIter: DoubleEndedIterator + ExactSizeIterator,
    {
        self.into_state().stack_mut::<U>().extend_tail(iter)?;

        Ok(self)
    }
}
