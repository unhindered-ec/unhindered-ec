use collectable::TryExtend;
use miette::Diagnostic;

use crate::error::{Error, InstructionResult, MapInstructionError};

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
            Err(Error::fatal(self, StackError::Overflow {
                // TODO: Should make sure to overflow a stack so we know what this looks like.
                stack_type: std::any::type_name::<T>(),
            }))
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
            Ok(()) => Ok(self),
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
    /// error?
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
        match stack.discard(num_to_replace) {
            Ok(()) => self.with_push(value),
            Err(error) => Err(Error::fatal(self, error)),
        }
    }
}

#[derive(thiserror::Error, Debug, Eq, PartialEq, Diagnostic)]
pub enum StackError {
    #[error("Requested {num_requested} elements from stack with {num_present} elements.")]
    #[diagnostic(severity(Warning))]
    Underflow {
        num_requested: usize,
        num_present: usize,
    },
    #[error("Pushed onto full stack of type {stack_type}.")]
    // The `Overflow` variant is usually not seen by the user as it is
    // typically processed by the interpreter, and a value from the appropriate
    // stack is returned.
    #[diagnostic(
        help = "You might want to increase your stack size if it seems to low",
        severity(Warning)
    )]
    Overflow { stack_type: &'static str },
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Stack<T> {
    max_stack_size: usize,
    values: Vec<T>,
}

pub trait StackType {
    type Type;
}

impl<T> StackType for Stack<T> {
    type Type = T;
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

impl<T, const N: usize> PartialEq<&[T; N]> for Stack<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &&[T; N]) -> bool {
        <Self as PartialEq<[T]>>::eq(self, &(**other)[..])
    }
}

impl<T, const N: usize> PartialEq<[T; N]> for Stack<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &[T; N]) -> bool {
        <Self as PartialEq<[T]>>::eq(self, &(*other)[..])
    }
}

impl<T> PartialEq<&[T]> for Stack<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &&[T]) -> bool {
        <Self as PartialEq<[T]>>::eq(self, *other)
    }
}

impl<T> PartialEq<&mut [T]> for Stack<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &&mut [T]) -> bool {
        <Self as PartialEq<[T]>>::eq(self, *other)
    }
}

impl<T> PartialEq<Vec<T>> for Stack<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Vec<T>) -> bool {
        <Self as PartialEq<[T]>>::eq(self, other)
    }
}

impl<T> PartialEq<[T]> for Stack<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &[T]) -> bool {
        self.values == other
    }
}

/// Extend the stack with values from the provided iterator
///
/// Note that the provided iterator will always be (partially) consumed, even
/// if the method errors.
///
/// This implementation exists to be able to use [`Stack<T>`] in more generic
/// contexts. If possible, try to use
/// [`Stack::<T>::push_many()`](Stack::push_many) instead since it's able to be
/// more optimized due to the additional bounds on the input, which this trait
/// doesn't have.
///
/// If you either need to use [`Stack`] in a generic fashion using `T:
/// TryExtend<I>` or you have an iterator where you can't guarantee the
/// neccessary bounds, using this implementation is the best way to go.
/// (this implementation of course tries to be as optimized as possible as well,
/// given the constraints)
impl<A> TryExtend<A> for Stack<A> {
    type Error = StackError;

    fn try_extend<T>(&mut self, iter: &mut T) -> Result<(), Self::Error>
    where
        T: Iterator<Item = A>,
    {
        let current_len = self.values.len();
        let current_capacity = self.values.capacity();

        let max_extended = self.max_stack_size.saturating_sub(current_len);

        self.values.extend(iter.take(max_extended));

        if iter.next().is_some() {
            self.values.truncate(current_len);
            self.values.shrink_to(current_capacity);
            return Err(StackError::Overflow {
                stack_type: std::any::type_name::<A>(),
            });
        }

        self.values[current_len..].reverse();

        Ok(())
    }
}

/// Stack
///
/// It's critical that all mutating stack operations be "transactional" in
/// the sense that they successfully perform all their side-effecting
/// modifications OR they perform none of them and return a `StackError`. If
/// this isn't true, then we can end up with inconsistent states when performing
/// instructions.
impl<T> Stack<T> {
    /// Sets the maximum size for this stack. Attempts to add elements that
    /// would take the stack above this size should return
    /// `StackError::Overflow`.
    pub fn set_max_stack_size(&mut self, max_stack_size: usize) {
        self.max_stack_size = max_stack_size;
    }

    /// Returns the maximum size for this stack.
    #[must_use]
    pub const fn max_stack_size(&self) -> usize {
        self.max_stack_size
    }

    /// Returns the size of this stack.
    #[must_use]
    pub fn size(&self) -> usize {
        self.values.len()
    }

    /// Returns `true` if the stack contains no elements.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Returns `true` if the stack has `max_stack_size()` elements.
    #[must_use]
    pub fn is_full(&self) -> bool {
        self.size() == self.max_stack_size
    }

    /// Returns a reference to the top value on this stack, or
    /// an error if the stack is empty.
    ///
    /// # Errors
    ///
    /// Returns `StackError::Underflow` error if the stack is empty.
    pub fn top(&self) -> Result<&T, StackError> {
        self.values.last().ok_or(StackError::Underflow {
            num_requested: 1,
            num_present: 0,
        })
    }

    /// Returns a pair of references to the top two elements of
    /// the stack, or an error if the stack has less than two
    /// elements.
    ///
    /// # Errors
    ///
    /// Returns `StackError::Underflow` error if the stack has less than
    /// two elements.
    pub fn top2(&self) -> Result<(&T, &T), StackError> {
        let index_second_to_top =
            self.size()
                .checked_sub(2)
                .ok_or_else(|| StackError::Underflow {
                    num_requested: 2,
                    num_present: self.size(),
                })?;
        let x = self.top()?;
        let y = self
            .values
            .get(index_second_to_top)
            .ok_or(StackError::Underflow {
                num_requested: 2,
                num_present: 1,
            })?;
        Ok((x, y))
    }

    /// Removes the top element from a stack and returns it, or
    /// `StackError::Underflow` if it is empty.
    ///
    /// # Errors
    ///
    /// Returns `StackError::Underflow` if the stack is empty.
    pub fn pop(&mut self) -> Result<T, StackError> {
        self.values.pop().ok_or(StackError::Underflow {
            num_requested: 1,
            num_present: 0,
        })
    }

    /// Removes the top two elements from a stack and returns them in a pair.
    /// Returns `StackError::Underflow` if the stack has fewer than two
    /// elements.
    ///
    /// # Errors
    ///
    /// Returns `StackError::Underflow` if the stack has fewer than two
    /// elements.
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

    /// Discards `num_to_discard` elements from the top of the stack, returning
    /// `StackError::StackUnderflow` if there are fewer than `num_to_discard`
    /// elements on the stack.
    ///
    /// # Errors
    ///
    /// Returns `StackError::Underflow` if the stack has fewer than
    /// `num_to_discard` elements on it.
    pub fn discard(&mut self, num_to_discard: usize) -> Result<(), StackError> {
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

    /// Pushes `value` onto the top of the stack, returning
    /// `StackError::StackOverflow` if doing so would exceed the
    /// `max_stack_size()` for this stack.
    ///
    /// # Errors
    ///
    /// Returns `StackError::Overflow` if the stack was already full, i.e.,
    /// pushing on `value` would cause the stack size to exceed
    /// `max_stack_size()`.
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
    /// - `values` - An implementation of [`IntoIterator`] which must also
    ///   implement both [`ExactSizeIterator`] and [`DoubleEndedIterator`].
    ///   `values` can be, for example, any collection of items of type `T` that
    ///   can be converted into an appropriate iterator, including both [`Vec`]
    ///   and arrays.
    ///
    ///   If this is too restrictive, take a look at the [`Stack::try_extend`]
    ///   method instead.
    ///
    /// # Errors
    ///
    /// - [`StackError::Overflow`] is returned when adding the provided elements
    ///   would cause the stack size to exceed maximum stack size for this
    ///   stack, as set with [`Stack::set_max_stack_size`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use push::push_vm::stack::StackError;
    /// # use push::push_vm::stack::Stack;
    /// #
    /// let mut stack: Stack<i64> = Stack::default();
    /// assert_eq!(stack.size(), 0);
    ///
    /// stack.push_many(vec![5, 8, 9])?;
    /// // Now the top of the stack is 5, followed by 8, then 9 at the bottom.
    /// assert_eq!(stack.size(), 3);
    /// assert_eq!(stack.top()?, &5);
    ///
    /// stack.push_many(vec![6, 3])?;
    /// // Now the top of the stack is 6 and the whole stack is 6, 3, 5, 8, 9.
    /// assert_eq!(stack.size(), 5);
    /// assert_eq!(stack.top()?, &6);
    ///
    /// # Ok::<(), StackError>(())
    /// ```
    pub fn push_many<I>(&mut self, iter: I) -> Result<(), StackError>
    where
        I: IntoIterator<Item = T>,
        // We need the iterator to implement `ExactSizeIterator` so that
        // `.len()` doesn't consume the iterator, and `DoubleEndedIterator`
        // so that `.rev()` works.
        I::IntoIter: ExactSizeIterator + DoubleEndedIterator,
    {
        let iter = iter.into_iter();
        // Check that adding these items won't overflow the stack.
        if iter
            .len()
            .checked_add(self.size())
            .is_none_or(|x| x > self.max_stack_size)
        {
            return Err(StackError::Overflow {
                stack_type: std::any::type_name::<T>(),
            });
        }
        self.values.extend(iter.rev());
        Ok(())
    }
}

/// Helper trait to chain instruction operations.
pub trait PushOnto<T, E> {
    /// Updates the state with `T` pushed to the stack.
    ///
    /// # Errors
    ///
    /// Returns an error of type `E` if pushing this value fails, e.g.,
    /// if adding this element exceeded the maximum stack size.
    fn push_onto<S>(self, state: S) -> InstructionResult<S, E>
    where
        S: HasStack<T>;

    /// Updates the state by replacing the top `num_to_replace` elements
    /// with `T`.
    ///
    /// # Errors
    ///
    /// Returns an error of type `E` if the replacement fails. This could
    /// be, for example, because there aren't `num_to_replace` items on the
    /// stack, or if adding the new element would exceed the maximum stack size.
    fn replace_on<S>(self, num_to_replace: usize, state: S) -> InstructionResult<S, E>
    where
        S: HasStack<T>;
}

impl<T, E1, E2> PushOnto<T, E2> for Result<T, E1>
where
    E2: From<E1> + From<StackError>,
{
    fn push_onto<S>(self, state: S) -> InstructionResult<S, E2>
    where
        S: HasStack<T>,
    {
        match self {
            Ok(val) => state.with_push(val).map_err_into(),
            Err(err) => Err(Error::recoverable(state, err)),
        }
    }

    fn replace_on<S>(self, num_to_replace: usize, state: S) -> InstructionResult<S, E2>
    where
        S: HasStack<T>,
    {
        match self {
            Ok(val) => state.with_replace(num_to_replace, val).map_err_into(),
            Err(err) => Err(Error::recoverable(state, err)),
        }
    }
}

pub trait StackPush<S, E> {
    /// Pushes `value` onto the stack of type `T` in state `S`, returning an
    /// error of type `E` if that fails.
    ///
    /// # Errors
    ///
    /// Returns an error of type `E` if this fails, e.g., if pushing onto the
    /// stack would exceed the maximum stack size.
    fn with_stack_push<T>(self, value: T) -> InstructionResult<S, E>
    where
        S: HasStack<T>;
}

impl<S, E> StackPush<S, E> for InstructionResult<S, E>
where
    E: From<StackError>,
{
    fn with_stack_push<T>(self, value: T) -> Self
    where
        S: HasStack<T>,
    {
        match self {
            Ok(mut state) => match state.stack_mut::<T>().push(value) {
                Ok(()) => Ok(state),
                // If this fails it's because we tried to push onto a full stack.
                // We _should_ have previously checked that the stack wasn't full
                // (using `not_full()` for example), so really this should never happen.
                Err(error) => Err(Error::fatal(state, error)),
            },
            Err(error) => Err(error),
        }
    }
}

pub trait StackDiscard<S, E> {
    /// Discards the top `num_to_discard` elements from the stack in `S` of type
    /// `T`, returning an error of type `E` if that fails.
    ///
    /// # Errors
    ///
    /// Returns an error of type `E` if this fails, e.g., if there are not
    /// `num_to_discard` elements in the stack.
    fn with_stack_discard<T>(self, num_to_discard: usize) -> InstructionResult<S, E>
    where
        S: HasStack<T>;
}

impl<S, E> StackDiscard<S, E> for InstructionResult<S, E>
where
    E: From<StackError>,
{
    fn with_stack_discard<T>(self, num_to_discard: usize) -> Self
    where
        S: HasStack<T>,
    {
        match self {
            Ok(mut state) => match state.stack_mut::<T>().discard(num_to_discard) {
                Ok(()) => Ok(state),
                // TODO: any::type_name::<T>() to get the type name – put this in Stack
                // If this fails it's because we tried to pop too many things from the stack.
                // We _should_ have previously checked that there were
                // that many things (using `top()` for example),
                // so really this should never happen.
                Err(error) => Err(Error::fatal(state, error)),
            },
            Err(error) => Err(error),
        }
    }
}

// TODO: Add a test to the `Stack` code that confirms that we return the
// correct `Underflow` and `Overflow` errors.

#[cfg(test)]
mod test {
    use super::{Stack, StackError};

    #[test]
    fn top_from_empty_fails() {
        let stack: Stack<bool> = Stack::default();
        let result = stack.top().unwrap_err();
        assert_eq!(result, StackError::Underflow {
            num_requested: 1,
            num_present: 0
        });
    }
}
