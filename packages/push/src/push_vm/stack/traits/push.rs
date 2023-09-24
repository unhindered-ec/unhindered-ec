use crate::{
    error::into_state::{State, StateMut},
    push_vm::{stack::StackError, state::with_state::WithState},
    tuples::MonotonicTuple,
    type_eq::TypeEq,
};

use super::{
    extend::{ExtendHead, ExtendTail},
    has_stack::{HasStack, HasStackMut},
    TypedStack,
};

pub trait PushHead: TypedStack {
    /// # Errors
    /// - [`StackError::Overflow`] is returned when the stack has not enough capacity for one new value remaining.
    fn push_head(&mut self, value: Self::Item) -> Result<(), StackError> {
        self.push_n_head((value,))
    }

    /// # Errors
    /// - [`StackError::Overflow`] is returned when the stack has not enough capacity for n new values remaining.
    fn push_n_head<Tuple: MonotonicTuple<Item = Self::Item>>(
        &mut self,
        value: Tuple,
    ) -> Result<(), StackError>;
}

pub trait PushTail: TypedStack {
    /// # Errors
    /// - [`StackError::Overflow`] is returned when the stack has not enough capacity for one new value remaining.
    fn push_tail(&mut self, value: Self::Item) -> Result<(), StackError> {
        self.push_n_tail((value,))
    }

    /// # Errors
    /// - [`StackError::Overflow`] is returned when the stack has not enough capacity for n new values remaining.
    fn push_n_tail<Tuple: MonotonicTuple<Item = Self::Item>>(
        &mut self,
        value: Tuple,
    ) -> Result<(), StackError>;
}

// Blanket impl - Is this a good idea even when specialization is not stable yet? May prevent some manual impls but
// I haven't run into that yet. If it does, we should temporarily remove it, although I don't think any other impl
// would be practical anyways.
impl<T> PushHead for T
where
    T: ExtendHead,
{
    #[inline]
    fn push_n_head<Tuple: MonotonicTuple<Item = Self::Item>>(
        &mut self,
        value: Tuple,
    ) -> Result<(), StackError> {
        self.extend_head(value.into_iterator())
    }
}

impl<T> PushTail for T
where
    T: ExtendTail,
{
    #[inline]
    fn push_n_tail<Tuple: MonotonicTuple<Item = Self::Item>>(
        &mut self,
        value: Tuple,
    ) -> Result<(), StackError> {
        self.extend_tail(value.into_iterator())
    }
}

pub trait PushHeadIn<Stack, State>: Sized
where
    State: HasStackMut<Stack>,
    <State as HasStack<Stack>>::StackType: PushHead + TypedStack,
{
    /// # Errors
    /// - [`StackError::Overflow`] is returned when the stack has not enough capacity for one new value remaining.
    fn push_head_in<U: TypeEq<This = Stack>>(
        self,
        value: <<State as HasStack<Stack>>::StackType as TypedStack>::Item,
    ) -> Result<Self, StackError>;

    /// # Errors
    /// - [`StackError::Overflow`] is returned when the stack has not enough capacity for n new values remaining.
    fn push_n_head_in<
        U: TypeEq<This = Stack>,
        Tuple: MonotonicTuple<Item = <<State as HasStack<Stack>>::StackType as TypedStack>::Item>,
    >(
        self,
        value: Tuple,
    ) -> Result<Self, StackError>;
}

pub trait PushTailIn<Stack, State>: Sized
where
    State: HasStackMut<Stack>,
    <State as HasStack<Stack>>::StackType: PushTail + TypedStack,
{
    /// # Errors
    /// - [`StackError::Overflow`] is returned when the stack has not enough capacity for one new value remaining.
    fn push_tail_in<U: TypeEq<This = Stack>>(
        self,
        value: <<State as HasStack<Stack>>::StackType as TypedStack>::Item,
    ) -> Result<Self, StackError>;

    /// # Errors
    /// - [`StackError::Overflow`] is returned when the stack has not enough capacity for n new values remaining.
    fn push_n_tail_in<
        U: TypeEq<This = Stack>,
        Tuple: MonotonicTuple<Item = <<State as HasStack<Stack>>::StackType as TypedStack>::Item>,
    >(
        self,
        value: Tuple,
    ) -> Result<Self, StackError>;
}

impl<Stack, T> PushHeadIn<Stack, <T as State>::State> for T
where
    T: StateMut,
    <T as State>::State: HasStackMut<Stack>,
    <<T as State>::State as HasStack<Stack>>::StackType: PushHead + TypedStack,
{
    fn push_head_in<U: TypeEq<This = Stack>>(
        mut self,
        value: <<<T as State>::State as HasStack<Stack>>::StackType as TypedStack>::Item,
    ) -> Result<Self, StackError> {
        self.state_mut().stack_mut::<U>().push_head(value)?;

        Ok(self)
    }

    fn push_n_head_in<
        U: TypeEq<This = Stack>,
        Tuple: MonotonicTuple<
            Item = <<<T as State>::State as HasStack<Stack>>::StackType as TypedStack>::Item,
        >,
    >(
        mut self,
        value: Tuple,
    ) -> Result<Self, StackError> {
        self.state_mut().stack_mut::<U>().push_n_head(value)?;

        Ok(self)
    }
}

impl<Stack, T> PushTailIn<Stack, <T as State>::State> for T
where
    T: StateMut,
    <T as State>::State: HasStackMut<Stack>,
    <<T as State>::State as HasStack<Stack>>::StackType: PushTail + TypedStack,
{
    fn push_tail_in<U: TypeEq<This = Stack>>(
        mut self,
        value: <<<T as State>::State as HasStack<Stack>>::StackType as TypedStack>::Item,
    ) -> std::result::Result<Self, StackError> {
        self.state_mut().stack_mut::<U>().push_tail(value)?;

        Ok(self)
    }

    fn push_n_tail_in<
        U: TypeEq<This = Stack>,
        Tuple: MonotonicTuple<
            Item = <<<T as State>::State as HasStack<Stack>>::StackType as TypedStack>::Item,
        >,
    >(
        mut self,
        value: Tuple,
    ) -> std::result::Result<Self, StackError> {
        self.state_mut().stack_mut::<U>().push_n_tail(value)?;

        Ok(self)
    }
}

pub trait AttemptPushHead<Stack, State> {
    fn attempt_push_head(self) -> Result<State, StackError>;
}

impl<Value, T> AttemptPushHead<Value, T> for WithState<Value, T>
where
    T: StateMut,
    <T as State>::State: HasStackMut<Value>,
    <<T as State>::State as HasStack<Value>>::StackType: PushHead + TypedStack<Item = Value>,
{
    fn attempt_push_head(self) -> Result<T, StackError> {
        let Self { value, state } = self;
        state.push_head_in::<Value>(value)
    }
}

pub trait AttemptPushHeadN<Stack, State> {
    fn attempt_push_head_n(self) -> Result<State, StackError>;
}

impl<Value, T> AttemptPushHeadN<Value, T> for WithState<Value, T>
where
    Value: MonotonicTuple,
    T: StateMut,
    <T as State>::State: HasStackMut<Value::Item>,
    <<T as State>::State as HasStack<Value::Item>>::StackType:
        PushHead + TypedStack<Item = Value::Item>,
{
    fn attempt_push_head_n(self) -> Result<T, StackError> {
        let Self { value, state } = self;
        state.push_n_head_in::<Value::Item, Value>(value)
    }
}

pub trait AttemptPushTail<Stack, State> {
    fn attempt_push_tail(self) -> Result<State, StackError>;
}

impl<Value, T> AttemptPushTail<Value, T> for WithState<Value, T>
where
    T: StateMut,
    <T as State>::State: HasStackMut<Value>,
    <<T as State>::State as HasStack<Value>>::StackType: PushTail + TypedStack<Item = Value>,
{
    fn attempt_push_tail(self) -> Result<T, StackError> {
        let Self { value, state } = self;
        state.push_tail_in::<Value>(value)
    }
}

pub trait AttemptPushTailN<Stack, State> {
    fn attempt_push_tail_n(self) -> Result<State, StackError>;
}

impl<Value, T> AttemptPushTailN<Value, T> for WithState<Value, T>
where
    Value: MonotonicTuple,
    T: StateMut,
    <T as State>::State: HasStackMut<Value::Item>,
    <<T as State>::State as HasStack<Value::Item>>::StackType:
        PushTail + TypedStack<Item = Value::Item>,
{
    fn attempt_push_tail_n(self) -> Result<T, StackError> {
        let Self { value, state } = self;
        state.push_n_tail_in::<Value::Item, Value>(value)
    }
}
