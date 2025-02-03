use miette::Diagnostic;
use rand::Rng;

use super::{super::Operator, Composable};

// TODO: May a `apply_n_times(usize)` operator
//   to use in cases where, e.g., we need to select
//   two parents using the same selection operator.
#[derive(Composable)]
pub struct And<F, G> {
    f: F,
    g: G,
}

impl<F, G> And<F, G> {
    pub const fn new(f: F, g: G) -> Self {
        Self { f, g }
    }
}

#[derive(Debug, thiserror::Error, Diagnostic)]
pub enum AndError<T, U> {
    #[error("Error while applying the first passed operator (`T`) in the `And<T,>` Operator")]
    First(
        #[diagnostic_source]
        #[source]
        T,
    ),
    #[error("Error while applying the second passed operator (`U`) in the `And<,U>` Operator")]
    Second(
        #[diagnostic_source]
        #[source]
        U,
    ),
}

impl<A, F, G> Operator<A> for And<F, G>
where
    A: Clone,
    F: Operator<A>,
    G: Operator<A>,
{
    type Output = (F::Output, G::Output);
    type Error = AndError<F::Error, G::Error>;

    fn apply<R: Rng + ?Sized>(&self, x: A, rng: &mut R) -> Result<Self::Output, Self::Error> {
        let f_value = self.f.apply(x.clone(), rng).map_err(AndError::First)?;
        let g_value = self.g.apply(x, rng).map_err(AndError::Second)?;
        Ok((f_value, g_value))
    }
}
