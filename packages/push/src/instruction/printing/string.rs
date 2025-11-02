use std::io::Write;

use super::{super::Instruction, error::AppendStdoutError};
use crate::{
    error::{Error, InstructionResult},
    instruction::NumOpens,
    push_vm::push_io::HasStdout,
};

/// An instruction that "prints" a string specified when the instruction
/// is created.
///
/// # Inputs
///
/// The `PrintString` instruction doesn't take any
/// inputs because the string to print is specified in
/// when the instruction is created.
///
/// # Behavior
///
/// The `PrintString` instruction "prints" a specified string to an internal
/// buffer in the state.
///
/// When we support a `String` stack then `Print<String>` will print the
/// top value of that stack. This instruction is intended more for printing
/// "constant" strings that are specified when the instruction is created.
/// This is potentially use for problems with strings that are embedded in
/// the problem statement such as "Fizz" and "Buzz" for the Fizz Buzz problem.
///
/// ## Action Table
///
/// `PrintString` doesn't affect any of the stacks.
///
/// # Examples
///
/// ```
/// # use push::{
/// #    instruction::{Instruction, printing::PrintString},
/// #    push_vm::push_state::PushState,
/// # };
/// #
/// // Build an initial state with empty stacks stack.
/// let push_state = PushState::builder()
///     .with_max_stack_size(0)
///     .with_no_program()
///     .with_instruction_step_limit(10)
///     .build();
/// // Print the string "Hello".
/// let mut result = PrintString("Hello".to_string())
///     .perform(push_state)
///     .unwrap();
/// // Extract the printed output.
/// let output = result.stdout_string().unwrap();
/// // Assert that this is equal to "Hello".
/// assert_eq!(output, "Hello");
/// ```
/// # Panics
///
/// This currently panics (due to an `unwrap()`) if attempting
/// to write the character fails.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct PrintString(pub String);

impl PrintString {
    #[must_use]
    pub const fn new(string: String) -> Self {
        Self(string)
    }
}

impl<State> Instruction<State> for PrintString
where
    State: HasStdout,
{
    type Error = AppendStdoutError;

    fn perform(&self, mut state: State) -> InstructionResult<State, Self::Error> {
        match write!(state.stdout(), "{}", self.0) {
            Ok(()) => Ok(state),
            Err(e) => Err(Error::fatal(state, e)),
        }
    }
}

impl NumOpens for PrintString {
    fn num_opens(&self) -> usize {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::push_vm::push_state::PushState;

    #[test]
    fn print_string_works() {
        let state = PushState::builder()
            .with_max_stack_size(0)
            .with_no_program()
            .with_instruction_step_limit(10)
            .build();

        let print_hello = PrintString("Hello, world!".to_string());
        let mut result = print_hello.perform(state).unwrap();
        let output = result.stdout_string().unwrap();
        assert_eq!(output, "Hello, world!");
    }

    #[test]
    fn print_empty_string_works() {
        let state = PushState::builder()
            .with_max_stack_size(0)
            .with_no_program()
            .with_instruction_step_limit(10)
            .build();

        let print_empty = PrintString(String::new());
        let mut result = print_empty.perform(state).unwrap();
        let output = result.stdout_string().unwrap();
        assert_eq!(output, "");
    }

    #[test]
    fn print_string_with_newline_works() {
        let state = PushState::builder()
            .with_max_stack_size(0)
            .with_no_program()
            .with_instruction_step_limit(10)
            .build();

        let print_newline = PrintString("Line 1\nLine 2".to_string());
        let mut result = print_newline.perform(state).unwrap();
        let output = result.stdout_string().unwrap();
        assert_eq!(output, "Line 1\nLine 2");
    }
}
