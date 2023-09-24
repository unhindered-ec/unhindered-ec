use crate::maybe_known::MaybeKnown;

pub mod simple;
pub mod traits;
pub mod transactional;

#[derive(thiserror::Error, Debug, Eq, PartialEq)]
pub enum StackError {
    #[error("Requested {num_requested} elements from stack of type {stack_type} with {num_present} elements.")]
    Underflow {
        num_requested: usize,
        num_present: usize,
        stack_type: &'static str,
    },
    #[error("Attempted to push to stack of type {stack_type} where the requested capacity was {num_requested} was larger than the one available {capacity_remaining}")]
    Overflow {
        num_requested: MaybeKnown<usize>,
        capacity_remaining: usize,
        stack_type: &'static str,
    },
}

impl StackError {
    #[must_use]
    pub fn overflow<T>(capacity: usize, requested: usize) -> Self {
        Self::Overflow {
            capacity_remaining: capacity,
            num_requested: MaybeKnown::Known(requested),
            stack_type: std::any::type_name::<T>(),
        }
    }

    #[must_use]
    pub fn overflow_unknown_requested<T>(capacity: usize) -> Self {
        Self::Overflow {
            capacity_remaining: capacity,
            num_requested: MaybeKnown::Unknown,
            stack_type: std::any::type_name::<T>(),
        }
    }

    #[must_use]
    pub fn underflow<T>(present: usize, requested: usize) -> Self {
        Self::Underflow {
            num_requested: requested,
            num_present: present,
            stack_type: std::any::type_name::<T>(),
        }
    }
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
