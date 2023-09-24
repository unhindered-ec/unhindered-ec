use crate::{
    push_vm::{
        stack::StackError,
        state::with_state::{AddState, WithState},
    },
    tuples::MonotonicTuple,
    type_eq::TypeEq,
};

use super::{has_stack::HasStack, TypedStack};

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

pub trait GetHeadIn<'a, Stack, State>: Sized
where
    State: HasStack<Stack>,
    <State as HasStack<Stack>>::StackType: GetHead + TypedStack,
    <<State as HasStack<Stack>>::StackType as TypedStack>::Item: 'a,
{
    /// # Errors
    /// - [`StackError::Underflow`] is returned when there is not at least one item on the Stack to return.
    fn head_in<U: TypeEq<This = Stack>>(
        self,
    ) -> Result<
        WithState<&'a <<State as HasStack<Stack>>::StackType as TypedStack>::Item, State>,
        StackError,
    >;

    /// # Errors
    /// - [`StackError::Underflow`] is returned when there are not at least [`MonotonicTuple::Length`] items on the Stack to return.
    fn get_n_head_in<
        U: TypeEq<This = Stack>,
        Tuple: MonotonicTuple<Item = &'a <<State as HasStack<Stack>>::StackType as TypedStack>::Item>,
    >(
        self,
    ) -> Result<WithState<Tuple, State>, StackError>;
}

pub trait GetTailIn<'a, Stack, State>: Sized
where
    State: HasStack<Stack>,
    <State as HasStack<Stack>>::StackType: GetTail + TypedStack,
    <<State as HasStack<Stack>>::StackType as TypedStack>::Item: 'a,
{
    /// # Errors
    /// - [`StackError::Underflow`] is returned when there is not at least one item on the Stack to return.
    fn tail_in<U: TypeEq<This = Stack>>(
        self,
    ) -> Result<
        WithState<&'a <<State as HasStack<Stack>>::StackType as TypedStack>::Item, State>,
        StackError,
    >;

    /// # Errors
    /// - [`StackError::Underflow`] is returned when there are not at least [`MonotonicTuple::Length`] items on the Stack to return.
    fn get_n_tail_in<
        U: TypeEq<This = Stack>,
        Tuple: MonotonicTuple<Item = &'a <<State as HasStack<Stack>>::StackType as TypedStack>::Item>,
    >(
        self,
    ) -> Result<WithState<Tuple, State>, StackError>;
}

impl<'a, Stack, State> GetHeadIn<'a, Stack, State> for State
where
    State: HasStack<Stack>,
    <State as HasStack<Stack>>::StackType: GetHead + TypedStack,
    <<State as HasStack<Stack>>::StackType as TypedStack>::Item: 'a,
{
    fn head_in<U: TypeEq<This = Stack>>(
        self,
    ) -> Result<
        WithState<&'a <<State as HasStack<Stack>>::StackType as TypedStack>::Item, State>,
        StackError,
    > {
        self.stack::<U>().head().map(|v| v.with_state(self))
    }

    fn get_n_head_in<
        U: TypeEq<This = Stack>,
        Tuple: MonotonicTuple<Item = &'a <<State as HasStack<Stack>>::StackType as TypedStack>::Item>,
    >(
        self,
    ) -> Result<WithState<Tuple, State>, StackError> {
        self.stack::<U>()
            .get_n_head::<Tuple>()
            .map(|v| v.with_state(self))
    }
}

impl<'a, Stack, State> GetTailIn<'a, Stack, State> for State
where
    State: HasStack<Stack>,
    <State as HasStack<Stack>>::StackType: GetTail + TypedStack,
    <<State as HasStack<Stack>>::StackType as TypedStack>::Item: 'a,
{
    fn tail_in<U: TypeEq<This = Stack>>(
        self,
    ) -> Result<
        WithState<&'a <<State as HasStack<Stack>>::StackType as TypedStack>::Item, State>,
        StackError,
    > {
        self.stack::<U>().tail().map(|v| v.with_state(self))
    }

    fn get_n_tail_in<
        U: TypeEq<This = Stack>,
        Tuple: MonotonicTuple<Item = &'a <<State as HasStack<Stack>>::StackType as TypedStack>::Item>,
    >(
        self,
    ) -> Result<WithState<Tuple, State>, StackError> {
        self.stack::<U>()
            .get_n_tail::<Tuple>()
            .map(|v| v.with_state(self))
    }
}
