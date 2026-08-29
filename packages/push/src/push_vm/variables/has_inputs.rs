use crate::{instruction::Instruction, push_vm::variables::VariableName};

pub trait HasInputs: Sized {
    // fn with_input(
    //     self,
    //     var_name: &VariableName,
    // ) -> InstructionResult<Self, <PushInstruction as Instruction<Self>>::Error>;

    type InputInstruction: Instruction<Self>;

    fn get_input_instruction(&self, name: &VariableName) -> Option<Self::InputInstruction>;
}
