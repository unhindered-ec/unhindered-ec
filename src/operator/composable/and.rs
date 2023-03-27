use rand::rngs::ThreadRng;

use super::super::Operator;

use super::Composable;

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
{
    type Output = (F::Output, G::Output);

    fn apply(&self, x: A, rng: &mut ThreadRng) -> Self::Output {
        (self.f.apply(x.clone(), rng), self.g.apply(x, rng))
    }
}
impl<F, G> Composable for And<F, G> {}
