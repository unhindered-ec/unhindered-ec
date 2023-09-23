use crate::push_vm::stack::StackError;

pub trait SizeLimit {
    #[must_use]
    fn max_size(&self) -> usize;

    /// # Errors
    /// - [`StackError::Overflow`] when the new `max_size` is smaller than the current length of the stack.
    fn set_max_size(&mut self, max_size: usize) -> Result<(), StackError>;

    #[must_use]
    #[inline]
    fn is_full(&self) -> bool
    where
        Self: StackSize,
    {
        self.max_size() == self.size()
    }

    #[must_use]
    #[inline]
    fn capacity(&self) -> usize
    where
        Self: StackSize,
    {
        self.max_size() - self.size()
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
