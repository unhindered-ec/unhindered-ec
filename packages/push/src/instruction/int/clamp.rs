use crate::{
    error::InstructionResult,
    instruction::{Instruction, instruction_error::PushInstructionError},
    push_vm::{HasStack, stack::PushOnto},
};

/// An instruction that clamps a value to within a given range.
///
/// # Inputs
///
/// The `Clamp` instruction takes the following inputs:
///    - Stack of type `i64`
///      - One value
///      - One minimum value
///      - One maximum value
///
/// # Behavior
///
/// The `Clamp` instruction takes three values from the top of the `i64` stack:
/// `value`, `min`, and `max`. The `min` and `max` values are the second and
/// third values on the `i64` stack, ordered so that the smaller is `min` and
/// the larger is `max`.
///
/// If `value` is less than `min`, `min` replaces `value` on the stack.
///
/// If `value` is greater than `max`, `max` replaces `value` on the stack.
///
/// If `value` is within the range `[min, max]` (inclusive on both sides), it
/// remains unchanged.
///
/// ## Action Table
///
/// The table below indicates the behavior in each of the different
/// cases.
///
///    - The "Value" column indicates the value being clamped, and is the top of
///      the `i64` stack.
///    - The "Min" column indicates the minimum value of the range and is the
///      second value on the `i64` stack.
///    - The "Max" column indicates the maximum value of the range and is the
///      third value on the `i64` stack.
///    - The "Result" column indicates the value left on the top of the stack.
///    - The "Success" column indicates whether the instruction succeeds, and if
///      not what kind of error is returned:
///       - ✅: success
///       - ❗: recoverable error, with links to the error kind
///       - ‼️: fatal error, with links to the error kind
///    - The "Note" column briefly summarizes the action state in that case
///
/// | Value  | Min | Max | Result | Success | Note |
/// | ------------- | ------------- | ------------- | ------------- | ------------- | ------------- |
/// | Less than Min | exists | exists | Min | ✅ | Value is replaced by Min |
/// | Greater than Max | exists | exists | Max | ✅ | Value is replaced by Max |
/// | In range [Min, Max] | exists | exists | Value | ✅ | Value remains unchanged |
/// | missing | irrelevant | irrelevant | irrelevant | [❗…](crate::push_vm::stack::StackError::Underflow) | State is unchanged |
/// | present | missing | irrelevant | irrelevant | [❗…](crate::push_vm::stack::StackError::Underflow) | State is unchanged |
/// | present | present | missing | irrelevant | [❗…](crate::push_vm::stack::StackError::Underflow) | State is unchanged |
///
/// # Errors
///
///
/// The only possible error here is
/// [`StackError::Underflow`](crate::push_vm::stack::StackError::Underflow)
/// and the instruction will then be ignored.
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct Clamp;

impl Clamp {
    pub const fn new() -> Self {
        Self
    }
}

impl<S> Instruction<S> for Clamp
where
    S: Clone + HasStack<i64>,
{
    type Error = PushInstructionError;

    fn perform(&self, mut state: S) -> InstructionResult<S, Self::Error> {
        let int_stack = state.stack_mut::<i64>();
        int_stack
            .top3()
            .map_err(PushInstructionError::from)
            .map(|(&value, &min, &max)| {
                let (min, max) = if min > max { (max, min) } else { (min, max) };
                value.clamp(min, max)
            })
            .replace_on(3, state)
    }
}

#[cfg(test)]
mod test {
    use proptest::prop_assert_eq;
    use test_strategy::proptest;

    use super::Clamp;
    use crate::{
        instruction::{Instruction, instruction_error::PushInstructionError},
        push_vm::{HasStack, push_state::PushState, stack::StackError},
    };

    #[test]
    fn clamp_less_than_min() {
        let state = PushState::builder()
            .with_max_stack_size(3)
            .with_int_values([5, 10, 20])
            .unwrap()
            .with_no_program()
            .with_instruction_step_limit(1000)
            .build();
        let result = Clamp.perform(state).unwrap();
        assert_eq!(result.stack::<i64>().top().unwrap(), &10);
        assert_eq!(result.stack::<i64>().size(), 1);
    }

    #[test]
    fn clamp_greater_than_max() {
        let state = PushState::builder()
            .with_max_stack_size(3)
            .with_int_values([25, 10, 20])
            .unwrap()
            .with_no_program()
            .with_instruction_step_limit(1000)
            .build();
        let result = Clamp.perform(state).unwrap();
        assert_eq!(result.stack::<i64>().top().unwrap(), &20);
        assert_eq!(result.stack::<i64>().size(), 1);
    }

    #[test]
    fn clamp_within_range() {
        let state = PushState::builder()
            .with_max_stack_size(3)
            .with_int_values([15, 10, 20])
            .unwrap()
            .with_no_program()
            .with_instruction_step_limit(1000)
            .build();
        let result = Clamp.perform(state).unwrap();
        assert_eq!(result.stack::<i64>().top().unwrap(), &15);
        assert_eq!(result.stack::<i64>().size(), 1);
    }

    #[test]
    fn clamp_min_equals_max() {
        let state = PushState::builder()
            .with_max_stack_size(3)
            .with_int_values([15, 10, 10])
            .unwrap()
            .with_no_program()
            .with_instruction_step_limit(1000)
            .build();
        let result = Clamp.perform(state).unwrap();
        assert_eq!(result.stack::<i64>().top().unwrap(), &10);
        assert_eq!(result.stack::<i64>().size(), 1);
    }

    #[test]
    fn clamp_underflow() {
        let state = PushState::builder()
            .with_max_stack_size(0)
            .with_no_program()
            .with_instruction_step_limit(1000)
            .build();
        let result = Clamp.perform(state).unwrap_err();
        assert!(result.is_recoverable());
        assert_eq!(
            result.error(),
            &PushInstructionError::StackError(StackError::Underflow {
                num_requested: 3,
                num_present: 0
            })
        );
    }

    #[test]
    fn clamp_underflow_one() {
        let state = PushState::builder()
            .with_max_stack_size(1)
            .with_int_values([1])
            .unwrap()
            .with_no_program()
            .with_instruction_step_limit(1000)
            .build();
        let result = Clamp.perform(state).unwrap_err();
        assert!(result.is_recoverable());
        assert_eq!(
            result.error(),
            &PushInstructionError::StackError(StackError::Underflow {
                num_requested: 3,
                num_present: 1
            })
        );
    }

    #[test]
    fn clamp_underflow_two() {
        let state = PushState::builder()
            .with_max_stack_size(2)
            .with_int_values([1, 2])
            .unwrap()
            .with_no_program()
            .with_instruction_step_limit(1000)
            .build();
        let result = Clamp.perform(state).unwrap_err();
        assert!(result.is_recoverable());
        assert_eq!(
            result.error(),
            &PushInstructionError::StackError(StackError::Underflow {
                num_requested: 3,
                num_present: 2
            })
        );
    }

    #[proptest]
    fn clamp_proptest(value: i64, min: i64, max: i64) {
        let state = PushState::builder()
            .with_max_stack_size(3)
            .with_int_values([value, min, max])
            .unwrap()
            .with_no_program()
            .with_instruction_step_limit(1000)
            .build();
        let result = Clamp.perform(state).unwrap();
        let (min, max) = if max < min { (max, min) } else { (min, max) };
        let expected = if value < min {
            min
        } else if value > max {
            max
        } else {
            value
        };
        prop_assert_eq!(result.stack::<i64>().top().unwrap(), &expected);
        prop_assert_eq!(result.stack::<i64>().size(), 1);
    }
}
