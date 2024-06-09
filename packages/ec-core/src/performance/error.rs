use std::{
    cmp::Ordering,
    fmt::{Debug, Display},
    iter::Sum,
};

#[derive(Eq, PartialEq)]
pub struct ErrorValue<T> {
    pub error: T,
}

impl<T: Debug> Debug for ErrorValue<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self.error))
    }
}

impl<T: Ord> Ord for ErrorValue<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.error.cmp(&other.error).reverse()
    }
}

impl<T: PartialOrd> PartialOrd for ErrorValue<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.error
            .partial_cmp(&other.error)
            .map(std::cmp::Ordering::reverse)
    }
}

impl<T: Display> Display for ErrorValue<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error (lower is better): {}", self.error)
    }
}

impl<T> From<T> for ErrorValue<T> {
    fn from(error: T) -> Self {
        Self { error }
    }
}

impl<T: Sum> Sum<T> for ErrorValue<T> {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = T>,
    {
        Self { error: iter.sum() }
    }
}

impl<T: Sum> Sum for ErrorValue<T> {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.map(|s| s.error).sum()
    }
}

impl<'a, T> Sum<&'a Self> for ErrorValue<T>
where
    T: ToOwned,
    Self: Sum<<T as ToOwned>::Owned>,
{
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.map(|s| s.error.to_owned()).sum()
    }
}

#[cfg(test)]
mod error_tests {
    use super::*;

    #[test]
    fn error_smaller_is_better() {
        let first = ErrorValue { error: 37 };
        let second = ErrorValue { error: 82 };
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
