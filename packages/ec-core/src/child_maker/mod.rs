use rand::Rng;

use crate::{operator::selector::Selector, population::Population};

#[cfg(feature = "erased")]
mod erased;
#[cfg(feature = "erased")]
pub use erased::*;

// TODO: esitsu@Twitch: "In my world the ChildMaker becomes
//   an operator that scores". So this could just be
//   something that takes a `genome` and returns a
//   scored `Individual`. That would be a lot cleaner.
// #[deprecated(note = "Turn this into a genome->Individual operator")]
pub trait ChildMaker<P, S>
where
    P: Population,
    S: Selector<P>,
{
    type Error;

    // TODO: Instead of passing 2/3 of  Generation` to this function, is there a
    // trait  we can have `Generation` implement, and pass in a reference to
    // something implementing  that trait instead? The trait would presumably
    // implement the `get_parent()` method  or similar.
    //
    /// # Errors
    ///
    /// This can return errors if any aspect of creating this child fail.
    /// That can include constructing or scoring the genome.
    fn make_child<R: Rng + ?Sized>(
        &self,
        rng: &mut R,
        population: &P,
        selector: &S,
    ) -> Result<P::Individual, Self::Error>;
}
