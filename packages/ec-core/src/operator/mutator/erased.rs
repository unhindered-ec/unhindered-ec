use rand::{Rng, RngCore};

use super::Mutator;

/// Erased
/// ([dyn-compatible](https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility))
/// version of the [`Mutator`] trait
///
/// # How does this work?
///
/// The `erased` pattern in rust aids in type-erasure for traits
/// that aren't themselves dyn-compatible by declaring a dyn-compatible
/// extension trait wrapper for the original trait and blanket-implementing
/// that for all types which implement the original trait.
///
/// In this case, the trait [`DynMutator`] can be seen as a dyn-compatible
/// version of the [`Mutator`] trait, and any `T: Mutator` can also be
/// interpreted as [`T: DynMutator`]
///
/// This allows you to use `dyn DynMutator<I>` trait objects to perform type
/// erasure on types implementing the [`Mutator`] trait.
///
/// # When to use it?
///
/// The original trait most of the time has a reason for not beeing
/// dyn-compatible. As such, usually the erased variants of traits come with
/// performance tradeoffs, and [`DynMutator`] is of course no exception either,
/// since it introduces additonal indirection and vtable-lookups.
///
/// Please prefer the [`Mutator`] trait whenever possible.
///
/// # How to use it?
///
/// tl;dr: use `dyn DynMutator<>` instead of `dyn Mutator<>` and still use
/// all the usual [`Mutator`] methods elsewhere.
///
/// This trait tries to provide some useful ergonomics to ease the interaction
/// with existing [`Mutator`] code.
/// For example, many common pointer types in Rust pointing to a [`dyn
/// DynMutator<>`](DynMutator) also implement the [`Mutator`] trait
/// themselves, so you most likely do not need to interact with this trait
/// directly.
///
/// For example: `Box<dyn DynMutator<>>` implements
/// [`Mutator<>`](Mutator) and as such you can directly call
/// [`.mutate()`](Mutator::mutate) on it and do not need to use
/// [`DynMutator::dyn_mutate`].
///
/// This also means, any `Box<dyn DynMutator<>>` can be passed to generic
/// functions expecting an [`Mutator`], like `fn foo(t: impl Mutator<>);`.
pub trait DynMutator<G, E = Box<dyn std::error::Error + Send + Sync>> {
    /// Mutate the given `genome` returning a new genome of the same type (`G`)
    /// using this typed erased [`Mutator`].
    ///
    /// It is recommended to not use this method directly and instead call the
    /// normal [`Mutator::mutate`] implemented on various container types
    /// containing `dyn DynMutator<_>`'s.
    ///
    /// If you want to call this method directly, make sure to only call it on a
    /// `&dyn DynMutator<_>` (i.e. dereference a box first) else you will
    /// introduce another layer of indirection because of the implementations on
    /// the various container types, and additionally type inference would
    /// require additional type annotations (this is usually a sign you are
    /// doing something wrong).
    ///
    /// # Example
    /// ```
    /// # use rand::{rng, Rng};
    /// # use ec_core::operator::mutator::{Mutator, DynMutator};
    /// # use std::convert::Infallible;
    /// #
    /// type Genome<T> = [T; 4];
    /// #
    /// # struct FlipOne;
    /// #
    /// # impl Mutator<Genome<bool>> for FlipOne {
    /// #     type Error = Infallible;
    /// #
    /// #     fn mutate<R: Rng + ?Sized>(
    /// #         &self,
    /// #         mut genome: Genome<bool>,
    /// #         rng: &mut R,
    /// #     ) -> Result<Genome<bool>, Self::Error> {
    /// #         let index = rng.random_range(0..genome.len());
    /// #         genome[index] = !genome[index];
    /// #         Ok(genome)
    /// #     }
    /// # }
    ///
    /// let my_erased_mutator: Box<dyn DynMutator<Genome<bool>>> = Box::new(FlipOne);
    ///
    /// let genome = [true, false, false, true];
    /// let child_genome = (*my_erased_mutator).dyn_mutate(genome, &mut rng()).unwrap();
    /// # let num_diffs = genome
    /// #     .iter()
    /// #     .zip(child_genome.iter()) // Pair up corresponding elements from the two genomes
    /// #     .filter(|(x, y)| x != y) // Filter out pairs where the elements are the same
    /// #     .count();
    /// # assert_eq!(num_diffs, 1);
    /// ```
    ///
    /// # Errors
    ///
    /// - `Error` if mutating the given `genome` errors, for example because the
    ///   `genome` is invalid in some way.
    fn dyn_mutate(&self, genome: G, rng: &mut dyn RngCore) -> Result<G, E>;
}

static_assertions::assert_obj_safe!(DynMutator<()>);

impl<T, G, E> DynMutator<G, E> for T
where
    T: Mutator<G, Error: Into<E>>,
{
    fn dyn_mutate(&self, genome: G, rng: &mut dyn RngCore) -> Result<G, E> {
        self.mutate(genome, rng).map_err(Into::into)
    }
}

#[ec_macros::dyn_ref_impls]
impl<G, E> Mutator<G> for &dyn DynMutator<G, E> {
    type Error = E;

    fn mutate<R: Rng + ?Sized>(&self, genome: G, mut rng: &mut R) -> Result<G, Self::Error> {
        (**self).dyn_mutate(genome, &mut rng)
    }
}
