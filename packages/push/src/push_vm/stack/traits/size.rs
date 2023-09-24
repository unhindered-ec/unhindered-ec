use crate::{
    error::into_state::{State, StateMut},
    push_vm::{
        stack::StackError,
        state::with_state::{AddState, WithState},
    },
    type_eq::TypeEq,
};

use super::has_stack::{HasStack, HasStackMut};

pub trait SizeLimit {
    #[must_use]
    fn max_size(&self) -> usize;

    /// # Errors
    /// - [`StackError::Overflow`] when the new `max_size` is smaller than the current length of the stack.
    fn set_max_size(&mut self, max_size: usize) -> Result<(), StackError>;

    #[must_use]
    #[inline]
    fn is_full(&self) -> bool
    where
        Self: StackSize,
    {
        self.max_size() == self.size()
    }

    #[must_use]
    #[inline]
    fn capacity(&self) -> usize
    where
        Self: StackSize,
    {
        self.max_size() - self.size()
    }
}

pub trait StackSize {
    #[must_use]
    fn size(&self) -> usize;

    #[must_use]
    fn is_empty(&self) -> bool {
        self.size() == 0
    }
}

pub trait SizeLimitOf<Stack, State>: Sized
where
    State: HasStack<Stack>,
{
    #[must_use]
    fn max_size_of<U: TypeEq<This = Stack>>(self) -> WithState<usize, Self>;

    #[must_use]
    #[allow(clippy::wrong_self_convention)]
    fn is_full_of<U: TypeEq<This = Stack>>(self) -> WithState<bool, Self>
    where
        <State as HasStack<Stack>>::StackType: StackSize;

    fn not_full_in<U: TypeEq<This = Stack>>(self) -> Result<Self, StackError>
    where
        <State as HasStack<Stack>>::StackType: StackSize;

    #[must_use]
    fn capacity_of<U: TypeEq<This = Stack>>(self) -> WithState<usize, Self>
    where
        <State as HasStack<Stack>>::StackType: StackSize;
}

pub trait SizeLimitOfMut<Stack, State>: Sized
where
    State: HasStack<Stack>,
{
    /// # Errors
    /// - [`StackError::Overflow`] when the new `max_size` is smaller than the current length of the stack.
    fn set_max_size_of<U: TypeEq<This = Stack>>(self, max_size: usize) -> Result<Self, StackError>;
}

pub trait StackSizeOf<Stack>: Sized {
    #[must_use]
    fn size_of<U: TypeEq<This = Stack>>(self) -> WithState<usize, Self>;

    #[must_use]
    #[allow(clippy::wrong_self_convention)]
    fn is_empty_of<U: TypeEq<This = Stack>>(self) -> WithState<bool, Self>;
}

impl<Stack, T> SizeLimitOf<Stack, <T as State>::State> for T
where
    T: State,
    <T as State>::State: HasStack<Stack>,
    <<T as State>::State as HasStack<Stack>>::StackType: SizeLimit,
{
    fn max_size_of<U: TypeEq<This = Stack>>(self) -> WithState<usize, Self> {
        self.state().stack::<U>().max_size().with_state(self)
    }

    fn is_full_of<U: TypeEq<This = Stack>>(self) -> WithState<bool, Self>
    where
        <<T as State>::State as HasStack<Stack>>::StackType: StackSize,
    {
        self.state().stack::<U>().is_full().with_state(self)
    }

    fn capacity_of<U: TypeEq<This = Stack>>(self) -> WithState<usize, Self>
    where
        <<T as State>::State as HasStack<Stack>>::StackType: StackSize,
    {
        self.state().stack::<U>().capacity().with_state(self)
    }

    fn not_full_in<U: TypeEq<This = Stack>>(self) -> Result<Self, StackError>
    where
        <<T as State>::State as HasStack<Stack>>::StackType: StackSize,
    {
        if self.state().stack::<U>().is_full() {
            Err(StackError::overflow_unknown_requested::<
                <<T as State>::State as HasStack<Stack>>::StackType,
            >(0))
        } else {
            Ok(self)
        }
    }
}

impl<Stack, T> SizeLimitOfMut<Stack, <T as State>::State> for T
where
    T: StateMut,
    <T as State>::State: HasStackMut<Stack>,
    <<T as State>::State as HasStack<Stack>>::StackType: SizeLimit,
{
    fn set_max_size_of<U: TypeEq<This = Stack>>(
        mut self,
        max_size: usize,
    ) -> Result<Self, StackError> {
        self.state_mut().stack_mut::<U>().set_max_size(max_size)?;
        Ok(self)
    }
}

impl<Stack, T> StackSizeOf<Stack> for T
where
    T: State,
    <T as State>::State: HasStack<Stack>,
    <<T as State>::State as HasStack<Stack>>::StackType: StackSize,
{
    fn size_of<U: TypeEq<This = Stack>>(self) -> WithState<usize, Self> {
        self.state().stack::<U>().size().with_state(self)
    }

    fn is_empty_of<U: TypeEq<This = Stack>>(self) -> WithState<bool, Self> {
        self.state().stack::<U>().is_empty().with_state(self)
    }
}
