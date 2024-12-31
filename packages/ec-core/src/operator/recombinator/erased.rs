use rand::{Rng, RngCore};

use super::Recombinator;

pub trait DynRecombinator<GS, E = Box<dyn std::error::Error + Send + Sync>> {
    type Output;

    /// Recombine the given `genomes` returning a new genome of type `Output`.
    ///
    /// # Errors
    ///
    /// This will return an error if there is an error recombining the given
    /// parent genomes. This will usually be because the given `genomes` are
    /// invalid in some way, thus making recombination impossible.
    fn dyn_recombine(&self, genomes: GS, rng: &mut dyn RngCore) -> Result<Self::Output, E>;
}

static_assertions::assert_obj_safe!(DynRecombinator<(), Output = ()>);

impl<T, GS, E> DynRecombinator<GS, E> for T
where
    T: Recombinator<GS, Error: Into<E>>,
{
    type Output = T::Output;

    fn dyn_recombine(&self, genomes: GS, rng: &mut dyn RngCore) -> Result<Self::Output, E> {
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
