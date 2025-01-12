use rand::{Rng, RngCore};

use super::ChildMaker;
use crate::{operator::selector::Selector, population::Population};

/// Erased
/// ([dyn-compatible](https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility))
/// version of the [`ChildMaker`] trait
///
/// # How does this work?
///
/// The `erased` pattern in rust aids in type-erasure for traits
/// that aren't themselves dyn-compatible by declaring a dyn-compatible
/// extension trait wrapper for the original trait and blanket-implementing
/// that for all types which implement the original trait.
///
/// In this case, the trait [`DynChildMaker`] can be seen as a dyn-compatible
/// version of the [`ChildMaker`] trait, and any `T: ChildMaker` can also be
/// interpreted as [`T: DynChildMaker`]
///
/// This allows you to use `dyn DynChildMaker<>` trait objects to perform type
/// erasure on types implementing the [`ChildMaker`] trait.
///
/// # When to use it?
///
/// The original trait most of the time has a reason for not beeing
/// dyn-compatible. As such, usually the erased variants of traits come with
/// performance tradeoffs, and [`DynChildMaker`] is of course no exception
/// either, since it introduces additonal indirection and vtable-lookups.
///
/// Please prefer the [`ChildMaker`] trait whenever possible.
///
/// # How to use it?
///
/// tl;dr: use [`dyn DynChildMaker<>`](DynChildMaker) instead of `dyn
/// ChildMaker<>` and still use all the usual [`ChildMaker`] methods elsewhere.
///
/// This trait tries to provide some useful ergonomics to ease the interaction
/// with existing [`ChildMaker`] code.
/// For example, many common pointer types in Rust pointing to a [`dyn
/// DynChildMaker<I>`](DynChildMaker) also implement the [`ChildMaker`] trait
/// themselves, so you most likely do not need to interact with this trait
/// directly.
///
/// For example: [`Box<dyn DynChildMaker<>>`] implements
/// [`ChildMaker<>`](ChildMaker) and as such you can directly call
/// [`.make_child()`](ChildMaker::make_child) on it and do not need to use
/// [`DynChildMaker::dyn_make_child`].
///
/// This also means, any [`Box<dyn DynChildMaker<()>>`] can be passed to generic
/// functions expecting an [`ChildMaker`], like `fn foo(t: impl
/// ChildMaker<>);`.
pub trait DynChildMaker<P, S, E = Box<dyn std::error::Error + Send + Sync>>
where
    P: Population,
    S: Selector<P>,
{
    /// # Errors
    ///
    /// This can return errors if any aspect of creating this child fail.
    /// That can include constructing or scoring the genome.
    fn dyn_make_child(
        &self,
        rng: &mut dyn RngCore,
        population: &P,
        selector: &S,
    ) -> Result<P::Individual, E>;
}

static_assertions::assert_obj_safe!(DynChildMaker<(), ()>);

impl<T, P, S, E> DynChildMaker<P, S, E> for T
where
    T: ChildMaker<P, S, Error: Into<E>>,
    P: Population,
    S: Selector<P>,
{
    fn dyn_make_child(
        &self,
        rng: &mut dyn RngCore,
        population: &P,
        selector: &S,
    ) -> Result<<P as Population>::Individual, E> {
        self.make_child(rng, population, selector)
            .map_err(Into::into)
    }
}

#[ec_macros::dyn_ref_impls]
impl<P, S, E> ChildMaker<P, S> for &dyn DynChildMaker<P, S, E>
where
    P: Population,
    S: Selector<P>,
{
    type Error = E;

    fn make_child<R: Rng + ?Sized>(
        &self,
        mut rng: &mut R,
        population: &P,
        selector: &S,
    ) -> Result<P::Individual, Self::Error> {
        (**self).dyn_make_child(&mut rng, population, selector)
    }
}
