use rand::rngs::ThreadRng;

pub mod composable;
pub mod genome_extractor;
pub mod identity;
pub mod mutator;
pub mod recombinator;
pub mod selector;

pub use composable::Composable;

pub trait Operator<Input>: Composable {
    type Output;

    fn apply(&self, input: Input, rng: &mut ThreadRng) -> Self::Output;
}
