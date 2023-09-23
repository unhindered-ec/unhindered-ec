use crate::push_vm::stack::Stack;

use super::has_stack::{HasStack, HasStackMut};

pub struct WithState<Value, State> {
    value: Value,
    state: State,
}

impl<Value, State> WithState<Value, State> {
    pub fn drop_state(self) -> Value {
        self.value
    }

    pub fn drop_value(self) -> State {
        self.state
    }
}

trait AddState<State>: Sized {
    fn with_state(self, state: State) -> WithState<Self, State>;
}

impl<State, Value> AddState<State> for Value {
    fn with_state(self, state: State) -> WithState<Self, State> {
        WithState { value: self, state }
    }
}

pub trait WithStack<T> {
    #[must_use]
    fn with_stack(self, f: impl FnOnce(&Stack<T>)) -> Self;
    /// # Errors
    /// Returns the error returned by the passed closure if one was returned
    fn with_stack_try<E>(
        self,
        f: impl FnOnce(&Stack<T>) -> Result<(), E>,
    ) -> Result<Self, WithState<E, Self>>
    where
        Self: Sized;
}

pub trait WithStackMut<T> {
    #[must_use]
    fn with_stack_mut(self, f: impl FnOnce(&mut Stack<T>)) -> Self;
    /// # Errors
    /// Returns the error returned by the passed closure if one was returned
    fn with_stack_mut_try<E>(
        self,
        f: impl FnOnce(&mut Stack<T>) -> Result<(), E>,
    ) -> Result<Self, E>
    where
        Self: Sized;
}

impl<T, U> WithStack<U> for T
where
    T: HasStack<U>,
{
    fn with_stack(self, f: impl FnOnce(&Stack<U>)) -> Self {
        f(self.stack::<U>());
        self
    }

    fn with_stack_try<E>(
        self,
        f: impl FnOnce(&Stack<U>) -> Result<(), E>,
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
    fn with_stack_mut(mut self, f: impl FnOnce(&mut Stack<U>)) -> Self {
        f(self.stack_mut::<U>());
        self
    }

    fn with_stack_mut_try<E>(
        mut self,
        f: impl FnOnce(&mut Stack<U>) -> Result<(), E>,
    ) -> Result<Self, E>
    where
        Self: Sized,
    {
        f(self.stack_mut::<U>())?;
        Ok(self)
    }
}
