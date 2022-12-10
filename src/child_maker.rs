use rand::rngs::ThreadRng;

use crate::{population::Population, selector::Selector};

pub trait ChildMaker<P: Population, S: Selector<P>> {
    // TODO: Instead of passing 2/3 of  Generation` to this function, is there a trait
    //  we can have `Generation` implement, and pass in a reference to something implementing
    //  that trait instead? The trait would presumably implement the `get_parent()` method
    //  or similar.
    fn make_child(&self, rng: &mut ThreadRng, population: &P, selector: &S) -> P::Individual;
}

impl<P: Population, S: Selector<P>> ChildMaker<P, S> for &dyn ChildMaker<P, S> {
    fn make_child(&self, rng: &mut ThreadRng, population: &P, selector: &S) -> P::Individual {
        (*self).make_child(rng, population, selector)
    }
}

impl<P: Population, S: Selector<P>> ChildMaker<P, S> for &(dyn ChildMaker<P, S> + Send) {
    fn make_child(&self, rng: &mut ThreadRng, population: &P, selector: &S) -> P::Individual {
        (*self).make_child(rng, population, selector)
    }
}

impl<P: Population, S: Selector<P>> ChildMaker<P, S> for &(dyn ChildMaker<P, S> + Send + Sync) {
    fn make_child(&self, rng: &mut ThreadRng, population: &P, selector: &S) -> P::Individual {
        (*self).make_child(rng, population, selector)
    }
}
