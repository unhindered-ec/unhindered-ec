use crate::{
    error::stateful::UnknownError,
    push_vm::{
        stack::StackError,
        state::with_state::{AddState, WithState},
    },
    tuples::MonotonicTuple,
    type_eq::TypeEq,
};

use super::{
    get::{GetHead, GetTail},
    has_stack::{HasStack, HasStackMut},
    TypedStack,
};

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

pub trait PopHeadIn<Stack, State>: Sized
where
    State: HasStackMut<Stack>,
    <State as HasStack<Stack>>::StackType: PopHead,
{
    /// # Errors
    /// - [`StackError::Underflow`] is returned when there is not at least one item on the Stack to return.
    fn pop_head_in<U: TypeEq<This = Stack>>(
        self,
    ) -> Result<
        WithState<<<State as HasStack<Stack>>::StackType as TypedStack>::Item, State>,
        UnknownError<State, StackError>,
    >;

    /// # Errors
    /// - [`StackError::Underflow`] is returned when there are not at least [`MonotonicTuple::Length`] items on the Stack to return.
    fn pop_n_head_in<
        U: TypeEq<This = Stack>,
        Tuple: MonotonicTuple<Item = <<State as HasStack<Stack>>::StackType as TypedStack>::Item>,
    >(
        self,
    ) -> Result<WithState<Tuple, State>, UnknownError<State, StackError>>;
}

pub trait PopTailIn<Stack, State>: Sized
where
    State: HasStackMut<Stack>,
    <State as HasStack<Stack>>::StackType: PopTail,
{
    /// # Errors
    /// - [`StackError::Underflow`] is returned when there is not at least one item on the Stack to return.
    fn pop_tail_in<U: TypeEq<This = Stack>>(
        self,
    ) -> Result<
        WithState<<<State as HasStack<Stack>>::StackType as TypedStack>::Item, State>,
        UnknownError<State, StackError>,
    >;

    /// # Errors
    /// - [`StackError::Underflow`] is returned when there are not at least [`MonotonicTuple::Length`] items on the Stack to return.
    fn pop_n_tail_in<
        U: TypeEq<This = Stack>,
        Tuple: MonotonicTuple<Item = <<State as HasStack<Stack>>::StackType as TypedStack>::Item>,
    >(
        self,
    ) -> Result<WithState<Tuple, State>, UnknownError<State, StackError>>;
}

impl<Stack, State> PopHeadIn<Stack, State> for State
where
    State: HasStackMut<Stack>,
    <State as HasStack<Stack>>::StackType: PopHead,
{
    fn pop_head_in<U: TypeEq<This = Stack>>(
        mut self,
    ) -> Result<
        WithState<<<State as HasStack<Stack>>::StackType as TypedStack>::Item, State>,
        UnknownError<State, StackError>,
    > {
        match self.stack_mut::<U>().pop_head() {
            Ok(v) => Ok(v.with_state(self)),
            Err(e) => Err(e.with_state(self))?,
        }
    }

    fn pop_n_head_in<
        U: TypeEq<This = Stack>,
        Tuple: MonotonicTuple<Item = <<State as HasStack<Stack>>::StackType as TypedStack>::Item>,
    >(
        mut self,
    ) -> Result<WithState<Tuple, State>, UnknownError<State, StackError>> {
        self.stack_mut::<U>()
            .pop_n_head::<Tuple>()
            .with_state(self)
            .into()
    }
}

impl<Stack, State> PopTailIn<Stack, State> for State
where
    State: HasStackMut<Stack>,
    <State as HasStack<Stack>>::StackType: PopTail,
{
    fn pop_tail_in<U: TypeEq<This = Stack>>(
        mut self,
    ) -> Result<
        WithState<<<State as HasStack<Stack>>::StackType as TypedStack>::Item, State>,
        UnknownError<State, StackError>,
    > {
        self.stack_mut::<U>().pop_tail().with_state(self).into()
    }

    fn pop_n_tail_in<
        U: TypeEq<This = Stack>,
        Tuple: MonotonicTuple<Item = <<State as HasStack<Stack>>::StackType as TypedStack>::Item>,
    >(
        mut self,
    ) -> Result<WithState<Tuple, State>, UnknownError<State, StackError>> {
        self.stack_mut::<U>()
            .pop_n_tail::<Tuple>()
            .with_state(self)
            .into()
    }
}
