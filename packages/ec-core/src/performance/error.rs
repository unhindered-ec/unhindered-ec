use std::{
    cmp::Ordering,
    fmt::{Debug, Display},
    iter::Sum,
    num::Saturating,
    ops::{Add, Sub},
};

use super::score::ScoreValue;

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

/// Provide saturating addition combining errors of type
/// `U` and scores of type `T`, returning an error of
/// type U. This effectively negates the score value (turning
/// it into a kind of error, where small is good) and
/// adds it to the error.
impl<T, U> Add<ScoreValue<T>> for ErrorValue<U>
where
    T: Into<U>,
    Saturating<U>: Sub<Saturating<U>, Output = Saturating<U>>,
{
    type Output = Self;

    /// Saturating addition combining values errors of type
    /// `U` and scores of type `T`, returning an error of type
    /// `U`. This effectively negates the score value (turning
    /// it into a kind of error, where small is good) and
    /// adds it to the error.
    #[allow(clippy::suspicious_arithmetic_impl)]
    fn add(self, rhs: ScoreValue<T>) -> Self::Output {
        Self((Saturating(self.0) - Saturating(rhs.score.into())).0)
    }
}

/// Provide saturating addition combining errors of type
/// `U` and references to scores of type `T`, returning an error of
/// type U. This effectively negates the score value (turning
/// it into a kind of error, where small is good) and
/// adds it to the error.
impl<'a, T, U> Add<&'a ScoreValue<T>> for ErrorValue<U>
where
    T: Clone,
    Self: Sub<ScoreValue<T>, Output = Self>,
{
    type Output = Self;

    /// Saturating addition combining values errors of type
    /// `U` and references to scores of type `T`, returning an error of type
    /// `U`. This effectively negates the score value (turning
    /// it into a kind of error, where small is good) and
    /// adds it to the error.
    // This subtraction will use the implementation of `Add<ErrorValue<T>>`
    // which guarantees saturating addition, so we can ignore this warning
    // here.
    #[allow(clippy::arithmetic_side_effects)]
    // Clippy is worried that we're using `-` in an implementation of `Add`,
    // but that's what we want here.
    #[allow(clippy::suspicious_arithmetic_impl)]
    fn add(self, rhs: &'a ScoreValue<T>) -> Self::Output {
        self - rhs.clone()
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

// Attempting to replace the previous three `Sum` implementation
// blocks with the following `impl` breaks the world because
// floating point values don't saturate. If we just keep the
// preceding ones, though, then we don't get saturation. Sighz.

// impl<'a, T, U> Sum<&'a ErrorValue<T>> for ErrorValue<U>
// where
//     for<'b> Self: Add<&'b ErrorValue<T>, Output = Self> + Default,
// {
//     fn sum<I: Iterator<Item = &'a ErrorValue<T>>>(iter: I) -> Self {
//         iter.fold(Self::default(), Add::add)
//     }
// }

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

    #[test]
    fn addition_saturates() {
        let first = ErrorValue(i32::MAX);
        let second = ErrorValue(18);
        let sum = first + second;
        assert_eq!(sum, i32::MAX);

        let sum = second + first;
        assert_eq!(sum, i32::MAX);
    }

    #[test]
    fn adding_score_to_negative_value_saturates() {
        let first = ErrorValue(-18);
        let second = ScoreValue { score: i32::MAX };
        let sum = first + second;
        assert_eq!(sum, i32::MIN);
    }
}
