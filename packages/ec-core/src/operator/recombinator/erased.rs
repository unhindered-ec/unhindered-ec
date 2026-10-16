use rand::Rng;

use super::Recombinator;

/// Erased
/// ([dyn-compatible](https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility))
/// version of the [`Recombinator`] trait
///
/// # How does this work?
///
/// The `erased` pattern in rust aids in type-erasure for traits
/// that aren't themselves dyn-compatible by declaring a dyn-compatible
/// extension trait wrapper for the original trait and blanket-implementing
/// that for all types which implement the original trait.
///
/// In this case, the trait [`DynRecombinator`] can be seen as a dyn-compatible
/// version of the [`Recombinator`] trait, and any `T: Recombinator` can also be
/// interpreted as [`T: DynRecombinator`]
///
/// This allows you to use `dyn DynRecombinator<I>` trait objects to perform
/// type erasure on types implementing the [`Recombinator`] trait.
///
/// # When to use it?
///
/// The original trait most of the time has a reason for not beeing
/// dyn-compatible. As such, usually the erased variants of traits come with
/// performance tradeoffs, and [`DynRecombinator`] is of course no exception
/// either, since it introduces additonal indirection and vtable-lookups.
///
/// Please prefer the [`Recombinator`] trait whenever possible.
///
/// # How to use it?
///
/// tl;dr: use `dyn DynRecombinator<>` instead of `dyn Recombinator<>` and still
/// use all the usual [`Recombinator`] methods elsewhere.
///
/// This trait tries to provide some useful ergonomics to ease the interaction
/// with existing [`Recombinator`] code.
/// For example, many common pointer types in Rust pointing to a [`dyn
/// DynRecombinator<>`](DynRecombinator) also implement the [`Recombinator`]
/// trait themselves, so you most likely do not need to interact with this trait
/// directly.
///
/// For example: `Box<dyn DynRecombinator<>>` implements
/// [`Recombinator<>`](Recombinator) and as such you can directly call
/// [`.recombine()`](Recombinator::recombine) on it and do not need to use
/// [`DynRecombinator::dyn_recombine`].
///
/// This also means, any `Box<dyn DynRecombinator<>>` can be passed to generic
/// functions expecting an [`Recombinator`], like `fn foo(t: impl
/// Recombinator<>);`.
pub trait DynRecombinator<GS, E = Box<dyn std::error::Error + Send + Sync>> {
    type Output;

    /// Recombine the given `genomes` using this type-erased recombinator,
    /// returning a new genome of type `Output`.
    ///
    /// It is recommended to not use this method directly and instead call the
    /// normal [`Recombinator::recombine`] implemented on various container
    /// types containing `dyn DynRecombinator<_>`'s.
    ///
    /// If you want to call this method directly, make sure to only call it on a
    /// `&dyn DynRecombinator<_>` (i.e. dereference a box first) else you will
    /// introduce another layer of indirection because of the implementations on
    /// the various container types, and additionally type inference would
    /// require additional type annotations (this is usually a sign you are
    /// doing something wrong).
    ///
    /// # Example
    /// ```
    /// # use rand::{rng, Rng, RngExt};
    /// # use ec_core::operator::recombinator::{Recombinator, DynRecombinator};
    /// # use std::convert::Infallible;
    /// #
    /// # struct SwapOne;
    /// type Genome<T> = [T; 4];
    /// type Parents<T> = (Genome<T>, Genome<T>);
    /// #
    /// # impl<T: Copy> Recombinator<Parents<T>> for SwapOne {
    /// #     type Output = Genome<T>;
    /// #     type Error = Infallible;
    /// #
    /// #     fn recombine<R: Rng + ?Sized>(
    /// #         &self,
    /// #         (mut first_parent, second_parent): Parents<T>,
    /// #         rng: &mut R,
    /// #     ) -> Result<Genome<T>, Self::Error> {
    /// #         let index = rng.random_range(0..first_parent.len());
    /// #         first_parent[index] = second_parent[index];
    /// #         Ok(first_parent)
    /// #     }
    /// # }
    /// let my_erased_recombinator: Box<dyn DynRecombinator<Parents<i32>, Output = Genome<i32>>> =
    ///     Box::new(SwapOne);
    ///
    /// let first_parent = [0, 0, 0, 0];
    /// let second_parent = [1, 1, 1, 1];
    /// let child =
    ///     (*my_erased_recombinator).dyn_recombine((first_parent, second_parent), &mut rng())?;
    /// # let num_zeros = child.iter().filter(|&&x| x == 0).count();
    /// # let num_ones = child.iter().filter(|&&x| x == 1).count();
    /// # assert_eq!(num_zeros, 3);
    /// # assert_eq!(num_ones, 1);
    /// # Ok::<(), Box<dyn std::error::Error + Send + Sync>>(())
    /// ```
    ///
    /// # Errors
    ///
    /// - `Error` if recombining the given parent genomes fails, for example
    ///   beacuse one of the parent genomes is invalid in some way.
    fn dyn_recombine(&self, genomes: GS, rng: &mut dyn Rng) -> Result<Self::Output, E>;
}

static_assertions::assert_obj_safe!(DynRecombinator<(), Output = ()>);

impl<T, GS, E> DynRecombinator<GS, E> for T
where
    T: Recombinator<GS, Error: Into<E>>,
{
    type Output = T::Output;

    fn dyn_recombine(&self, genomes: GS, rng: &mut dyn Rng) -> Result<Self::Output, E> {
        self.recombine(genomes, rng).map_err(Into::into)
    }
}

#[ec_macros::dyn_ref_impls]
impl<GS, O, E> Recombinator<GS> for &dyn DynRecombinator<GS, E, Output = O> {
    type Output = O;
    type Error = E;

    fn recombine<R: Rng + ?Sized>(&self, genomes: GS, mut rng: &mut R) -> Result<Self::Output, E> {
        (**self).dyn_recombine(genomes, &mut rng)
    }
}

#[cfg(test)]
mod tests {
    use std::{rc::Rc, sync::Arc};

    use rand::{Rng, rng};

    use super::DynRecombinator;
    use crate::operator::recombinator::Recombinator;

    type Parents = (Vec<i32>, Vec<i32>);
    type Erased = dyn DynRecombinator<Parents, LengthMismatch, Output = Vec<i32>>;

    #[derive(Debug, thiserror::Error)]
    #[error("the second parent must have the same length as the first")]
    struct LengthMismatch;

    /// Deterministically zips the parents, taking the first element from the
    /// first parent and the second from the second parent, so the output
    /// depends on both parents and differs from either of them.
    ///
    /// Rejects a second parent whose length differs from the first, giving us a
    /// fallible case to exercise the erased error path.
    struct Zipper;

    impl Recombinator<Parents> for Zipper {
        type Output = Vec<i32>;
        type Error = LengthMismatch;

        fn recombine<R: Rng + ?Sized>(
            &self,
            (first, second): Parents,
            _rng: &mut R,
        ) -> Result<Self::Output, Self::Error> {
            if first.len() != second.len() {
                return Err(LengthMismatch);
            }
            Ok(first
                .into_iter()
                .zip(second)
                .enumerate()
                .map(|(i, (a, b))| if i % 2 == 0 { a } else { b })
                .collect())
        }
    }

    fn parents() -> Parents {
        (vec![1, 2, 3, 4], vec![5, 6, 7, 8])
    }

    #[test]
    fn dyn_recombine_forwards_to_the_inner_recombinator() {
        let erased: &Erased = &Zipper;
        assert_eq!(
            erased.dyn_recombine(parents(), &mut rng()).unwrap(),
            vec![1, 6, 3, 8]
        );
    }

    #[test]
    fn box_forwards_recombine_to_the_inner_recombinator() {
        let recombinator: Box<Erased> = Box::new(Zipper);
        assert_eq!(
            recombinator.recombine(parents(), &mut rng()).unwrap(),
            vec![1, 6, 3, 8]
        );
    }

    #[test]
    fn arc_forwards_recombine_to_the_inner_recombinator() {
        let recombinator: Arc<Erased> = Arc::new(Zipper);
        assert_eq!(
            recombinator.recombine(parents(), &mut rng()).unwrap(),
            vec![1, 6, 3, 8]
        );
    }

    #[test]
    fn rc_forwards_recombine_to_the_inner_recombinator() {
        let recombinator: Rc<Erased> = Rc::new(Zipper);
        assert_eq!(
            recombinator.recombine(parents(), &mut rng()).unwrap(),
            vec![1, 6, 3, 8]
        );
    }

    #[test]
    fn shared_reference_forwards_recombine_to_the_inner_recombinator() {
        let inner = Zipper;
        let erased: &Erased = &inner;
        assert_eq!(
            erased.recombine(parents(), &mut rng()).unwrap(),
            vec![1, 6, 3, 8]
        );
    }

    #[test]
    fn dyn_recombine_forwards_errors() {
        let erased: &Erased = &Zipper;

        let result = erased.dyn_recombine((vec![1, 2, 3], vec![4, 5]), &mut rng());

        assert!(matches!(result, Err(LengthMismatch)));
    }

    #[test]
    fn box_forwards_recombine_errors() {
        let recombinator: Box<Erased> = Box::new(Zipper);

        let result = recombinator.recombine((vec![1, 2, 3], vec![4, 5]), &mut rng());

        assert!(matches!(result, Err(LengthMismatch)));
    }

    #[test]
    fn arc_forwards_recombine_errors() {
        let recombinator: Arc<Erased> = Arc::new(Zipper);

        let result = recombinator.recombine((vec![1, 2, 3], vec![4, 5]), &mut rng());

        assert!(matches!(result, Err(LengthMismatch)));
    }

    #[test]
    fn rc_forwards_recombine_errors() {
        let recombinator: Rc<Erased> = Rc::new(Zipper);

        let result = recombinator.recombine((vec![1, 2, 3], vec![4, 5]), &mut rng());

        assert!(matches!(result, Err(LengthMismatch)));
    }

    #[test]
    fn shared_reference_forwards_recombine_errors() {
        let inner = Zipper;
        let erased: &Erased = &inner;

        let result = erased.recombine((vec![1, 2, 3], vec![4, 5]), &mut rng());

        assert!(matches!(result, Err(LengthMismatch)));
    }
}
