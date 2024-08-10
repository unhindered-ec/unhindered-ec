use anyhow::Context;
use rand::rngs::ThreadRng;

use super::{super::Operator, Composable};

// TODO: May a `apply_n_times(usize)` operator
//   to use in cases where, e.g., we need to select
//   two parents using the same selection operator.
pub struct And<F, G> {
    f: F,
    g: G,
}

impl<F, G> And<F, G> {
    pub const fn new(f: F, g: G) -> Self {
        Self { f, g }
    }
}

impl<A, F, G> Operator<A> for And<F, G>
where
    A: Clone,
    F: Operator<A>,
    G: Operator<A>,
    anyhow::Error: From<F::Error> + From<G::Error>,
{
    type Output = (F::Output, G::Output);
    type Error = anyhow::Error;

    fn apply(&self, x: A, rng: &mut ThreadRng) -> Result<Self::Output, Self::Error> {
        let f_value = self
            .f
            .apply(x.clone(), rng)
            .map_err(anyhow::Error::from)
            .context("f in `And` failed")?;
        let g_value = self
            .g
            .apply(x, rng)
            .map_err(anyhow::Error::from)
            .context("g in `And` failed")?;
        Ok((f_value, g_value))
    }
}
impl<F, G> Composable for And<F, G> {}
