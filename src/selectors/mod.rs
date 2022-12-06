use rand::rngs::ThreadRng;

use crate::{individual::ec::EcIndividual, population::VecPop};

pub mod selectors;
pub mod random;
pub mod best;
pub mod tournament;
pub mod lexicase;

// TODO: Change `Selector` so it acts on a more general collection than `Population`.
//  I think that all we need are some sort of collection or iterator, and then all
//  dependency on `Population` and `Individual` should be able to be removed from
//  this module.
// TODO: Is there a circumstance where selection should fail? If so, do we want to have
//  it return `Option<Individual>` or even `Result<Individual, Error>`? Not sure.
//  esitsu@Twitch suggested, for example, having a selector with a thresh hold and then
//  a composite that keeps trying selectors until it finds one that works.
// TODO: Change the name of this lifetime from `'a` to `'pop` (or something similar that
//  actually conveys some useful information). This is probably a "grinding" sort of
//  activity and best done outside of the stream.
pub trait Selector<G, R>: Sync {
    fn select<'a>(
        &self,
        rng: &mut ThreadRng,
        population: &'a VecPop<EcIndividual<G, R>>,
    ) -> &'a EcIndividual<G, R>;
}
