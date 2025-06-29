use std::marker::PhantomData;

use crate::{
    error::MapInstructionError,
    instruction::{Instruction, NumOpens, instruction_error::PushInstructionError},
    push_vm::HasStack,
};

/// An instruction that pushes a integer value onto the int stack
/// that is the number of elements in the stack of type `T`.
///
/// # Inputs
///
/// The `StackDepth` instruction doesn't take any inputs from
/// the stacks, but does compute the number of elements on the
/// stack of type `T`.
///
/// # Behavior
///
/// The `StackDepth` instruction pushes a value onto the int
/// stack that is the number of items currently on the stack
/// of type `T` prior to the execution of this instruction.
///
/// If the type `T` is `int`, this will push the size of the `int`
/// stack prior to the execution of this instruction onto the `int` stack,
/// causing it to grow by one. If the `int`
/// stack was empty, it would push the value 0 onto that stack.
/// Otherwise the stack of type `T` is unchanged.
///
/// If the int stack is full before this instruction is performed, then
/// a fatal [`StackError::Overflow`](crate::push_vm::stack::StackError::Overflow) is returned,
/// terminating the running of the program.
///
/// If the size of the stack exceeds the maximum value of an
/// `i64`, then `i64::MAX` will be pushed onto the int stack. This would only
/// happen if there are 2^63 items on `Stack<T>`, which probably should
/// not be allowed to happen since that is almost certainly the result
/// of an infinite loop or similar and we should use `.with_max_stack_size()`
/// to set the limit on stack sizes much lower than 2^63.
///
/// ## Action Table
///
/// The table below indicates the behavior in each of the different
/// cases.
///
///    - The "`Stack<int> full`" column indicates whether the stack of type
///      `bool` is full before this instruction is performed.
///    - The "Success" column indicates whether the instruction succeeds, and if
///      not what kind of error is returned:
///       - ✅: success (so two copies of the top block on the `Exec stack`)
///       - ❗: recoverable error, with links to the error kind
///       - ‼️: fatal error, with links to the error kind
///    - The "Note" column briefly summarizes the action state in that case
///
/// | `Stack<int>` full |  Success | Note |
/// | ------------- | ------------- | ------------- |
/// | false | ✅ | the size of `Stack<T>` is pushed onto the int stack |
/// | true | [‼️..](crate::push_vm::stack::StackError::Overflow) | Program is terminated |
///
/// Note that if the size of `Stack<T>` is too big to fit in an `i64`, then
/// `i64::MAX` will be pushed onto the int stack instead.
///
/// # Errors
///
/// This returns a [`Error::Fatal`](crate::error::Error::Fatal)
/// [`StackError::Overflow`](crate::push_vm::stack::StackError::Overflow)
/// error if pushing the depth overflows the integer stack.
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub struct StackDepth<T> {
    _p: PhantomData<T>,
}

impl<T> StackDepth<T> {
    #[must_use]
    pub const fn new() -> Self {
        Self { _p: PhantomData }
    }
}

impl<T> NumOpens for StackDepth<T> {
    fn num_opens(&self) -> usize {
        0
    }
}

impl<S, T> Instruction<S> for StackDepth<T>
where
    S: HasStack<T> + HasStack<i64>,
{
    type Error = PushInstructionError;

    fn perform(&self, state: S) -> crate::error::InstructionResult<S, Self::Error> {
        // If the number of items on the stack is too large to fit in an `i64`, then
        // we'll replace it with `i64::MAX`.
        let stack_size = state.stack::<T>().size().try_into().unwrap_or(i64::MAX);
        state.with_push(stack_size).map_err_into()
    }
}

// We aren't testing the case where a stack has 2^63 items and thus overflows
// `i64` because we can't reasonably create stack with that many items on
// current hardware.

#[cfg(test)]
mod tests {
    use proptest::sample::size_range;
    use test_strategy::proptest;

    use crate::{
        instruction::{
            Instruction, common::stack_depth::StackDepth, instruction_error::PushInstructionError,
        },
        push_vm::{HasStack, push_state::PushState, stack::StackError},
    };

    #[test]
    fn stack_size_for_empty_stack() {
        let state = PushState::builder()
            .with_max_stack_size(1)
            .with_no_program()
            .with_instruction_step_limit(1000)
            .build();
        let stack_depth = StackDepth::<bool>::new();
        let state = stack_depth.perform(state).unwrap();
        assert_eq!(state.stack::<i64>().top().unwrap(), &0);
    }

    #[proptest]
    #[expect(
        clippy::arithmetic_side_effects,
        reason = "We're constraining the size of the vector so that adding 1 will never overflow"
    )]
    #[expect(
        clippy::cast_possible_wrap,
        reason = "We're constraining the size of the vector so to i64 will never fail"
    )]
    #[expect(
        clippy::as_conversions,
        reason = "We're constraining the size of the vector so to i64 will never fail"
    )]
    fn correct_stack_sizes_for_int(#[any(size_range(0..1_000).lift())] values: Vec<i64>) {
        let num_values = values.len();
        let state = PushState::builder()
            .with_max_stack_size(num_values + 1)
            .with_int_values(values)
            .unwrap()
            .with_no_program()
            .with_instruction_step_limit(1000)
            .build();
        let stack_depth = StackDepth::<i64>::new();
        let state = stack_depth.perform(state).unwrap();
        assert_eq!(state.stack::<i64>().size(), num_values + 1);
        assert_eq!(state.stack::<i64>().top().unwrap(), &(num_values as i64));
    }

    #[proptest]
    #[expect(
        clippy::cast_possible_wrap,
        reason = "We're constraining the size of the vector so to i64 will never fail"
    )]
    #[expect(
        clippy::as_conversions,
        reason = "We're constraining the size of the vector so to i64 will never fail"
    )]
    fn correct_stack_sizes_for_bool(#[any(size_range(0..1_000).lift())] values: Vec<bool>) {
        let num_values = values.len();
        let state = PushState::builder()
            // We `.max(1)` in case `values` has length 0 as we still need room
            // for the value to be pushed onto the int stack.
            .with_max_stack_size(num_values.max(1))
            .with_bool_values(values)
            .unwrap()
            .with_no_program()
            .with_instruction_step_limit(1000)
            .build();
        let stack_depth = StackDepth::<bool>::new();
        let state = stack_depth.perform(state).unwrap();
        assert_eq!(state.stack::<bool>().size(), num_values);
        assert_eq!(state.stack::<i64>().size(), 1);
        assert_eq!(state.stack::<i64>().top().unwrap(), &(num_values as i64));
    }

    // This sets the maximum stack size to three, and initially populates the
    // int stack with 3 items. This will cause an `Overflow` error when we run
    // `StackDepth` since that will want to push a 4th value on the int stack,
    // exceeding the maximum stack size of 3.
    #[test]
    fn full_int_stack() {
        let state = PushState::builder()
            .with_max_stack_size(3)
            .with_int_values([0, 1, 2])
            .unwrap()
            .with_no_program()
            .with_instruction_step_limit(1000)
            .build();
        let stack_depth = StackDepth::<bool>::default();
        let result = stack_depth.perform(state).unwrap_err();
        assert!(result.is_fatal());
        assert_eq!(
            result.error(),
            &PushInstructionError::StackError(StackError::Overflow { stack_type: "i64" })
        );
    }
}
