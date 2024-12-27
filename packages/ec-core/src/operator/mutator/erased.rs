use rand::{Rng, RngCore};

use super::Mutator;

#[cfg(feature = "erased")]
pub trait DynMutator<G, E = Box<dyn std::error::Error + Send + Sync>> {
    /// # Errors
    ///
    /// This will return an error if there is an error mutating the given
    /// genome. This will usually be because the given `genome` is invalid in
    /// some way, thus making the mutation impossible.
    fn dyn_mutate(&self, genome: G, rng: &mut dyn RngCore) -> Result<G, E>;
}

static_assertions::assert_obj_safe!(DynMutator<()>);

impl<T, G, E> DynMutator<G, E> for T
where
    T: Mutator<G, Error: Into<E>>,
{
    fn dyn_mutate(&self, genome: G, rng: &mut dyn RngCore) -> Result<G, E> {
        self.mutate(genome, rng).map_err(Into::into)
    }
}

#[ec_macros::dyn_ref_impls]
impl<G, E> Mutator<G> for &dyn DynMutator<G, E> {
    type Error = E;

    fn mutate<R: Rng + ?Sized>(&self, genome: G, mut rng: &mut R) -> Result<G, Self::Error> {
        (**self).dyn_mutate(genome, &mut rng)
    }
}
