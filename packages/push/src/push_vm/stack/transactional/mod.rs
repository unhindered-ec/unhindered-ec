use crate::{error::into_state::StateMut, push_vm::PushInteger, type_eq::TypeEq};

use super::traits::has_stack::{HasStack, HasStackMut};

#[cfg(feature = "macros")]
use push_macros::wrapper_transaction;

fn test<T>() {}

fn foo<S>(state: S)
where
    S: HasStackMut<bool> + StateMut,
{
    let transaction = {
        struct Test {}

        impl Into<usize> for Test {
            fn into(self) -> usize {
                0
            }
        }

        Test {}
    };

    let val: usize = transaction.into();

    let transaction = wrapper_transaction!(state, stacks = {
        bool,
        mut PushInteger: Cloning
    });
    // let transaction = state.new_transaction::<for_stacks! { bool: Cloning }>()
}

trait Transaction<'a, Stack> {
    fn create(stack: &'a mut Stack) -> Self;
}

trait CreateTransaction<Backend, Transaction> {
    fn new_transaction<U: TypeEq<This = Backend>>(&mut self) -> Transaction;
}

struct SomeValue<const N: usize, T>(T);
