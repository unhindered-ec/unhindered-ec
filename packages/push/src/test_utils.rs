use crate::{instruction::PushInstruction, push_vm::program::PushProgram};

/// Wrap an instruction in a [`PushProgram`], converting it to a
/// [`PushInstruction`] if necessary.
pub fn p(i: impl Into<PushInstruction>) -> PushProgram {
    PushProgram::Instruction(i.into())
}
