use std::fmt::Display;

#[derive(Debug, Eq, PartialEq)]
pub enum MaybeKnown<T> {
    Known(T),
    Unknown,
}

impl<T> From<Option<T>> for MaybeKnown<T> {
    fn from(value: Option<T>) -> Self {
        value.map_or_else(|| Self::Unknown, |v| Self::Known(v))
    }
}

impl<T> From<MaybeKnown<T>> for Option<T> {
    fn from(value: MaybeKnown<T>) -> Self {
        value.into_option()
    }
}

impl<T> MaybeKnown<T> {
    pub fn into_option(self) -> Option<T> {
        match self {
            Self::Known(v) => Some(v),
            Self::Unknown => None,
        }
    }
}

impl<T> Display for MaybeKnown<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unknown => write!(f, "unknown"),
            Self::Known(t) => t.fmt(f),
        }
    }
}
