use crate::{
    instruction::{Error, InstructionResult, MakeError},
    push_vm::stack::{Stack, StackError},
    type_eq::TypeEq,
};

#[deprecated(note = "Use HasStack and HasStackMut instead")]
pub trait HasStackOld<T> {
    fn stack<U: TypeEq<This = T>>(&self) -> &Stack<T>;
    #[deprecated(note = "Use HasStackMut::stack_mut instead")]
    fn stack_mut<U: TypeEq<This = T>>(&mut self) -> &mut Stack<T>;

    /// # Errors
    ///
    /// Returns a fatal error if the stack is in fact full.
    #[deprecated(
        note = "Use WithStack::with_stack_try::<U>(Stack::not_full).map_err(WithState::drop_state).map_err(MakeError::make_fatal) instead"
    )]
    fn not_full<U: TypeEq<This = T>>(self) -> InstructionResult<Self, StackError>
    where
        Self: Sized,
    {
        if self.stack::<U>().is_full() {
            Err(Error::fatal(
                self,
                StackError::overflow_unknown_requested::<T>(0),
            ))
        } else {
            Ok(self)
        }
    }

    /// # Errors
    ///
    /// Returns a fatal error if pushing onto the specified stack overflows.
    #[deprecated(
        note = "Use WithStack::with_stack_try::<U>(|s| s.push(value)).make_fatal() instead"
    )]
    fn with_push(mut self, value: T) -> InstructionResult<Self, StackError>
    where
        Self: Sized,
    {
        match self.stack_mut::<T>().push(value) {
            Ok(_) => Ok(self),
            Err(error) => Err(error).make_fatal(self).map_err(Into::into),
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
    #[deprecated(note = "Use transactions instead")]
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

pub trait HasStack<T> {
    fn stack<U: TypeEq<This = T>>(&self) -> &Stack<T>;
}

impl<'a, T, R> HasStack<T> for &'a R
where
    R: HasStack<T>,
{
    fn stack<U: TypeEq<This = T>>(&self) -> &Stack<T> {
        (**self).stack::<U>()
    }
}

impl<'a, T, R> HasStack<T> for &'a mut R
where
    R: HasStack<T>,
{
    fn stack<U: TypeEq<This = T>>(&self) -> &Stack<T> {
        (**self).stack::<U>()
    }
}

pub trait HasStackMut<T>: HasStack<T> {
    fn stack_mut<U: TypeEq<This = T>>(&mut self) -> &mut Stack<T>;
}

impl<'a, T, R> HasStackMut<T> for &'a mut R
where
    R: HasStackMut<T>,
{
    fn stack_mut<U: TypeEq<This = T>>(&mut self) -> &mut Stack<T> {
        (*self).stack_mut::<U>()
    }
}
