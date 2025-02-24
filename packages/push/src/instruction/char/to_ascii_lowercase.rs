use crate::{
    error::InstructionResult,
    instruction::{Instruction, instruction_error::PushInstructionError},
    push_vm::{HasStack, stack::PushOnto},
};

/// An instruction that takes the top value from the `char` stack and
/// converts it to an ASCII lowercase `char` if it's an ASCII uppercase
/// character, leaving it unchanged if it's not.
///
/// # Inputs
///
/// The `CharInstruction::ToAsciiLowercase` instruction takes the
/// following inputs:
///    - `char` stack
///      - One value
///
/// # Behavior
///
/// The `CharInstruction::ToAsciiLowercase` instruction takes the top
/// value of the `char` stack and converts it to an ASCII lowercase character
/// if it's an ASCII uppercase character, leaving it unchanged if it's not.
///
/// ## Action Table
///
/// The table below indicates the behavior in each of the different
/// cases.
///
///    - The "`char` stack" column indicates the value of the top of the `char`
///      stack, or whether it exists, all before the instruction is executed.
///    - The "Success" column indicates whether the instruction succeeds, and if
///      not what kind of error is returned:
///       - ✅: success
///       - ❗: recoverable error, with links to the error kind
///       - ‼️: fatal error, with links to the error kind
///    - The "Note" column briefly summarizes the action state in that case
///
/// | `char` stack  |  Success | Note |
/// | ------------- | ------------- | ------------- | ------------- |
/// | exists | ✅ | Replaces the character with its ASCII lowercase value if it was an ASCII uppercase |
/// | missing | nothing is pushed onto the `char` stack | [❗..](crate::push_vm::stack::StackError::Underflow) | State is unchanged |
///
/// # Errors
///
/// Returns a
/// [`StackError::Underflow`](crate::push_vm::stack::StackError::Underflow)
/// error when the `char` stack is empty.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct ToAsciiLowercase;

impl<S> Instruction<S> for ToAsciiLowercase
where
    S: Clone + HasStack<char>,
{
    type Error = PushInstructionError;

    fn perform(&self, state: S) -> InstructionResult<S, Self::Error> {
        let char_stack = state.stack::<char>();
        let lowercase_ascii_character = char_stack.top().map(char::to_ascii_lowercase);
        lowercase_ascii_character.replace_on(1, state)
    }
}

#[cfg(test)]
mod tests {
    use proptest::prop_assert_eq;
    use test_strategy::proptest;

    use super::*;
    use crate::push_vm::push_state::PushState;

    /// Performing `ToAsciiLowercase` when the `char` stack is empty
    /// should return a recoverable error with the state unchanged.
    #[test]
    fn to_ascii_lowercase_empty_stack() {
        let state = PushState::builder()
            .with_max_stack_size(3)
            .with_int_values([1, 2, 3])
            .unwrap()
            .with_bool_values([true, false])
            .unwrap()
            .with_no_program()
            .build();
        let result = ToAsciiLowercase.perform(state).unwrap_err();
        assert!(result.is_recoverable());
        let result_state = result.state();
        assert_eq!(result_state.stack::<i64>().size(), 3);
        assert_eq!(result_state.stack::<bool>().size(), 2);
    }

    #[test]
    fn convert_ascii_uppercase() {
        let input = 'M';
        let state = PushState::builder()
            .with_max_stack_size(1)
            .with_char_values(std::iter::once(input))
            .unwrap()
            .with_no_program()
            .build();
        let result = ToAsciiLowercase.perform(state).unwrap();
        assert_eq!(result.stack::<char>().size(), 1);
        assert_eq!(*result.stack::<char>().top().unwrap(), 'm');
    }

    // This should do nothing since the value on the character stack is already an
    // ASCII lowercase character.
    #[test]
    fn convert_ascii_lowercase() {
        let input = 'm';
        let state = PushState::builder()
            .with_max_stack_size(1)
            .with_char_values(std::iter::once(input))
            .unwrap()
            .with_no_program()
            .build();
        let result = ToAsciiLowercase.perform(state).unwrap();
        assert_eq!(result.stack::<char>().size(), 1);
        assert_eq!(*result.stack::<char>().top().unwrap(), 'm');
    }

    // This should do nothing since the value on the character stack is an ASCII
    // non-letter.
    #[test]
    fn convert_ascii_non_letter() {
        let input = '7';
        let state = PushState::builder()
            .with_max_stack_size(1)
            .with_char_values(std::iter::once(input))
            .unwrap()
            .with_no_program()
            .build();
        let result = ToAsciiLowercase.perform(state).unwrap();
        assert_eq!(result.stack::<char>().size(), 1);
        assert_eq!(*result.stack::<char>().top().unwrap(), '7');
    }

    // This should do nothing since the value on the character stack is a Unicode
    // character outside the ASCII set.
    #[test]
    fn convert_unicode() {
        let input = 'π';
        let state = PushState::builder()
            .with_max_stack_size(1)
            .with_char_values(std::iter::once(input))
            .unwrap()
            .with_no_program()
            .build();
        let result = ToAsciiLowercase.perform(state).unwrap();
        assert_eq!(result.stack::<char>().size(), 1);
        assert_eq!(*result.stack::<char>().top().unwrap(), 'π');
    }

    #[proptest]
    fn ascii_from_wrapping_integer_proptest(c: char) {
        let state = PushState::builder()
            .with_max_stack_size(1)
            .with_char_values(std::iter::once(c))
            .unwrap()
            .with_no_program()
            .build();
        let result = ToAsciiLowercase.perform(state).unwrap();
        prop_assert_eq!(result.stack::<char>().size(), 1);
        let top_char = *result.stack::<char>().top().unwrap();
        prop_assert_eq!(top_char, c.to_ascii_lowercase());
    }
}
