use std::{fmt::Display, io::Write, marker::PhantomData};

use super::super::instruction_error::PushInstructionError;
use crate::{
    error::{Error, InstructionResult},
    instruction::Instruction,
    push_vm::{HasStack, push_io::HasStdout},
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Print<T> {
    pub(crate) _p: PhantomData<T>,
}

impl<State, T> Instruction<State> for Print<T>
where
    T: Display,
    State: HasStack<T> + HasStdout,
{
    type Error = PushInstructionError;

    fn perform(&self, mut state: State) -> InstructionResult<State, Self::Error> {
        let value = match state.stack_mut::<T>().pop() {
            Ok(value) => value,
            Err(error) => return Err(Error::recoverable(state, error)),
        };
        let stdout = state.stdout();
        // We need to remove this `unwrap()`.
        write!(stdout, "{value}").unwrap();
        Ok(state)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct PrintLn<T> {
    pub(crate) _p: PhantomData<T>,
}

impl<State, T> Instruction<State> for PrintLn<T>
where
    T: Display,
    State: HasStack<T> + HasStdout,
{
    type Error = PushInstructionError;

    fn perform(&self, mut state: State) -> InstructionResult<State, Self::Error> {
        let value = match state.stack_mut::<T>().pop() {
            Ok(value) => value,
            Err(error) => return Err(Error::recoverable(state, error)),
        };
        let stdout = state.stdout();
        // We need to remove this `unwrap()`.
        writeln!(stdout, "{value}").unwrap();
        Ok(state)
    }
}

#[cfg(test)]
mod tests {
    use ordered_float::OrderedFloat;

    use super::*;
    use crate::push_vm::{push_state::PushState, stack::StackError};

    #[test]
    fn print_int() {
        let push_state = PushState::builder()
            .with_max_stack_size(1)
            .with_no_program()
            .with_int_values([42])
            .unwrap()
            .with_instruction_step_limit(10)
            .build();
        let mut result = Print::<i64>::default().perform(push_state).unwrap();
        assert_eq!(result.stack::<i64>().size(), 0);
        let output = result.stdout_string().unwrap();
        assert_eq!(output, "42");
    }

    #[test]
    fn print_float() {
        let push_state = PushState::builder()
            .with_max_stack_size(1)
            .with_float_values([OrderedFloat(5.89)])
            .unwrap()
            .with_no_program()
            .with_instruction_step_limit(10)
            .build();
        let mut result = Print::<OrderedFloat<f64>>::default()
            .perform(push_state)
            .unwrap();
        assert_eq!(result.stack::<OrderedFloat<f64>>().size(), 0);
        let output = result.stdout_string().unwrap();
        assert_eq!(output, "5.89");
    }

    #[test]
    fn print_bool() {
        let push_state = PushState::builder()
            .with_max_stack_size(1)
            .with_bool_values([true])
            .unwrap()
            .with_no_program()
            .with_instruction_step_limit(10)
            .build();
        let mut result = Print::<bool>::default().perform(push_state).unwrap();
        assert_eq!(result.stack::<bool>().size(), 0);
        let output = result.stdout_string().unwrap();
        assert_eq!(output, "true");
    }

    #[test]
    fn print_underflow() {
        let push_state = PushState::builder()
            .with_max_stack_size(1)
            .with_no_program()
            .with_instruction_step_limit(10)
            .build();
        let result = Print::<i64>::default().perform(push_state).unwrap_err();
        assert!(result.is_recoverable());
        assert_eq!(
            result.error(),
            &PushInstructionError::StackError(StackError::Underflow {
                num_requested: 1,
                num_present: 0
            })
        );
    }

    #[test]
    fn println_int() {
        let push_state = PushState::builder()
            .with_max_stack_size(1)
            .with_no_program()
            .with_int_values([42])
            .unwrap()
            .with_instruction_step_limit(10)
            .build();
        let mut result = PrintLn::<i64>::default().perform(push_state).unwrap();
        assert_eq!(result.stack::<i64>().size(), 0);
        let output = result.stdout_string().unwrap();
        assert_eq!(output, "42\n");
    }

    #[test]
    fn println_float() {
        let push_state = PushState::builder()
            .with_max_stack_size(1)
            .with_float_values([OrderedFloat(5.89)])
            .unwrap()
            .with_no_program()
            .with_instruction_step_limit(10)
            .build();
        let mut result = PrintLn::<OrderedFloat<f64>>::default()
            .perform(push_state)
            .unwrap();
        assert_eq!(result.stack::<OrderedFloat<f64>>().size(), 0);
        let output = result.stdout_string().unwrap();
        assert_eq!(output, "5.89\n");
    }

    #[test]
    fn println_bool() {
        let push_state = PushState::builder()
            .with_max_stack_size(1)
            .with_bool_values([false])
            .unwrap()
            .with_no_program()
            .with_instruction_step_limit(10)
            .build();
        let mut result = PrintLn::<bool>::default().perform(push_state).unwrap();
        assert_eq!(result.stack::<bool>().size(), 0);
        let output = result.stdout_string().unwrap();
        assert_eq!(output, "false\n");
    }

    #[test]
    fn println_underflow() {
        let push_state = PushState::builder()
            .with_max_stack_size(1)
            .with_no_program()
            .with_instruction_step_limit(10)
            .build();
        let result = PrintLn::<i64>::default().perform(push_state).unwrap_err();
        assert!(result.is_recoverable());
        assert_eq!(
            result.error(),
            &PushInstructionError::StackError(StackError::Underflow {
                num_requested: 1,
                num_present: 0
            })
        );
    }
}
