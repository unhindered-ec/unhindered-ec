use rand::rngs::ThreadRng;

use crate::{operator::Operator, population::Population};

pub mod two_point_xo_mutate;

// TODO: esitsu@Twitch: "In my world the ChildMaker becomes
//   an operator that scores". So this could just be
//   something that takes a `genome` and returns a
//   scored `Individual`. That would be a lot cleaner.
#[deprecated(note = "Turn this into a genome->Individual operator")]
pub trait ChildMaker<P, S>
where
    P: Population,
    S: for<'pop> Operator<&'pop P, Output = &'pop P::Individual>,
{
    // TODO: Instead of passing 2/3 of  Generation` to this function, is there a trait
    //  we can have `Generation` implement, and pass in a reference to something implementing
    //  that trait instead? The trait would presumably implement the `get_parent()` method
    //  or similar.
    fn make_child(&self, rng: &mut ThreadRng, population: &P, selector: &S) -> P::Individual;
}

// NOTE: These further impls aren't actually needed anymore because
//   we (as of 19 Feb 2023) take ownership of the ChildMaker instead
//   of storing a `& dyn ChildMaker` in `Generation`.
impl<P, S> ChildMaker<P, S> for &dyn ChildMaker<P, S>
where
    P: Population,
    S: for<'pop> Operator<&'pop P, Output = &'pop P::Individual>,
{
    fn make_child(&self, rng: &mut ThreadRng, population: &P, selector: &S) -> P::Individual {
        (*self).make_child(rng, population, selector)
    }
}

impl<P, S> ChildMaker<P, S> for &(dyn ChildMaker<P, S> + Send)
where
    P: Population,
    S: for<'pop> Operator<&'pop P, Output = &'pop P::Individual>,
{
    fn make_child(&self, rng: &mut ThreadRng, population: &P, selector: &S) -> P::Individual {
        (*self).make_child(rng, population, selector)
    }
}

impl<P, S> ChildMaker<P, S> for &(dyn ChildMaker<P, S> + Send + Sync)
where
    P: Population,
    S: for<'pop> Operator<&'pop P, Output = &'pop P::Individual>,
{
    fn make_child(&self, rng: &mut ThreadRng, population: &P, selector: &S) -> P::Individual {
        (*self).make_child(rng, population, selector)
    }
}
