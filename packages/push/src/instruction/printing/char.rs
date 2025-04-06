use std::io::Write;

use super::super::{Instruction, instruction_error::PushInstructionError};
use crate::{error::InstructionResult, instruction::NumOpens, push_vm::push_io::HasStdout};

/// An instruction that "prints" a single character.
///
/// # Inputs
///
/// The `PrintChar<CHAR>` instruction doesn't take any
/// inputs because the character to print is specified in
/// the generic parameter.
///
/// # Behavior
///
/// The `PrintChar<CHAR>` instruction "prints" a single character specified
/// in the const generic for `PrintChar` to an internal buffer in the state.
///
/// ## Action Table
///
/// `PrintChar` doesn't affect any of the stacks.
///
/// # Examples
///
/// ```
/// # use push::{
/// #    instruction::{Instruction, printing::PrintChar},
/// #    push_vm::{push_io::HasStdout, push_state::PushState},
/// # };
/// #
/// // Build an initial state with empty stacks stack.
/// let push_state = PushState::builder()
///     .with_max_stack_size(0)
///     .with_no_program()
///     .with_instruction_step_limit(10)
///     .build();
/// // Print the character 'x'.
/// let mut result = PrintChar::<'x'>::default().perform(push_state).unwrap();
/// // Extract the printed output.
/// let output = result.stdout_string().unwrap();
/// // Assert that this is equal to "x".
/// assert_eq!(output, "x");
/// ```
/// # Panics
///
/// This currently panics (due to an `unwrap()`) if attempting
/// to write the character fails.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct PrintChar<const CHAR: char>;

impl<const CHAR: char> PrintChar<CHAR> {
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}

impl<State, const CHAR: char> Instruction<State> for PrintChar<CHAR>
where
    State: HasStdout,
{
    type Error = PushInstructionError;

    fn perform(&self, mut state: State) -> InstructionResult<State, Self::Error> {
        // We need to remove this `unwrap()`.
        write!(state.stdout(), "{CHAR}").unwrap();
        Ok(state)
    }
}

impl<const CHAR: char> NumOpens for PrintChar<CHAR> {
    fn num_opens(&self) -> usize {
        0
    }
}

pub type PrintSpace = PrintChar<' '>;
pub type PrintNewline = PrintChar<'\n'>;
pub type PrintPeriod = PrintChar<'.'>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::push_vm::push_state::PushState;

    #[test]
    fn print_char_a() {
        let push_state = PushState::builder()
            .with_max_stack_size(1)
            .with_no_program()
            .with_instruction_step_limit(10)
            .build();

        let mut result = PrintChar::<'a'>.perform(push_state).unwrap();
        let output = result.stdout_string().unwrap();
        assert_eq!(output, "a");
    }

    #[test]
    fn print_char_newline() {
        let push_state = PushState::builder()
            .with_max_stack_size(1)
            .with_no_program()
            .with_instruction_step_limit(10)
            .build();

        let mut result = PrintChar::<'\n'>.perform(push_state).unwrap();
        let output = result.stdout_string().unwrap();
        assert_eq!(output, "\n");
    }

    #[test]
    fn print_space() {
        let push_state = PushState::builder()
            .with_max_stack_size(1)
            .with_no_program()
            .with_instruction_step_limit(10)
            .build();

        let mut result = PrintSpace::default().perform(push_state).unwrap();
        let output = result.stdout_string().unwrap();
        assert_eq!(output, " ");
    }

    #[test]
    fn print_newline() {
        let push_state = PushState::builder()
            .with_max_stack_size(1)
            .with_no_program()
            .with_instruction_step_limit(10)
            .build();

        let mut result = PrintNewline::default().perform(push_state).unwrap();
        let output = result.stdout_string().unwrap();
        assert_eq!(output, "\n");
    }

    #[test]
    fn print_period() {
        let push_state = PushState::builder()
            .with_max_stack_size(1)
            .with_no_program()
            .with_instruction_step_limit(10)
            .build();

        let mut result = PrintPeriod::default().perform(push_state).unwrap();
        let output = result.stdout_string().unwrap();
        assert_eq!(output, ".");
    }
}
