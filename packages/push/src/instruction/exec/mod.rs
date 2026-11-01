mod dup_block;
mod ifelse;
mod noop;
mod unless;
mod when;

use strum_macros::EnumIter;

pub use self::{dup_block::DupBlock, ifelse::IfElse, noop::Noop, unless::Unless, when::When};
use super::{
    NumOpens, Perform, PushInstruction,
    common::{Dup, Flush, IsEmpty, Pop, PushValue, StackDepth, Swap},
};
use crate::{
    error::{InstructionResult, MapInstructionError},
    instruction::Instruction,
    push_vm::{HasStack, program::PushProgram, stack::StackError},
};

// TODO: This (and things like `IfElse`) are dependent on `PushInstruction`.
// They should be made generic over the instruction type.
#[derive(Debug, strum_macros::Display, Clone, Eq, PartialEq, EnumIter)]
#[must_use]
#[non_exhaustive]
pub enum ExecInstruction<I> {
    // "Common" instructions specialized for the integer stack
    Pop(Pop<PushProgram<I>>),

    #[strum(to_string = "{0}")]
    Push(Box<PushValue<PushProgram<I>>>),
    Dup(Dup<PushProgram<I>>),
    Swap(Swap<PushProgram<I>>),
    IsEmpty(IsEmpty<PushProgram<I>>),
    StackDepth(StackDepth<PushProgram<I>>),
    Flush(Flush<PushProgram<I>>),

    Noop(Noop),
    DupBlock(DupBlock<I>),
    When(When<I>),
    Unless(Unless<I>),
    IfElse(IfElse<I>),
}

impl<I> ExecInstruction<I> {
    pub const fn pop() -> Self {
        Self::Pop(Pop::<PushProgram<_>>::new())
    }
    pub fn push(value: PushProgram<I>) -> Self {
        Self::Push(Box::new(PushValue::<PushProgram<_>>::new(value)))
    }
    pub const fn dup() -> Self {
        Self::Dup(Dup::<PushProgram<_>>::new())
    }
    pub const fn swap() -> Self {
        Self::Swap(Swap::<PushProgram<_>>::new())
    }
    pub const fn is_empty() -> Self {
        Self::IsEmpty(IsEmpty::<PushProgram<_>>::new())
    }
    pub const fn stack_depth() -> Self {
        Self::StackDepth(StackDepth::<PushProgram<_>>::new())
    }
    pub const fn flush() -> Self {
        Self::Flush(Flush::<PushProgram<_>>::new())
    }
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

impl From<ExecInstruction<Self>> for PushInstruction {
    fn from(instr: ExecInstruction<Self>) -> Self {
        Self::Exec(instr)
    }
}

impl<I> NumOpens for ExecInstruction<I> {
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

impl<I> Instruction for ExecInstruction<I> {}

impl<I, S> Perform<S> for ExecInstruction<I>
where
    S: Clone + HasStack<PushProgram<I>> + HasStack<bool> + HasStack<i64>,
    I: Clone,
{
    type Error = StackError;

    fn perform(&self, state: S) -> InstructionResult<S, Self::Error> {
        match self {
            Self::Pop(pop) => pop.perform(state).map_err_into(),
            Self::Push(push) => push.perform(state).map_err_into(),
            Self::Dup(dup) => dup.perform(state).map_err_into(),
            Self::Swap(swap) => swap.perform(state).map_err_into(),
            Self::IsEmpty(is_empty) => is_empty.perform(state).map_err_into(),
            Self::StackDepth(stack_depth) => stack_depth.perform(state).map_err_into(),
            Self::Flush(flush) => flush.perform(state).map_err_into(),
            Self::Noop(noop) => noop.perform(state).map_err_into(),
            Self::When(when) => when.perform(state).map_err_into(),
            Self::Unless(unless) => unless.perform(state).map_err_into(),
            Self::IfElse(if_else) => if_else.perform(state).map_err_into(),
            Self::DupBlock(dup) => dup.perform(state).map_err_into(),
        }
    }
}
