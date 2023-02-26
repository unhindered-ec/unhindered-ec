use rand::rngs::ThreadRng;

mod composable;
pub mod recombinator;
pub mod selector;

pub use composable::Composable;

pub trait Operator<Input>: Composable {
    type Output;

    fn apply(&self, input: Input, rng: &mut ThreadRng) -> Self::Output;
}

impl<T, Input> Operator<Input> for &T
where
    T: Operator<Input>,
{
    type Output = T::Output;

    fn apply(&self, input: Input, rng: &mut ThreadRng) -> Self::Output {
        (*self).apply(input, rng)
    }
}

impl<T> Composable for &T where T: Composable {}
