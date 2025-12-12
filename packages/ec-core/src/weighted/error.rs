use std::{
    error::Error,
    fmt::{self, Display},
};

use miette::{Diagnostic, Severity};

/// Error that occurs when creating a
/// [`WeightedPair`](super::weighted_pair::WeightedPair) from
/// weights whose sum overflows.
#[derive(
    Debug, thiserror::Error, Diagnostic, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash,
)]
#[error("Overflow while trying to calculate the sum of the weights {0} and {1}.")]
pub struct WeightSumOverflow(pub u32, pub u32);

/// Error that occurs when trying to use a 0-weight
/// [`Weighted`](super::Weighted) in a context which expects a non-zero weight,
/// for example selection
#[derive(
    Debug, thiserror::Error, Diagnostic, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash,
)]
#[error("Tried to choose from a structure with (total) weight zero.")]
#[diagnostic(help = "Choosing requires at least one non-zero weight")]
pub struct ZeroWeight;

/// Error that occurs when doing a fallible operation on a
/// [`WeightedPair`](super::weighted_pair::WeightedPair) (for example
/// selection).
///
/// This type simply wraps two error types and has either of them stored, it's
/// basically a specialized `Either<A,B>` type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WeightedPairError<A, B> {
    A(A),
    B(B),
}

impl<A, B> Error for WeightedPairError<A, B>
where
    A: Error + 'static,
    B: Error + 'static,
{
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::A(a) => Some(a),
            Self::B(b) => Some(b),
        }
    }
}

impl<A, B> Display for WeightedPairError<A, B>
where
    A: Display,
    B: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::A(a) => a.fmt(f),
            Self::B(b) => b.fmt(f),
        }
    }
}

impl<A, B> Diagnostic for WeightedPairError<A, B>
where
    A: Diagnostic + 'static,
    B: Diagnostic + 'static,
{
    fn code<'a>(&'a self) -> Option<Box<dyn Display + 'a>> {
        match self {
            Self::A(a) => a.code(),
            Self::B(b) => b.code(),
        }
    }

    fn severity(&self) -> Option<Severity> {
        match self {
            Self::A(a) => a.severity(),
            Self::B(b) => b.severity(),
        }
    }

    fn help<'a>(&'a self) -> Option<Box<dyn Display + 'a>> {
        match self {
            Self::A(a) => a.help(),
            Self::B(b) => b.help(),
        }
    }

    fn url<'a>(&'a self) -> Option<Box<dyn Display + 'a>> {
        match self {
            Self::A(a) => a.url(),
            Self::B(b) => b.url(),
        }
    }

    fn source_code(&self) -> Option<&dyn miette::SourceCode> {
        match self {
            Self::A(a) => a.source_code(),
            Self::B(b) => b.source_code(),
        }
    }

    fn labels(&self) -> Option<Box<dyn Iterator<Item = miette::LabeledSpan> + '_>> {
        match self {
            Self::A(a) => a.labels(),
            Self::B(b) => b.labels(),
        }
    }

    fn related<'a>(&'a self) -> Option<Box<dyn Iterator<Item = &'a dyn Diagnostic> + 'a>> {
        match self {
            Self::A(a) => a.related(),
            Self::B(b) => b.related(),
        }
    }

    fn diagnostic_source(&self) -> Option<&dyn Diagnostic> {
        match self {
            Self::A(a) => a.diagnostic_source(),
            Self::B(b) => b.diagnostic_source(),
        }
    }
}

/// Error that occurs when selecting from a [`Weighted`](super::Weighted) that
/// either has a zero weight or the underlying Selector errored.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SelectionError<E> {
    Selector(E),
    ZeroWeight(ZeroWeight),
}

impl<E> From<ZeroWeight> for SelectionError<E> {
    fn from(zero_weight: ZeroWeight) -> Self {
        Self::ZeroWeight(zero_weight)
    }
}

impl<E> Display for SelectionError<E>
where
    E: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Selector(e) => e.fmt(f),
            Self::ZeroWeight(_) => f.write_str("Tried to select from a selector with zero weight"),
        }
    }
}

impl<E> Error for SelectionError<E>
where
    E: Error + 'static,
{
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Selector(e) => Some(e),
            Self::ZeroWeight(zero_weight) => Some(zero_weight),
        }
    }
}

impl<E> Diagnostic for SelectionError<E>
where
    E: Diagnostic + 'static,
{
    fn code<'a>(&'a self) -> Option<Box<dyn Display + 'a>> {
        match self {
            Self::Selector(e) => e.code(),
            Self::ZeroWeight(zero_weight) => zero_weight.code(),
        }
    }

    fn severity(&self) -> Option<Severity> {
        match self {
            Self::Selector(e) => e.severity(),
            Self::ZeroWeight(zero_weight) => zero_weight.severity(),
        }
    }

    fn help<'a>(&'a self) -> Option<Box<dyn Display + 'a>> {
        match self {
            Self::Selector(e) => e.help(),
            Self::ZeroWeight(_) => Some(Box::new(
                "Ensure that the chosen (compound) selector you are selecting from has an option \
                 with a non-zero weight",
            )),
        }
    }

    fn url<'a>(&'a self) -> Option<Box<dyn Display + 'a>> {
        match self {
            Self::Selector(e) => e.url(),
            Self::ZeroWeight(zero_weight) => zero_weight.url(),
        }
    }

    fn source_code(&self) -> Option<&dyn miette::SourceCode> {
        match self {
            Self::Selector(e) => e.source_code(),
            Self::ZeroWeight(zero_weight) => zero_weight.source_code(),
        }
    }

    fn labels(&self) -> Option<Box<dyn Iterator<Item = miette::LabeledSpan> + '_>> {
        match self {
            Self::Selector(e) => e.labels(),
            Self::ZeroWeight(zero_weight) => zero_weight.labels(),
        }
    }

    fn related<'a>(&'a self) -> Option<Box<dyn Iterator<Item = &'a dyn Diagnostic> + 'a>> {
        match self {
            Self::Selector(e) => e.related(),
            Self::ZeroWeight(zero_weight) => zero_weight.related(),
        }
    }

    fn diagnostic_source(&self) -> Option<&dyn Diagnostic> {
        match self {
            Self::Selector(e) => e.diagnostic_source(),
            Self::ZeroWeight(zero_weight) => zero_weight.diagnostic_source(),
        }
    }
}
