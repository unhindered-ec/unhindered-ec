#[cfg(feature = "erased")]
use std::{
    cell::{Ref, RefMut},
    rc::Rc,
    sync::Arc,
};

use rand::Rng;
#[cfg(feature = "erased")]
use rand::RngCore;

use crate::{operator::selector::Selector, population::Population};

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

#[cfg(feature = "erased")]
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

#[cfg(feature = "erased")]
static_assertions::assert_obj_safe!(DynChildMaker<(), ()>);

#[cfg(feature = "erased")]
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

#[cfg(feature = "erased")]
macro_rules! dyn_child_maker_impl {
    ($t: ty) => {
        #[cfg(feature = "erased")]
        impl<P, S, E> ChildMaker<P, S> for $t
        where
            P: Population,
            S: Selector<P>
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
    };
    ($($t: ty),+ $(,)?) => {
        $(dyn_child_maker_impl!($t);)+
    }
}

#[cfg(feature = "erased")]
// TODO: Create a macro to do this in a nicer way without needing to manually
// repeat all the pointer types everywhere we provide a type erased trait
dyn_child_maker_impl!(
    &dyn DynChildMaker<P, S, E>,
    &(dyn DynChildMaker<P, S, E> + Send),
    &(dyn DynChildMaker<P, S, E> + Sync),
    &(dyn DynChildMaker<P, S, E> + Send + Sync),
    &mut dyn DynChildMaker<P, S, E>,
    &mut (dyn DynChildMaker<P, S, E> + Send),
    &mut (dyn DynChildMaker<P, S, E> + Sync),
    &mut (dyn DynChildMaker<P, S, E> + Send + Sync),
    Box<dyn DynChildMaker<P, S, E>>,
    Box<dyn DynChildMaker<P, S, E> + Send>,
    Box<dyn DynChildMaker<P, S, E> + Sync>,
    Box<dyn DynChildMaker<P, S, E> + Send + Sync>,
    Arc<dyn DynChildMaker<P, S, E>>,
    Arc<dyn DynChildMaker<P, S, E> + Send>,
    Arc<dyn DynChildMaker<P, S, E> + Sync>,
    Arc<dyn DynChildMaker<P, S, E> + Send + Sync>,
    Rc<dyn DynChildMaker<P, S, E>>,
    Rc<dyn DynChildMaker<P, S, E> + Send>,
    Rc<dyn DynChildMaker<P, S, E> + Sync>,
    Rc<dyn DynChildMaker<P, S, E> + Send + Sync>,
    Ref<'_, dyn DynChildMaker<P, S, E>>,
    Ref<'_, dyn DynChildMaker<P, S, E> + Send>,
    Ref<'_, dyn DynChildMaker<P, S, E> + Sync>,
    Ref<'_, dyn DynChildMaker<P, S, E> + Send + Sync>,
    RefMut<'_, dyn DynChildMaker<P, S, E>>,
    RefMut<'_, dyn DynChildMaker<P, S, E> + Send>,
    RefMut<'_, dyn DynChildMaker<P, S, E> + Sync>,
    RefMut<'_, dyn DynChildMaker<P, S, E> + Send + Sync>,
);
