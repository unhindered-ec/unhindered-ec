use crate::{
    error::InstructionResult,
    instruction::{Instruction, instruction_error::PushInstructionError},
    push_vm::{
        HasStack,
        stack::{PushOnto, StackDiscard},
    },
};

/// An instruction that takes the top value from the `char` stack and
/// pushes a `bool` on the bool stack that is `true` if that character
/// is a alphabetic (as defined in
/// [the Unicode standard](https://www.unicode.org/Public/UCD/latest/ucd/DerivedCoreProperties.txt)),
/// and `false` otherwise.
///
/// # Inputs
///
/// The `CharInstruction::IsAlphabetic` instruction takes the
/// following inputs:
///    - `char` stack
///      - One value
///
/// # Behavior
///
/// The `CharInstruction::IsAlphabetic` instruction takes the top
/// value of the `char` stack and pushes a `bool` onto the bool stack
/// that is `true` if that character is a alphabetic (as described in
/// [the Unicode standard](https://www.unicode.org/Public/UCD/latest/ucd/DerivedCoreProperties.txt)),
/// and `false` if it isn't.
///
/// ## Action Table
///
/// The table below indicates the behavior in each of the different
/// cases.
///
///    - The "`char` stack" column indicates the value of the top of the `char`
///      stack, or whether it exists, all before the instruction is executed.
///    - The "`bool` stack" column indicates the value of the top of the `bool`
///      stack after the instruction is executed.
///    - The "Success" column indicates whether the instruction succeeds, and if
///      not what kind of error is returned:
///       - ✅: success
///       - ❗: recoverable error, with links to the error kind
///       - ‼️: fatal error, with links to the error kind
///    - The "Note" column briefly summarizes the action state in that case
///
/// | `char` stack  | `bool` stack  |  Success | Note |
/// | ------------- | ------------- | ------------- | ------------- |
/// | exists | whether the `char` is alphabetic | ✅ | |
/// | missing | nothing is pushed onto the `bool` stack | [❗..](crate::push_vm::stack::StackError::Underflow) | State is unchanged |
///
/// # Errors
///
/// Returns a
/// [`StackError::Underflow`](crate::push_vm::stack::StackError::Underflow)
/// error when the `char` stack is empty.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct IsAlphabetic;

impl<S> Instruction<S> for IsAlphabetic
where
    S: Clone + HasStack<char> + HasStack<bool>,
{
    type Error = PushInstructionError;

    fn perform(&self, state: S) -> InstructionResult<S, Self::Error> {
        state
            .stack::<char>()
            .top()
            .map(|x| x.is_alphabetic())
            .push_onto(state)
            .with_stack_discard::<char>(1)
    }
}

/// An instruction that pushes a `bool` on the bool stack that is `true` if that
/// character on top of the character stack is a alphabetic (as defined in
/// [the Unicode standard](https://www.unicode.org/Public/UCD/latest/ucd/DerivedCoreProperties.txt)),
/// and `false` otherwise. This is *not* consuming in the sense that it leaves
/// the character on top of the character stack unchanged.
///
/// # Inputs
///
/// The `CharInstruction::IsAlphabetic` instruction takes the
/// following inputs:
///    - `char` stack
///      - One value
///
/// # Behavior
///
/// The `CharInstruction::IsAlphabetic` pushes a `bool` onto the bool stack
/// that is `true` if that character on top of the character stack is a
/// alphabetic (as described in [the Unicode standard](https://www.unicode.org/Public/UCD/latest/ucd/DerivedCoreProperties.txt)),
/// and `false` if it isn't. This is *not* consuming in the sense that it leaves
/// the character on top of the character stack unchanged.
///
/// ## Action Table
///
/// The table below indicates the behavior in each of the different
/// cases.
///
///    - The "`char` stack" column indicates the value of the top of the `char`
///      stack, or whether it exists, all before the instruction is executed.
///    - The "`bool` stack" column indicates the value of the top of the `bool`
///      stack after the instruction is executed.
///    - The "Success" column indicates whether the instruction succeeds, and if
///      not what kind of error is returned:
///       - ✅: success
///       - ❗: recoverable error, with links to the error kind
///       - ‼️: fatal error, with links to the error kind
///    - The "Note" column briefly summarizes the action state in that case
///
/// | `char` stack  | `bool` stack  |  Success | Note |
/// | ------------- | ------------- | ------------- | ------------- |
/// | exists, remains unchanged | whether the `char` is alphabetic | ✅ | The top of the `char` stack remains unchanged |
/// | missing | nothing is pushed onto the `bool` stack | [❗..](crate::push_vm::stack::StackError::Underflow) | State is unchanged |
///
/// # Errors
///
/// Returns a
/// [`StackError::Underflow`](crate::push_vm::stack::StackError::Underflow)
/// error when the `char` stack is empty.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct IsAlphabeticNonConsuming;

impl<S> Instruction<S> for IsAlphabeticNonConsuming
where
    S: Clone + HasStack<char> + HasStack<bool>,
{
    type Error = PushInstructionError;

    fn perform(&self, state: S) -> InstructionResult<S, Self::Error> {
        state
            .stack::<char>()
            .top()
            .map(|x| x.is_alphabetic())
            .push_onto(state)
    }
}

#[cfg(test)]
mod tests {
    use proptest::prop_assert_eq;
    use test_strategy::proptest;

    use super::IsAlphabetic;
    use crate::{
        instruction::{Instruction, char::is_alphabetic::IsAlphabeticNonConsuming},
        push_vm::{HasStack, push_state::PushState},
    };

    /// Performing `IsAlphabetic` when the `char` stack is empty should
    /// return a recoverable error with the state unchanged.
    #[test]
    fn is_alphabetic_empty_stack() {
        let state = PushState::builder()
            .with_max_stack_size(3)
            .with_int_values([5, 8, 9])
            .unwrap()
            .with_bool_values([true, false])
            .unwrap()
            .with_no_program()
            .build();
        let result = IsAlphabetic.perform(state).unwrap_err();
        assert!(result.is_recoverable());
        let result_state = result.state();
        assert_eq!(result_state.stack::<i64>().size(), 3);
        assert_eq!(result_state.stack::<bool>().size(), 2);
    }

    /// Performing `IsAlphabetic` when the `char` stack is empty should
    /// return a recoverable error with the state unchanged.
    #[test]
    fn is_alphabetic_non_consuming_empty_stack() {
        let state = PushState::builder()
            .with_max_stack_size(3)
            .with_int_values([5, 8, 9])
            .unwrap()
            .with_bool_values([true, false])
            .unwrap()
            .with_no_program()
            .build();
        let result = IsAlphabeticNonConsuming.perform(state).unwrap_err();
        assert!(result.is_recoverable());
        let result_state = result.state();
        assert_eq!(result_state.stack::<i64>().size(), 3);
        assert_eq!(result_state.stack::<bool>().size(), 2);
    }

    #[proptest]
    // We need to make sure that `x` is greater than `i64::MIN` since
    // we handle that case differently. This is described in the documentation
    // for `Abs`, and handled in the preceding test.
    fn is_alphabetic(c: char) {
        let state = PushState::builder()
            .with_max_stack_size(1)
            .with_char_values(std::iter::once(c))
            .unwrap()
            .with_no_program()
            .build();
        let result = IsAlphabetic.perform(state).unwrap();
        prop_assert_eq!(result.stack::<char>().size(), 0);
        prop_assert_eq!(result.stack::<bool>().size(), 1);
        prop_assert_eq!(*result.stack::<bool>().top().unwrap(), c.is_alphabetic());
    }

    #[proptest]
    // We need to make sure that `x` is greater than `i64::MIN` since
    // we handle that case differently. This is described in the documentation
    // for `Abs`, and handled in the preceding test.
    fn is_alphabetic_non_consuming(c: char) {
        let state = PushState::builder()
            .with_max_stack_size(1)
            .with_char_values(std::iter::once(c))
            .unwrap()
            .with_no_program()
            .build();
        let result = IsAlphabeticNonConsuming.perform(state).unwrap();
        prop_assert_eq!(result.stack::<char>().size(), 1);
        prop_assert_eq!(*result.stack::<char>().top().unwrap(), c);
        prop_assert_eq!(result.stack::<bool>().size(), 1);
        prop_assert_eq!(*result.stack::<bool>().top().unwrap(), c.is_alphabetic());
    }
}
