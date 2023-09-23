use crate::{
    instruction::{Error, InstructionResult},
    push_vm::HasStackOld,
};

use super::StackError;

pub trait StackDiscard<S, E> {
    /// # Errors
    /// - [`StackError::underflow`] is returned when there are fewer elements in the stack than are to be discarded
    fn with_stack_pop_discard<T>(self, num_to_discard: usize) -> InstructionResult<S, E>
    where
        S: HasStackOld<T>;
}

impl<S, E> StackDiscard<S, E> for InstructionResult<S, E>
where
    E: From<StackError>,
{
    fn with_stack_pop_discard<T>(self, num_to_discard: usize) -> Self
    where
        S: HasStackOld<T>,
    {
        let mut state = self?;

        match state.stack_mut::<T>().discard_from_top(num_to_discard) {
            Ok(_) => Ok(state),
            Err(error) => Err(Error::fatal(state, error)),
        }
    }
}
