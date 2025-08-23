use std::{
    error::Error,
    fmt::{Debug, Display},
};

use miette::{Diagnostic, LabeledSpan, Severity, SourceCode};

#[derive(Debug, thiserror::Error, Diagnostic)]
#[error("Attempted to perform TwoPointXo on genomes of different lengths {0} and {1}")]
#[diagnostic(help = "Ensure your genomes are of uniform length")]
pub struct DifferentGenomeLength(pub usize, pub usize);

#[derive(Debug)]
pub enum CrossoverGeneError<E> {
    /// Attempted to crossover genomes with differing lengths
    DifferentGenomeLength(DifferentGenomeLength),
    /// Some other error specific to a crossover operation
    Crossover(E),
}

// We need to hand implement all these traits because `derive` for
// `thiserror::Error` and `miette::Diagnostic` don't
// handle generics well in this context. Hopefully that will be fixed in
// the future and we can simplify this considerably.

impl<E> Error for CrossoverGeneError<E>
where
    E: Error + 'static,
    Self: Debug + Display,
{
    fn source(&self) -> ::core::option::Option<&(dyn Error + 'static)> {
        match self {
            Self::DifferentGenomeLength(transparent) => Error::source(transparent),
            Self::Crossover(source) => Some(source),
        }
    }
}
impl<E> Display for CrossoverGeneError<E> {
    fn fmt(&self, formatter: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            Self::DifferentGenomeLength(g) => Display::fmt(&g, formatter),
            Self::Crossover(_) => formatter.write_str("Failed to crossover segment"),
        }
    }
}
impl<E> From<DifferentGenomeLength> for CrossoverGeneError<E> {
    fn from(source: DifferentGenomeLength) -> Self {
        Self::DifferentGenomeLength(source)
    }
}
impl<E> Diagnostic for CrossoverGeneError<E>
where
    E: Error + Diagnostic + 'static,
{
    fn code(&self) -> Option<Box<dyn Display + '_>> {
        match self {
            Self::DifferentGenomeLength(unnamed, ..) => unnamed.code(),
            Self::Crossover(unnamed, ..) => unnamed.code(),
        }
    }
    fn help(&self) -> Option<Box<dyn Display + '_>> {
        match self {
            Self::DifferentGenomeLength(unnamed, ..) => unnamed.help(),
            Self::Crossover(unnamed, ..) => unnamed.help(),
        }
    }
    fn severity(&self) -> Option<Severity> {
        match self {
            Self::DifferentGenomeLength(unnamed, ..) => unnamed.severity(),
            Self::Crossover(unnamed, ..) => unnamed.severity(),
        }
    }
    fn labels(&self) -> Option<Box<dyn Iterator<Item = LabeledSpan> + '_>> {
        match self {
            Self::DifferentGenomeLength(unnamed, ..) => unnamed.labels(),
            Self::Crossover(unnamed, ..) => unnamed.labels(),
        }
    }
    fn source_code(&self) -> Option<&dyn SourceCode> {
        match self {
            Self::DifferentGenomeLength(unnamed, ..) => unnamed.source_code(),
            Self::Crossover(unnamed, ..) => unnamed.source_code(),
        }
    }
    fn related(&self) -> Option<Box<dyn Iterator<Item = &dyn Diagnostic> + '_>> {
        match self {
            Self::DifferentGenomeLength(unnamed, ..) => unnamed.related(),
            Self::Crossover(unnamed, ..) => unnamed.related(),
        }
    }
    fn url(&self) -> Option<Box<dyn Display + '_>> {
        match self {
            Self::DifferentGenomeLength(unnamed, ..) => unnamed.url(),
            Self::Crossover(unnamed, ..) => unnamed.url(),
        }
    }
    fn diagnostic_source(&self) -> Option<&dyn Diagnostic> {
        match self {
            Self::DifferentGenomeLength(unnamed, ..) => unnamed.diagnostic_source(),
            Self::Crossover(unnamed, ..) => unnamed.diagnostic_source(),
        }
    }
}

#[derive(Debug, thiserror::Error, Diagnostic)]
#[error("Genome access failed for genome of type {genome_type} with size {size} at {index:?}")]
#[diagnostic(
    help = "Ensure that your indices {index:?} are legal, i.e., within the range 0..{size}"
)]
pub struct GeneAccess<Index>
where
    Index: Debug,
{
    pub index: Index,
    pub size: usize,
    genome_type: &'static str,
}

impl<Index> GeneAccess<Index>
where
    Index: Debug,
{
    /// Changes the contained `genome_type` captured for the
    /// error message to another genome type. This is necessary
    /// when the actual type being recombined (e.g., `Vec<bool>`)
    /// is wrapped in a container type like `Bitstring`.
    pub(crate) fn for_genome_type<NewGenome>(mut self) -> Self {
        self.genome_type = std::any::type_name::<NewGenome>();
        self
    }
}

impl<Index: Debug> GeneAccess<Index> {
    pub fn new<Genome>(index: Index, size: usize) -> Self {
        Self {
            index,
            size,
            genome_type: std::any::type_name::<Genome>(),
        }
    }
}

#[derive(Debug, thiserror::Error, Diagnostic)]
pub enum MultipleGeneAccess<Index: Debug + 'static> {
    #[error("Gene access on the lhs genome (self) failed")]
    Lhs(
        #[source]
        #[diagnostic_source]
        GeneAccess<Index>,
    ),
    #[error("Gene access on the rhs genome (other) failed")]
    Rhs(
        #[source]
        #[diagnostic_source]
        GeneAccess<Index>,
    ),
    #[error("Gene access on both genomes, lhs (self) and rhs (other), failed")]
    Both {
        #[source]
        #[diagnostic_source]
        lhs: GeneAccess<Index>,
        rhs: GeneAccess<Index>,
    },
}

impl<Index> MultipleGeneAccess<Index>
where
    Index: Debug,
{
    pub(crate) fn lhs<Genome>(index: Index, size: usize) -> Self {
        Self::Lhs(GeneAccess::new::<Genome>(index, size))
    }

    pub(crate) fn rhs<Genome>(index: Index, size: usize) -> Self {
        Self::Rhs(GeneAccess::new::<Genome>(index, size))
    }

    pub(crate) fn both<Genome>(index: Index, lhs_size: usize, rhs_size: usize) -> Self
    where
        Index: Clone,
    {
        Self::Both {
            lhs: GeneAccess::new::<Genome>(index.clone(), lhs_size),
            rhs: GeneAccess::new::<Genome>(index, rhs_size),
        }
    }

    /// Changes the contained `genome_type` captured for the
    /// error message to another genome type. This is necessary
    /// when the actual type being recombined (e.g., `Vec<bool>`)
    /// is wrapped in a container type like `Bitstring`.
    pub(crate) fn for_genome_type<NewGenome>(self) -> Self {
        match self {
            Self::Lhs(gene_access) => Self::Lhs(gene_access.for_genome_type::<NewGenome>()),
            Self::Rhs(gene_access) => Self::Rhs(gene_access.for_genome_type::<NewGenome>()),
            Self::Both { lhs, rhs } => Self::Both {
                lhs: lhs.for_genome_type::<NewGenome>(),
                rhs: rhs.for_genome_type::<NewGenome>(),
            },
        }
    }
}
