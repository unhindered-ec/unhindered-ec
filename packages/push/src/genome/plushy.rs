use crate::instruction::PushInstruction;
use ec_core::{
    generator::{collection::CollectionGenerator, Generator},
    genome::Genome,
};
use ec_linear::genome::Linear;
use rand::rngs::ThreadRng;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Plushy {
    instructions: Vec<PushInstruction>,
}

// TODO: We might want to implement some sort of `Into`
//   trait instead of just having a getter. Having something
//   like `to_instructions()` since we're cloning?
impl Plushy {
    #[must_use]
    pub fn get_instructions(&self) -> Vec<PushInstruction> {
        self.instructions.clone()
    }
}

impl Genome for Plushy {
    type Gene = PushInstruction;
}

impl Linear for Plushy {
    fn size(&self) -> usize {
        self.instructions.len()
    }

    fn gene_mut(&mut self, index: usize) -> Option<&mut Self::Gene> {
        self.instructions.get_mut(index)
    }
}

impl Generator<Plushy> for CollectionGenerator<Vec<PushInstruction>> {
    fn generate(&self, rng: &mut ThreadRng) -> anyhow::Result<Plushy> {
        let instructions: Vec<PushInstruction> = self.generate(rng)?;
        Ok(Plushy { instructions })
    }
}

impl IntoIterator for Plushy {
    type Item = PushInstruction;

    type IntoIter = std::vec::IntoIter<PushInstruction>;

    fn into_iter(self) -> Self::IntoIter {
        self.instructions.into_iter()
    }
}

impl FromIterator<PushInstruction> for Plushy {
    fn from_iter<T: IntoIterator<Item = PushInstruction>>(iterable: T) -> Self {
        Self {
            instructions: iterable.into_iter().collect(),
        }
    }
}

#[cfg(test)]
mod test {
    use std::sync::Arc;

    use crate::instruction::{BoolInstruction, IntInstruction};
    use ec_core::operator::mutator::Mutator;
    use ec_linear::mutator::umad::Umad;
    use rand::thread_rng;

    use super::*;

    #[test]
    #[allow(clippy::unwrap_used)]
    fn generator() {
        let instructions = vec![
            PushInstruction::IntInstruction(IntInstruction::Add),
            PushInstruction::IntInstruction(IntInstruction::Subtract),
            PushInstruction::IntInstruction(IntInstruction::Multiply),
            PushInstruction::IntInstruction(IntInstruction::ProtectedDivide),
        ];
        let mut rng = thread_rng();
        let plushy: Plushy = CollectionGenerator {
            size: 10,
            element_generator: instructions,
        }
        .generate(&mut rng)
        .unwrap();
        assert_eq!(10, plushy.instructions.len());
    }

    #[test]
    fn umad() {
        let mut rng = thread_rng();

        let instruction_options = [PushInstruction::InputVar(Arc::from("x"))];
        let umad = Umad::new(0.3, 0.3, instruction_options);

        let parent_instructions = vec![
            PushInstruction::IntInstruction(IntInstruction::Add),
            PushInstruction::BoolInstruction(BoolInstruction::BoolAnd),
            PushInstruction::BoolInstruction(BoolInstruction::BoolOr),
            PushInstruction::IntInstruction(IntInstruction::Multiply),
            PushInstruction::IntInstruction(IntInstruction::Add),
            PushInstruction::BoolInstruction(BoolInstruction::BoolAnd),
            PushInstruction::BoolInstruction(BoolInstruction::BoolOr),
            PushInstruction::IntInstruction(IntInstruction::Multiply),
            PushInstruction::IntInstruction(IntInstruction::Add),
            PushInstruction::BoolInstruction(BoolInstruction::BoolAnd),
            PushInstruction::BoolInstruction(BoolInstruction::BoolOr),
            PushInstruction::IntInstruction(IntInstruction::Multiply),
            PushInstruction::IntInstruction(IntInstruction::Add),
            PushInstruction::BoolInstruction(BoolInstruction::BoolAnd),
            PushInstruction::BoolInstruction(BoolInstruction::BoolOr),
            PushInstruction::IntInstruction(IntInstruction::Multiply),
            PushInstruction::IntInstruction(IntInstruction::Add),
            PushInstruction::BoolInstruction(BoolInstruction::BoolAnd),
            PushInstruction::BoolInstruction(BoolInstruction::BoolOr),
            PushInstruction::IntInstruction(IntInstruction::Multiply),
        ];
        let parent = Plushy {
            instructions: parent_instructions,
        };

        let child = umad.mutate(parent, &mut rng);

        #[allow(clippy::unwrap_used)]
        let num_inputs = child
            .unwrap()
            .instructions
            .iter()
            .filter(|c| **c == PushInstruction::InputVar(Arc::from("x")))
            .count();
        assert!(
            num_inputs > 0,
            "Expected at least one input instruction to be added, but none were."
        );
    }

    // TODO: Test that `Umad` works here on Plushy genomes.
}
