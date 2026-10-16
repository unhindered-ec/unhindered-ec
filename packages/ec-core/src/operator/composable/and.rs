use std::{error::Error, fmt::Display};

use miette::Diagnostic;
use rand::Rng;

use super::{super::Operator, Composable};

/// A composable [`Operator`] applying two operators and returning a tuple of
/// results.
///
/// # Example
/// ```
/// # use ec_core::operator::{
/// #     Operator,
/// #     composable::{And},
/// #     constant::Constant
/// # };
/// #
/// let operator = And::new(Constant::new(1), Constant::new(2));
///
/// assert_eq!(operator.apply((), &mut rand::rng())?, (1, 2));
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Composable)]
pub struct And<F, G> {
    f: F,
    g: G,
}

impl<F, G> And<F, G> {
    /// Create a new [`And`] operator, applying two operators, producing two
    /// results
    ///
    /// # Example
    /// ```
    /// # use ec_core::operator::{
    /// #     Operator,
    /// #     composable::{And},
    /// #     constant::Constant
    /// # };
    /// #
    /// let operator_1 = Constant::new(1);
    /// let operator_2 = Constant::new(2);
    ///
    /// let operator = And::new(operator_1, operator_2);
    /// #
    /// # assert_eq!(operator.apply((), &mut rand::rng())?, (1, 2));
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub const fn new(f: F, g: G) -> Self {
        Self { f, g }
    }
}

/// Error that can occur when applying the [`And`] operator.
///
/// It's either an error from the first or the second operator, depending on
/// which failed.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum AndError<T, U> {
    /// The first operator failed with Error `T`
    First(T),
    /// The second operator failed with Error `T`
    Second(U),
}

// The implementation of Default here is somewhat arbitrary and more for
// actually having a value in e.g. a global state if it is required rather than
// for the actual value contained.
impl<T: Default, U> Default for AndError<T, U> {
    fn default() -> Self {
        Self::First(T::default())
    }
}

impl<T, U> Display for AndError<T, U> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::First(_) => f.write_str(
                "Error while applying the first passed operator (`T`) in the `And<T,>` Operator",
            ),
            Self::Second(_) => f.write_str(
                "Error while applying the second passed operator (`U`) in the `And<,U>` Operator",
            ),
        }
    }
}

impl<T, U> Error for AndError<T, U>
where
    T: Error + 'static,
    U: Error + 'static,
{
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::First(t) => Some(t),
            Self::Second(t) => Some(t),
        }
    }
}

impl<T, U> Diagnostic for AndError<T, U>
where
    T: Diagnostic + 'static,
    U: Diagnostic + 'static,
{
    fn diagnostic_source(&self) -> Option<&dyn Diagnostic> {
        match self {
            Self::First(t) => Some(t),
            Self::Second(t) => Some(t),
        }
    }
}

impl<A, F, G> Operator<A> for And<F, G>
where
    A: Clone,
    F: Operator<A>,
    G: Operator<A>,
{
    type Output = (F::Output, G::Output);
    type Error = AndError<F::Error, G::Error>;

    /// Apply the two [`Operator`]'s this [`And`] [`Operator`] is based on to
    /// the same input independently, cloning the input, returning a tuple
    /// of results.
    ///
    /// # Errors
    /// - [`AndError::First`] if the first operator failed, or
    /// - [`AndError::Second`] if the second operator failed.
    ///
    /// # Example
    /// ```
    /// # use ec_core::operator::{
    /// #     Operator,
    /// #     composable::{And},
    /// #     constant::Constant
    /// # };
    /// #
    /// let operator = And::new(Constant::new(1), Constant::new(2));
    ///
    /// let result = operator.apply((), &mut rand::rng())?;
    ///
    /// assert_eq!(result, (1, 2));
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    fn apply<R: Rng + ?Sized>(&self, x: A, rng: &mut R) -> Result<Self::Output, Self::Error> {
        let f_value = self.f.apply(x.clone(), rng).map_err(AndError::First)?;
        let g_value = self.g.apply(x, rng).map_err(AndError::Second)?;
        Ok((f_value, g_value))
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use rand::Rng;

    use super::{And, AndError};
    use crate::operator::{Composable, Operator, constant::Constant};

    #[derive(Debug, PartialEq, Eq)]
    struct MyError;

    impl std::fmt::Display for MyError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str("my error")
        }
    }

    impl Error for MyError {}

    /// An operator that always fails.
    struct FailAlways;

    impl Composable for FailAlways {}

    impl Operator<()> for FailAlways {
        type Output = i32;
        type Error = MyError;

        fn apply<R: Rng + ?Sized>(&self, _input: (), _rng: &mut R) -> Result<i32, MyError> {
            Err(MyError)
        }
    }

    #[test]
    fn both_succeed() {
        let result = And::new(Constant::new(1), Constant::new(2)).apply((), &mut rand::rng());
        assert_eq!(result.unwrap(), (1, 2));
    }

    #[test]
    fn first_failure_maps_to_first() {
        let result = And::new(FailAlways, Constant::new(2)).apply((), &mut rand::rng());
        assert!(matches!(result, Err(AndError::First(MyError))));
    }

    #[test]
    fn second_failure_maps_to_second() {
        let result = And::new(Constant::new(1), FailAlways).apply((), &mut rand::rng());
        assert!(matches!(result, Err(AndError::Second(MyError))));
    }

    #[test]
    fn error_source_is_the_inner_error() {
        let first: AndError<MyError, std::convert::Infallible> = AndError::First(MyError);
        assert_eq!(
            first
                .source()
                .and_then(|inner| inner.downcast_ref::<MyError>()),
            Some(&MyError)
        );

        let second: AndError<std::convert::Infallible, MyError> = AndError::Second(MyError);
        assert_eq!(
            second
                .source()
                .and_then(|inner| inner.downcast_ref::<MyError>()),
            Some(&MyError)
        );
    }

    #[test]
    fn display_distinguishes_the_failing_operator() {
        let first: AndError<MyError, std::convert::Infallible> = AndError::First(MyError);
        let second: AndError<std::convert::Infallible, MyError> = AndError::Second(MyError);
        assert_ne!(first.to_string(), second.to_string());
    }
}
