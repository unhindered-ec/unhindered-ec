use rand::rngs::ThreadRng;

use crate::{generation::Generation, individual::ec::EcIndividual, selectors::Selector, population::VecPop};

pub trait ChildMaker<G, R>: ChildMakerI<EcIndividual<G, R>> + Sync {
    fn make_child(&self, rng: &mut ThreadRng, selector: &Generation<G, R>) -> EcIndividual<G, R>;
}

impl<T, G, R> ChildMaker<G, R> for T
where
    T: ChildMakerI<EcIndividual<G, R>> + Sync
{
    fn make_child(&self, rng: &mut ThreadRng, generation: &Generation<G, R>) -> EcIndividual<G, R> {
        T::make_child_i(self, rng, &generation.population, generation.selector())
    }
}

pub trait ChildMakerI<I> {
    fn make_child_i(&self, rng: &mut ThreadRng, population: &VecPop<I>, selector: &dyn Selector<I>) -> I;
}