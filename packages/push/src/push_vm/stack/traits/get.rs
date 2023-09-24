use crate::{
    error::into_state::State, push_vm::stack::StackError, tuples::MonotonicTuple, type_eq::TypeEq,
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
    ) -> Result<&'a <<State as HasStack<Stack>>::StackType as TypedStack>::Item, StackError>;

    /// # Errors
    /// - [`StackError::Underflow`] is returned when there are not at least [`MonotonicTuple::Length`] items on the Stack to return.
    fn get_n_head_in<
        U: TypeEq<This = Stack>,
        Tuple: MonotonicTuple<Item = &'a <<State as HasStack<Stack>>::StackType as TypedStack>::Item>,
    >(
        self,
    ) -> Result<Tuple, StackError>;
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
    ) -> Result<&'a <<State as HasStack<Stack>>::StackType as TypedStack>::Item, StackError>;

    /// # Errors
    /// - [`StackError::Underflow`] is returned when there are not at least [`MonotonicTuple::Length`] items on the Stack to return.
    fn get_n_tail_in<
        U: TypeEq<This = Stack>,
        Tuple: MonotonicTuple<Item = &'a <<State as HasStack<Stack>>::StackType as TypedStack>::Item>,
    >(
        self,
    ) -> Result<Tuple, StackError>;
}

impl<'a, Stack, T> GetHeadIn<'a, Stack, <T as State>::State> for &'a T
where
    T: State + 'a,
    <T as State>::State: HasStack<Stack> + 'a,
    <<T as State>::State as HasStack<Stack>>::StackType: GetHead + TypedStack + 'a,
    <<<T as State>::State as HasStack<Stack>>::StackType as TypedStack>::Item: 'a,
{
    fn head_in<U: TypeEq<This = Stack>>(
        self,
    ) -> Result<
        &'a <<<T as State>::State as HasStack<Stack>>::StackType as TypedStack>::Item,
        StackError,
    > {
        self.state().stack::<U>().head()
    }

    fn get_n_head_in<
        U: TypeEq<This = Stack>,
        Tuple: MonotonicTuple<
            Item = &'a <<<T as State>::State as HasStack<Stack>>::StackType as TypedStack>::Item,
        >,
    >(
        self,
    ) -> Result<Tuple, StackError> {
        self.state().stack::<U>().get_n_head::<Tuple>()
    }
}

impl<'a, Stack, T> GetTailIn<'a, Stack, <T as State>::State> for &'a T
where
    T: State + 'a,
    <T as State>::State: HasStack<Stack> + 'a,
    <<T as State>::State as HasStack<Stack>>::StackType: GetTail + TypedStack + 'a,
    <<<T as State>::State as HasStack<Stack>>::StackType as TypedStack>::Item: 'a,
{
    fn tail_in<U: TypeEq<This = Stack>>(
        self,
    ) -> Result<
        &'a <<<T as State>::State as HasStack<Stack>>::StackType as TypedStack>::Item,
        StackError,
    > {
        self.state().stack::<U>().tail()
    }

    fn get_n_tail_in<
        U: TypeEq<This = Stack>,
        Tuple: MonotonicTuple<
            Item = &'a <<<T as State>::State as HasStack<Stack>>::StackType as TypedStack>::Item,
        >,
    >(
        self,
    ) -> Result<Tuple, StackError> {
        self.state().stack::<U>().get_n_tail::<Tuple>()
    }
}
