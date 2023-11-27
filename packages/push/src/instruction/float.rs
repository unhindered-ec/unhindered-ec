use crate::{instruction::MapInstructionError, push_vm::HasStack};

use super::{Instruction, PushInstruction, PushInstructionError};
use strum_macros::EnumIter;

#[derive(Debug, strum_macros::Display, Copy, Clone, EnumIter, Eq, PartialEq)]
#[non_exhaustive]
pub enum FloatInstruction {
    Push(f64),
    Add,
    Subtract,
    Multiply,
    ProtectedDivide,
}

impl From<FloatInstruction> for PushInstruction {
    fn from(instr: FloatInstruction) -> Self {
        Self::FloatInstruction(instr)
    }
}

impl<S> Instruction<S> for FloatInstruction
where
    S: Clone + HasStack<f64>,
{
    type Error = PushInstructionError;

    fn perform(&self, state: S) -> crate::error::InstructionResult<S, Self::Error> {
        match self {
            Self::Push(f) => state.with_push(*f).map_err_into(),
            _ => todo!(),
        }
    }
}
