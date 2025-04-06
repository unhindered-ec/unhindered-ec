use std::io::Write;

use super::super::{Instruction, instruction_error::PushInstructionError};
use crate::{error::InstructionResult, push_vm::push_io::HasStdout};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct PrintChar<const CHAR: char>;

impl<const CHAR: char> PrintChar<CHAR> {
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

pub type PrintSpace = PrintChar<' '>;
pub type PrintNewline = PrintChar<'\n'>;

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
}
