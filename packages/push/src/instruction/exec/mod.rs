mod dup_block;
mod ifelse;
mod noop;
mod unless;
mod when;

use strum_macros::EnumIter;

use self::{dup_block::DupBlock, ifelse::IfElse, noop::Noop, unless::Unless, when::When};
use super::{
    Instruction, NumOpens, PushInstruction,
    common::{
        dup::Dup, flush::Flush, is_empty::IsEmpty, pop::Pop, push_value::PushValue,
        stack_depth::StackDepth, swap::Swap,
    },
    instruction_error::PushInstructionError,
};
use crate::{
    error::InstructionResult,
    push_vm::{HasStack, program::PushProgram},
};

#[derive(Debug, strum_macros::Display, Clone, Eq, PartialEq, EnumIter)]
#[must_use]
pub enum ExecInstruction {
    // "Common" instructions specialized for the integer stack
    Pop(Pop<PushProgram>),
    #[strum(to_string = "{0}")]
    Push(Box<PushValue<PushProgram>>),
    Dup(Dup<PushProgram>),
    Swap(Swap<PushProgram>),
    IsEmpty(IsEmpty<PushProgram>),
    StackDepth(StackDepth<PushProgram>),
    Flush(Flush<PushProgram>),

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

    pub const fn dup_block() -> Self {
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
            Self::Pop(pop) => pop.num_opens(),
            Self::Push(push) => push.num_opens(),
            Self::Dup(dup) => dup.num_opens(),
            Self::Swap(swap) => swap.num_opens(),
            Self::IsEmpty(is_empty) => is_empty.num_opens(),
            Self::StackDepth(stack_depth) => stack_depth.num_opens(),
            Self::Flush(flush) => flush.num_opens(),
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
    S: Clone + HasStack<PushProgram> + HasStack<bool> + HasStack<i64>,
{
    type Error = PushInstructionError;

    fn perform(&self, state: S) -> InstructionResult<S, Self::Error> {
        match self {
            Self::Pop(pop) => pop.perform(state),
            Self::Push(push) => push.perform(state),
            Self::Dup(dup) => dup.perform(state),
            Self::Swap(swap) => swap.perform(state),
            Self::IsEmpty(is_empty) => is_empty.perform(state),
            Self::StackDepth(stack_depth) => stack_depth.perform(state),
            Self::Flush(flush) => flush.perform(state),
            Self::Noop(noop) => noop.perform(state),
            Self::When(when) => when.perform(state),
            Self::Unless(unless) => unless.perform(state),
            Self::IfElse(if_else) => if_else.perform(state),
            Self::DupBlock(dup) => dup.perform(state),
        }
    }
}
