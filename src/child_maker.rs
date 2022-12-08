use rand::rngs::ThreadRng;

use crate::{population::Population, selector::Selector};

pub trait ChildMaker<P: Population> {
    // TODO: Instead of passing 2/3 of  Generation` to this function, is there a trait
    //  we can have `Generation` implement, and pass in a reference to something implementing
    //  that trait instead? The trait would presumably implement the `get_parent()` method
    //  or similar.
    fn make_child(
        &self,
        rng: &mut ThreadRng,
        population: &P,
        selector: &dyn Selector<P>,
    ) -> P::Individual;
}
