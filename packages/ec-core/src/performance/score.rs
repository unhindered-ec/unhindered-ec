use std::{
    fmt::{Debug, Display},
    iter::Sum,
};

/// Score implicitly follows a "bigger is better" model.
#[derive(Eq, PartialEq, Ord, PartialOrd, Clone)]
pub struct ScoreValue<T> {
    pub score: T,
}

impl<T: Debug> Debug for ScoreValue<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self.score))
    }
}

impl<T: Display> Display for ScoreValue<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Score (higher is better): {}", self.score)
    }
}

impl<T> From<T> for ScoreValue<T> {
    fn from(score: T) -> Self {
        Self { score }
    }
}

impl<T: Sum> Sum<T> for ScoreValue<T> {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = T>,
    {
        Self { score: iter.sum() }
    }
}

impl<T: Sum> Sum for ScoreValue<T> {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.map(|s| s.score).sum()
    }
}

impl<'a, T> Sum<&'a Self> for ScoreValue<T>
where
    T: ToOwned,
    Self: Sum<<T as ToOwned>::Owned>,
{
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.map(|s| s.score.to_owned()).sum()
    }
}

#[cfg(test)]
mod score_tests {
    use std::cmp::Ordering;

    use super::*;

    #[test]
    fn score_bigger_is_better() {
        let first = ScoreValue { score: 37 };
        let second = ScoreValue { score: 82 };
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
