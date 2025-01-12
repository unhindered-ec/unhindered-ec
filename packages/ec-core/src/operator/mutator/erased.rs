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
    /// # Errors
    ///
    /// This will return an error if there is an error mutating the given
    /// genome. This will usually be because the given `genome` is invalid in
    /// some way, thus making the mutation impossible.
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
