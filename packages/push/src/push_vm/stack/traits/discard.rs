use crate::{
    error::{into_state::IntoState, stateful::UnknownError},
    push_vm::{stack::StackError, state::with_state::AddState},
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

pub trait DiscardHeadIn<Stack, State>: Sized {
    /// # Errors
    /// - [`StackError::Underflow`] is returned when there is not at least one item on the Stack to discard
    fn discard_head_in<U: TypeEq<This = Stack>>(
        self,
    ) -> Result<Self, UnknownError<State, StackError>>;

    /// # Errors
    /// - [`StackError::Underflow`] is returned when there are not at least [`MonotonicTuple::Length`] items on the Stack to discard.
    fn discard_n_head_in<U: TypeEq<This = Stack>>(
        self,
        n: usize,
    ) -> Result<Self, UnknownError<State, StackError>>;
}

pub trait DiscardTailIn<Stack, State>: Sized {
    /// # Errors
    /// - [`StackError::Underflow`] is returned when there is not at least one item on the Stack to discard
    fn discard_tail_in<U: TypeEq<This = Stack>>(
        self,
    ) -> Result<Self, UnknownError<State, StackError>>;

    /// # Errors
    /// - [`StackError::Underflow`] is returned when there are not at least [`MonotonicTuple::Length`] items on the Stack to discard.
    fn discard_n_tail_in<U: TypeEq<This = Stack>>(
        self,
        n: usize,
    ) -> Result<Self, UnknownError<State, StackError>>;
}

impl<WithState, State, Stack> DiscardHeadIn<Stack, State> for WithState
where
    WithState: IntoState<State>,
    State: HasStackMut<Stack>,
    <State as HasStack<Stack>>::StackType: DiscardHead,
{
    fn discard_head_in<U: TypeEq<This = Stack>>(
        self,
    ) -> Result<Self, UnknownError<State, StackError>> {
        let mut state = self.into_state();
        state
            .stack_mut::<U>()
            .discard_head()
            .map_err(|e| e.with_state(state))?;

        Ok(self)
    }

    fn discard_n_head_in<U: TypeEq<This = Stack>>(
        self,
        n: usize,
    ) -> Result<Self, UnknownError<State, StackError>> {
        let mut state = self.into_state();
        state
            .stack_mut::<U>()
            .discard_n_head(n)
            .map_err(|e| e.with_state(state))?;

        Ok(self)
    }
}

impl<WithState, State, Stack> DiscardTailIn<Stack, State> for WithState
where
    WithState: IntoState<State>,
    State: HasStackMut<Stack>,
    <State as HasStack<Stack>>::StackType: DiscardTail,
{
    fn discard_tail_in<U: TypeEq<This = Stack>>(
        self,
    ) -> Result<Self, UnknownError<State, StackError>> {
        let mut state = self.into_state();
        state
            .stack_mut::<U>()
            .discard_tail()
            .map_err(|e| e.with_state(state))?;

        Ok(self)
    }

    fn discard_n_tail_in<U: TypeEq<This = Stack>>(
        self,
        n: usize,
    ) -> Result<Self, UnknownError<State, StackError>> {
        let mut state = self.into_state();
        state
            .stack_mut::<U>()
            .discard_n_tail(n)
            .map_err(|e| e.with_state(state))?;

        Ok(self)
    }
}
