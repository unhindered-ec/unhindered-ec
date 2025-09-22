use std::{
    error::Error,
    fmt::{Debug, Display},
};

use miette::{Diagnostic, LabeledSpan, Severity, SourceCode};

#[derive(Debug, thiserror::Error, Diagnostic)]
#[error("Attempted to perform TwoPointXo on genomes of different lengths {0} and {1}")]
#[diagnostic(help = "Ensure your genomes are of uniform length")]
pub struct DifferentGenomeLength(pub usize, pub usize);

#[derive(Debug, thiserror::Error, Diagnostic)]
#[error(
    "Attempted to perform TwoPointXo with more crossover points than possible for the current \
     genome length {genome_length}. Need a minimum of {min_size} > {genome_length}"
)]
#[diagnostic(help = "Ensure your genomes are long enough or use less crossover points.")]
pub struct GenomeLengthTooShort {
    pub genome_length: usize,
    pub min_size: usize,
}

// TODO: Split this in 2 so that we don't have the unneeded 2nd variant for for
// example UniformXo.
#[derive(Debug)]
pub enum CrossoverGeneError<E> {
    /// Attempted to crossover genomes with differing lengths
    DifferentGenomeLength(DifferentGenomeLength),
    /// Attempted to crossover too short of a genome for the crossover point
    /// count.
    GenomeLengthTooShort(GenomeLengthTooShort),
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
            Self::GenomeLengthTooShort(transparent) => Error::source(transparent),
            Self::Crossover(source) => Some(source),
        }
    }
}
impl<E> Display for CrossoverGeneError<E> {
    fn fmt(&self, formatter: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            Self::DifferentGenomeLength(g) => Display::fmt(&g, formatter),
            Self::GenomeLengthTooShort(g) => Display::fmt(&g, formatter),
            Self::Crossover(_) => formatter.write_str("Failed to crossover segment"),
        }
    }
}
impl<E> From<DifferentGenomeLength> for CrossoverGeneError<E> {
    fn from(source: DifferentGenomeLength) -> Self {
        Self::DifferentGenomeLength(source)
    }
}
impl<E> From<GenomeLengthTooShort> for CrossoverGeneError<E> {
    fn from(source: GenomeLengthTooShort) -> Self {
        Self::GenomeLengthTooShort(source)
    }
}
impl<E> Diagnostic for CrossoverGeneError<E>
where
    E: Error + Diagnostic + 'static,
{
    fn code(&self) -> Option<Box<dyn Display + '_>> {
        match self {
            Self::DifferentGenomeLength(unnamed, ..) => unnamed.code(),
            Self::GenomeLengthTooShort(unnamed) => unnamed.code(),
            Self::Crossover(unnamed, ..) => unnamed.code(),
        }
    }
    fn help(&self) -> Option<Box<dyn Display + '_>> {
        match self {
            Self::DifferentGenomeLength(unnamed, ..) => unnamed.help(),
            Self::GenomeLengthTooShort(unnamed) => unnamed.help(),
            Self::Crossover(unnamed, ..) => unnamed.help(),
        }
    }
    fn severity(&self) -> Option<Severity> {
        match self {
            Self::DifferentGenomeLength(unnamed, ..) => unnamed.severity(),
            Self::GenomeLengthTooShort(unnamed) => unnamed.severity(),
            Self::Crossover(unnamed, ..) => unnamed.severity(),
        }
    }
    fn labels(&self) -> Option<Box<dyn Iterator<Item = LabeledSpan> + '_>> {
        match self {
            Self::DifferentGenomeLength(unnamed, ..) => unnamed.labels(),
            Self::GenomeLengthTooShort(unnamed) => unnamed.labels(),
            Self::Crossover(unnamed, ..) => unnamed.labels(),
        }
    }
    fn source_code(&self) -> Option<&dyn SourceCode> {
        match self {
            Self::DifferentGenomeLength(unnamed, ..) => unnamed.source_code(),
            Self::GenomeLengthTooShort(unnamed) => unnamed.source_code(),
            Self::Crossover(unnamed, ..) => unnamed.source_code(),
        }
    }
    fn related(&self) -> Option<Box<dyn Iterator<Item = &dyn Diagnostic> + '_>> {
        match self {
            Self::DifferentGenomeLength(unnamed, ..) => unnamed.related(),
            Self::GenomeLengthTooShort(unnamed) => unnamed.related(),
            Self::Crossover(unnamed, ..) => unnamed.related(),
        }
    }
    fn url(&self) -> Option<Box<dyn Display + '_>> {
        match self {
            Self::DifferentGenomeLength(unnamed, ..) => unnamed.url(),
            Self::GenomeLengthTooShort(unnamed) => unnamed.url(),
            Self::Crossover(unnamed, ..) => unnamed.url(),
        }
    }
    fn diagnostic_source(&self) -> Option<&dyn Diagnostic> {
        match self {
            Self::DifferentGenomeLength(unnamed, ..) => unnamed.diagnostic_source(),
            Self::GenomeLengthTooShort(unnamed) => unnamed.diagnostic_source(),
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
    pub(crate) fn for_genome_type<NewGenome: 'static>(mut self) -> Self {
        self.genome_type = std::any::type_name::<NewGenome>();
        self
    }
}

impl<Index: Debug> GeneAccess<Index> {
    pub fn new<Genome: 'static>(index: Index, size: usize) -> Self {
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
    pub(crate) fn lhs<Genome: 'static>(index: Index, size: usize) -> Self {
        Self::Lhs(GeneAccess::new::<Genome>(index, size))
    }

    pub(crate) fn rhs<Genome: 'static>(index: Index, size: usize) -> Self {
        Self::Rhs(GeneAccess::new::<Genome>(index, size))
    }

    pub(crate) fn both<Genome: 'static>(index: Index, lhs_size: usize, rhs_size: usize) -> Self
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
    pub(crate) fn for_genome_type<NewGenome: 'static>(self) -> Self {
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

#[cfg(test)]
mod tests {
    use std::ops::Range;

    use super::*;

    // Dummy structs for testing genome types
    struct GenomeA;
    struct GenomeB;

    #[test]
    fn gene_access_new() {
        let access = GeneAccess::<usize>::new::<GenomeA>(10, 20);
        assert_eq!(access.index, 10);
        assert_eq!(access.size, 20);
        assert!(access.genome_type.contains("GenomeA"));
    }

    #[test]
    fn gene_access_for_genome_type() {
        let access = GeneAccess::<usize>::new::<GenomeA>(10, 20);
        let original_genome_type = access.genome_type;

        let updated_access = access.for_genome_type::<GenomeB>();

        assert_eq!(updated_access.index, 10);
        assert_eq!(updated_access.size, 20);
        assert!(updated_access.genome_type.contains("GenomeB"));
        assert_ne!(original_genome_type, updated_access.genome_type);
    }

    #[test]
    fn gene_access_display_contains_info() {
        let access = GeneAccess::<usize>::new::<GenomeA>(10, 20);
        let display_str = format!("{access}");
        assert!(display_str.contains("10"));
        assert!(display_str.contains("20"));
        assert!(display_str.contains("GenomeA"));
    }

    #[test]
    fn gene_access_display_range_contains_info() {
        let access = GeneAccess::<Range<usize>>::new::<GenomeA>(10..15, 20);
        let display_str = format!("{access}");
        assert!(display_str.contains("10..15"));
        assert!(display_str.contains("20"));
        assert!(display_str.contains("GenomeA"));
    }

    #[test]
    fn multiple_gene_access_lhs() {
        let err = MultipleGeneAccess::<usize>::lhs::<GenomeA>(10, 20);
        if let MultipleGeneAccess::Lhs(access) = err {
            assert_eq!(access.index, 10);
            assert_eq!(access.size, 20);
            assert!(access.genome_type.contains("GenomeA"));
        } else {
            panic!("Expected MultipleGeneAccess::Lhs");
        }
    }

    #[test]
    fn multiple_gene_access_rhs() {
        let err = MultipleGeneAccess::<usize>::rhs::<GenomeA>(10, 20);
        if let MultipleGeneAccess::Rhs(access) = err {
            assert_eq!(access.index, 10);
            assert_eq!(access.size, 20);
            assert!(access.genome_type.contains("GenomeA"));
        } else {
            panic!("Expected MultipleGeneAccess::Rhs");
        }
    }

    #[test]
    fn multiple_gene_access_both() {
        let err = MultipleGeneAccess::<usize>::both::<GenomeA>(10, 20, 30);
        if let MultipleGeneAccess::Both { lhs, rhs } = err {
            assert_eq!(lhs.index, 10);
            assert_eq!(lhs.size, 20);
            assert!(lhs.genome_type.contains("GenomeA"));
            assert_eq!(rhs.index, 10);
            assert_eq!(rhs.size, 30);
            assert!(rhs.genome_type.contains("GenomeA"));
        } else {
            panic!("Expected MultipleGeneAccess::Both");
        }
    }

    #[test]
    fn multiple_gene_access_for_genome_type_lhs() {
        let err = MultipleGeneAccess::<usize>::lhs::<GenomeA>(10, 20);
        let updated_err = err.for_genome_type::<GenomeB>();
        if let MultipleGeneAccess::Lhs(access) = updated_err {
            assert!(access.genome_type.contains("GenomeB"));
        } else {
            panic!("Expected MultipleGeneAccess::Lhs");
        }
    }

    #[test]
    fn multiple_gene_access_for_genome_type_rhs() {
        let err = MultipleGeneAccess::<usize>::rhs::<GenomeA>(10, 20);
        let updated_err = err.for_genome_type::<GenomeB>();
        if let MultipleGeneAccess::Rhs(access) = updated_err {
            assert!(access.genome_type.contains("GenomeB"));
        } else {
            panic!("Expected MultipleGeneAccess::Rhs");
        }
    }

    #[test]
    fn multiple_gene_access_for_genome_type_both() {
        let err = MultipleGeneAccess::<usize>::both::<GenomeA>(10, 20, 30);
        let updated_err = err.for_genome_type::<GenomeB>();
        if let MultipleGeneAccess::Both { lhs, rhs } = updated_err {
            assert!(lhs.genome_type.contains("GenomeB"));
            assert!(rhs.genome_type.contains("GenomeB"));
        } else {
            panic!("Expected MultipleGeneAccess::Both");
        }
    }

    #[test]
    fn multiple_gene_access_display() {
        let lhs_err = MultipleGeneAccess::<usize>::lhs::<GenomeA>(10, 20);
        assert!(format!("{lhs_err}").contains("lhs"));

        let rhs_err = MultipleGeneAccess::<usize>::rhs::<GenomeA>(10, 20);
        assert!(format!("{rhs_err}").contains("rhs"));

        let both_err = MultipleGeneAccess::<usize>::both::<GenomeA>(10, 20, 30);
        let display_str = format!("{both_err}");
        assert!(display_str.contains("both"));
        assert!(display_str.contains("lhs"));
        assert!(display_str.contains("rhs"));
    }
}
