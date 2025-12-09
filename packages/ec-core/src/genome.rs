/// A genome for the ec system, made up of individual [`Genome::Gene`]'s.
///
/// Genomes are at the core of any ec system, since they are what all operations
/// are operating on and what is scored to evaluate evolution progress.
///
/// Usually you will be using a pre-defined genome like
/// [`ec-linear::Bitstring`](#) or [`push::Plushy`](#),
/// and not define your own. Take a look at the accompanying crates for
/// different kinds of evolution for more examples of genome types.
///
/// # Example
/// This is how you might implement a Bitstring genome type:
/// ```
/// # use ec_core::genome::Genome;
/// # #[allow(dead_code)]
/// struct Bitstring {
///     length: usize,
///     storage: Vec<u64>,
/// }
///
/// impl Genome for Bitstring {
///     type Gene = bool;
/// }
/// ```
pub trait Genome {
    /// Type of each individual gene in this genome
    type Gene;
}
static_assertions::assert_obj_safe!(Genome<Gene = ()>);

impl<T> Genome for Vec<T> {
    /// For linear genomes (`Vec<T>`) each genome is of the type `T`.
    type Gene = T;
}
