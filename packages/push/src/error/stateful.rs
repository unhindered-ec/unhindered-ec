use std::{convert::Infallible, marker::PhantomData};

use super::{into_state::IntoState, try_recover::TryRecover};

mod private {
    use super::{Fatal, Recoverable};

    pub trait SealedMarker {}
    impl SealedMarker for Fatal {}
    impl SealedMarker for Recoverable {}
}

pub trait ErrorSeverity: private::SealedMarker {}

#[derive(Debug)]
pub struct Fatal;
impl ErrorSeverity for Fatal {}

#[derive(Debug)]
pub struct Recoverable;
impl ErrorSeverity for Recoverable {}

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

    pub const fn new_boxed(state: Box<S>, error: E) -> Self {
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

pub type FatalError<S, E> = StatefulError<S, E, Fatal>;
pub type RecoverableError<S, E> = StatefulError<S, E, Recoverable>;
