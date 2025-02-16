use crate::{
    error::InstructionResult,
    instruction::{Instruction, instruction_error::PushInstructionError},
    push_vm::{
        HasStack,
        stack::{PushOnto, StackDiscard},
    },
};

/// An instruction that takes the top value from the `i64` stack and
/// converts it to an ASCII `char`, modding it by 128 to ensure that
/// the resulting value is a legal ASCII character code.
///
/// By using `.euclid_rem()` to compute the modulus, we're guaranteed
/// that we can interpret any value on the `i64` as a legal ASCII character
/// code. An alternative would be to have this instruction "fail"
/// in some way if the value on the `i64` stack was outside the range
/// 0..128, presumably by either skipping the instruction or
/// generating a fatal error and terminating program evaluation. Neither of
/// these options seem terribly reasonable from an evolutionary standpoint.
/// Skipping would leave the value on the `i64` stack, which would
/// be quite "surprising" and almost certainly lead to unexpected behavior.
/// Failing doesn't seem to be in the spirit of Push, where we try to make
/// sure almost every instruction "succeeds" in some reasonable way.
///
/// # Inputs
///
/// The `CharInstruction::AsciiFromWrappingInteger` instruction takes the
/// following inputs:
///    - `i64` stack
///      - One value
///
/// # Behavior
///
/// The `CharInstruction::AsciiFromWrappingInteger` instruction takes the top
/// value of the `i64` stack and converts it to an ASCII character after taking
/// it modulo 128 to ensure that it's a legal ASCII character code.
///
/// ## Action Table
///
/// The table below indicates the behavior in each of the different
/// cases.
///
///    - The "`i64` stack" column indicates the value of the top of the `i64`
///      stack, or whether it exists, all before the instruction is executed.
///    - The "`char` stack" column indicates the value of the top of the `char`
///      stack after the instruction is executed.
///    - The "Success" column indicates whether the instruction succeeds, and if
///      not what kind of error is returned:
///       - ✅: success
///       - ❗: recoverable error, with links to the error kind
///       - ‼️: fatal error, with links to the error kind
///    - The "Note" column briefly summarizes the action state in that case
///
/// | `i64` stack  | `char` stack  |  Success | Note |
/// | ------------- | ------------- | ------------- | ------------- |
/// | exists | ASCII char | ✅ | |
/// | missing | nothing is pushed onto the `char` stack | [❗..](crate::push_vm::stack::StackError::Underflow) | State is unchanged |
///
/// # Errors
///
/// Returns a
/// [`StackError::Underflow`](crate::push_vm::stack::StackError::Underflow)
/// error when the `i64` stack is empty.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct AsciiFromWrappingInteger;

impl<S> Instruction<S> for AsciiFromWrappingInteger
where
    S: Clone + HasStack<i64> + HasStack<char>,
{
    type Error = PushInstructionError;

    fn perform(&self, state: S) -> InstructionResult<S, Self::Error> {
        let int_stack = state.stack::<i64>();
        #[expect(
            clippy::cast_possible_truncation,
            reason = "We know that after taking `.rem_euclid(128)`, the value will be in the \
                      range 0..128 and will fit in a `u8`."
        )]
        #[expect(
            clippy::as_conversions,
            reason = "We know that the value is in the range 0..128 and thus can be converted \
                      safely to a `u8` and then to a `char`."
        )]
        let ascii_character = int_stack.top().map(|x| (x.rem_euclid(128)) as u8 as char);
        ascii_character
            .push_onto(state)
            .with_stack_discard::<i64>(1)
    }
}

#[cfg(test)]
mod tests {
    use proptest::prop_assert_eq;
    use test_strategy::proptest;

    use super::*;
    use crate::push_vm::push_state::PushState;

    /// Performing `AsciiFromWrappingInteger` when the `i64` stack is empty
    /// should return a recoverable error with the state unchanged.
    #[test]
    fn ascii_from_wrapping_integer_empty_stack() {
        let state = PushState::builder()
            .with_max_stack_size(3)
            .with_char_values(['a', 'b', 'c'])
            .unwrap()
            .with_bool_values([true, false])
            .unwrap()
            .with_no_program()
            .build();
        let result = AsciiFromWrappingInteger.perform(state).unwrap_err();
        assert!(result.is_recoverable());
        let result_state = result.state();
        assert_eq!(result_state.stack::<char>().size(), 3);
        assert_eq!(result_state.stack::<bool>().size(), 2);
    }

    #[test]
    fn ascii_from_wrapping_integer() {
        let input = 65;
        let state = PushState::builder()
            .with_max_stack_size(1)
            .with_int_values(std::iter::once(input))
            .unwrap()
            .with_no_program()
            .build();
        let result = AsciiFromWrappingInteger.perform(state).unwrap();
        assert_eq!(result.stack::<char>().size(), 1);
        assert_eq!(*result.stack::<char>().top().unwrap(), 'A');
    }

    #[test]
    fn ascii_from_wrapping_integer_overflow() {
        let input = 128;
        let state = PushState::builder()
            .with_max_stack_size(1)
            .with_int_values(std::iter::once(input))
            .unwrap()
            .with_no_program()
            .build();
        let result = AsciiFromWrappingInteger.perform(state).unwrap();
        assert_eq!(result.stack::<char>().size(), 1);
        assert_eq!(*result.stack::<char>().top().unwrap(), '\0');
    }

    #[test]
    fn ascii_from_wrapping_integer_underflow() {
        let input = -1;
        let state = PushState::builder()
            .with_max_stack_size(1)
            .with_int_values(std::iter::once(input))
            .unwrap()
            .with_no_program()
            .build();
        let result = AsciiFromWrappingInteger.perform(state).unwrap();
        assert_eq!(result.stack::<char>().size(), 1);
        assert_eq!(*result.stack::<char>().top().unwrap(), '\u{7f}');
    }

    #[expect(
        clippy::as_conversions,
        reason = "We know that the value is in the range 0..128 so these conversions are safe."
    )]
    #[expect(
        clippy::cast_possible_truncation,
        reason = "We know that after taking `.rem_euclid(128)`, the value will be in the range \
                  0..128 and will fit in a `u8`."
    )]
    #[proptest]
    fn ascii_from_wrapping_integer_proptest(x: i64) {
        let state = PushState::builder()
            .with_max_stack_size(1)
            .with_int_values(std::iter::once(x))
            .unwrap()
            .with_no_program()
            .build();
        let result = AsciiFromWrappingInteger.perform(state).unwrap();
        prop_assert_eq!(result.stack::<i64>().size(), 0);
        prop_assert_eq!(result.stack::<char>().size(), 1);
        let top_char = *result.stack::<char>().top().unwrap();
        let top_char_ascii = top_char as u8;
        prop_assert_eq!(top_char_ascii, x.rem_euclid(128) as u8);
    }
}
