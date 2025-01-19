use miette::Diagnostic;

#[derive(
    Debug, thiserror::Error, Diagnostic, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash,
)]
#[error("Can't select from an empty population")]
#[diagnostic(
    help = "Add a minimum of one individual to your your population before trying to select"
)]
pub struct EmptyPopulation;
