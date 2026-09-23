#![cfg(test)]

use ec_core::{
    distributions::collection::ConvertToCollectionDistribution, operator::mutator::Mutator,
    uniform_distribution_of,
};
use ec_linear::{genome::Linear, mutator::umad::Umad};
use push::{
    genome::plushy::{ConvertToGeneGenerator, Plushy, PushGene},
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

impl std::fmt::Display for MyInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Add => f.write_str("add"),
            Self::Block => f.write_str("block"),
        }
    }
}

type MyPlushy = Plushy<MyInstruction>;
type MyProgram = PushProgram<MyInstruction>;

#[test]
fn custom_instruction_parses_into_the_expected_program() {
    let plushy: MyPlushy = Plushy::new([
        PushGene::new_instruction(MyInstruction::Add),
        PushGene::new_instruction(MyInstruction::Block),
        PushGene::new_instruction(MyInstruction::Add),
        PushGene::Close,
        PushGene::new_instruction(MyInstruction::Block),
    ]);

    // The block opened by the first `Block` captures the following `Add` and is
    // closed by the `Close`; the trailing `Block` opens an unterminated block
    // that parses as empty.
    let program: Vec<MyProgram> = plushy.clone().into();
    assert_eq!(
        program,
        vec![
            MyProgram::Instruction(MyInstruction::Add),
            MyProgram::Instruction(MyInstruction::Block),
            MyProgram::Block(vec![MyProgram::Instruction(MyInstruction::Add)]),
            MyProgram::Instruction(MyInstruction::Block),
            MyProgram::Block(vec![]),
        ]
    );

    assert_eq!(
        plushy.to_string(),
        "add block { add } block {",
        "Display should render `Close` as `}}` and append ` {{` per open"
    );
}

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

    let instructions = leaf_instructions(&program);
    assert_ne!(instructions, []);
    assert!(
        instructions
            .iter()
            .all(|i| matches!(i, MyInstruction::Add | MyInstruction::Block))
    );
}

fn leaf_instructions(program: &[MyProgram]) -> Vec<MyInstruction> {
    program
        .iter()
        .flat_map(|program| match program {
            MyProgram::Instruction(i) => vec![*i],
            MyProgram::Block(block) => leaf_instructions(block),
        })
        .collect()
}
