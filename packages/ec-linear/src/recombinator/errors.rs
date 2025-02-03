use miette::Diagnostic;

#[derive(Debug, thiserror::Error, Diagnostic)]
#[error("Attempted to perform TwoPointXo on genomes of different lengths {0} and {1}")]
#[diagnostic(help = "Ensure your genomes are of uniform length")]
pub struct DifferentGenomeLength(pub usize, pub usize);

#[derive(Debug, thiserror::Error, Diagnostic)]
pub enum CrossoverGeneError<E> {
    /// Attempted to crossover genomes with differing lengths
    #[error(transparent)]
    #[diagnostic(transparent)]
    DifferentGenomeLength(#[from] DifferentGenomeLength),
    /// Some other error specific to a crossover operation
    #[error("Failed to crossover segment")]
    Crossover(
        #[diagnostic_source]
        #[source]
        E,
    ),
}
