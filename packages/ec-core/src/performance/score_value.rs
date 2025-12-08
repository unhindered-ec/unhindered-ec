use std::{cmp::Ordering, fmt::Display, iter::Sum};

/// A result of a single test, bigger is better.
///
/// See also [`ErrorValue`], for which smaller
/// is better.
///
/// # Examples
/// ```
/// # use ec_core::performance::score_value::ScoreValue;
/// #
/// assert!(ScoreValue(5) > ScoreValue(-4));
/// ```
/// ```
/// # use ec_core::performance::score_value::ScoreValue;
/// #
/// assert!(ScoreValue(5) == ScoreValue(5));
/// ```
/// ```
/// # use ec_core::performance::score_value::ScoreValue;
/// #
/// assert!(ScoreValue(-100) < ScoreValue(-4));
/// ```
/// [`ErrorValue`]: super::error_value::ErrorValue
#[derive(Eq, PartialEq, Ord, PartialOrd, Debug, Clone, Copy, Hash, Default)]
#[repr(transparent)]
pub struct ScoreValue<T>(pub T);

// We need `ScoreValue` to be cloneable in many of our applications,
// even if it's not needed here in `ec_core`. For `ScoreValue` to be
// cloneable, the generic type must also be cloneable.
static_assertions::assert_impl_all!(ScoreValue<()>: Clone);

impl<T: Display> Display for ScoreValue<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Score (higher is better): {}", self.0)
    }
}

// TODO: Write tests for the `From` and `Sum` trait implementations.

impl<T> ScoreValue<T> {
    /// Create a new [`ScoreValue`] from the given value
    ///
    /// Also see [`ScoreValue::from`].
    ///
    /// # Example
    /// ```
    /// # use ec_core::performance::score_value::ScoreValue;
    /// #
    /// assert_eq!(ScoreValue::new(5), ScoreValue(5));
    /// ```
    #[must_use]
    pub const fn new(score: T) -> Self {
        Self(score)
    }
}

impl<T: PartialOrd> PartialOrd<T> for ScoreValue<T> {
    /// Compares the value of a score
    ///
    /// There are two distinct notions of score comparison:
    ///
    /// 1. Fitness comparison:
    ///
    ///    When comparing two scores as a measure of genome fitness, a greater
    ///    score means a better (greater) genome. Therefore two score values are
    ///    ordered such that this relation is respected.
    ///
    ///    This notion is also required
    ///    for selection operators to work properly with both [`ScoreValue`] and
    ///    [`ErrorValue`] values.
    /// 2. **Numeric comparison (implemented here):**
    ///
    ///    When comparing scores with a scalar, usually numeric comparison is
    ///    desired, meaning we directly want to compare the value of the score
    ///    with the scalar.
    ///
    /// These two notions coincide for [`ScoreValue`] values, but diverge
    /// [`ErrorValue`] values.
    ///
    /// # Example
    /// ```
    /// # use ec_core::performance::score_value::ScoreValue;
    /// # use std::cmp::Ordering;
    /// #
    /// assert!(ScoreValue(100) < 1000);
    /// assert!(ScoreValue(15) == 15);
    /// assert!(ScoreValue(10) > 1);
    /// assert_eq!(ScoreValue(20).partial_cmp(&100), Some(Ordering::Less));
    /// ```
    /// [`ErrorValue`]: super::error_value::ErrorValue
    fn partial_cmp(&self, other: &T) -> Option<Ordering> {
        self.0.partial_cmp(other)
    }
}

impl<T: PartialEq> PartialEq<T> for ScoreValue<T> {
    /// Checks the value of a score for equality
    ///
    /// # Example
    /// ```
    /// # use ec_core::performance::score_value::ScoreValue;
    /// #
    /// assert_eq!(ScoreValue(100), 100);
    /// assert_ne!(ScoreValue(10), 1);
    /// ```
    fn eq(&self, other: &T) -> bool {
        self.0.eq(other)
    }
}

impl<T> From<T> for ScoreValue<T> {
    /// Create a new [`ScoreValue`] from the given value
    ///
    /// Also see [`ScoreValue::new`].
    ///
    /// # Example
    /// ```
    /// # use ec_core::performance::score_value::ScoreValue;
    /// #
    /// assert_eq!(ScoreValue::from(5), ScoreValue(5));
    /// ```
    fn from(score: T) -> Self {
        Self(score)
    }
}

impl<T: Sum> Sum<T> for ScoreValue<T> {
    /// Create a new [`ScoreValue`] from summing up an iterator of values.
    ///
    /// # Example
    /// ```
    /// # use ec_core::performance::score_value::ScoreValue;
    /// #
    /// assert_eq!(
    ///     [5, 8, -3, 10].into_iter().sum::<ScoreValue<_>>(),
    ///     ScoreValue(20)
    /// );
    /// ```
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = T>,
    {
        Self(iter.sum())
    }
}

impl<T: Sum> Sum for ScoreValue<T> {
    /// Sum up an iterator of [`ScoreValue`]'s.
    ///
    /// # Example
    /// ```
    /// # use ec_core::performance::score_value::ScoreValue;
    /// #
    /// assert_eq!(
    ///     [ScoreValue(5), ScoreValue(8), ScoreValue(-3), ScoreValue(10)]
    ///         .into_iter()
    ///         .sum::<ScoreValue<i32>>(),
    ///     ScoreValue(20)
    /// );
    /// ```
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.map(|s| s.0).sum()
    }
}

impl<'a, T> Sum<&'a Self> for ScoreValue<T>
where
    T: ToOwned,
    Self: Sum<<T as ToOwned>::Owned>,
{
    /// Sum up an iterator of references to [`ScoreValue`]'s.
    ///
    /// This implementation requires that `T` implement `ToOwned`. Note
    /// that `ToOwned ⊃ Clone ⊃ Copy`. Since `T` is usually a primitive
    /// numeric type it probably implements at least one of these traits.
    ///
    /// # Example
    /// ```
    /// # use ec_core::performance::score_value::ScoreValue;
    /// #
    /// assert_eq!(
    ///     [
    ///         &ScoreValue(5),
    ///         &ScoreValue(8),
    ///         &ScoreValue(-3),
    ///         &ScoreValue(10)
    ///     ]
    ///     .into_iter()
    ///     .sum::<ScoreValue<_>>(),
    ///     ScoreValue(20)
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
    fn score_bigger_is_better() {
        let first = ScoreValue(37);
        let second = ScoreValue(82);
        // These use `Ord`
        assert_eq!(first.cmp(&second), Ordering::Less);
        assert_eq!(second.cmp(&first), Ordering::Greater);
        assert_eq!(first.cmp(&first), Ordering::Equal);
        // Now use `PartialOrd`
        assert_eq!(first.partial_cmp(&second), Some(Ordering::Less));
        assert_eq!(second.partial_cmp(&first), Some(Ordering::Greater));
        assert_eq!(first.partial_cmp(&first), Some(Ordering::Equal));
    }
}
