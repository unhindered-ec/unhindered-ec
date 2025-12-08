use std::{cmp::Ordering, fmt::Display, iter::Sum};

/// A result of a single test, smaller is better.
///
/// See also [`ScoreValue`](super::score_value::ScoreValue), for which bigger is
/// better.
///
/// # Examples
/// ```
/// # use ec_core::performance::error_value::ErrorValue;
/// #
/// assert!(ErrorValue(5) < ErrorValue(-4));
/// ```
/// ```
/// # use ec_core::performance::error_value::ErrorValue;
/// #
/// assert!(ErrorValue(5) == ErrorValue(5));
/// ```
/// ```
/// # use ec_core::performance::error_value::ErrorValue;
/// #
/// assert!(ErrorValue(-100) > ErrorValue(-4));
/// ```
#[derive(Eq, PartialEq, Debug, Clone, Copy, Hash, Default)]
#[repr(transparent)]
pub struct ErrorValue<T>(pub T);

// We need `Error` to be cloneable in many of our applications,
// even if it's not needed here in `ec_core`. For `Error` to be
// cloneable, the generic type must also be cloneable.
static_assertions::assert_impl_all!(ErrorValue<()>: Clone);

impl<T: Ord> Ord for ErrorValue<T> {
    /// Compares two errors.
    ///
    /// Errors are ordered in reverse wrt
    /// [`ScoreValue<T>`](super::score_value::ScoreValue) because higher
    /// error values are considered worse.
    ///
    /// # Example
    /// ```
    /// # use ec_core::performance::error_value::ErrorValue;
    /// # use std::cmp::Ordering;
    /// #
    /// assert!(ErrorValue(100) < ErrorValue(10));
    /// assert!(ErrorValue(15) == ErrorValue(15));
    /// assert!(ErrorValue(20) > ErrorValue(1000));
    /// assert_eq!(ErrorValue(20).cmp(&ErrorValue(100)), Ordering::Greater);
    /// ```
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0).reverse()
    }
}

impl<T: PartialOrd> PartialOrd for ErrorValue<T> {
    /// Compares two errors.
    ///
    /// Errors are ordered in reverse wrt
    /// [`ScoreValue<T>`] because higher
    /// error values are considered worse.
    ///
    /// There are two distinct notions of error comparison:
    ///
    /// 1. **Fitness comparison (implemented here):**
    ///
    ///    When comparing two errors as a measure of genome fitness, a smaller
    ///    error means a better (greater) genome. Therefore two error values are
    ///    ordered such that this relation is respected.
    ///
    ///    This notion is also required
    ///    for selection operators to work properly with both [`ScoreValue`] and
    ///    [`ErrorValue`] values.
    /// 2. Numeric comparison:
    ///
    ///    When comparing errors with a scalar, usually numeric comparison is
    ///    desired, meaning we directly want to compare the value of the error
    ///    with the scalar.
    ///
    /// These two notions coincide for [`ScoreValue`] values, but diverge
    /// [`ErrorValue`] values.
    ///
    /// # Example
    /// ```
    /// # use ec_core::performance::error_value::ErrorValue;
    /// # use std::cmp::Ordering;
    /// #
    /// assert!(ErrorValue(100) < ErrorValue(10));
    /// assert!(ErrorValue(15) == ErrorValue(15));
    /// assert!(ErrorValue(20) > ErrorValue(1000));
    /// assert_eq!(
    ///     ErrorValue(20).partial_cmp(&ErrorValue(100)),
    ///     Some(Ordering::Greater)
    /// );
    /// ```
    /// [`ScoreValue<T>`]: super::score_value::ScoreValue
    /// [`ScoreValue`]: super::score_value::ScoreValue
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0
            .partial_cmp(&other.0)
            .map(std::cmp::Ordering::reverse)
    }
}

impl<T: PartialOrd> PartialOrd<T> for ErrorValue<T> {
    /// Compares the value of an error
    ///
    /// In contrast to comparing two errors, this is *not* ordered in reverse
    /// wrt to the errors value.
    ///
    /// There are two distinct notions of error comparison:
    ///
    /// 1. Fitness comparison:
    ///
    ///    When comparing two errors as a measure of genome fitness, a smaller
    ///    error means a better (greater) genome. Therefore two error values are
    ///    ordered such that this relation is respected.
    ///
    ///    This notion is also required
    ///    for selection operators to work properly with both [`ScoreValue`] and
    ///    [`ErrorValue`] values.
    /// 2. **Numeric comparison (implemented here):**
    ///
    ///    When comparing errors with a scalar, usually numeric comparison is
    ///    desired, meaning we directly want to compare the value of the error
    ///    with the scalar.
    ///
    /// These two notions coincide for [`ScoreValue`] values, but diverge
    /// [`ErrorValue`] values.
    ///
    /// # Example
    /// ```
    /// # use ec_core::performance::error_value::ErrorValue;
    /// # use std::cmp::Ordering;
    /// #
    /// assert!(ErrorValue(100) < 1000);
    /// assert!(ErrorValue(15) == 15);
    /// assert!(ErrorValue(10) > 1);
    /// assert_eq!(ErrorValue(20).partial_cmp(&100), Some(Ordering::Less));
    /// ```
    /// [`ScoreValue`]: super::score_value::ScoreValue
    fn partial_cmp(&self, other: &T) -> Option<Ordering> {
        self.0.partial_cmp(other)
    }
}

impl<T: PartialEq> PartialEq<T> for ErrorValue<T> {
    /// Checks the value of an error for equality
    ///
    /// # Example
    /// ```
    /// # use ec_core::performance::error_value::ErrorValue;
    /// #
    /// assert_eq!(ErrorValue(100), 100);
    /// assert_ne!(ErrorValue(10), 1);
    /// ```
    fn eq(&self, other: &T) -> bool {
        self.0.eq(other)
    }
}

impl<T: Display> Display for ErrorValue<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error (lower is better): {}", self.0)
    }
}
// TODO: Write tests for the `From` and `Sum` trait implementations.

impl<T> ErrorValue<T> {
    /// Create a new [`ErrorValue`] from the given value
    ///
    /// Also see [`ErrorValue::from`].
    ///
    /// # Example
    /// ```
    /// # use ec_core::performance::error_value::ErrorValue;
    /// #
    /// assert_eq!(ErrorValue::new(5), ErrorValue(5));
    /// ```
    #[must_use]
    pub const fn new(score: T) -> Self {
        Self(score)
    }
}

impl<T> From<T> for ErrorValue<T> {
    /// Create a new [`ErrorValue`] from the given value
    ///
    /// Also see [`ErrorValue::new`].
    ///
    /// # Example
    /// ```
    /// # use ec_core::performance::error_value::ErrorValue;
    /// #
    /// assert_eq!(ErrorValue::from(5), ErrorValue(5));
    /// ```
    fn from(error: T) -> Self {
        Self(error)
    }
}

impl<T: Sum> Sum<T> for ErrorValue<T> {
    /// Create a new [`ErrorValue`] from summing up an iterator of values.
    ///
    /// # Example
    /// ```
    /// # use ec_core::performance::error_value::ErrorValue;
    /// #
    /// assert_eq!(
    ///     [5, 8, -3, 10].into_iter().sum::<ErrorValue<_>>(),
    ///     ErrorValue(20)
    /// );
    /// ```
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = T>,
    {
        Self(iter.sum())
    }
}

impl<T: Sum> Sum for ErrorValue<T> {
    /// Sum up an iterator of [`ErrorValue`]'s.
    ///
    /// # Example
    /// ```
    /// # use ec_core::performance::error_value::ErrorValue;
    /// #
    /// assert_eq!(
    ///     [ErrorValue(5), ErrorValue(8), ErrorValue(-3), ErrorValue(10)]
    ///         .into_iter()
    ///         .sum::<ErrorValue<i32>>(),
    ///     ErrorValue(20)
    /// );
    /// ```
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.map(|s| s.0).sum()
    }
}

impl<'a, T> Sum<&'a Self> for ErrorValue<T>
where
    T: ToOwned,
    Self: Sum<<T as ToOwned>::Owned>,
{
    /// Sum up an iterator of references to [`ErrorValue`]'s.
    ///
    /// This implementation requires that `T` implement `ToOwned`. Note
    /// that `ToOwned ⊃ Clone ⊃ Copy`. Since `T` is usually a primitive
    /// numeric type it probably implements at least one of these traits.
    ///
    /// # Example
    /// ```
    /// # use ec_core::performance::error_value::ErrorValue;
    /// #
    /// assert_eq!(
    ///     [
    ///         &ErrorValue(5),
    ///         &ErrorValue(8),
    ///         &ErrorValue(-3),
    ///         &ErrorValue(10)
    ///     ]
    ///     .into_iter()
    ///     .sum::<ErrorValue<_>>(),
    ///     ErrorValue(20)
    /// );
    /// ```
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.map(|s| s.0.to_owned()).sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn error_smaller_is_better() {
        let first = ErrorValue(37);
        let second = ErrorValue(82);
        // These use `Ord`
        assert_eq!(first.cmp(&second), Ordering::Greater);
        assert_eq!(second.cmp(&first), Ordering::Less);
        assert_eq!(first.cmp(&first), Ordering::Equal);
        // Now use `PartialOrd`
        assert_eq!(first.partial_cmp(&second), Some(Ordering::Greater));
        assert_eq!(second.partial_cmp(&first), Some(Ordering::Less));
        assert_eq!(first.partial_cmp(&first), Some(Ordering::Equal));
    }
}
