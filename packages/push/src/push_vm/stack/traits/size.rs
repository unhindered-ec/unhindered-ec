use crate::{
    error::stateful::UnknownError,
    push_vm::{
        stack::StackError,
        state::with_state::{AddState, WithState, WithStateOps},
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
    fn max_size_of<U: TypeEq<This = Stack>>(self) -> WithState<usize, State>;

    #[must_use]
    fn is_full_of<U: TypeEq<This = Stack>>(self) -> WithState<bool, State>
    where
        <State as HasStack<Stack>>::StackType: StackSize;

    fn not_full<U: TypeEq<This = Stack>>(self) -> Result<State, UnknownError<State, StackError>>
    where
        <State as HasStack<Stack>>::StackType: StackSize;

    #[must_use]
    fn capacity_of<U: TypeEq<This = Stack>>(self) -> WithState<usize, State>
    where
        <State as HasStack<Stack>>::StackType: StackSize;
}

pub trait SizeLimitOfMut<Stack, State>: Sized
where
    State: HasStack<Stack>,
{
    /// # Errors
    /// - [`StackError::Overflow`] when the new `max_size` is smaller than the current length of the stack.
    fn set_max_size_of<U: TypeEq<This = Stack>>(
        self,
        max_size: usize,
    ) -> Result<State, UnknownError<State, StackError>>;
}

pub trait StackSizeOf<Stack, State>: Sized
where
    State: HasStack<Stack>,
{
    #[must_use]
    fn size_of<U: TypeEq<This = Stack>>(self) -> WithState<usize, State>;

    #[must_use]
    fn is_empty_of<U: TypeEq<This = Stack>>(self) -> WithState<bool, State>;
}

impl<Stack, State> SizeLimitOf<Stack, State> for State
where
    State: HasStack<Stack>,
    <State as HasStack<Stack>>::StackType: SizeLimit,
{
    fn max_size_of<U: TypeEq<This = Stack>>(self) -> WithState<usize, State> {
        self.stack::<U>().max_size().with_state(self)
    }

    fn is_full_of<U: TypeEq<This = Stack>>(self) -> WithState<bool, State>
    where
        <State as HasStack<Stack>>::StackType: StackSize,
    {
        self.stack::<U>().is_full().with_state(self)
    }

    fn capacity_of<U: TypeEq<This = Stack>>(self) -> WithState<usize, State>
    where
        <State as HasStack<Stack>>::StackType: StackSize,
    {
        self.stack::<U>().capacity().with_state(self)
    }

    fn not_full<U: TypeEq<This = Stack>>(self) -> Result<State, UnknownError<State, StackError>>
    where
        <State as HasStack<Stack>>::StackType: StackSize,
    {
        if self.is_full_of::<U>().drop_state() {
            Err(
                StackError::overflow_unknown_requested::<<State as HasStack<Stack>>::StackType>(0)
                    .with_state(self)
                    .into(),
            )
        } else {
            Ok(self)
        }
    }
}

impl<State, Stack> SizeLimitOfMut<Stack, State> for State
where
    State: HasStackMut<Stack>,
    <State as HasStack<Stack>>::StackType: SizeLimit,
{
    fn set_max_size_of<U: TypeEq<This = Stack>>(
        mut self,
        max_size: usize,
    ) -> Result<Self, UnknownError<Self, StackError>> {
        self.stack_mut::<U>()
            .set_max_size(max_size)
            .map_err(|e| e.with_state(self))?;

        Ok(self)
    }
}

impl<State, Stack> StackSizeOf<Stack, State> for State
where
    State: HasStack<Stack>,
    <State as HasStack<Stack>>::StackType: StackSize,
{
    fn size_of<U: TypeEq<This = Stack>>(self) -> WithState<usize, State> {
        self.stack::<U>().size().with_state(self)
    }

    fn is_empty_of<U: TypeEq<This = Stack>>(self) -> WithState<bool, State> {
        self.stack::<U>().is_empty().with_state(self)
    }
}
