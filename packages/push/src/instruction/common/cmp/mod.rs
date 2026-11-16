mod equal;
mod greater_than;
mod greater_than_equal;
mod less_than;
mod less_than_equal;
mod not_equal;

pub use equal::Equal;
pub use greater_than::GreaterThan;
pub use greater_than_equal::GreaterThanEqual;
pub use less_than::LessThan;
pub use less_than_equal::LessThanEqual;
pub use not_equal::NotEqual;

/// Test helpers for the comparison instructions.
#[cfg(test)]
pub(super) mod test_support {
    use std::cmp::Ordering;

    use crate::{
        instruction::Perform,
        push_vm::{
            HasStack,
            push_state::PushState,
            stack::{Stack, TypeEq},
        },
    };

    /// A type that `i64` can be compared with, so that a test can compare
    /// two different stack types.
    #[derive(Debug, Default, Clone, PartialEq, Eq)]
    pub struct Small(pub i64);

    impl PartialEq<Small> for i64 {
        fn eq(&self, other: &Small) -> bool {
            *self == other.0
        }
    }

    impl PartialOrd<Small> for i64 {
        fn partial_cmp(&self, other: &Small) -> Option<Ordering> {
            self.partial_cmp(&other.0)
        }
    }

    /// A state with an `i64` stack, a `Small` stack, and a `bool` stack.
    #[derive(Debug, Default, Clone)]
    pub struct MixedState {
        ints: Stack<i64>,
        smalls: Stack<Small>,
        bools: Stack<bool>,
    }

    impl HasStack<i64> for MixedState {
        fn stack<U: TypeEq<This = i64>>(&self) -> &Stack<i64> {
            &self.ints
        }
        fn stack_mut<U: TypeEq<This = i64>>(&mut self) -> &mut Stack<i64> {
            &mut self.ints
        }
    }

    impl HasStack<Small> for MixedState {
        fn stack<U: TypeEq<This = Small>>(&self) -> &Stack<Small> {
            &self.smalls
        }
        fn stack_mut<U: TypeEq<This = Small>>(&mut self) -> &mut Stack<Small> {
            &mut self.smalls
        }
    }

    impl HasStack<bool> for MixedState {
        fn stack<U: TypeEq<This = bool>>(&self) -> &Stack<bool> {
            &self.bools
        }
        fn stack_mut<U: TypeEq<This = bool>>(&mut self) -> &mut Stack<bool> {
            &mut self.bools
        }
    }

    /// Compares `first` from the `i64` stack with `second` from the `Small`
    /// stack, and returns the result from the `bool` stack.
    pub fn compare_mixed<I>(instruction: &I, first: i64, second: i64) -> bool
    where
        I: Perform<MixedState, Error: std::fmt::Debug>,
    {
        let mut state = MixedState::default();
        state.ints.push(first).unwrap();
        state.smalls.push(Small(second)).unwrap();
        let result = instruction.perform(state).unwrap();
        *result.bools.top().unwrap()
    }

    /// Performs `instruction` on a full `bool` stack that holds `true` on top
    /// of `false`, and returns the result. The instruction pops two values, so
    /// there is room for the result.
    pub fn compare_on_full_bool_stack<I>(instruction: &I) -> bool
    where
        I: Perform<PushState, Error: std::fmt::Debug>,
    {
        let state = PushState::builder()
            .with_max_stack_size(2)
            .with_bool_values([true, false])
            .unwrap()
            .with_no_program()
            .with_instruction_step_limit(1000)
            .build();
        let result = instruction.perform(state).unwrap();
        assert_eq!(result.stack::<bool>().size(), 1);
        *result.stack::<bool>().top().unwrap()
    }
}
