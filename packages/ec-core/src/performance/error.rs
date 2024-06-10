use std::{
    cmp::Ordering,
    fmt::{Debug, Display},
    iter::Sum,
};

/// A performance measure where smaller is better
///
/// ## Addition
///
/// We implement both `Add` and `Sum`, allowing us
/// to effectively add U + T returning a U, and
/// summing a collection of T to get a U.
///
/// We also implement `Sub` so that we can "subtract"
/// a `ScoreValue` from an `ErrorValue`, effectively
/// flipping the sign on the `ScoreValue` so that smaller
/// becomes better.
#[derive(Default, Clone, Eq, PartialEq)]
pub struct ErrorValue<T>(pub T);

impl<T: Debug> Debug for ErrorValue<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self.0))
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
        self.0.cmp(&other.0).reverse()
    }
}

impl<T: PartialOrd> PartialOrd for ErrorValue<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
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

impl<T: Sum> Sum<T> for ErrorValue<T> {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = T>,
    {
        Self(iter.sum())
    }
}

impl<T: Sum> Sum for ErrorValue<T> {
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
