use rand::{Rng, RngCore};

use super::ChildMaker;
use crate::{operator::selector::Selector, population::Population};

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
