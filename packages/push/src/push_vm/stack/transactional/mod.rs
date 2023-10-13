use crate::{error::into_state::StateMut, push_vm::PushInteger, type_eq::TypeEq};

use super::traits::has_stack::{HasStack, HasStackMut};

#[cfg(feature = "macros")]
use push_macros::wrapper_transaction;

trait Transaction<'a, Stack> {
    fn create(stack: &'a mut Stack) -> Self;
}

trait CreateTransaction<Backend, Transaction> {
    fn new_transaction<U: TypeEq<This = Backend>>(&mut self) -> Transaction;
}

struct SomeValue<const N: usize, T>(T);
