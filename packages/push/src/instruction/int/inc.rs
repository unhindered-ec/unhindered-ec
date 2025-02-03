use super::{IntInstruction, IntInstructionError};
use crate::{
    instruction::{Instruction, InstructionResult, PushInstructionError},
    push_vm::{HasStack, stack::PushOnto},
};

/// An instruction that increments (adds one to) the top
/// value on the `i64` stack.
///
/// There's an edge case when the value is `i64::MAX` since
/// adding one would overflow the `i64`.
/// In our implementation, `Inc` returns `IntInstructionError::Overflow`
/// when an attempt is made to increment `i64::MAX`. This does _not_
/// consume the value on the top of the stack, so the `i64::MAX` value
/// is still there for subsequent instructions.
///
/// An alternative would be to have this instruction use saturating addition,
/// so incrementing `i64::MAX` would generate `i64::MAX` again. This might
/// be the better option in evolutionary systems, but our current plan is
/// to implement that as a separate Push instruction so users can choose
/// which of these versions (or both) they wish to make available in their
/// system. Since
///
/// # Inputs
///
/// The `IntInstruction::Inc` instruction takes the following inputs:
///    - `i64` stack
///      - One value
///
/// # Behavior
///
/// The `IntInstruction::Inc` instruction adds one to the top
/// value of the `i64` stack. The one exception (as described above) is when
/// the value is `i64::MAX`, where `Inc` returns an
/// `IntInstructionError::Overflow` error.
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
/// | `i64::MAX`    | [❗..](crate::instruction::IntInstructionError::Overflow) | State is unchanged |
/// | exists, not `i64::MAX` | ✅ | Adds one to the top value of the `i64` stack |
/// | missing | [❗..](crate::push_vm::stack::StackError::Underflow) | State is unchanged |
///
/// # Errors
///
/// Returns an
/// [`IntInstructionError::Overflow`]
/// error when the top of the `i64` stack is `i64::MAX`. It does _not_ consume
/// the `i64::MAX` value from the top of the `i64` stack.
///
/// Returns a
/// [`StackError::Underflow`](crate::push_vm::stack::StackError::Underflow)
/// error when the `i64` stack is empty.
///
/// # Differences
/// Implementations of integer increment instructions in Clojure (e.g., Clojush
/// or Propeller) or Python (e.g. `PyshGP`) won't have the wrapping issue
/// because they act on arbitrary precision integers.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct Inc;

impl<S> Instruction<S> for Inc
where
    S: Clone + HasStack<i64>,
{
    type Error = PushInstructionError;

    fn perform(&self, state: S) -> InstructionResult<S, Self::Error> {
        let int_stack = state.stack::<i64>();
        int_stack
            .top()
            .map_err(PushInstructionError::from)
            .map(|&i| i.checked_add(1))
            .and_then(|v| {
                v.ok_or(IntInstructionError::Overflow {
                    op: IntInstruction::Inc(*self),
                })
                .map_err(Into::into)
            })
            .replace_on(1, state)
    }
}

#[cfg(test)]
mod tests {
    use test_strategy::proptest;

    use super::Inc;
    use crate::{
        instruction::{Instruction, IntInstruction, IntInstructionError},
        push_vm::{HasStack, push_state::PushState},
    };

    #[test]
    fn inc_works() {
        let state = PushState::builder()
            .with_max_stack_size(1)
            .with_int_values(std::iter::once(1))
            .unwrap()
            .with_no_program()
            .build();
        let result = Inc.perform(state).unwrap();
        assert_eq!(*result.stack::<i64>().top().unwrap(), 2);
    }

    #[test]
    fn inc_overflows() {
        let x = i64::MAX;
        let state = PushState::builder()
            .with_max_stack_size(1)
            .with_int_values(std::iter::once(x))
            .unwrap()
            .with_no_program()
            .build();

        let result = Inc.perform(state).unwrap_err();
        assert_eq!(result.state().stack::<i64>().size(), 1);
        assert_eq!(result.state().stack::<i64>().top().unwrap(), &i64::MAX);
        assert_eq!(
            result.error(),
            &IntInstructionError::Overflow {
                op: IntInstruction::Inc(Inc)
            }
            .into()
        );
        assert!(result.is_recoverable());
    }

    #[proptest]
    fn inc_does_not_crash(#[any] x: i64) {
        let state = PushState::builder()
            .with_max_stack_size(1)
            .with_int_values(std::iter::once(x))
            .unwrap()
            .with_no_program()
            .build();
        let _ = Inc.perform(state);
    }
}
