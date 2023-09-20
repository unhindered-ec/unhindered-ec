use crate::{
    instruction::{Error, InstructionResult},
    type_eq::TypeEq,
};

use super::{Stack, StackError};

pub trait HasStack<T> {
    fn stack<U: TypeEq<This = T>>(&self) -> &Stack<T>;
    fn stack_mut<U: TypeEq<This = T>>(&mut self) -> &mut Stack<T>;

    /// # Errors
    ///
    /// Returns a fatal error if the stack is in fact full.
    fn not_full<U: TypeEq<This = T>>(self) -> InstructionResult<Self, StackError>
    where
        Self: Sized,
    {
        if self.stack::<U>().is_full() {
            Err(Error::fatal(
                self,
                StackError::Overflow {
                    // TODO: Should make sure to overflow a stack so we know what this looks like.
                    stack_type: std::any::type_name::<T>(),
                },
            ))
        } else {
            Ok(self)
        }
    }

    /// # Errors
    ///
    /// Returns a fatal error if pushing onto the specified stack overflows.
    fn with_push(mut self, value: T) -> InstructionResult<Self, StackError>
    where
        Self: Sized,
    {
        match self.stack_mut::<T>().push(value) {
            Ok(_) => Ok(self),
            Err(error) => Err(Error::fatal(self, error)),
        }
    }

    /// This removes `num_to_replace` items from the `<T>` stack,
    /// and then pushes on the given value. This supports the common
    /// pattern where a Push instruction removes one or more arguments
    /// from a stack of `T`, computes a value from those arguments,
    /// and pushes the result back on that stack.
    ///
    /// This assumes that there are at least `num_to_replace` values on
    /// the stack in questions; if there aren't we'll generate a fatal
    /// error since that is probably a programming error where an instruction
    /// wasn't implemented properly.
    ///  
    /// # Errors
    ///
    /// Returns a fatal error if we can't actually pop off `num_to_replace`
    /// values. This is actually probably a programming error where an
    /// instruction wasn't implemented properly.
    ///
    /// TODO: Maybe we should `panic` here instead of returning a fatal
    ///   error?
    ///
    /// This also returns a fatal error if pushing onto the specified stack
    /// overflows, which should really never happen assuming we pop at least
    /// one value off the stack.
    fn with_replace(
        mut self,
        num_to_replace: usize,
        value: T,
    ) -> InstructionResult<Self, StackError>
    where
        Self: Sized,
    {
        let stack = self.stack_mut::<T>();
        match stack.discard_from_top(num_to_replace) {
            Ok(_) => self.with_push(value),
            Err(error) => Err(Error::fatal(self, error)),
        }
    }
}
