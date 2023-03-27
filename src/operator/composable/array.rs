use crate::operator::Operator;

use super::Composable;

pub struct Array<F, const N: usize> {
    f: F,
}

impl<F, const N: usize> Array<F, N> {
    pub const fn new(f: F) -> Self {
        Self { f }
    }
}

impl<F, Input, const N: usize> Operator<Input> for Array<F, N>
where
    Input: Clone,
    F: Operator<Input>,
{
    type Output = [F::Output; N];

    fn apply(&self, input: Input, rng: &mut rand::rngs::ThreadRng) -> Self::Output {
        [(); N].map(|_| self.f.apply(input.clone(), rng))
    }
}

impl<F, const N: usize> Composable for Array<F, N> {}
