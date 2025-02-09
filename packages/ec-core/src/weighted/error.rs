use miette::Diagnostic;

#[derive(Debug, thiserror::Error, Diagnostic, PartialEq, Eq)]
#[error("Overflow while trying to calculate the sum of the weights {0} and {1}.")]
pub struct WeightSumOverflow(pub u32, pub u32);

#[derive(Debug, thiserror::Error, Diagnostic, PartialEq, Eq)]
#[error("Tried to choose from a structure with (total) weight zero.")]
#[diagnostic(help = "Choosing requires at least one non-zero weight")]
pub struct ZeroWeight;

#[derive(Debug, thiserror::Error, Diagnostic, PartialEq, Eq)]
pub enum WeightedPairError<A, B> {
    #[error(transparent)]
    #[diagnostic(transparent)]
    A(A),
    #[error(transparent)]
    #[diagnostic(transparent)]
    B(B),
}

#[derive(Debug, thiserror::Error, Diagnostic, PartialEq, Eq)]
pub enum SelectionError<E> {
    #[error(transparent)]
    #[diagnostic(transparent)]
    Selector(E),
    #[error("Tried to select from a selector with zero weight")]
    #[diagnostic(
        help = "Ensure that the chosen (compound) selector you are selecting from has an option \
                with a non-zero weight"
    )]
    ZeroWeight(
        #[diagnostic_source]
        #[from]
        ZeroWeight,
    ),
}
