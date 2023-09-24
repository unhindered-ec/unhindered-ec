use std::{convert::Infallible, marker::PhantomData};

use crate::push_vm::state::with_state::WithState;

use super::{into_state::IntoState, try_recover::TryRecover};

mod private {
    use super::{Fatal, Recoverable, Unknown};

    pub trait SealedMarker {}
    impl SealedMarker for Fatal {}
    impl SealedMarker for Recoverable {}
    impl SealedMarker for Unknown {}
}

pub trait ErrorSeverity: private::SealedMarker {}

#[derive(Debug)]
pub struct Fatal;
impl ErrorSeverity for Fatal {}

#[derive(Debug)]
pub struct Recoverable;
impl ErrorSeverity for Recoverable {}

#[derive(Debug)]
pub struct Unknown;
impl ErrorSeverity for Unknown {}

#[derive(Debug)]
pub struct StatefulError<S, E, Severity: ErrorSeverity> {
    // Without the `Box` the size of this Error ended up being 156 bytes
    // with a `PushState` and a `PushInstructionError`. That led to a Clippy
    // warning (https://rust-lang.github.io/rust-clippy/master/index.html#/result_large_err)
    // our `Error` was then larger than the 128 byte limit. They recommended boxing
    // the big piece (the state in our case), and doing that brought the size down to
    // 40 bytes. Since `Error`s are only constructed through `::fatal()` or `::recoverable()`,
    // we'd nicely encapsulated this and only had to make changes in those two places to
    // get things working.
    pub(super) state: Box<S>,
    pub(super) error: E,
    _p: PhantomData<Severity>,
}

impl<S, E, Severity: ErrorSeverity> StatefulError<S, E, Severity> {
    pub fn new(state: S, error: E) -> Self {
        Self::new_boxed(Box::new(state), error)
    }

    pub fn new_boxed(state: Box<S>, error: E) -> Self {
        Self {
            state,
            error,
            _p: PhantomData,
        }
    }
}

impl<S, E, Severity: ErrorSeverity> IntoState<S> for StatefulError<S, E, Severity> {
    fn into_state(self) -> S {
        *self.state
    }
}

impl<S, E> TryRecover<S> for Result<S, StatefulError<S, E, Recoverable>> {
    type Error = Infallible;

    fn try_recover(self) -> Result<S, Infallible> {
        Ok(self.unwrap_or_else(IntoState::into_state))
    }
}

impl<S, E> StatefulError<S, E, Unknown> {
    fn make_fatal(self) -> StatefulError<S, E, Fatal> {
        StatefulError::new_boxed(self.state, self.error)
    }

    fn make_recoverable(self) -> StatefulError<S, E, Recoverable> {
        StatefulError::new_boxed(self.state, self.error)
    }
}

pub type FatalError<S, E> = StatefulError<S, E, Fatal>;
pub type RecoverableError<S, E> = StatefulError<S, E, Recoverable>;
pub type UnknownError<S, E> = StatefulError<S, E, Unknown>;

impl<E, S> From<WithState<E, S>> for UnknownError<S, E> {
    fn from(value: WithState<E, S>) -> Self {
        UnknownError::new(value.state, value.value)
    }
}

pub trait SpecifySeverity<V, S, E> {
    type Output<Error>;

    fn make_fatal(self) -> Self::Output<FatalError<S, E>>;
    fn make_recoverable(self) -> Self::Output<RecoverableError<S, E>>;
}

impl<Value, State, Error, T> SpecifySeverity<Value, State, Error> for T
where
    T: Into<Result<Value, UnknownError<State, Error>>>,
{
    type Output<E> = Result<Value, E>;

    fn make_fatal(self) -> Self::Output<FatalError<State, Error>> {
        self.into().map_err(|err| err.make_fatal().into())
    }

    fn make_recoverable(self) -> Self::Output<RecoverableError<State, Error>> {
        self.into().map_err(|err| err.make_recoverable().into())
    }
}
