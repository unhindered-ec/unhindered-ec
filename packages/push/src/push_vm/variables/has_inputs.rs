use crate::{instruction::Instruction, push_vm::variables::VariableName};

pub trait HasInputs: Sized {
    type InputInstruction: Instruction<Self>;

    fn get_input_instruction(&self, name: &VariableName) -> Option<Self::InputInstruction>;
}
