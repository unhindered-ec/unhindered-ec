use std::marker::PhantomData;

use super::{Instruction, instruction_error::PushInstructionError};
use crate::{
    error::InstructionResult,
    push_vm::{HasStack, stack::PushOnto},
};

/// An instruction that duplicates (clones) the top
/// value of a stack of type `T` in the given state.
///
/// # Inputs
///
/// The `Dup` instruction takes the following inputs:
///    - Stack of type `T`
///      - One value
///
/// # Behavior
///
/// The `Dup` instruction clones the top block on the stack of type `T`,
/// leaving both the original and the copy on that stack.
///
/// If stack is empty, this is a no-op,
/// and the state is returned unchanged.
///
/// If the stack is full before this instruction is performed, then
/// a fatal [`StackError::Overflow`](crate::push_vm::stack::StackError::Overflow) is returned,
/// terminating the running of the program.
///
/// This is similar to the `Exec::DupBlock` instruction, but that instruction:
///
/// - is specific to the `Exec` stack, and
/// - adds an implied "open" to create a block that runs from this instruction
///   to the first `Close` in a Plushy genome.
///
/// Applying `Dup` to the `Exec` stack will duplicate the top element of that
/// stack, but will _not_ insert an implied "open".
///
/// ## Action Table
///
/// The table below indicates the behavior in each of the different
/// cases.
///
///    - The "`Stack<T>`" column indicates the value of the top of the stack of
///      type `T`, or whether it exists.
///    - The "`Stack<T>` full" column indicates whether the stack of type `T` is
///      full before this instruction is performed.
///    - The "Success" column indicates whether the instruction succeeds, and if
///      not what kind of error is returned:
///       - ✅: success (so two copies of the top block on the `Exec stack`)
///       - ❗: recoverable error, with links to the error kind
///       - ‼️: fatal error, with links to the error kind
///    - The "Note" column briefly summarizes the action state in that case
///
/// | `Stack<T>`  | `Stack<T>` full |  Success | Note |
/// | ------------- | ------------- | ------------- | ------------- |
/// | exists  | false | ✅ | The top value is cloned |
/// | missing | irrelevant | [❗…](crate::push_vm::stack::StackError::Underflow) | State is unchanged |
/// | exists  | true | [‼️..](crate::push_vm::stack::StackError::Overflow) | Program is terminated |
///
/// # Errors
///
/// If the stack access returns any error other than a
/// [`StackError::Underflow`](crate::push_vm::stack::StackError::Underflow)
/// then this returns that as a [`Error::Fatal`](crate::error::Error::Fatal)
/// error.
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub struct Dup<T> {
    _p: PhantomData<T>,
}

impl<S, T> Instruction<S> for Dup<T>
where
    S: Clone + HasStack<T>,
    T: Clone,
{
    type Error = PushInstructionError;

    fn perform(&self, state: S) -> InstructionResult<S, Self::Error> {
        let dup_block: DupBlock;
        state.stack::<T>().top().cloned().push_onto(state)
    }
}

#[cfg(test)]
mod tests {
    use proptest::{collection::vec, prelude::*};
    use test_strategy::proptest;

    use super::*;
    use crate::push_vm::{push_state::PushState, stack::StackError};

    #[test]
    fn dup_empty_stack() {
        let state = PushState::builder()
            .with_max_stack_size(0)
            .with_no_program()
            .build();
        let dup = Dup::<i64>::default();
        let result = dup.perform(state).unwrap_err();
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
    fn dup_non_empty_stack() {
        let state = PushState::builder()
            .with_max_stack_size(2)
            .with_int_values([17])
            .unwrap()
            .with_no_program()
            .build();
        let dup = Dup::<i64>::default();
        let result = dup.perform(state).unwrap();
        assert_eq!(result.stack::<i64>().size(), 2);
        assert_eq!(result.stack::<i64>().top2().unwrap(), (&17, &17));
    }

    #[test]
    fn dup_full_stack() {
        let state = PushState::builder()
            .with_max_stack_size(1)
            .with_int_values([17])
            .unwrap()
            .with_no_program()
            .build();
        let dup = Dup::<i64>::default();
        let result = dup.perform(state).unwrap_err();
        assert!(result.is_fatal());
        assert_eq!(
            result.error(),
            &PushInstructionError::StackError(StackError::Overflow { stack_type: "i64" })
        );
    }

    #[proptest]
    fn dup_variety_of_values(value: i64) {
        let state = PushState::builder()
            .with_max_stack_size(2)
            .with_int_values([value])
            .unwrap()
            .with_no_program()
            .build();
        let dup = Dup::<i64> { _p: PhantomData };
        let result = dup.perform(state).unwrap();
        assert_eq!(result.stack::<i64>().size(), 2);
        assert_eq!(result.stack::<i64>().top2().unwrap(), (&value, &value));
    }

    // Test that `Dup` works on stacks of 1..100 elements of type `i64`.
    #[proptest]
    fn dup_stacks_with_multiple_items(#[strategy(vec(any::<i64>(), 1..100))] values: Vec<i64>) {
        let state = PushState::builder()
            .with_max_stack_size(100)
            .with_int_values(values.clone().into_iter())
            .unwrap()
            .with_no_program()
            .build();
        let dup = Dup::<i64>::default();
        let result = dup.perform(state).unwrap();
        #[expect(
            clippy::arithmetic_side_effects,
            reason = "The length of `values` is less than 100, so adding 1 is guaranteed to be \
                      safe."
        )]
        let expected_num_values = values.len() + 1;
        assert_eq!(result.stack::<i64>().size(), expected_num_values);
        assert_eq!(
            *result.stack::<i64>().top().unwrap(),
            *values.first().unwrap()
        );
    }
}
