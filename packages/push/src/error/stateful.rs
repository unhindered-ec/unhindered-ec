use std::{
    convert::Infallible,
    error::Error,
    fmt::{Debug, Display},
    marker::PhantomData,
};

use miette::Diagnostic;

use super::{into_state::IntoState, try_recover::TryRecover};

mod private {
    use super::{Fatal, Recoverable};

    pub trait SealedMarker {}
    impl SealedMarker for Fatal {}
    impl SealedMarker for Recoverable {}
}

pub trait ErrorSeverity: private::SealedMarker {
    fn miette_severity() -> miette::Severity
    where
        Self: Sized;
}

static_assertions::assert_obj_safe!(ErrorSeverity);

#[derive(Debug, PartialEq, Eq)]
pub struct Fatal;
impl ErrorSeverity for Fatal {
    fn miette_severity() -> miette::Severity {
        miette::Severity::Error
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Recoverable;
impl ErrorSeverity for Recoverable {
    fn miette_severity() -> miette::Severity {
        miette::Severity::Advice
    }
}

#[derive(PartialEq, Eq)]
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

impl<S: Debug, E: Debug, Severity: ErrorSeverity> Debug for StatefulError<S, E, Severity> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StatefulError")
            .field("state", &self.state)
            .field("error", &self.error)
            .field("severity", &self._p)
            .finish()
    }
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

impl<S, E: Display, Severity: ErrorSeverity> Display for StatefulError<S, E, Severity> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.error.fmt(f)
    }
}

impl<S: Debug, E: Error, Severity: ErrorSeverity> Error for StatefulError<S, E, Severity> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.error.source()
    }
}

impl<S: Debug, E: Diagnostic, Severity: ErrorSeverity> Diagnostic
    for StatefulError<S, E, Severity>
{
    fn code<'a>(&'a self) -> Option<Box<dyn Display + 'a>> {
        self.error.code()
    }

    fn severity(&self) -> Option<miette::Severity> {
        Some(
            self.error
                .severity()
                .unwrap_or_else(Severity::miette_severity),
        )
    }

    fn help<'a>(&'a self) -> Option<Box<dyn Display + 'a>> {
        self.error.help()
    }

    fn url<'a>(&'a self) -> Option<Box<dyn Display + 'a>> {
        self.error.url()
    }

    fn source_code(&self) -> Option<&dyn miette::SourceCode> {
        self.error.source_code()
    }

    fn labels(&self) -> Option<Box<dyn Iterator<Item = miette::LabeledSpan> + '_>> {
        self.error.labels()
    }

    fn related<'a>(&'a self) -> Option<Box<dyn Iterator<Item = &'a dyn Diagnostic> + 'a>> {
        self.error.related()
    }

    fn diagnostic_source(&self) -> Option<&dyn Diagnostic> {
        self.error.diagnostic_source()
    }
}

pub type FatalError<S, E> = StatefulError<S, E, Fatal>;
pub type RecoverableError<S, E> = StatefulError<S, E, Recoverable>;
