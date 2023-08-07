// TODO: Add a test to the `Stack` code that confirms that we return the
//   correct `Underflow` and `Overflow` errors.

pub trait HasStack<T> {
    fn stack_mut(&mut self) -> &mut Stack<T>;
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

#[derive(Debug, Eq, PartialEq)]
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

impl<T> Stack<T> {
    pub fn set_max_stack_size(&mut self, max_stack_size: usize) {
        self.max_stack_size = max_stack_size;
    }

    #[must_use]
    pub fn size(&self) -> usize {
        self.values.len()
    }

    #[must_use]
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
        if self.values.len() == self.max_stack_size {
            Err(StackError::Overflow {
                stack_type: std::any::type_name::<T>(),
            })
        } else {
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
    /// let mut stack: Stack<i64> = Stack::default();
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
