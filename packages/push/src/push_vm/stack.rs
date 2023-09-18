use crate::{
    error::{Error, InstructionResult},
    instruction::MapInstructionError,
};

pub trait TypeEq {
    type This: ?Sized;
}

impl<T: ?Sized> TypeEq for T {
    type This = Self;
}

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
        match stack.pop_discard(num_to_replace) {
            Ok(_) => self.with_push(value),
            Err(error) => Err(Error::fatal(self, error)),
        }
    }
}

#[derive(thiserror::Error, Debug, Eq, PartialEq)]
pub enum StackError {
    #[error("Requested {num_requested} elements from stack with {num_present} elements.")]
    Underflow {
        num_requested: usize,
        num_present: usize,
    },
    #[error("Pushed onto full stack of type {stack_type}.")]
    Overflow { stack_type: &'static str },
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Stack<T> {
    max_stack_size: usize,
    values: Vec<T>,
}

// We implemented this by hand instead of using `derive`
// because `derive` would have required that `T: Default`,
// but that's not necessary for an empty stack. Doing this
// by hand avoids that requirement.
impl<T> Default for Stack<T> {
    fn default() -> Self {
        Self {
            max_stack_size: usize::MAX,
            values: Vec::default(),
        }
    }
}

impl<T> PartialEq<Vec<T>> for Stack<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Vec<T>) -> bool {
        &self.values == other
    }
}

/// Stack
///
/// It's critical that all mutating stack operations be "transactional" in
/// the sense that they successfully perform all their side-effecting modifications
/// OR they perform none of them and return a `StackError`. If this isn't true,
/// then we can end up with inconsistent states when performing instructions.
impl<T> Stack<T> {
    pub fn set_max_stack_size(&mut self, max_stack_size: usize) {
        self.max_stack_size = max_stack_size;
    }

    #[must_use]
    pub fn size(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn is_full(&self) -> bool {
        self.size() == self.max_stack_size
    }

    pub fn top(&self) -> Result<&T, StackError> {
        self.values.last().ok_or(StackError::Underflow {
            num_requested: 1,
            num_present: 0,
        })
    }

    pub fn top2(&self) -> Result<(&T, &T), StackError> {
        if self.size() >= 2 {
            let x = self.top()?;
            let y = self
                .values
                .get(self.size() - 2)
                .ok_or(StackError::Underflow {
                    num_requested: 2,
                    num_present: 1,
                })?;
            Ok((x, y))
        } else {
            Err(StackError::Underflow {
                num_requested: 2,
                num_present: self.size(),
            })
        }
    }

    pub fn pop(&mut self) -> Result<T, StackError> {
        self.values.pop().ok_or(StackError::Underflow {
            num_requested: 1,
            num_present: 0,
        })
    }

    pub fn pop2(&mut self) -> Result<(T, T), StackError> {
        if self.size() >= 2 {
            let x = self.pop()?;
            let y = self.pop()?;
            Ok((x, y))
        } else {
            Err(StackError::Underflow {
                num_requested: 2,
                num_present: self.size(),
            })
        }
    }

    pub fn pop_discard(&mut self, num_to_discard: usize) -> Result<(), StackError> {
        let stack_size = self.size();
        if num_to_discard > stack_size {
            return Err(StackError::Underflow {
                num_requested: num_to_discard,
                num_present: stack_size,
            });
        }
        for _ in 0..num_to_discard {
            match self.pop() {
                Ok(_) => continue,
                Err(error) => {
                    return Err(error);
                }
            }
        }
        Ok(())
    }

    pub fn push(&mut self, value: T) -> Result<(), StackError> {
        if self.size() == self.max_stack_size {
            Err(StackError::Overflow {
                stack_type: std::any::type_name::<T>(),
            })
        } else {
            self.values.push(value);
            Ok(())
        }
    }

    /// Adds the given sequence of values to this stack.
    ///
    /// The first value in `values` will be the new top of the
    /// stack. If the stack was initially empty, the last value
    /// in `values` will be the new bottom of the stack.
    ///
    /// # Arguments
    ///
    /// * `values` - A `Vec` holding the values to add to the stack
    ///
    /// # Examples
    ///
    /// ```
    /// use push::push_vm::push_state::Stack;
    /// let mut stack: Stack<PushInteger> = Stack::default();
    /// assert_eq!(stack.size(), 0);
    /// stack.extend(vec![5, 8, 9]);
    /// // Now the top of the stack is 5, followed by 8, then 9 at the bottom.
    /// assert_eq!(stack.size(), 3);
    /// assert_eq!(stack.top().unwrap(), &5);
    /// stack.extend(vec![6, 3]);
    /// // Now the top of the stack is 6 and the whole stack is 6, 3, 5, 8, 9.
    /// assert_eq!(stack.size(), 5);
    /// assert_eq!(stack.top().unwrap(), &6);
    /// ```  
    pub fn extend(&mut self, values: Vec<T>) {
        self.values.extend(values.into_iter().rev());
    }
}

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

pub trait StackDiscard<S, E> {
    fn with_stack_pop_discard<T>(self, num_to_discard: usize) -> InstructionResult<S, E>
    where
        S: HasStack<T>;
}

impl<S, E> StackDiscard<S, E> for InstructionResult<S, E>
where
    E: From<StackError>,
{
    fn with_stack_pop_discard<T>(self, num_to_discard: usize) -> Self
    where
        S: HasStack<T>,
    {
        match self {
            Ok(mut state) => match state.stack_mut::<T>().pop_discard(num_to_discard) {
                Ok(_) => Ok(state),
                // TODO: any::type_name::<T>() to get the type name – put this in Stack
                // If this fails it's because we tried to pop too many things from the stack.
                // We _should_ have previously checked that there were that many things (using `top()` for example),
                // so really this should never happen.
                Err(error) => Err(Error::fatal(state, error)),
            },
            Err(error) => Err(error),
        }
    }
}

// TODO: Add a test to the `Stack` code that confirms that we return the
//   correct `Underflow` and `Overflow` errors.

#[cfg(test)]
mod test {
    use super::{Stack, StackError};

    #[test]
    #[allow(clippy::unwrap_used)]
    fn top_from_empty_fails() {
        let stack: Stack<bool> = Stack::default();
        let result = stack.top().unwrap_err();
        assert_eq!(
            result,
            StackError::Underflow {
                num_requested: 1,
                num_present: 0
            }
        );
    }
}
