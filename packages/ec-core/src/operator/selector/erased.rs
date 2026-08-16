use rand::{Rng, RngCore};

use super::Selector;
use crate::population::Population;

/// Erased
/// ([dyn-compatible](https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility))
/// version of the [`Selector`] trait
///
/// # How does this work?
///
/// The `erased` pattern in rust aids in type-erasure for traits
/// that aren't themselves dyn-compatible by declaring a dyn-compatible
/// extension trait wrapper for the original trait and blanket-implementing
/// that for all types which implement the original trait.
///
/// In this case, the trait [`DynSelector`] can be seen as a dyn-compatible
/// version of the [`Selector`] trait, and any `T: Selector` can also be
/// interpreted as [`T: DynSelector`]
///
/// This allows you to use `dyn DynSelector<I>` trait objects to perform type
/// erasure on types implementing the [`Selector`] trait.
///
/// # When to use it?
///
/// The original trait most of the time has a reason for not beeing
/// dyn-compatible. As such, usually the erased variants of traits come with
/// performance tradeoffs, and [`DynSelector`] is of course no exception either,
/// since it introduces additonal indirection and vtable-lookups.
///
/// Please prefer the [`Selector`] trait whenever possible.
///
/// # How to use it?
///
/// tl;dr: use `dyn DynSelector<>` instead of `dyn Selector<>` and still use
/// all the usual [`Selector`] methods elsewhere.
///
/// This trait tries to provide some useful ergonomics to ease the interaction
/// with existing [`Selector`] code.
/// For example, many common pointer types in Rust pointing to a [`dyn
/// DynSelector<>`](DynSelector) also implement the [`Selector`] trait
/// themselves, so you most likely do not need to interact with this trait
/// directly.
///
/// For example: `Box<dyn DynSelector<>>` implements
/// [`Selector<>`](Selector) and as such you can directly call
/// [`.select()`](Selector::select) on it and do not need to use
/// [`DynSelector::dyn_select`].
///
/// This also means, any `Box<dyn DynSelector<>>` can be passed to generic
/// functions expecting an [`Selector`], like `fn foo(t: impl Selector<>);`.
pub trait DynSelector<P, Error = Box<dyn std::error::Error + Send + Sync>>
where
    P: Population,
{
    /// Select an individual from the given `population`, using this type-erased
    /// selector.
    ///
    /// It is recommended to not use this method directly and instead call the
    /// normal [`Selector::select`] implemented on various container types
    /// containing `dyn DynSelector<_>`'s.
    ///
    /// If you want to call this method directly, make sure to only call it on a
    /// `&dyn DynSelector<_>` (i.e. dereference a box first) else you will
    /// introduce another layer of indirection because of the implementations on
    /// the various container types, and additionally type inference would
    /// require additional type annotations (this is usually a sign you are
    /// doing something wrong).
    ///
    /// # Example
    /// ```
    /// # use ec_core::operator::selector::{best::Best, DynSelector};
    /// # use rand::rng;
    /// #
    /// let population = [5, 8, 9, 2, 3, 6];
    /// let my_erased_selector: Box<dyn DynSelector<[i32; 6]>> = Box::new(Best);
    ///
    /// let winner = (*my_erased_selector).dyn_select(&population, &mut rng())?;
    /// assert_eq!(*winner, 9);
    /// # Ok::<(), Box<dyn std::error::Error + Send + Sync>>(())
    /// ```
    ///
    ///
    /// # Errors
    /// - `Error` if a problem selecting occurs, for example because the
    ///   population is empty or not big enough for the selector.
    fn dyn_select<'pop>(
        &self,
        population: &'pop P,
        rng: &mut dyn RngCore,
    ) -> Result<&'pop P::Individual, Error>;
}

static_assertions::assert_obj_safe!(DynSelector<()>);

impl<P, T, E> DynSelector<P, E> for T
where
    P: Population,
    T: Selector<P, Error: Into<E>>,
{
    fn dyn_select<'pop>(
        &self,
        population: &'pop P,
        rng: &mut dyn RngCore,
    ) -> Result<&'pop <P as Population>::Individual, E> {
        self.select(population, rng).map_err(Into::into)
    }
}

#[ec_macros::dyn_ref_impls]
impl<P, E> Selector<P> for &dyn DynSelector<P, E>
where
    P: Population,
{
    type Error = E;

    fn select<'pop, R: Rng + ?Sized>(
        &self,
        population: &'pop P,
        mut rng: &mut R,
    ) -> Result<&'pop <P as Population>::Individual, Self::Error> {
        (**self).dyn_select(population, &mut rng)
    }
}
