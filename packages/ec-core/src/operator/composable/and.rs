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
#[derive(Debug, Composable, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AndError<T, U> {
    First(T),
    Second(U),
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
    /// the input in parrallel, cloning the input, returning a tuple of results.
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
