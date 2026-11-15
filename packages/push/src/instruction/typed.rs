use std::fmt::{self, Display};

use strum::IntoEnumIterator;

use super::{
    Instruction, Perform, PushInstructionError,
    common::{Dup, Flush, IsEmpty, Pop, PushValue, StackDepth, StackInstruction, Swap},
};
use crate::error::{InstructionResult, MapInstructionError};

/// The instructions for a stack of type `T`: the common stack
/// instructions, plus the type-specific instructions in `Op`.
///
/// For example, [`BoolInstruction`](super::BoolInstruction) is
/// `TypedInstruction<bool, BoolOp>`.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[must_use]
pub enum TypedInstruction<T, Op> {
    Stack(StackInstruction<T>),
    Op(Op),
}

impl<T, Op> TypedInstruction<T, Op> {
    pub const fn pop() -> Self {
        Self::Stack(StackInstruction::pop())
    }
    pub const fn push(value: T) -> Self {
        Self::Stack(StackInstruction::push(value))
    }
    pub const fn dup() -> Self {
        Self::Stack(StackInstruction::dup())
    }
    pub const fn swap() -> Self {
        Self::Stack(StackInstruction::swap())
    }
    pub const fn is_empty() -> Self {
        Self::Stack(StackInstruction::is_empty())
    }
    pub const fn stack_depth() -> Self {
        Self::Stack(StackInstruction::stack_depth())
    }
    pub const fn flush() -> Self {
        Self::Stack(StackInstruction::flush())
    }
}

macro_rules! from_stack_instruction {
    ($($instruction:ident => $variant:ident),* $(,)?) => {
        $(
            impl<T, Op> From<$instruction<T>> for TypedInstruction<T, Op> {
                fn from(instruction: $instruction<T>) -> Self {
                    Self::Stack(StackInstruction::$variant(instruction))
                }
            }
        )*
    };
}

from_stack_instruction! {
    Pop => Pop,
    PushValue => Push,
    Dup => Dup,
    Swap => Swap,
    IsEmpty => IsEmpty,
    StackDepth => StackDepth,
    Flush => Flush,
}

impl<T, Op> Display for TypedInstruction<T, Op>
where
    T: Display,
    Op: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Stack(instruction) => instruction.fmt(f),
            Self::Op(op) => op.fmt(f),
        }
    }
}

// `Chain` is not an `ExactSizeIterator`, so the items go into a `Vec`.
impl<T, Op> IntoEnumIterator for TypedInstruction<T, Op>
where
    T: Clone + Default,
    Op: Clone + IntoEnumIterator,
{
    type Iterator = std::vec::IntoIter<Self>;

    fn iter() -> Self::Iterator {
        StackInstruction::iter()
            .map(Self::Stack)
            .chain(Op::iter().map(Self::Op))
            .collect::<Vec<_>>()
            .into_iter()
    }
}

impl<T, Op> Instruction for TypedInstruction<T, Op> {}

impl<S, T, Op> Perform<S> for TypedInstruction<T, Op>
where
    StackInstruction<T>: Perform<S, Error: Into<PushInstructionError>>,
    Op: Perform<S, Error: Into<PushInstructionError>>,
{
    type Error = PushInstructionError;

    fn perform(&self, state: S) -> InstructionResult<S, Self::Error> {
        match self {
            Self::Stack(instruction) => instruction.perform(state).map_err_into(),
            Self::Op(op) => op.perform(state).map_err_into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Not;

    use strum::IntoEnumIterator;
    use strum_macros::EnumIter;

    use super::TypedInstruction;
    use crate::{
        error::InstructionResult,
        instruction::{
            Perform,
            common::{Dup, Flush, IsEmpty, Pop, PushValue, StackDepth, StackInstruction, Swap},
        },
        push_vm::{
            HasStack,
            push_state::PushState,
            stack::{PushOnto, StackError},
        },
    };

    #[derive(Debug, Clone, Copy, PartialEq, Eq, strum_macros::Display, EnumIter)]
    enum TestOp {
        Not,
    }

    impl Perform<PushState> for TestOp {
        type Error = StackError;

        fn perform(&self, mut state: PushState) -> InstructionResult<PushState, StackError> {
            state
                .stack_mut::<bool>()
                .pop()
                .map(Not::not)
                .push_onto(state)
        }
    }

    type TestInstruction = TypedInstruction<bool, TestOp>;

    fn state() -> PushState {
        PushState::builder()
            .with_max_stack_size(10)
            .with_no_program()
            .with_bool_values([true])
            .unwrap()
            .with_instruction_step_limit(1000)
            .build()
    }

    #[test]
    fn constructors_wrap_stack_instructions() {
        assert_eq!(
            [
                TestInstruction::pop(),
                TestInstruction::push(true),
                TestInstruction::dup(),
                TestInstruction::swap(),
                TestInstruction::is_empty(),
                TestInstruction::stack_depth(),
                TestInstruction::flush(),
            ],
            [
                StackInstruction::pop(),
                StackInstruction::push(true),
                StackInstruction::dup(),
                StackInstruction::swap(),
                StackInstruction::is_empty(),
                StackInstruction::stack_depth(),
                StackInstruction::flush(),
            ]
            .map(TestInstruction::Stack)
        );
    }

    #[test]
    fn from_wraps_each_stack_instruction() {
        assert_eq!(
            [
                TestInstruction::from(Pop::new()),
                TestInstruction::from(PushValue::new(true)),
                TestInstruction::from(Dup::new()),
                TestInstruction::from(Swap::new()),
                TestInstruction::from(IsEmpty::new()),
                TestInstruction::from(StackDepth::new()),
                TestInstruction::from(Flush::new()),
            ],
            [
                TestInstruction::pop(),
                TestInstruction::push(true),
                TestInstruction::dup(),
                TestInstruction::swap(),
                TestInstruction::is_empty(),
                TestInstruction::stack_depth(),
                TestInstruction::flush(),
            ]
        );
    }

    #[test]
    fn iter_gives_stack_instructions_and_ops() {
        let all: Vec<TestInstruction> = TestInstruction::iter().collect();
        assert_eq!(all.len(), 8);
        assert!(all.contains(&TestInstruction::dup()));
        assert!(all.contains(&TestInstruction::Op(TestOp::Not)));
    }

    #[test]
    fn display_shows_stack_instruction_and_op() {
        assert_eq!(TestInstruction::push(true).to_string(), "Push(true)");
        assert_eq!(TestInstruction::Op(TestOp::Not).to_string(), "Not");
    }

    #[test]
    fn perform_runs_stack_instruction() {
        let result = TestInstruction::dup().perform(state()).unwrap();
        assert_eq!(result.stack::<bool>().size(), 2);
    }

    #[test]
    fn perform_runs_op() {
        let result = TestInstruction::Op(TestOp::Not).perform(state()).unwrap();
        assert_eq!(result.stack::<bool>().top(), Ok(&false));
    }
}
