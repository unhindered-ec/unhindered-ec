#![cfg(test)]

use ec_core::{
    distributions::collection::ConvertToCollectionDistribution, operator::mutator::Mutator,
    uniform_distribution_of,
};
use ec_linear::{genome::Linear, mutator::umad::Umad};
use push::{
    genome::plushy::{ConvertToGeneGenerator, Plushy},
    instruction::NumOpens,
    push_vm::program::PushProgram,
};
use rand::{SeedableRng, prelude::Distribution, rngs::StdRng};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MyInstruction {
    Add,
    Block,
}

impl NumOpens for MyInstruction {
    fn num_opens(&self) -> usize {
        match self {
            Self::Add => 0,
            Self::Block => 1,
        }
    }
}

type MyPlushy = Plushy<MyInstruction>;
type MyProgram = PushProgram<MyInstruction>;

#[test]
fn custom_instruction_composes_across_the_full_pipeline() {
    // Fixed seed for determinism: if Umad, Collection, or GeneGenerator
    // change their RNG draw order/count, the seed's output will shift
    // and these assertions may need re-seeding.
    let mut rng = StdRng::seed_from_u64(42);

    let gene_generator =
        uniform_distribution_of![<MyInstruction> MyInstruction::Add, MyInstruction::Block]
            .into_gene_generator();

    let parent: MyPlushy = gene_generator.clone().into_collection(20).sample(&mut rng);
    assert_eq!(parent.size(), 20);

    let mutated = Umad::new(0.3, 0.3, gene_generator)
        .mutate(parent, &mut rng)
        .unwrap();
    assert_ne!(mutated.size(), 0);

    let program: Vec<MyProgram> = mutated.into();
    assert_ne!(program, []);
}
