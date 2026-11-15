mod dup_block;
mod ifelse;
mod noop;
mod unless;
mod when;

use strum::IntoEnumIterator;

pub use self::{dup_block::DupBlock, ifelse::IfElse, noop::Noop, unless::Unless, when::When};
use super::{NumOpens, Perform, PushInstruction, common::StackInstruction};
use crate::{
    error::{InstructionResult, MapInstructionError},
    instruction::Instruction,
    push_vm::{HasStack, program::PushProgram, stack::StackError},
};

// TODO: This (and things like `IfElse`) are dependent on `PushInstruction`.
// They should be made generic over the instruction type.
// `IntoEnumIterator` is written by hand, because the `EnumIter` derive needs
// `Default` for the boxed `StackInstruction`.
#[derive(Debug, strum_macros::Display, Clone, Eq, PartialEq)]
#[must_use]
#[non_exhaustive]
pub enum ExecInstruction<I> {
    // Boxed to keep `ExecInstruction` small: a `PushProgram` holds a `Vec`.
    #[strum(to_string = "{0}")]
    Stack(Box<StackInstruction<PushProgram<I>>>),

    Noop(Noop),
    DupBlock(DupBlock<I>),
    When(When<I>),
    Unless(Unless<I>),
    IfElse(IfElse<I>),
}

impl<I> ExecInstruction<I> {
    pub fn pop() -> Self {
        Self::stack(StackInstruction::pop())
    }
    pub fn push(value: PushProgram<I>) -> Self {
        Self::stack(StackInstruction::push(value))
    }
    pub fn dup() -> Self {
        Self::stack(StackInstruction::dup())
    }
    pub fn swap() -> Self {
        Self::stack(StackInstruction::swap())
    }
    pub fn is_empty() -> Self {
        Self::stack(StackInstruction::is_empty())
    }
    pub fn stack_depth() -> Self {
        Self::stack(StackInstruction::stack_depth())
    }
    pub fn flush() -> Self {
        Self::stack(StackInstruction::flush())
    }
    fn stack(instruction: StackInstruction<PushProgram<I>>) -> Self {
        Self::Stack(Box::new(instruction))
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

// Keep this list in step with the `ExecInstruction` variants.
impl<I> IntoEnumIterator for ExecInstruction<I>
where
    I: Clone,
{
    type Iterator = std::vec::IntoIter<Self>;

    fn iter() -> Self::Iterator {
        StackInstruction::iter()
            .map(Self::stack)
            .chain([
                Self::noop(),
                Self::dup_block(),
                Self::when(),
                Self::unless(),
                Self::if_else(),
            ])
            .collect::<Vec<_>>()
            .into_iter()
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
            Self::Stack(instruction) => instruction.num_opens(),
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
            Self::Stack(instruction) => instruction.perform(state),
            Self::Noop(noop) => noop.perform(state).map_err_into(),
            Self::When(when) => when.perform(state).map_err_into(),
            Self::Unless(unless) => unless.perform(state).map_err_into(),
            Self::IfElse(if_else) => if_else.perform(state).map_err_into(),
            Self::DupBlock(dup) => dup.perform(state).map_err_into(),
        }
    }
}
