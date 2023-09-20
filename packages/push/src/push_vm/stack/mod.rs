use crate::tuples::MonotonicTuple;

pub mod has_stack;
pub mod stack_discard;
pub mod stack_push;

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

pub trait TypedStack {
    type Item;
}

pub trait SizeLimit {
    #[must_use]
    fn max_size(&self) -> usize;

    fn set_max_size(&mut self, max_size: usize);

    #[must_use]
    fn is_full(&self) -> bool
    where
        Self: StackSize,
    {
        self.max_size() == self.size()
    }
}

pub trait StackSize {
    #[must_use]
    fn size(&self) -> usize;

    #[must_use]
    fn is_empty(&self) -> bool {
        self.size() == 0
    }
}

// All items returning a tuple of items from the stack should return them in a first-in-stack goes first ordering.

pub trait GetHead: TypedStack {
    /// # Errors
    /// - [`StackError::Underflow`] is returned when there is not at least one item on the Stack to return.
    fn head(&self) -> Result<&Self::Item, StackError> {
        Ok(self.get_n_head::<(_,)>()?.0)
    }

    /// # Errors
    /// - [`StackError::Underflow`] is returned when there are not at least [`MonotonicTuple::Length`] items on the Stack to return.
    fn get_n_head<'a, Tuple: MonotonicTuple<Item = &'a Self::Item>>(
        &'a self,
    ) -> Result<Tuple, StackError>;
}

pub trait PopHead: GetHead {
    /// # Errors
    /// - [`StackError::Underflow`] is returned when there is not at least one item on the Stack to return.
    fn pop_head(&mut self) -> Result<&Self::Item, StackError> {
        Ok(self.pop_n_head::<(_,)>()?.0)
    }

    /// # Errors
    /// - [`StackError::Underflow`] is returned when there are not at least [`MonotonicTuple::Length`] items on the Stack to return.
    fn pop_n_head<'a, Tuple: MonotonicTuple<Item = &'a Self::Item>>(
        &'a mut self,
    ) -> Result<Tuple, StackError>;
}

pub trait GetTail: TypedStack {
    /// # Errors
    /// - [`StackError::Underflow`] is returned when there is not at least one item on the Stack to return.
    fn tail(&self) -> Result<&Self::Item, StackError> {
        Ok(self.get_n_tail::<(_,)>()?.0)
    }

    /// # Errors
    /// - [`StackError::Underflow`] is returned when there are not at least [`MonotonicTuple::Length`] items on the Stack to return.
    fn get_n_tail<'a, Tuple: MonotonicTuple<Item = &'a Self::Item>>(
        &'a self,
    ) -> Result<Tuple, StackError>;
}

pub trait PopTail: GetTail {
    /// # Errors
    /// - [`StackError::Underflow`] is returned when there is not at least one item on the Stack to return.
    fn pop_tail(&mut self) -> Result<&Self::Item, StackError> {
        Ok(self.pop_n_tail::<(_,)>()?.0)
    }

    /// # Errors
    /// - [`StackError::Underflow`] is returned when there are not at least [`MonotonicTuple::Length`] items on the Stack to return.
    fn pop_n_tail<'a, Tuple: MonotonicTuple<Item = &'a Self::Item>>(
        &'a mut self,
    ) -> Result<Tuple, StackError>;
}

pub trait DiscardHead {
    /// # Errors
    /// - [`StackError::Underflow`] is returned when there is not at least one item on the Stack to discard
    fn discard_head(&mut self) -> Result<(), StackError> {
        self.discard_n_head(1)
    }

    /// # Errors
    /// - [`StackError::Underflow`] is returned when there are not at least [`MonotonicTuple::Length`] items on the Stack to discard.
    fn discard_n_head(&mut self, n: usize) -> Result<(), StackError>;
}

pub trait DiscardTail {
    /// # Errors
    /// - [`StackError::Underflow`] is returned when there is not at least one item on the Stack to discard
    fn discard_tail(&mut self) -> Result<(), StackError> {
        self.discard_n_tail(1)
    }

    /// # Errors
    /// - [`StackError::Underflow`] is returned when there are not at least [`MonotonicTuple::Length`] items on the Stack to discard.
    fn discard_n_tail(&mut self, n: usize) -> Result<(), StackError>;
}

pub trait PushHead: TypedStack {
    /// # Errors
    /// - [`StackError::Overflow`] is returned when the stack has not enough capacity for one new value remaining.
    fn push_head(&mut self, value: Self::Item) -> Result<(), StackError> {
        self.push_n_head((value,))
    }

    /// # Errors
    /// - [`StackError::Overflow`] is returned when the stack has not enough capacity for n new values remaining.
    fn push_n_head<Tuple: MonotonicTuple<Item = Self::Item>>(
        &mut self,
        value: Tuple,
    ) -> Result<(), StackError>;
}

pub trait PushTail: TypedStack {
    /// # Errors
    /// - [`StackError::Overflow`] is returned when the stack has not enough capacity for one new value remaining.
    fn push_tail(&mut self, value: Self::Item) -> Result<(), StackError> {
        self.push_n_tail((value,))
    }

    /// # Errors
    /// - [`StackError::Overflow`] is returned when the stack has not enough capacity for n new values remaining.
    fn push_n_tail<Tuple: MonotonicTuple<Item = Self::Item>>(
        &mut self,
        value: Tuple,
    ) -> Result<(), StackError>;
}

pub trait ExtendTail: TypedStack {
    /// # Errors
    /// - [`StackError::Overflow`] is returned when the stack has not enough capacity for [`Iter::len()`] new values remaining.
    fn extend_tail<Iter>(&mut self, iter: Iter) -> Result<(), StackError>
    where
        Iter: IntoIterator<Item = Self::Item>,
        Iter::IntoIter: DoubleEndedIterator + ExactSizeIterator;
}

pub trait ExtendHead: TypedStack {
    /// # Errors
    /// - [`StackError::Overflow`] is returned when the stack has not enough capacity for [`Iter::len()`] new values remaining.
    fn extend_head<Iter>(&mut self, iter: Iter) -> Result<(), StackError>
    where
        Iter: IntoIterator<Item = Self::Item>,
        Iter::IntoIter: DoubleEndedIterator + ExactSizeIterator;
}

// Blanket impl - Is this a good idea even when specialization is not stable yet?
impl<T> PushHead for T
where
    T: ExtendHead,
{
    fn push_n_head<Tuple: MonotonicTuple<Item = Self::Item>>(
        &mut self,
        value: Tuple,
    ) -> Result<(), StackError> {
        self.extend_head(value.into_iterator())
    }
}

impl<T> PushTail for T
where
    T: ExtendTail,
{
    fn push_n_tail<Tuple: MonotonicTuple<Item = Self::Item>>(
        &mut self,
        value: Tuple,
    ) -> Result<(), StackError> {
        self.extend_tail(value.into_iterator())
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
    pub const fn max_size(&self) -> usize {
        self.max_stack_size
    }

    #[must_use]
    pub fn size(&self) -> usize {
        self.values.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    #[must_use]
    pub fn is_full(&self) -> bool {
        self.size() == self.max_stack_size
    }

    /// # Errors
    /// - [`StackError::Underflow`] this Error is returned when there is not at least one Element in the Stack
    pub fn top(&self) -> Result<&T, StackError> {
        self.values.last().ok_or(StackError::Underflow {
            num_requested: 1,
            num_present: 0,
        })
    }

    /// # Errors
    /// - [`StackError::Underflow`] this Error is returned when there is not the requested amount of Elements in the Stack
    pub fn top_n<'a, Tuple: MonotonicTuple<Item = &'a T>>(&'a self) -> Result<Tuple, StackError> {
        let initial_stack_size = self.size();

        let construct_underflow_error = || StackError::Underflow {
            num_requested: Tuple::LENGTH,
            num_present: initial_stack_size,
        };

        if self.size() < Tuple::LENGTH {
            return Err(construct_underflow_error());
        }

        Tuple::from_iterator(self.values.iter().rev()).ok_or_else(construct_underflow_error)
    }

    #[deprecated(note = "Use [`Stack::top_n`] instead.")]
    #[allow(clippy::missing_errors_doc)]
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

    /// # Errors
    /// - [`StackError::Underflow`] this Error is returned when there is not at least one Element in the Stack
    pub fn pop(&mut self) -> Result<T, StackError> {
        self.values.pop().ok_or(StackError::Underflow {
            num_requested: 1,
            num_present: 0,
        })
    }

    /// # Errors
    /// - [`StackError::Underflow`] this Error is returned when there is not the requested amount of Elements in the Stack
    pub fn pop_n<Tuple: MonotonicTuple<Item = T>>(&mut self) -> Result<Tuple, StackError> {
        let initial_stack_size = self.size();

        let construct_underflow_error = || StackError::Underflow {
            num_requested: Tuple::LENGTH,
            num_present: initial_stack_size,
        };

        if self.size() < Tuple::LENGTH {
            return Err(construct_underflow_error());
        }

        Tuple::from_init_fn_option(|| self.values.pop()).ok_or_else(construct_underflow_error)
    }

    #[deprecated(note = "Use [`Stack::pop_n`] instead.")]
    #[allow(clippy::missing_errors_doc)]
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

    /// # Errors
    /// - [`StackError::Underflow`] this Error is thrown when the stack is smaller than the elements requested to be discarded
    pub fn discard_from_top(&mut self, num_to_discard: usize) -> Result<(), StackError> {
        let stack_size = self.size();
        if num_to_discard > stack_size {
            return Err(StackError::Underflow {
                num_requested: num_to_discard,
                num_present: stack_size,
            });
        }
        // truncate is more performant than popping each individually
        self.values.truncate(self.values.len() - num_to_discard);
        Ok(())
    }

    /// # Errors
    /// - [`StackError::Overflow`] is returned when adding `value` to the stack would increase the stack size above the allowed maximum
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
    /// use push::push_vm::stack::Stack;
    /// use push::push_vm::PushInteger;
    ///
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
    /// # Errors
    /// - [`StackError::Overflow`] is returned when adding all values would push the stack over its allowed size limit
    pub fn extend<I>(&mut self, values: I) -> Result<(), StackError>
    where
        I: IntoIterator<Item = T>,
        I::IntoIter: DoubleEndedIterator + ExactSizeIterator,
    {
        let iter = values.into_iter();
        if iter.len() > self.max_stack_size - self.values.len() {
            return Err(StackError::Overflow {
                stack_type: std::any::type_name::<T>(),
            });
        }
        self.values.extend(iter.rev());
        Ok(())
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
