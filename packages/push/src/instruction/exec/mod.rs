mod dup_block;
mod ifelse;
mod noop;
mod unless;
mod when;

use strum_macros::EnumIter;

use self::{dup_block::DupBlock, ifelse::IfElse, noop::Noop, unless::Unless, when::When};
use super::{instruction_error::PushInstructionError, Instruction, NumOpens, PushInstruction};
use crate::{
    error::InstructionResult,
    push_vm::{program::PushProgram, HasStack},
};

#[derive(Debug, strum_macros::Display, Clone, Eq, PartialEq, EnumIter)]
#[must_use]
pub enum ExecInstruction {
    Noop(Noop),
    DupBlock(DupBlock),
    When(When),
    Unless(Unless),
    IfElse(IfElse),
}

impl ExecInstruction {
    pub const fn noop() -> Self {
        Self::Noop(Noop)
    }

    pub const fn dup() -> Self {
        Self::DupBlock(DupBlock)
    }

    pub const fn when() -> Self {
        Self::When(When)
    }

    pub const fn unless() -> Self {
        Self::Unless(Unless)
    }

    pub const fn if_else() -> Self {
        Self::IfElse(IfElse)
    }
}
impl From<ExecInstruction> for PushInstruction {
    fn from(instr: ExecInstruction) -> Self {
        Self::Exec(instr)
    }
}

impl NumOpens for ExecInstruction {
    fn num_opens(&self) -> usize {
        match self {
            Self::Noop(noop) => noop.num_opens(),
            Self::DupBlock(dup) => dup.num_opens(),
            Self::When(when) => when.num_opens(),
            Self::Unless(unless) => unless.num_opens(),
            Self::IfElse(if_else) => if_else.num_opens(),
        }
    }
}

impl<S> Instruction<S> for ExecInstruction
where
    S: Clone + HasStack<PushProgram> + HasStack<bool>,
{
    type Error = PushInstructionError;

    fn perform(&self, state: S) -> InstructionResult<S, Self::Error> {
        match self {
            Self::Noop(noop) => noop.perform(state),
            Self::When(when) => when.perform(state),
            Self::Unless(unless) => unless.perform(state),
            Self::IfElse(if_else) => if_else.perform(state),
            Self::DupBlock(dup) => dup.perform(state),
        }
    }
}
