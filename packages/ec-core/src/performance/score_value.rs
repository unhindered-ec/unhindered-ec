use std::{cmp::Ordering, fmt::Display, iter::Sum};

/// Score implicitly follows a "bigger is better" model.
#[derive(Eq, PartialEq, Ord, PartialOrd, Debug, Clone, Copy, Hash, Default)]
#[repr(transparent)]
pub struct ScoreValue<T>(pub T);

// We need `Score` to be cloneable in many of our applications,
// even if it's not needed here in `ec_core`. For `Score` to be
// cloneable, the generic type must also be cloneable.
static_assertions::assert_impl_all!(ScoreValue<()>: Clone);

impl<T: Display> Display for ScoreValue<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Score (higher is better): {}", self.0)
    }
}

// TODO: Write tests for the `From` and `Sum` trait implementations.

impl<T> ScoreValue<T> {
    #[must_use]
    pub const fn new(score: T) -> Self {
        Self(score)
    }
}

impl<T: PartialOrd> PartialOrd<T> for ScoreValue<T> {
    fn partial_cmp(&self, other: &T) -> Option<Ordering> {
        self.0.partial_cmp(other)
    }
}

impl<T: PartialEq> PartialEq<T> for ScoreValue<T> {
    fn eq(&self, other: &T) -> bool {
        self.0.eq(other)
    }
}

impl<T> From<T> for ScoreValue<T> {
    fn from(score: T) -> Self {
        Self(score)
    }
}

impl<T: Sum> Sum<T> for ScoreValue<T> {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = T>,
    {
        Self(iter.sum())
    }
}

impl<T: Sum> Sum for ScoreValue<T> {
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
