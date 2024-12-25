use rand::Rng;

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

#[derive(Debug)]
pub enum AndError<T, U> {
    First(T),
    Second(U),
}

impl<T, U> std::fmt::Display for AndError<T, U> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::First(_) => f.write_str(
                "Error while applying the first passed operator (`T`) in the `And<T,>` Operator",
            ),
            Self::Second(_) => f.write_str(
                "Error while applying the second passed operator (`U`) in the `And<,U>` Operator",
            ),
        }
    }
}

impl<T, U> std::error::Error for AndError<T, U>
where
    T: std::error::Error + 'static,
    U: std::error::Error + 'static,
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::First(t) => Some(t),
            Self::Second(t) => Some(t),
        }
    }
}

impl<T, U> miette::Diagnostic for AndError<T, U>
where
    T: miette::Diagnostic + 'static,
    U: miette::Diagnostic + 'static,
{
    fn diagnostic_source(&self) -> Option<&dyn miette::Diagnostic> {
        match self {
            Self::First(t) => Some(t),
            Self::Second(t) => Some(t),
        }
    }
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
impl<F, G> Composable for And<F, G> {}
