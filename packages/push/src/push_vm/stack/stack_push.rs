use crate::{
    instruction::{Error, InstructionResult, MapInstructionError},
    push_vm::HasStack,
};

use super::StackError;

/// Helper trait to chain instruction operations.
pub trait StackPush<T, E> {
    /// Updates the state with `T` pushed to the stack.
    fn with_stack_push<S>(self, state: S) -> InstructionResult<S, E>
    where
        S: HasStack<T>;

    fn with_stack_replace<S>(self, num_to_replace: usize, state: S) -> InstructionResult<S, E>
    where
        S: HasStack<T>;
}

impl<T, E1, E2> StackPush<T, E2> for Result<T, E1>
where
    E2: From<E1> + From<StackError>,
{
    fn with_stack_push<S>(self, state: S) -> InstructionResult<S, E2>
    where
        S: HasStack<T>,
    {
        match self {
            Ok(val) => state.with_push(val).map_err_into(),
            Err(err) => Err(Error::recoverable(state, err)),
        }
    }

    fn with_stack_replace<S>(self, num_to_replace: usize, state: S) -> InstructionResult<S, E2>
    where
        S: HasStack<T>,
    {
        match self {
            Ok(val) => state.with_replace(num_to_replace, val).map_err_into(),
            Err(err) => Err(Error::recoverable(state, err)),
        }
    }
}
