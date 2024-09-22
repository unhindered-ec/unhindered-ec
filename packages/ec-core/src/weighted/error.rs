use miette::Diagnostic;

#[derive(Debug, thiserror::Error, Diagnostic, PartialEq, Eq)]
#[error("Overflow while trying to calculate the sum of the weights {0} and {1}.")]
pub struct WeightSumOverflow(pub u32, pub u32);

#[derive(Debug, PartialEq, Eq)]
pub enum WeightedSelectorsError<A, B> {
    A(A),
    B(B),

    ZeroWeightSum,
}

impl<A, B> std::error::Error for WeightedSelectorsError<A, B>
where
    A: std::error::Error + 'static,
    B: std::error::Error + 'static,
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::A(a) => Some(a),
            Self::B(b) => Some(b),
            Self::ZeroWeightSum => None,
        }
    }
}

impl<A, B> std::fmt::Display for WeightedSelectorsError<A, B>
where
    A: std::fmt::Display,
    B: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::A(a) => a.fmt(f),
            Self::B(b) => b.fmt(f),
            Self::ZeroWeightSum => f.write_str("Tried to select from a zero-weight selector"),
        }
    }
}

impl<A, B> miette::Diagnostic for WeightedSelectorsError<A, B>
where
    A: miette::Diagnostic + 'static,
    B: miette::Diagnostic + 'static,
{
    fn code<'a>(&'a self) -> Option<Box<dyn std::fmt::Display + 'a>> {
        match self {
            Self::A(a) => a.code(),
            Self::B(b) => b.code(),
            Self::ZeroWeightSum => None,
        }
    }

    fn severity(&self) -> Option<miette::Severity> {
        match self {
            Self::A(a) => a.severity(),
            Self::B(b) => b.severity(),
            Self::ZeroWeightSum => None,
        }
    }

    fn help<'a>(&'a self) -> Option<Box<dyn std::fmt::Display + 'a>> {
        match self {
            Self::A(a) => a.help(),
            Self::B(b) => b.help(),
            Self::ZeroWeightSum => {
                Some(Box::new("Selection requires at least one non-zero weight"))
            }
        }
    }

    fn url<'a>(&'a self) -> Option<Box<dyn std::fmt::Display + 'a>> {
        match self {
            Self::A(a) => a.url(),
            Self::B(b) => b.url(),
            Self::ZeroWeightSum => None,
        }
    }

    fn source_code(&self) -> Option<&dyn miette::SourceCode> {
        match self {
            Self::A(a) => a.source_code(),
            Self::B(b) => b.source_code(),
            Self::ZeroWeightSum => None,
        }
    }

    fn labels(&self) -> Option<Box<dyn Iterator<Item = miette::LabeledSpan> + '_>> {
        match self {
            Self::A(a) => a.labels(),
            Self::B(b) => b.labels(),
            Self::ZeroWeightSum => None,
        }
    }

    fn related<'a>(&'a self) -> Option<Box<dyn Iterator<Item = &'a dyn miette::Diagnostic> + 'a>> {
        match self {
            Self::A(a) => a.related(),
            Self::B(b) => b.related(),
            Self::ZeroWeightSum => None,
        }
    }

    fn diagnostic_source(&self) -> Option<&dyn miette::Diagnostic> {
        match self {
            Self::A(a) => a.diagnostic_source(),
            Self::B(b) => b.diagnostic_source(),
            Self::ZeroWeightSum => None,
        }
    }
}
