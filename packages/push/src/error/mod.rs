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

pub type InstructionResult<S, E> = core::result::Result<S, Error<S, E>>;

/// Maps a (presumably error) type into an `InstructionResult`.
///
/// This is used to convert `InstructionResult<S, E1>`
/// into `InstructionResult<S, E2>`, i.e. do `map_err()` on
/// the inner error types of an `InstructionResult`, preserving
/// the other fields in `Error`.
pub trait MapInstructionError<S, E> {
    ///
    /// # Errors
    ///
    /// This always returns an error type.
    fn map_err_into(self) -> InstructionResult<S, E>;
}

// MizardX@Twitch's initial suggestion here had `E2` as a generic on the
// _function_ `map_err_into()` instead of at the `impl` level. That provided
// some additional flexibility, although it wasn't clear that we would use it.
// The current approach (suggested by esitsu@Twitch) simplified the
// `MapInstructionError` trait in a nice way, so I went with that.
impl<S, E1, E2> MapInstructionError<S, E2> for InstructionResult<S, E1>
where
    E1: Into<E2>,
{
    fn map_err_into(self) -> InstructionResult<S, E2> {
        self.map_err(|e| e.map_inner_err(Into::into))
    }
}

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

    pub const fn state(&self) -> &S {
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

impl<S, E> TryRecover<S> for Result<S, Error<S, E>> {
    type Error = FatalError<S, E>;

    fn try_recover(self) -> Result<S, FatalError<S, E>> {
        self.or_else(|err| match err {
            Error::Recoverable(s) => Ok(s.into_state()),
            Error::Fatal(error) => Err(error),
        })
    }
}
