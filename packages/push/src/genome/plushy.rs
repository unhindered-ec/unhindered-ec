use easy_cast::ConvApprox;
use ec_core::{
    generator::{collection::CollectionGenerator, Generator},
    genome::Genome,
};
use ec_linear::genome::Linear;
use rand::{rngs::ThreadRng, Rng};

use crate::instruction::PushInstruction;

#[derive(Clone, Eq, PartialEq)]
pub enum PushGene {
    Close,
    Instruction(PushInstruction),
}

impl<T> From<T> for PushGene
where
    T: Into<PushInstruction>,
{
    fn from(instruction: T) -> Self {
        Self::Instruction(instruction.into())
    }
}

impl std::fmt::Debug for PushGene {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Close => write!(f, "Close"),
            Self::Instruction(instruction) => instruction.fmt(f),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GeneGenerator {
    close_probability: f32,
    instructions: Vec<PushInstruction>,
}

impl GeneGenerator {
    #[must_use]
    pub fn new(close_probability: f32, instructions: Vec<PushInstruction>) -> Self {
        Self {
            close_probability,
            instructions,
        }
    }

    #[must_use]
    pub fn with_uniform_close_probability(instructions: Vec<PushInstruction>) -> Self {
        Self {
            close_probability: 1.0 / f32::conv_approx(instructions.len() + 1),
            instructions,
        }
    }
}

impl Generator<PushGene> for GeneGenerator {
    fn generate(&self, rng: &mut ThreadRng) -> anyhow::Result<PushGene> {
        if rng.gen::<f32>() < self.close_probability {
            Ok(PushGene::Close)
        } else {
            self.instructions.generate(rng).map(PushGene::Instruction)
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Plushy {
    genes: Vec<PushGene>,
}

// TODO: We might want to implement some sort of `Into`
// trait instead of just having a getter. Having something
// like `to_instructions()` since we're cloning?
impl Plushy {
    pub fn new(iterable: impl IntoIterator<Item = PushGene>) -> Self {
        Self {
            genes: iterable.into_iter().collect(),
        }
    }

    #[must_use]
    pub fn get_genes(&self) -> Vec<PushGene> {
        self.genes.clone()
    }
}

impl Genome for Plushy {
    type Gene = PushGene;
}

impl Linear for Plushy {
    fn size(&self) -> usize {
        self.genes.len()
    }

    fn gene_mut(&mut self, index: usize) -> Option<&mut Self::Gene> {
        self.genes.get_mut(index)
    }
}

impl Generator<Plushy> for CollectionGenerator<GeneGenerator> {
    fn generate(&self, rng: &mut ThreadRng) -> anyhow::Result<Plushy> {
        Ok(Plushy {
            genes: self.generate(rng)?,
        })
    }
}

impl IntoIterator for Plushy {
    type Item = PushGene;

    type IntoIter = std::vec::IntoIter<PushGene>;

    fn into_iter(self) -> Self::IntoIter {
        self.genes.into_iter()
    }
}

impl FromIterator<PushGene> for Plushy {
    fn from_iter<T: IntoIterator<Item = PushGene>>(iterable: T) -> Self {
        Self {
            genes: iterable.into_iter().collect(),
        }
    }
}

#[cfg(test)]
mod test {
    use ec_core::operator::mutator::Mutator;
    use ec_linear::mutator::umad::Umad;
    use rand::thread_rng;

    use super::*;
    use crate::{
        instruction::{variable_name::VariableName, BoolInstruction, IntInstruction},
        list_into::{arr_into, vec_into},
    };

    #[ignore = "this has about a 2.665% chance on failing at least once across the three test \
                runners in ci"]
    #[test]
    #[allow(clippy::unwrap_used)]
    fn generator() {
        let instructions: Vec<PushInstruction> = vec_into![
            IntInstruction::Add,
            IntInstruction::Subtract,
            IntInstruction::Multiply,
            IntInstruction::ProtectedDivide,
        ];
        let gene_generator = GeneGenerator::with_uniform_close_probability(instructions);
        let mut rng = thread_rng();
        let plushy: Plushy = CollectionGenerator {
            size: 10,
            element_generator: gene_generator,
        }
        .generate(&mut rng)
        .unwrap();
        assert_eq!(10, plushy.genes.len());
    }

    #[test]
    fn umad() {
        let mut rng = thread_rng();

        let instruction_options = arr_into![<PushGene> VariableName::from("x")];
        let umad = Umad::new(0.3, 0.3, instruction_options);

        let parent = Plushy {
            genes: vec_into![
                IntInstruction::Add,
                BoolInstruction::And,
                BoolInstruction::Or,
                IntInstruction::Multiply,
                IntInstruction::Add,
                BoolInstruction::And,
                BoolInstruction::Or,
                IntInstruction::Multiply,
                IntInstruction::Add,
                BoolInstruction::And,
                BoolInstruction::Or,
                IntInstruction::Multiply,
                IntInstruction::Add,
                BoolInstruction::And,
                BoolInstruction::Or,
                IntInstruction::Multiply,
                IntInstruction::Add,
                BoolInstruction::And,
                BoolInstruction::Or,
                IntInstruction::Multiply,
            ],
        };

        let child = umad.mutate(parent, &mut rng);

        #[allow(clippy::unwrap_used)]
        let num_inputs = child
            .unwrap()
            .genes
            .iter()
            .filter(|c| matches!(c, PushGene::Instruction(PushInstruction::InputVar(v)) if v == &VariableName::from("x")))
            .count();
        assert!(
            num_inputs > 0,
            "Expected at least one input instruction to be added, but none were."
        );
    }

    // TODO: Test that `Umad` works here on Plushy genomes.
}
