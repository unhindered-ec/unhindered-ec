use rand::rngs::ThreadRng;

pub mod composable;
pub mod genome_extractor;
pub mod genome_scorer;
pub mod identity;
pub mod mutator;
pub mod recombinator;
pub mod selector;

pub use composable::Composable;

pub trait Operator<Input>: Composable {
    type Output;
    type Error;

    /// # Errors
    /// This will return an error if there's some problem applying the operator.
    /// Given how general this concept is, there's no good way of saying here
    /// what that might be.
    fn apply(&self, input: Input, rng: &mut ThreadRng) -> Result<Self::Output, Self::Error>;
}
