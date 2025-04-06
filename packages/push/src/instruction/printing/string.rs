use std::io::Write;

use super::super::{Instruction, instruction_error::PushInstructionError};
use crate::{error::InstructionResult, push_vm::push_io::HasStdout};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct PrintString(pub String);

impl<State> Instruction<State> for PrintString
where
    State: HasStdout,
{
    type Error = PushInstructionError;

    fn perform(&self, mut state: State) -> InstructionResult<State, Self::Error> {
        let stdout = state.stdout();
        // We need to remove this `unwrap()`.
        writeln!(stdout, "{}", self.0).unwrap();
        Ok(state)
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
        assert_eq!(output, "Hello, world!\n");
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
        assert_eq!(output, "\n");
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
        assert_eq!(output, "Line 1\nLine 2\n");
    }
}
