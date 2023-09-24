use crate::push_vm::state::with_state::{AddState, WithState};

use super::has_stack::{HasStack, HasStackMut};

pub trait WithStack<T> {
    type StackType;
    #[must_use]
    fn with_stack(self, f: impl FnOnce(&Self::StackType)) -> Self;
    /// # Errors
    /// Returns the error returned by the passed closure if one was returned
    fn with_stack_try<E>(
        self,
        f: impl FnOnce(&Self::StackType) -> Result<(), E>,
    ) -> Result<Self, WithState<E, Self>>
    where
        Self: Sized;
}

pub trait WithStackMut<T>: WithStack<T> {
    #[must_use]
    fn with_stack_mut(self, f: impl FnOnce(&mut Self::StackType)) -> Self;
    /// # Errors
    /// Returns the error returned by the passed closure if one was returned
    fn with_stack_mut_try<E>(
        self,
        f: impl FnOnce(&mut Self::StackType) -> Result<(), E>,
    ) -> Result<Self, E>
    where
        Self: Sized;
}

impl<T, U> WithStack<U> for T
where
    T: HasStack<U>,
{
    type StackType = T::StackType;

    fn with_stack(self, f: impl FnOnce(&T::StackType)) -> Self {
        f(self.stack::<U>());
        self
    }

    fn with_stack_try<E>(
        self,
        f: impl FnOnce(&T::StackType) -> Result<(), E>,
    ) -> Result<Self, WithState<E, Self>>
    where
        Self: Sized,
    {
        match f(self.stack::<U>()) {
            Ok(_) => Ok(self),
            Err(err) => Err(err.with_state(self)),
        }
    }
}

impl<T, U> WithStackMut<U> for T
where
    T: HasStackMut<U>,
{
    fn with_stack_mut(mut self, f: impl FnOnce(&mut T::StackType)) -> Self {
        f(self.stack_mut::<U>());
        self
    }

    fn with_stack_mut_try<E>(
        mut self,
        f: impl FnOnce(&mut T::StackType) -> Result<(), E>,
    ) -> Result<Self, E>
    where
        Self: Sized,
    {
        f(self.stack_mut::<U>())?;
        Ok(self)
    }
}
