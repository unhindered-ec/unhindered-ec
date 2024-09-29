use std::{
    error::Error,
    fmt::{self, Display},
};

use miette::{Diagnostic, Severity};

#[derive(Debug, thiserror::Error, Diagnostic, PartialEq, Eq)]
#[error("Overflow while trying to calculate the sum of the weights {0} and {1}.")]
pub struct WeightSumOverflow(pub u32, pub u32);

#[derive(Debug, PartialEq, Eq)]
pub enum WeightedPairError<A, B> {
    A(A),
    B(B),

    ZeroWeight,
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
            Self::ZeroWeight => None,
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
            Self::ZeroWeight => f.write_str("Tried to choose from a zero-weight pair"),
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
            Self::ZeroWeight => None,
        }
    }

    fn severity(&self) -> Option<Severity> {
        match self {
            Self::A(a) => a.severity(),
            Self::B(b) => b.severity(),
            Self::ZeroWeight => None,
        }
    }

    fn help<'a>(&'a self) -> Option<Box<dyn Display + 'a>> {
        match self {
            Self::A(a) => a.help(),
            Self::B(b) => b.help(),
            Self::ZeroWeight => Some(Box::new("Choosing requires at least one non-zero weight")),
        }
    }

    fn url<'a>(&'a self) -> Option<Box<dyn Display + 'a>> {
        match self {
            Self::A(a) => a.url(),
            Self::B(b) => b.url(),
            Self::ZeroWeight => None,
        }
    }

    fn source_code(&self) -> Option<&dyn miette::SourceCode> {
        match self {
            Self::A(a) => a.source_code(),
            Self::B(b) => b.source_code(),
            Self::ZeroWeight => None,
        }
    }

    fn labels(&self) -> Option<Box<dyn Iterator<Item = miette::LabeledSpan> + '_>> {
        match self {
            Self::A(a) => a.labels(),
            Self::B(b) => b.labels(),
            Self::ZeroWeight => None,
        }
    }

    fn related<'a>(&'a self) -> Option<Box<dyn Iterator<Item = &'a dyn Diagnostic> + 'a>> {
        match self {
            Self::A(a) => a.related(),
            Self::B(b) => b.related(),
            Self::ZeroWeight => None,
        }
    }

    fn diagnostic_source(&self) -> Option<&dyn Diagnostic> {
        match self {
            Self::A(a) => a.diagnostic_source(),
            Self::B(b) => b.diagnostic_source(),
            Self::ZeroWeight => None,
        }
    }
}
