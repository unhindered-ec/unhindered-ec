use std::{convert::Infallible, marker::PhantomData};

use super::try_recover::TryRecover;

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
pub struct StatefulError<E, Severity: ErrorSeverity> {
    pub(super) error: E,
    _p: PhantomData<Severity>,
}

impl<E, Severity: ErrorSeverity> StatefulError<E, Severity> {
    pub const fn new(error: E) -> Self {
        Self {
            error,
            _p: PhantomData,
        }
    }
}

impl<E, V> TryRecover<V> for Result<V, StatefulError<E, Recoverable>>
where
    V: Default,
{
    type Error = Infallible;

    fn try_recover(self) -> Result<V, Infallible> {
        Ok(self.unwrap_or_else(|_| V::default()))
    }
}

impl<E> StatefulError<E, Unknown> {
    fn make_fatal(self) -> StatefulError<E, Fatal> {
        StatefulError::new(self.error)
    }

    fn make_recoverable(self) -> StatefulError<E, Recoverable> {
        StatefulError::new(self.error)
    }
}

pub type FatalError<E> = StatefulError<E, Fatal>;
pub type RecoverableError<E> = StatefulError<E, Recoverable>;
pub type UnknownError<E> = StatefulError<E, Unknown>;

impl<E> From<E> for UnknownError<E> {
    fn from(value: E) -> Self {
        Self::new(value)
    }
}

pub trait SpecifySeverity<V, E> {
    type Output<Error>;

    fn make_fatal(self) -> Self::Output<FatalError<E>>;
    fn make_recoverable(self) -> Self::Output<RecoverableError<E>>;
}

impl<Value, Error, T> SpecifySeverity<Value, Error> for T
where
    T: Into<Result<Value, Error>>,
{
    type Output<E> = Result<Value, E>;

    fn make_fatal(self) -> Self::Output<FatalError<Error>> {
        self.into()
            .map_err(|err| UnknownError::from(err).make_fatal())
    }

    fn make_recoverable(self) -> Self::Output<RecoverableError<Error>> {
        self.into()
            .map_err(|err| UnknownError::from(err).make_recoverable())
    }
}
