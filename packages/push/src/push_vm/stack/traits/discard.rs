use crate::push_vm::stack::StackError;

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
