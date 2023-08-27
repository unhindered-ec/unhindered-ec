use crate::instruction::{Error, InstructionResult, MapInstructionError};

pub trait TypeEq {
    type This: ?Sized;
}

impl<T: ?Sized> TypeEq for T {
    type This = Self;
}

pub trait HasStack<T> {
    fn stack<U: TypeEq<This = T>>(&self) -> &Stack<T>;
    fn stack_mut<U: TypeEq<This = T>>(&mut self) -> &mut Stack<T>;

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

    fn with_push(mut self, value: T) -> InstructionResult<Self, StackError>
    where
        Self: Sized,
    {
        match self.stack_mut::<T>().push(value) {
            Ok(_) => Ok(self),
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

#[cfg(test)]
mod test {
    use super::{Stack, StackError};

    #[test]
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
