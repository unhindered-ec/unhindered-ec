use crate::{
    error::InstructionResult,
    instruction::{Instruction, PushInstructionError},
    push_vm::{stack::PushOnto, HasStack},
};

/// ![`Negate` saturates on `i64::MIN`, returning `i64::MAX`](https://img.shields.io/badge/Saturates_on_overflow-orange)
///
/// An instruction that negates the top
/// value on the `i64` stack.
///
/// There's an edge case when the value is `i64::MIN` since
/// negating that generates a wrapping error. This is because the two's
/// complement representation used for integer types has an asymmetry
/// where the magnitude of `MIN` is one larger than the magnitude of `MAX`.
/// See <https://en.wikipedia.org/wiki/Two%27s_complement#Most_negative_number>
/// for additional details and examples.
///
/// In our implementation, `Negate` returns `i64::MAX` when it negates
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
/// The `IntInstruction::Negate` instruction takes the following inputs:
///    - `i64` stack
///      - One value
///
/// # Behavior
///
/// The `IntInstruction::Negate` instruction negates the top value of
/// the `i64` stack. The one exception (as described above) is when
/// the value is `i64::MIN`, where `Negate` removes it and pushes
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
/// | exists, not `i64::MIN` | ✅ | Negates top value of the `i64` stack |
/// | missing | [❗..](crate::push_vm::stack::StackError::Underflow) | State is unchanged |
///
/// # Errors
///
/// Returns a
/// [`StackError::Underflow`](crate::push_vm::stack::StackError::Underflow)
/// error when the `i64` stack is empty.
///
/// # Differences
#[allow(clippy::doc_markdown)]
/// Implementations of integer negation instructions in Clojure (e.g., Clojush
/// or Propeller) or Python (e.g., PyshGP) won't have the wrapping issue because
/// they act on arbitrary precision integers.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct Negate;

impl<S> Instruction<S> for Negate
where
    S: Clone + HasStack<i64>,
{
    type Error = PushInstructionError;

    fn perform(&self, state: S) -> InstructionResult<S, Self::Error> {
        let int_stack = state.stack::<i64>();
        int_stack
            .top()
            .map(|x| x.saturating_neg())
            .replace_on(1, state)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {

    use proptest::prop_assert_eq;
    use test_strategy::proptest;

    use crate::{
        instruction::{Instruction, IntInstruction},
        push_vm::{push_state::PushState, HasStack},
    };

    // We need to make sure `Negate` properly handles the
    // case where the value being negated is `i64::MIN`.
    // Simply negating that value will wrap around to
    // `i64::MAX_VALUE`, which is definitely not what we want.
    #[test]
    fn handle_i64_min() {
        let input = i64::MIN;
        let state = PushState::builder()
            .with_max_stack_size(1)
            .with_int_values(std::iter::once(input))
            .unwrap()
            .with_no_program()
            .build();
        let result = IntInstruction::negate().perform(state).unwrap();
        assert_eq!(result.stack::<i64>().size(), 1);
        assert_eq!(*result.stack::<i64>().top().unwrap(), i64::MAX);
    }

    #[proptest]
    // We need to make sure that `x` is greater than `i64::MIN` since
    // we handle that case differently. This is described in the documentation
    // for `Negate`, and handled in the preceding test.
    fn negate(#[filter(#x > i64::MIN)] x: i64) {
        let state = PushState::builder()
            .with_max_stack_size(1)
            .with_int_values(std::iter::once(x))
            .unwrap()
            .with_no_program()
            .build();
        let result = IntInstruction::negate().perform(state).unwrap();
        prop_assert_eq!(result.stack::<i64>().size(), 1);
        prop_assert_eq!(
            *result.stack::<i64>().top().unwrap(),
            x.checked_neg().unwrap()
        );
    }
}
