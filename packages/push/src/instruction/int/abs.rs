use crate::{
    error::InstructionResult,
    instruction::{instruction_error::PushInstructionError, Instruction},
    push_vm::{stack::PushOnto, HasStack},
};

/// An instruction that takes the absolute value of the top
/// value on the `i64` stack.
///
/// There's an edge case when the value is `i64::MIN` since
/// negating that generates a wrapping error. This is because the two's
/// complement representation used for integer types has an asymmetry
/// where the magnitude of `MIN` is one larger than the magnitude of `MAX`.
/// See <https://en.wikipedia.org/wiki/Two%27s_complement#Most_negative_number>
/// for additional details and examples.
///
/// In our implementation, `Abs` returns `i64::MAX` when taking the absolute
/// value of
/// `i64::MIN`. This isn't mathematically accurate, but is semantically
/// plausible since it converts the smallest negative number into the largest
/// positive number.
///
/// The alternative would be to have this instruction "fail"
/// in some way on `i64::MIN`, presumably by either skipping the instruction or
/// generating a fatal error and terminating program evaluation. Neither of
/// these options seem terribly reasonable from an evolutionary standpoint.
/// Skipping would leave the value on the stack, un-negated, which would
/// be quite "surprising" and almost certainly lead to unexpected behavior.
/// Failing doesn't seem to be in the spirit of Push, where we try to make
/// sure almost every instruction "succeeds" in some reasonable way.
///
/// # Inputs
///
/// The `IntInstruction::Abs` instruction takes the following inputs:
///    - `i64` stack
///      - One value
///
/// # Behavior
///
/// The `IntInstruction::Abs` instruction takes the absolute value of the top
/// value of the `i64` stack. The one exception (as described above) is when
/// the value is `i64::MIN`, where `Abs` removes it and pushes
/// on `i64::MAX` in its place.
///
/// ## Action Table
///
/// The table below indicates the behavior in each of the different
/// cases.
///
///    - The "`i64` stack" column indicates the value of the top of the `i64`
///      stack, or whether it exists.
///    - The "Success" column indicates whether the instruction succeeds, and if
///      not what kind of error is returned:
///       - ✅: success
///       - ❗: recoverable error, with links to the error kind
///       - ‼️: fatal error, with links to the error kind
///    - The "Note" column briefly summarizes the action state in that case
///
/// | `i64` stack  |  Success | Note |
/// | ------------- | ------------- | ------------- |
/// | `i64::MIN`    | ✅ | `i64::MIN` is replaced with `i64::MAX` |
/// | exists, not `i64::MIN` | ✅ | Takes the absolute value of the top value of the `i64` stack |
/// | missing | [❗..](crate::push_vm::stack::StackError::Underflow) | State is unchanged |
///
/// # Errors
///
/// Returns a
/// [`StackError::Underflow`](crate::push_vm::stack::StackError::Underflow)
/// error when the `i64` stack is empty.
///
/// # Differences
/// Implementations of integer negation instructions in Clojure (e.g., Clojush
/// or Propeller) or Python (e.g. `PyshGP`) won't have the wrapping issue
/// because they act on arbitrary precision integers.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct Abs;

impl<S> Instruction<S> for Abs
where
    S: Clone + HasStack<i64>,
{
    type Error = PushInstructionError;

    fn perform(&self, state: S) -> InstructionResult<S, Self::Error> {
        let int_stack = state.stack::<i64>();
        int_stack
            .top()
            .map(|x| x.saturating_abs())
            .replace_on(1, state)
    }
}

#[cfg(test)]
#[rustversion::attr(before(1.81), allow(clippy::unwrap_used))]
#[rustversion::attr(
    since(1.81),
    expect(
        clippy::unwrap_used,
        reason = "Panicking is the best way to deal with errors in unit tests"
    )
)]
mod tests {
    use proptest::prop_assert_eq;
    use test_strategy::proptest;

    use crate::{
        instruction::{Instruction, IntInstruction},
        push_vm::{push_state::PushState, HasStack},
    };

    // We need to make sure `Abs` properly handles the
    // case where the value we're the absolute value of is `i64::MIN`.
    // Simply taking the absolute value of `i64::MIN` will generate an overflow
    // error because `i64::MIN` has a larger magnitude than
    // the largest representable positive value (`i64::MAX`).
    // We want to using a saturating version of absolute value
    // that converts `i64::MIN` to `i64::MAX`.
    #[test]
    fn abs_with_i64_min() {
        let input = i64::MIN;
        let state = PushState::builder()
            .with_max_stack_size(1)
            .with_int_values(std::iter::once(input))
            .unwrap()
            .with_no_program()
            .build();
        let result = Abs.perform(state).unwrap();
        assert_eq!(result.stack::<i64>().size(), 1);
        assert_eq!(*result.stack::<i64>().top().unwrap(), i64::MAX);
    }

    #[proptest]
    // We need to make sure that `x` is greater than `i64::MIN` since
    // we handle that case differently. This is described in the documentation
    // for `Abs`, and handled in the preceding test.
    fn abs(#[strategy((i64::MIN+1)..=i64::MAX)] x: i64) {
        let state = PushState::builder()
            .with_max_stack_size(1)
            .with_int_values(std::iter::once(x))
            .unwrap()
            .with_no_program()
            .build();
        let result = Abs.perform(state).unwrap();
        prop_assert_eq!(result.stack::<i64>().size(), 1);
        prop_assert_eq!(
            *result.stack::<i64>().top().unwrap(),
            x.checked_abs().unwrap()
        );
    }
}
