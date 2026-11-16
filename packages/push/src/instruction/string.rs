use strum_macros::EnumIter;

use super::{
    PushInstructionError, TypedInstruction,
    printing::{Print, PrintLn},
};
use crate::{
    error::{InstructionResult, MapInstructionError},
    instruction::{Instruction, Perform},
    push_vm::{
        push_io::HasStdout,
        stack::{HasStack, PushOnto},
    },
};

/// The instructions for the string stack.
pub type StringInstruction = TypedInstruction<String, StringOp>;

/// The string-specific instructions in [`StringInstruction`].
#[derive(Debug, strum_macros::Display, Clone, PartialEq, Eq, EnumIter)]
#[non_exhaustive]
#[must_use]
pub enum StringOp {
    Print(Print<String>),
    PrintLn(PrintLn<String>),

    /// Pops two strings and pushes the second-to-top string followed by the
    /// top string.
    Concat,
    /// Pops a string and pushes its length in bytes onto the int stack.
    Length,
    /// Pops an int and pushes its decimal text onto the string stack.
    FromInt,
}

impl StringInstruction {
    pub const fn print() -> Self {
        Self::Op(StringOp::Print(Print::new()))
    }
    pub const fn print_ln() -> Self {
        Self::Op(StringOp::PrintLn(PrintLn::new()))
    }
    pub const fn concat() -> Self {
        Self::Op(StringOp::Concat)
    }
    pub const fn length() -> Self {
        Self::Op(StringOp::Length)
    }
    pub const fn from_int() -> Self {
        Self::Op(StringOp::FromInt)
    }
}

impl Instruction for StringOp {}

impl<S> Perform<S> for StringOp
where
    S: Clone + HasStack<String> + HasStack<i64> + HasStdout,
{
    type Error = PushInstructionError;

    fn perform(&self, mut state: S) -> InstructionResult<S, Self::Error> {
        match self {
            Self::Print(print) => print.perform(state).map_err_into(),
            Self::PrintLn(println) => println.perform(state).map_err_into(),

            Self::Concat => state
                .stack_mut::<String>()
                .pop2()
                .map(|(top, second)| second + &top)
                .push_onto(state),
            // These two pop from one stack and push onto another, so check
            // first that the target stack has room.
            Self::Length => {
                let mut state = state.not_full::<i64>().map_err_into()?;
                state
                    .stack_mut::<String>()
                    .pop()
                    // ponytail: saturates; no string is longer than `i64::MAX` bytes.
                    .map(|s| i64::try_from(s.len()).unwrap_or(i64::MAX))
                    .push_onto(state)
            }
            Self::FromInt => {
                let mut state = state.not_full::<String>().map_err_into()?;
                state
                    .stack_mut::<i64>()
                    .pop()
                    .map(|i| i.to_string())
                    .push_onto(state)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Print, PrintLn, StringInstruction, StringOp};
    use crate::{
        instruction::{Perform, instruction_error::PushInstructionError},
        push_vm::{HasStack, push_state::PushState, stack::StackError},
    };

    // The top of the string stack is "b", and the value below it is "a".
    fn state<const N: usize>(max_stack_size: usize, ints: [i64; N]) -> PushState {
        PushState::builder()
            .with_max_stack_size(max_stack_size)
            .with_no_program()
            .with_string_values(["b".to_string(), "a".to_string()])
            .unwrap()
            .with_int_values(ints)
            .unwrap()
            .with_instruction_step_limit(1000)
            .build()
    }

    #[test]
    fn constructors_wrap_string_ops() {
        assert_eq!(
            [
                StringInstruction::print(),
                StringInstruction::print_ln(),
                StringInstruction::concat(),
                StringInstruction::length(),
                StringInstruction::from_int(),
            ],
            [
                StringOp::Print(Print::new()),
                StringOp::PrintLn(PrintLn::new()),
                StringOp::Concat,
                StringOp::Length,
                StringOp::FromInt,
            ]
            .map(StringInstruction::Op)
        );
    }

    #[test]
    fn concat_joins_second_then_top() {
        let result = StringOp::Concat.perform(state(2, [42])).unwrap();
        assert_eq!(result.stack::<String>().size(), 1);
        assert_eq!(result.stack::<String>().top(), Ok(&"ab".to_string()));
    }

    #[test]
    fn length_pushes_byte_count() {
        let result = StringOp::Length.perform(state(2, [42])).unwrap();
        assert_eq!(result.stack::<String>().size(), 1);
        assert_eq!(result.stack::<i64>().top2(), Ok((&1, &42)));
    }

    #[test]
    fn length_fails_when_int_stack_is_full() {
        let error = StringOp::Length.perform(state(2, [42, 7])).unwrap_err();
        assert!(matches!(
            error.error(),
            PushInstructionError::StackError(StackError::Overflow { .. })
        ));
    }

    #[test]
    fn from_int_pushes_decimal_text() {
        let result = StringOp::FromInt.perform(state(3, [42])).unwrap();
        assert_eq!(result.stack::<i64>().size(), 0);
        assert_eq!(result.stack::<String>().top(), Ok(&"42".to_string()));
    }

    #[test]
    fn from_int_fails_when_string_stack_is_full() {
        let error = StringOp::FromInt.perform(state(2, [42])).unwrap_err();
        assert!(matches!(
            error.error(),
            PushInstructionError::StackError(StackError::Overflow { .. })
        ));
    }

    #[test]
    fn print_writes_top_string() {
        let result = StringOp::Print(Print::new())
            .perform(state(2, [42]))
            .unwrap();
        assert_eq!(result.stdout_string().unwrap(), "b");
    }

    #[test]
    fn print_ln_writes_top_string_and_newline() {
        let result = StringOp::PrintLn(PrintLn::new())
            .perform(state(2, [42]))
            .unwrap();
        assert_eq!(result.stdout_string().unwrap(), "b\n");
    }
}
