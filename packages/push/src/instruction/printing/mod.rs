mod char;
mod error;
mod string;

use std::{fmt::Display, io::Write, marker::PhantomData};

pub use char::{PrintChar, PrintNewline, PrintPeriod, PrintSpace};
pub use error::{AppendStdoutError, PrintingError};
pub use string::PrintString;

use super::NumOpens;
use crate::{
    error::{Error, InstructionResult},
    instruction::Instruction,
    push_vm::{HasStack, push_io::HasStdout},
};

/// An instruction that "prints" the top value of the stack
/// of type `T` to an internal buffer in the state.
///
/// This requires that `T: Display` and uses the `Display` implementation
/// of `T` to convert the value on the top of the stack to a `String`.
///
/// # Inputs
///
/// The `Print<T>` instruction takes the following inputs:
///    - `T` stack
///      - One value
///
/// # Behavior
///
/// The `Print<T>` instruction "prints" the top value of
/// the stack of type `T` to an internal buffer in the state.
///
/// This requires that `T: Display` and uses the `Display` implementation
/// of `T` to convert the value on the top of the stack to a `String`.
///
/// This does _not_ put a newline after the printed value;
/// see `PrintLn` for that behavior.
///
/// ## Action Table
///
/// The table below indicates the behavior in each of the different
/// cases.
///
///    - The `Stack<T>` column indicates the value of the top of the `T` stack,
///      or whether it exists.
///    - The "Success" column indicates whether the instruction succeeds, and if
///      not what kind of error is returned:
///       - ✅: success
///       - ❗: recoverable error, with links to the error kind
///       - ‼️: fatal error, with links to the error kind
///    - The "Note" column briefly summarizes the action state in that case
///
/// | `Stack<T>`  |  Success | Note |
/// | ------------- | ------------- | ------------- |
/// | exists    | ✅ | Prints the value to the internal buffer |
/// | missing | [❗..](crate::push_vm::stack::StackError::Underflow) | State is unchanged |
///
/// # Examples
///
/// ```
/// # use push::{
/// #    instruction::{Instruction, printing::Print},
/// #    push_vm::{HasStack, push_state::PushState},
/// # };
/// #
/// // Build an initial state with one value (`42`) on the integer (`i64`) stack.
/// let push_state = PushState::builder()
///     .with_max_stack_size(1)
///     .with_no_program()
///     .with_int_values([42])
///     .unwrap()
///     .with_instruction_step_limit(10)
///     .build();
/// // Print the value on the integer stack.
/// let mut result = Print::<i64>::default().perform(push_state).unwrap();
/// // Assert that the integer stack is now empty.
/// assert_eq!(result.stack::<i64>().size(), 0);
/// // Extract the printed output.
/// let output = result.stdout_string().unwrap();
/// // Assert that this is equal to the printed version of the one value
/// // that was on the integer stack, i.e., `"42"`.
/// assert_eq!(output, "42");
/// ```
/// # Errors
///
/// Returns a
/// [`StackError::Underflow`](crate::push_vm::stack::StackError::Underflow)
/// error when the `Stack<T>` is empty.
///
/// # Panics
///
/// This currently panics (due to an `unwrap()`) if attempting
/// to write the character fails.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Print<T> {
    pub(crate) _p: PhantomData<T>,
}

impl<T> Print<T> {
    #[must_use]
    pub const fn new() -> Self {
        Self { _p: PhantomData }
    }
}

impl<State, T> Instruction<State> for Print<T>
where
    T: Display,
    State: HasStack<T> + HasStdout,
{
    type Error = PrintingError;

    fn perform(&self, mut state: State) -> InstructionResult<State, Self::Error> {
        let value = match state.stack_mut::<T>().pop() {
            Ok(value) => value,
            Err(error) => return Err(Error::recoverable(state, error)),
        };
        match write!(state.stdout(), "{value}") {
            Ok(()) => Ok(state),
            Err(e) => Err(Error::fatal(state, AppendStdoutError::from(e))),
        }
    }
}

impl<T> NumOpens for Print<T> {
    fn num_opens(&self) -> usize {
        0
    }
}

/// An instruction that "prints" the top value of the stack
/// of type `T` to an internal buffer in the state, followed
/// by a newline.
///
/// This requires that `T: Display` and uses the `Display` implementation
/// of `T` to convert the value on the top of the stack to a `String`.
///
/// # Inputs
///
/// The `PrintLn<T>` instruction takes the following inputs:
///    - `T` stack
///      - One value
///
/// # Behavior
///
/// The `PrintLn<T>` instruction "prints" the top value of
/// the stack of type `T` to an internal buffer in the state,
/// followed by a newline.
///
/// This requires that `T: Display` and uses the `Display` implementation
/// of `T` to convert the value on the top of the stack to a `String`.
///
/// See `Print` for similar behavior that does not automatically
/// include a newline after the value.
///
/// ## Action Table
///
/// The table below indicates the behavior in each of the different
/// cases.
///
///    - The `Stack<T>` column indicates the value of the top of the `T` stack,
///      or whether it exists.
///    - The "Success" column indicates whether the instruction succeeds, and if
///      not what kind of error is returned:
///       - ✅: success
///       - ❗: recoverable error, with links to the error kind
///       - ‼️: fatal error, with links to the error kind
///    - The "Note" column briefly summarizes the action state in that case
///
/// | `Stack<T>`  |  Success | Note |
/// | ------------- | ------------- | ------------- |
/// | exists    | ✅ | Prints the value to the internal buffer followed by a newline |
/// | missing | [❗..](crate::push_vm::stack::StackError::Underflow) | State is unchanged |
///
/// ```
/// # use push::{
/// #    instruction::{Instruction, printing::PrintLn},
/// #    push_vm::{HasStack, push_state::PushState},
/// # };
/// #
/// // Build an initial state with one value (`42`) on the integer (`i64`) stack.
/// let push_state = PushState::builder()
///     .with_max_stack_size(1)
///     .with_no_program()
///     .with_int_values([42])
///     .unwrap()
///     .with_instruction_step_limit(10)
///     .build();
/// // Print the value on the integer stack.
/// let mut result = PrintLn::<i64>::default().perform(push_state).unwrap();
/// // Assert that the integer stack is now empty.
/// assert_eq!(result.stack::<i64>().size(), 0);
/// // Extract the printed output.
/// let output = result.stdout_string().unwrap();
/// // Assert that this is equal to the printed version of the one value
/// // that was on the integer stack, i.e., `"42"`, followed by a newline.
/// assert_eq!(output, "42\n");
/// ```
///
/// # Errors
///
/// Returns a
/// [`StackError::Underflow`](crate::push_vm::stack::StackError::Underflow)
/// error when the `Stack<T>` is empty.
///
/// # Panics
///
/// This currently panics (due to an `unwrap()`) if attempting
/// to write the character fails.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct PrintLn<T> {
    pub(crate) _p: PhantomData<T>,
}

impl<T> PrintLn<T> {
    #[must_use]
    pub const fn new() -> Self {
        Self { _p: PhantomData }
    }
}

impl<State, T> Instruction<State> for PrintLn<T>
where
    T: Display,
    State: HasStack<T> + HasStdout,
{
    type Error = PrintingError;

    fn perform(&self, mut state: State) -> InstructionResult<State, Self::Error> {
        let value = match state.stack_mut::<T>().pop() {
            Ok(value) => value,
            Err(error) => return Err(Error::recoverable(state, error)),
        };
        match writeln!(state.stdout(), "{value}") {
            Ok(()) => Ok(state),
            Err(e) => Err(Error::fatal(state, AppendStdoutError::from(e))),
        }
    }
}

impl<T> NumOpens for PrintLn<T> {
    fn num_opens(&self) -> usize {
        0
    }
}

#[cfg(test)]
mod tests {
    use ordered_float::OrderedFloat;

    use super::*;
    use crate::{
        genome::plushy::{Plushy, PushGene},
        instruction::{BoolInstruction, IntInstruction},
        list_into::vec_into,
        push_vm::{State, program::PushProgram, push_state::PushState, stack::StackError},
    };

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
            &PrintingError::StackError(StackError::Underflow {
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
            &PrintingError::StackError(StackError::Underflow {
                num_requested: 1,
                num_present: 0
            })
        );
    }

    #[test]
    fn print_multiple_values() {
        let genes: Vec<PushGene> = vec_into![
            IntInstruction::Print(Print::<i64>::default()),
            BoolInstruction::Print(Print::<bool>::default()),
            IntInstruction::PrintLn(PrintLn::<i64>::default()),
            IntInstruction::Print(Print::<i64>::default()),
        ];
        let program = Vec::<PushProgram>::from(Plushy::new(genes));
        let push_state = PushState::builder()
            .with_max_stack_size(4)
            .with_bool_values([false])
            .unwrap()
            .with_int_values([5, 8, 9])
            .unwrap()
            .with_program(program)
            .unwrap()
            .with_instruction_step_limit(10)
            .build();
        let mut result = push_state.run_to_completion().unwrap();
        assert_eq!(result.stack::<bool>().size(), 0);
        let output = result.stdout_string().unwrap();
        assert_eq!(output, "5false8\n9");
    }
}
