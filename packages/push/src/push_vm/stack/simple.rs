use std::marker::PhantomData;

use crate::{
    push_vm::stack::{
        traits::{
            discard::DiscardHead,
            extend::ExtendHead,
            get::{GetHead, GetTail},
            pop::PopHead,
            size::{SizeLimit, StackSize},
            TypedStack,
        },
        StackError,
    },
    tuples::MonotonicTuple,
};

mod sealed {
    pub trait Sealed {}
}

pub trait SimpleStackType: sealed::Sealed {}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default)]
pub struct Limited;
impl sealed::Sealed for Limited {}
impl SimpleStackType for Limited {}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default)]
pub struct Unlimited;
impl sealed::Sealed for Unlimited {}
impl SimpleStackType for Unlimited {}

// This may be cleaner if we use two seperate structs, SimpleStackLimited and SimpleStackUnlimited instead
// but I choose the Type Generic to avoid the code duplication that comes with that. Currently the `max_size`
// field should obviously not be used when the type is Unlimited so it just kinda sits there in that case.
// I don't know if the rust compiler is smart enough to figure that out and it gets removed, may be worth a
// look at using something like CompilerExplorer.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Default)]
pub struct SimpleStack<T, Type: SimpleStackType = Unlimited> {
    backend: Vec<T>,
    max_size: usize,
    _p: PhantomData<Type>,
}

impl<T, Type: SimpleStackType> SimpleStack<T, Type> {
    #[inline]
    fn build_underflow_error(&self, requested_length: usize) -> StackError {
        StackError::underflow::<Self>(self.backend.len(), requested_length)
    }

    #[inline]
    fn ensure_elements_remaining(&self, min_remaining: usize) -> Result<(), StackError> {
        if self.backend.len() < min_remaining {
            return Err(self.build_underflow_error(min_remaining));
        }

        Ok(())
    }
}

impl<T> SimpleStack<T, Limited> {
    #[inline]
    fn build_overflow_error(&self, requested_length: usize) -> StackError {
        StackError::overflow::<Self>(self.capacity(), requested_length)
    }

    #[inline]
    fn ensure_space_remaining(&self, min_remaining: usize) -> Result<(), StackError> {
        if self.capacity() < min_remaining {
            return Err(self.build_overflow_error(min_remaining));
        }
        Ok(())
    }
}

impl<T, Type: SimpleStackType> TypedStack for SimpleStack<T, Type> {
    type Item = T;
}

impl<T> SizeLimit for SimpleStack<T, Limited> {
    #[inline]
    fn max_size(&self) -> usize {
        self.max_size
    }

    fn set_max_size(&mut self, max_size: usize) -> Result<(), StackError> {
        if self.backend.len() > max_size {
            return Err(StackError::overflow::<Self>(
                0,
                self.backend.len() - max_size,
            ));
        }

        self.max_size = max_size;
        Ok(())
    }

    #[inline]
    fn is_full(&self) -> bool
    where
        Self: StackSize,
    {
        self.max_size == self.backend.len()
    }
}

impl<T, Type: SimpleStackType> StackSize for SimpleStack<T, Type> {
    #[inline]
    fn size(&self) -> usize {
        self.backend.len()
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.backend.is_empty()
    }
}

impl<T, Type: SimpleStackType> GetHead for SimpleStack<T, Type> {
    fn get_n_head<'a, Tuple: MonotonicTuple<Item = &'a Self::Item>>(
        &'a self,
    ) -> Result<Tuple, StackError> {
        self.ensure_elements_remaining(Tuple::LENGTH)?;

        Tuple::from_iterator(self.backend.iter().rev().take(Tuple::LENGTH).rev())
            .ok_or_else(|| self.build_underflow_error(Tuple::LENGTH))
    }
}

impl<T, Type: SimpleStackType> GetTail for SimpleStack<T, Type> {
    fn get_n_tail<'a, Tuple: MonotonicTuple<Item = &'a Self::Item>>(
        &'a self,
    ) -> Result<Tuple, StackError> {
        self.ensure_elements_remaining(Tuple::LENGTH)?;

        Tuple::from_iterator(self.backend.iter())
            .ok_or_else(|| self.build_underflow_error(Tuple::LENGTH))
    }
}

impl<T, Type: SimpleStackType> PopHead for SimpleStack<T, Type> {
    fn pop_n_head<Tuple: MonotonicTuple<Item = Self::Item>>(
        &mut self,
    ) -> Result<Tuple, StackError> {
        self.ensure_elements_remaining(Tuple::LENGTH)?;

        Ok(Tuple::from_init_fn_option(|| self.backend.pop())
            .ok_or_else(|| self.build_underflow_error(Tuple::LENGTH))?
            .reverse())
    }
}

impl<T> ExtendHead for SimpleStack<T, Limited> {
    fn extend_head<Iter>(&mut self, iter: Iter) -> Result<(), StackError>
    where
        Iter: IntoIterator<Item = Self::Item>,
        Iter::IntoIter: DoubleEndedIterator + ExactSizeIterator,
    {
        let iter = iter.into_iter();
        self.ensure_space_remaining(iter.len())?;

        for element in iter {
            self.backend.push(element);
        }

        Ok(())
    }
}

impl<T> ExtendHead for SimpleStack<T, Unlimited> {
    fn extend_head<Iter>(&mut self, iter: Iter) -> Result<(), StackError>
    where
        Iter: IntoIterator<Item = Self::Item>,
        Iter::IntoIter: DoubleEndedIterator + ExactSizeIterator,
    {
        let iter = iter.into_iter();

        for element in iter {
            self.backend.push(element);
        }

        Ok(())
    }
}

impl<T, Type: SimpleStackType> DiscardHead for SimpleStack<T, Type> {
    fn discard_n_head(&mut self, n: usize) -> Result<(), StackError> {
        self.ensure_elements_remaining(n)?;

        self.backend.truncate(self.backend.len() - n);

        Ok(())
    }
}

pub type SimpleStackLimited<T> = SimpleStack<T, Limited>;
pub type SimpleStackUnlimited<T> = SimpleStack<T, Unlimited>;
