use crate::error::{into_state::IntoState, stateful::UnknownError};

pub struct WithState<Value, State> {
    pub value: Value,
    pub state: State,
}

impl<Value, State> IntoState<State> for WithState<Value, State> {
    fn into_state(self) -> State {
        self.state
    }
}

pub trait WithStateOps<Value, State> {
    type Wrapper<T>;

    fn drop_state(self) -> Self::Wrapper<Value>;

    fn drop_value(self) -> Self::Wrapper<State>;

    fn handle_state(self, f: impl FnOnce(State)) -> Self::Wrapper<Value>;

    fn handle_value(self, f: impl FnOnce(Value)) -> Self::Wrapper<State>;

    fn try_handle_state<E>(
        self,
        f: impl FnOnce(State) -> Result<(), E>,
    ) -> Self::Wrapper<Result<Value, E>>;

    fn try_handle_value<E>(
        self,
        f: impl FnOnce(Value) -> Result<(), E>,
    ) -> Self::Wrapper<Result<State, E>>;

    fn map_state<T>(self, f: impl FnOnce(State) -> T) -> Self::Wrapper<WithState<Value, T>>;
    fn map_value<T>(self, f: impl FnOnce(Value) -> T) -> Self::Wrapper<WithState<T, State>>;

    fn try_map_state<T, E>(
        self,
        f: impl FnOnce(State) -> Result<T, E>,
    ) -> Self::Wrapper<Result<WithState<Value, T>, E>>;
    fn try_map_value<T, E>(
        self,
        f: impl FnOnce(Value) -> Result<T, E>,
    ) -> Self::Wrapper<Result<WithState<T, State>, E>>;
}

impl<Value, State> WithStateOps<Value, State> for WithState<Value, State> {
    type Wrapper<T> = T;

    fn drop_state(self) -> Value {
        self.value
    }

    fn drop_value(self) -> State {
        self.state
    }

    fn handle_state(self, f: impl FnOnce(State)) -> Value {
        f(self.state);

        self.value
    }

    fn handle_value(self, f: impl FnOnce(Value)) -> State {
        f(self.value);

        self.state
    }

    fn try_handle_state<E>(self, f: impl FnOnce(State) -> Result<(), E>) -> Result<Value, E> {
        f(self.state)?;

        Ok(self.value)
    }

    fn try_handle_value<E>(self, f: impl FnOnce(Value) -> Result<(), E>) -> Result<State, E> {
        f(self.value)?;

        Ok(self.state)
    }

    fn map_state<T>(self, f: impl FnOnce(State) -> T) -> Self::Wrapper<WithState<Value, T>> {
        WithState {
            value: self.value,
            state: f(self.state),
        }
    }

    fn map_value<T>(self, f: impl FnOnce(Value) -> T) -> Self::Wrapper<WithState<T, State>> {
        WithState {
            value: f(self.value),
            state: self.state,
        }
    }

    fn try_map_state<T, E>(
        self,
        f: impl FnOnce(State) -> Result<T, E>,
    ) -> Self::Wrapper<Result<WithState<Value, T>, E>> {
        Ok(WithState {
            value: self.value,
            state: f(self.state)?,
        })
    }

    fn try_map_value<T, E>(
        self,
        f: impl FnOnce(Value) -> Result<T, E>,
    ) -> Self::Wrapper<Result<WithState<T, State>, E>> {
        Ok(WithState {
            value: f(self.value)?,
            state: self.state,
        })
    }
}

impl<Value, State, Error> WithStateOps<Value, State> for Result<WithState<Value, State>, Error> {
    type Wrapper<T> = Result<T, Error>;

    fn drop_state(self) -> Self::Wrapper<Value> {
        self.map(WithStateOps::drop_state)
    }

    fn drop_value(self) -> Self::Wrapper<State> {
        self.map(WithStateOps::drop_value)
    }

    fn handle_state(self, f: impl FnOnce(State)) -> Self::Wrapper<Value> {
        self.map(|v| v.handle_state(f))
    }

    fn handle_value(self, f: impl FnOnce(Value)) -> Self::Wrapper<State> {
        self.map(|v| v.handle_value(f))
    }

    fn try_handle_state<E>(
        self,
        f: impl FnOnce(State) -> Result<(), E>,
    ) -> Self::Wrapper<Result<Value, E>> {
        self.map(|v| v.try_handle_state(f))
    }

    fn try_handle_value<E>(
        self,
        f: impl FnOnce(Value) -> Result<(), E>,
    ) -> Self::Wrapper<Result<State, E>> {
        self.map(|v| v.try_handle_value(f))
    }

    fn map_state<T>(self, f: impl FnOnce(State) -> T) -> Self::Wrapper<WithState<Value, T>> {
        self.map(|v| v.map_state(f))
    }

    fn map_value<T>(self, f: impl FnOnce(Value) -> T) -> Self::Wrapper<WithState<T, State>> {
        self.map(|v| v.map_value(f))
    }

    fn try_map_state<T, E>(
        self,
        f: impl FnOnce(State) -> Result<T, E>,
    ) -> Self::Wrapper<Result<WithState<Value, T>, E>> {
        self.map(|v| v.try_map_state(f))
    }

    fn try_map_value<T, E>(
        self,
        f: impl FnOnce(Value) -> Result<T, E>,
    ) -> Self::Wrapper<Result<WithState<T, State>, E>> {
        self.map(|v| v.try_map_value(f))
    }
}

pub trait AddState<State>: Sized {
    type Output;
    fn with_state(self, state: State) -> Self::Output;
}

impl<State, Value> AddState<State> for Value {
    type Output = WithState<Self, State>;

    #[inline(always)]
    fn with_state(self, state: State) -> WithState<Self, State> {
        WithState { value: self, state }
    }
}

impl<Value, Error, State> From<WithState<Result<Value, Error>, State>>
    for Result<WithState<Value, State>, UnknownError<State, Error>>
{
    #[inline(always)]
    fn from(WithState { value, state }: WithState<Result<Value, Error>, State>) -> Self {
        match value {
            Err(error) => Err(UnknownError::new(state, error)),
            Ok(value) => Ok(value.with_state(state)),
        }
    }
}
