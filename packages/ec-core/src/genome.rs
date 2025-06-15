/// A genome for the ec system, made up of individual [`Geome::Gene`]'s.
///
/// Genomes are at the core of any ec system, since they are what all operations
/// are operating on and what is scored to evaluate evolution progress.
///
/// Usually you will be using a pre-defined genome like
/// [`ec-linear::Bitstring`](ec-linear-genome) or [`push::Plushy`](push-genome),
/// and not define your own. Take a look at the accompanying crates for
/// different kinds of evolution for more examples of genome types.
///
/// # Example
/// This is how you might implement a Bitstring genome type:
/// ```
/// # use ec_core::genome::Genome;
/// struct Bitstring {
///     length: usize,
///     storage: Vec<u64>,
/// }
///
/// impl Genome for Bitstring {
///     type Gene = bool;
/// }
/// ```
/// <!-- placeholders till packages are published to docs.rs -->
/// [push-genome]: #
/// [ec-linear-genome]: #
pub trait Genome {
    type Gene;
}
static_assertions::assert_obj_safe!(Genome<Gene = ()>);
