use std::marker::PhantomData;

use crate::{
    instruction::{Instruction, instruction_error::PushInstructionError},
    push_vm::HasStack,
};

/// An instruction that flushes (i.e., empties) a stack of type `T`.
///
/// # Inputs
///
/// The `Flush` instruction takes no inputs from the stacks.
///
/// # Behavior
///
/// The `Flush` instruction removes all the elements from the stack of type `T`.
///
/// If the type `T` is `bool`, this will remove all `bool` values from the
/// stack. If the type `T` is `i64` then all `i64` values will be removed from
/// the stack.
///
/// ## Action Table
///
/// The table below indicates the behavior in each of the different
/// cases.
///
///    - The "`Stack<T>` empty or not" column indicates whether the stack of
///      type `T` is empty or not.
///    - The "Success" column indicates whether the instruction succeeds, and if
///      not what kind of error is returned:
///       - ✅: success (so two copies of the top block on the `Exec stack`)
///       - ❗: recoverable error, with links to the error kind
///       - ‼️: fatal error, with links to the error kind
///    - The "Note" column briefly summarizes the action state in that case
///
/// | `Stack<T>` empty or not |  Success | Note |
/// | ------------- | ------------- | ------------- |
/// | empty | ✅ | The stack is unchanged |
/// | not empty | ✅ | All the elements are removed from the stack |
///
/// # Errors
///
/// If the stack access returns any error other than a
/// [`StackError::Underflow`](crate::push_vm::stack::StackError::Underflow)
/// then this returns that as a [`Error::Fatal`](crate::error::Error::Fatal)
/// error.
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub struct Flush<T> {
    _p: PhantomData<T>,
}

impl<T> Flush<T> {
    pub const fn new() -> Self {
        Self { _p: PhantomData }
    }
}

impl<S, T> Instruction<S> for Flush<T>
where
    S: HasStack<T>,
{
    type Error = PushInstructionError;

    fn perform(&self, mut state: S) -> crate::error::InstructionResult<S, Self::Error> {
        let stack = state.stack_mut::<T>();
        // Pop all the items on the stack
        // until we run into an underflow error.
        while stack.pop().is_ok() {}
        Ok(state)
    }
}

#[cfg(test)]
mod test {
    use super::Flush;
    use crate::{
        instruction::Instruction,
        push_vm::{HasStack, push_state::PushState},
    };

    #[test]
    fn flush_i64() {
        let state = PushState::builder()
            .with_max_stack_size(3)
            .with_int_values([1, 2, 3])
            .unwrap()
            .with_no_program()
            .build();
        let flush = Flush::<i64>::new();
        let state = flush.perform(state).unwrap();
        assert_eq!(state.stack::<i64>().size(), 0);
    }
}
