use crate::{push_vm::state::with_state::WithState, type_eq::TypeEq};

pub trait HasStack<T> {
    type StackType;

    fn stack<U: TypeEq<This = T>>(&self) -> &Self::StackType;
}

impl<'a, T, R> HasStack<T> for &'a R
where
    R: HasStack<T>,
{
    type StackType = R::StackType;

    fn stack<U: TypeEq<This = T>>(&self) -> &Self::StackType {
        (**self).stack::<U>()
    }
}

impl<'a, T, R, V> HasStack<T> for WithState<V, &'a R>
where
    R: HasStack<T>,
{
    type StackType = R::StackType;

    fn stack<U: TypeEq<This = T>>(&self) -> &Self::StackType {
        self.state.stack::<U>()
    }
}

impl<'a, T, R> HasStack<T> for &'a mut R
where
    R: HasStack<T>,
{
    type StackType = R::StackType;

    fn stack<U: TypeEq<This = T>>(&self) -> &Self::StackType {
        (**self).stack::<U>()
    }
}

impl<'a, T, R, V> HasStack<T> for WithState<V, &'a mut R>
where
    R: HasStack<T>,
{
    type StackType = R::StackType;

    fn stack<U: TypeEq<This = T>>(&self) -> &Self::StackType {
        self.state.stack::<U>()
    }
}

pub trait HasStackMut<T>: HasStack<T> {
    fn stack_mut<U: TypeEq<This = T>>(&mut self) -> &mut Self::StackType;
}

impl<'a, T, R> HasStackMut<T> for &'a mut R
where
    R: HasStackMut<T>,
{
    fn stack_mut<U: TypeEq<This = T>>(&mut self) -> &mut Self::StackType {
        (*self).stack_mut::<U>()
    }
}

impl<'a, T, R, V> HasStackMut<T> for WithState<V, &'a mut R>
where
    R: HasStackMut<T>,
{
    fn stack_mut<U: TypeEq<This = T>>(&mut self) -> &mut Self::StackType {
        self.state.stack_mut::<U>()
    }
}
