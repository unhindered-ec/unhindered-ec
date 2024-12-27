use rand::{Rng, RngCore};

use super::Selector;
use crate::population::Population;

/// Object-safe version of the [`Selector`] trait.
pub trait DynSelector<P, Error = Box<dyn std::error::Error + Send + Sync>>
where
    P: Population,
{
    /// Select an individual from the given `population`, in a dyn compatible
    /// fashion
    ///
    /// You should probably not use this directly and instead rely on the
    /// `Selector` implementations on all common pointer types in rust
    /// pointing to a object of this trait.
    ///
    /// # Errors
    ///
    /// This will return an error if there's some problem selecting. That will
    /// usually be because the population is empty or not large enough for
    /// the desired selector.
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
