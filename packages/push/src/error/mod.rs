use self::{
    into_state::IntoState,
    stateful::{FatalError, RecoverableError, StatefulError},
    try_recover::TryRecover,
};

pub mod into_state;
pub mod stateful;
pub mod try_recover;

#[derive(Debug)]
pub enum Error<S, E> {
    Recoverable(RecoverableError<S, E>),
    Fatal(FatalError<S, E>),
}

pub type InstructionResult<S, E> = core::result::Result<(), Error<S, E>>;

impl<S, E> Error<S, E> {
    pub fn fatal(state: S, error: impl Into<E>) -> Self {
        Self::Fatal(FatalError::new(state, error.into()))
    }

    pub fn recoverable(state: S, error: impl Into<E>) -> Self {
        Self::Recoverable(RecoverableError::new(state, error.into()))
    }

    pub const fn is_recoverable(&self) -> bool {
        matches!(self, Self::Recoverable(_))
    }

    pub const fn is_fatal(&self) -> bool {
        matches!(self, Self::Fatal(_))
    }

    pub fn state(&self) -> &S {
        match self {
            Self::Recoverable(StatefulError { state, .. })
            | Self::Fatal(StatefulError { state, .. }) => state,
        }
    }

    pub const fn error(&self) -> &E {
        match self {
            Self::Recoverable(StatefulError { error, .. })
            | Self::Fatal(StatefulError { error, .. }) => error,
        }
    }

    pub fn map_inner_err<F, E1>(self, f: F) -> Error<S, E1>
    where
        F: FnOnce(E) -> E1,
    {
        match self {
            Self::Recoverable(RecoverableError { state, error, .. }) => {
                Error::Recoverable(RecoverableError::new_boxed(state, f(error)))
            }
            Self::Fatal(FatalError { state, error, .. }) => {
                Error::Fatal(FatalError::new_boxed(state, f(error)))
            }
        }
    }
}

impl<S, E> IntoState<S> for Error<S, E> {
    fn into_state(self) -> S {
        match self {
            Self::Recoverable(StatefulError { state, .. })
            | Self::Fatal(StatefulError { state, .. }) => *state,
        }
    }
}

impl<S, E, V> TryRecover<S> for Result<S, Error<V, E>>
where
    S: Default,
{
    type Error = FatalError<V, E>;

    fn try_recover(self) -> Result<S, FatalError<V, E>> {
        self.or_else(|err| match err {
            Error::Recoverable(s) => Ok(S::default()),
            Error::Fatal(error) => Err(error),
        })
    }
}

impl<S, E> From<RecoverableError<S, E>> for Error<S, E> {
    fn from(value: RecoverableError<S, E>) -> Self {
        Self::Recoverable(value)
    }
}

impl<S, E> From<FatalError<S, E>> for Error<S, E> {
    fn from(value: FatalError<S, E>) -> Self {
        Self::Fatal(value)
    }
}
