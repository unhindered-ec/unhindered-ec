use crate::{instruction::Perform, push_vm::variables::VariableName};

pub trait HasInputs: Sized {
    type InputInstruction: Perform<Self>;

    fn get_input_instruction(&self, name: &VariableName) -> Option<Self::InputInstruction>;
}
