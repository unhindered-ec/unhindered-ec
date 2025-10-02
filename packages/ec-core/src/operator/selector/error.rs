use miette::Diagnostic;

/// Error that occurs in various selectors when trying to select from a empty
/// population
#[derive(
    Debug, thiserror::Error, Diagnostic, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash,
)]
#[error("Can't select from an empty population")]
#[diagnostic(
    help = "Add a minimum of one individual to your your population before trying to select"
)]
pub struct EmptyPopulation;
