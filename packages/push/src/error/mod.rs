use self::{
    stateful::{FatalError, RecoverableError, StatefulError},
    try_recover::TryRecover,
};

pub mod into_state;
pub mod stateful;
pub mod try_recover;

#[derive(Debug)]
pub enum Error<E> {
    Recoverable(RecoverableError<E>),
    Fatal(FatalError<E>),
}

pub type InstructionResult<E> = core::result::Result<(), Error<E>>;

impl<E> Error<E> {
    pub fn fatal(error: impl Into<E>) -> Self {
        Self::Fatal(FatalError::new(error.into()))
    }

    pub fn recoverable(error: impl Into<E>) -> Self {
        Self::Recoverable(RecoverableError::new(error.into()))
    }

    pub const fn is_recoverable(&self) -> bool {
        matches!(self, Self::Recoverable(_))
    }

    pub const fn is_fatal(&self) -> bool {
        matches!(self, Self::Fatal(_))
    }

    pub const fn error(&self) -> &E {
        match self {
            Self::Recoverable(StatefulError { error, .. })
            | Self::Fatal(StatefulError { error, .. }) => error,
        }
    }

    pub fn map_inner_err<F, E1>(self, f: F) -> Error<E1>
    where
        F: FnOnce(E) -> E1,
    {
        match self {
            Self::Recoverable(RecoverableError { error, .. }) => {
                Error::Recoverable(RecoverableError::new(f(error)))
            }
            Self::Fatal(FatalError { error, .. }) => Error::Fatal(FatalError::new(f(error))),
        }
    }
}

impl<E, V> TryRecover<V> for Result<V, Error<E>>
where
    V: Default,
{
    type Error = FatalError<E>;

    fn try_recover(self) -> Result<V, FatalError<E>> {
        self.or_else(|err| match err {
            Error::Recoverable(_) => Ok(V::default()),
            Error::Fatal(error) => Err(error),
        })
    }
}

impl<E1, E2> From<RecoverableError<E1>> for Error<E2>
where
    E1: Into<E2>,
{
    fn from(value: RecoverableError<E1>) -> Self {
        Error::Recoverable(value).map_inner_err(Into::into)
    }
}

impl<E1, E2> From<FatalError<E1>> for Error<E2>
where
    E1: Into<E2>,
{
    fn from(value: FatalError<E1>) -> Self {
        Error::Fatal(value).map_inner_err(Into::into)
    }
}
