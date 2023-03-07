use crate::operator::Operator;

use super::Composable;

pub struct Map<F> {
    f: F,
}

impl<F> Map<F> {
    pub const fn new(f: F) -> Self {
        Self { f }
    }
}

// I think I can parameterize over the 2 here to make this more general?
impl<F, Input> Operator<[Input; 2]> for Map<F>
where
    F: Operator<Input>,
{
    type Output = [F::Output; 2];

    fn apply(&self, input: [Input; 2], rng: &mut rand::rngs::ThreadRng) -> Self::Output {
        input.map(|x| self.f.apply(x, rng))
    }
}

impl<F, Input> Operator<(Input, Input)> for Map<F>
where
    F: Operator<Input>,
{
    type Output = (F::Output, F::Output);

    fn apply(&self, (x, y): (Input, Input), rng: &mut rand::rngs::ThreadRng) -> Self::Output {
        (self.f.apply(x, rng), self.f.apply(y, rng))
    }
}

impl<F, Input> Operator<Vec<Input>> for Map<F>
where
    F: Operator<Input>,
{
    type Output = Vec<F::Output>;

    fn apply(&self, input: Vec<Input>, rng: &mut rand::rngs::ThreadRng) -> Self::Output {
        input.into_iter().map(|x| self.f.apply(x, rng)).collect()
    }
}

// TODO: Impl `Map` over iterators.

impl<F> Composable for Map<F> {}
