use crate::{
    error::into_state::{State, StateMut},
    type_eq::TypeEq,
};

pub trait HasStack<T> {
    type StackType;

    fn stack<U: TypeEq<This = T>>(&self) -> &Self::StackType;
}

impl<T, R> HasStack<T> for R
where
    R: State,
    <R as State>::State: HasStack<T>,
{
    type StackType = <<R as State>::State as HasStack<T>>::StackType;

    fn stack<U: TypeEq<This = T>>(&self) -> &Self::StackType {
        self.state().stack::<U>()
    }
}

pub trait HasStackMut<T>: HasStack<T> {
    fn stack_mut<U: TypeEq<This = T>>(&mut self) -> &mut Self::StackType;
}

impl<T, R> HasStackMut<T> for R
where
    R: StateMut,
    <R as State>::State: HasStackMut<T>,
{
    fn stack_mut<U: TypeEq<This = T>>(&mut self) -> &mut Self::StackType {
        self.state_mut().stack_mut::<U>()
    }
}
