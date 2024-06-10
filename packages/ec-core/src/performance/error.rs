use std::{
    cmp::Ordering,
    fmt::{Debug, Display},
    iter::Sum,
    num::Saturating,
    ops::Add,
};

/// A performance measure where smaller is better
///
/// ## Addition
///
/// We implement both `Add` and `Sum`, allowing us
/// to effectively add U + T returning a U, and
/// summing a collection of T to get a U. This summation saturates,
/// so we we reach `::MIN` or `::MAX` when summing
/// errors then we will remain there instead of
/// overflowing or wrapping.
///
/// We also implement `Sub` so that we can "subtract"
/// a `ScoreValue` from an `ErrorValue`, effectively
/// flipping the sign on the `ScoreValue` so that smaller
/// becomes better.
#[derive(Default, Copy, Clone, Eq, PartialEq)]
pub struct ErrorValue<T>(pub T);

impl<T: Debug> Debug for ErrorValue<T> {
    // We only want the wrapped value displayed.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

impl<T: Display> Display for ErrorValue<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error (lower is better): {}", self.0)
    }
}

impl<T: PartialEq> PartialEq<T> for ErrorValue<T> {
    fn eq(&self, other: &T) -> bool {
        self.0.eq(other)
    }
}

impl<T: Ord> Ord for ErrorValue<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse is necessary since we want small to be "higher"/"better".
        self.0.cmp(&other.0).reverse()
    }
}

impl<T: PartialOrd> PartialOrd for ErrorValue<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // Reverse is necessary since we want small to be "higher"/"better".
        self.0
            .partial_cmp(&other.0)
            .map(std::cmp::Ordering::reverse)
    }
}

impl<T> From<T> for ErrorValue<T> {
    fn from(error: T) -> Self {
        Self(error)
    }
}

/// Provide saturating addition combining values of type
/// `U` and `T`.
impl<T, U> Add<ErrorValue<T>> for ErrorValue<U>
where
    T: Into<U>,
    Saturating<U>: Add<Saturating<U>, Output = Saturating<U>>,
{
    type Output = Self;

    /// Saturating addition combining values of type
    /// `U` and `T`.
    fn add(self, rhs: ErrorValue<T>) -> Self::Output {
        Self((Saturating(self.0) + Saturating(rhs.0.into())).0)
    }
}

/// Provide saturating addition combining values of type
/// `U` and `&T`.
impl<'a, T, U> Add<&'a ErrorValue<T>> for ErrorValue<U>
where
    T: Clone,
    Self: Add<ErrorValue<T>, Output = Self>,
{
    type Output = Self;

    /// Saturating addition combining values of type
    /// `U` and `&T`.
    // This addition will use the implementation of `Add<ErrorValue<T>>`
    // which guarantees saturating addition, so we can ignore this warning
    // here.
    #[allow(clippy::arithmetic_side_effects)]
    fn add(self, rhs: &'a ErrorValue<T>) -> Self::Output {
        self + rhs.clone()
    }
}

/// Provide saturating summation combining values of type
/// `T` into a sum of type `ErrorValue<T>`.
impl<T: Sum> Sum<T> for ErrorValue<T> {
    /// Saturating summation combining values of type
    /// `T` into a sum of type `ErrorValue<T>`.
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = T>,
    {
        Self(iter.sum())
    }
}

/// Provide saturating summation combining values of type
/// `ErrorValue<T>` into a sum of type `ErrorValue<T>`.
impl<T: Sum> Sum for ErrorValue<T> {
    /// Saturating summation combining values of type
    /// `ErrorValue<T>` into a sum of type `ErrorValue<T>`.
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.map(|s| s.0).sum()
    }
}

/// Provide saturating summation combining values of type
/// `&ErrorValue<T>` into a sum of type `ErrorValue<T>`.
impl<'a, T> Sum<&'a Self> for ErrorValue<T>
where
    T: ToOwned,
    Self: Sum<<T as ToOwned>::Owned>,
{
    /// Saturating summation combining values of type
    /// `&ErrorValue<T>` into a sum of type `ErrorValue<T>`.
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.map(|s| s.0.to_owned()).sum()
    }
}

#[cfg(test)]
mod error_tests {
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
