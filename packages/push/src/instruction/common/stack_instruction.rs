use std::fmt::{self, Display};

use strum::IntoEnumIterator;

use super::{Dup, Flush, IsEmpty, Pop, PushValue, StackDepth, Swap};
use crate::{
    error::{InstructionResult, MapInstructionError},
    instruction::{Instruction, NumOpens, Perform},
    push_vm::{HasStack, stack::StackError},
};

/// The instructions that every stack of type `T` has.
///
/// The typed instruction enums (for example
/// [`BoolInstruction`](crate::instruction::BoolInstruction)) include these
/// instructions instead of listing each one again.
// `Display` and `IntoEnumIterator` are written by hand: the strum derives
// would put `T: Display` and `T: Default` bounds on the whole enum.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[must_use]
pub enum StackInstruction<T> {
    Pop(Pop<T>),
    Push(PushValue<T>),
    Dup(Dup<T>),
    Swap(Swap<T>),
    IsEmpty(IsEmpty<T>),
    StackDepth(StackDepth<T>),
    Flush(Flush<T>),
}

impl<T> StackInstruction<T> {
    pub const fn pop() -> Self {
        Self::Pop(Pop::new())
    }
    pub const fn push(value: T) -> Self {
        Self::Push(PushValue::new(value))
    }
    pub const fn dup() -> Self {
        Self::Dup(Dup::new())
    }
    pub const fn swap() -> Self {
        Self::Swap(Swap::new())
    }
    pub const fn is_empty() -> Self {
        Self::IsEmpty(IsEmpty::new())
    }
    pub const fn stack_depth() -> Self {
        Self::StackDepth(StackDepth::new())
    }
    pub const fn flush() -> Self {
        Self::Flush(Flush::new())
    }
}

impl<T> Display for StackInstruction<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pop(_) => f.write_str("Pop"),
            Self::Push(push) => push.fmt(f),
            Self::Dup(_) => f.write_str("Dup"),
            Self::Swap(_) => f.write_str("Swap"),
            Self::IsEmpty(_) => f.write_str("IsEmpty"),
            Self::StackDepth(_) => f.write_str("StackDepth"),
            Self::Flush(_) => f.write_str("Flush"),
        }
    }
}

impl<T> IntoEnumIterator for StackInstruction<T>
where
    T: Clone + Default,
{
    type Iterator = std::array::IntoIter<Self, 7>;

    fn iter() -> Self::Iterator {
        [
            Self::pop(),
            Self::push(T::default()),
            Self::dup(),
            Self::swap(),
            Self::is_empty(),
            Self::stack_depth(),
            Self::flush(),
        ]
        .into_iter()
    }
}

impl<T> NumOpens for StackInstruction<T> {}

impl<T> Instruction for StackInstruction<T> {}

impl<S, T> Perform<S> for StackInstruction<T>
where
    S: Clone + HasStack<T> + HasStack<bool> + HasStack<i64>,
    T: Clone,
{
    type Error = StackError;

    fn perform(&self, state: S) -> InstructionResult<S, Self::Error> {
        match self {
            Self::Pop(pop) => pop.perform(state),
            Self::Push(push) => push.perform(state),
            Self::Dup(dup) => dup.perform(state),
            Self::Swap(swap) => swap.perform(state),
            Self::IsEmpty(is_empty) => is_empty.perform(state),
            Self::StackDepth(stack_depth) => stack_depth.perform(state),
            Self::Flush(flush) => flush.perform(state).map_err_into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    use super::StackInstruction;
    use crate::{
        instruction::Perform,
        push_vm::{HasStack, push_state::PushState},
    };

    // The top of the bool stack is `true`, and the value below it is `false`.
    fn state() -> PushState {
        PushState::builder()
            .with_max_stack_size(10)
            .with_no_program()
            .with_bool_values([true, false])
            .unwrap()
            .with_instruction_step_limit(1000)
            .build()
    }

    fn perform(instruction: StackInstruction<bool>) -> PushState {
        instruction.perform(state()).unwrap()
    }

    #[test]
    fn iter_gives_each_instruction() {
        assert_eq!(
            StackInstruction::<bool>::iter().collect::<Vec<_>>(),
            [
                StackInstruction::pop(),
                StackInstruction::push(false),
                StackInstruction::dup(),
                StackInstruction::swap(),
                StackInstruction::is_empty(),
                StackInstruction::stack_depth(),
                StackInstruction::flush(),
            ]
        );
    }

    #[test]
    fn display_names_each_instruction() {
        let names: Vec<String> = StackInstruction::<i64>::iter()
            .chain([StackInstruction::push(5)])
            .map(|i| i.to_string())
            .collect();
        assert_eq!(
            names,
            [
                "Pop",
                "Push(0)",
                "Dup",
                "Swap",
                "IsEmpty",
                "StackDepth",
                "Flush",
                "Push(5)",
            ]
        );
    }

    #[test]
    fn pop_removes_top() {
        let result = perform(StackInstruction::pop());
        assert_eq!(result.stack::<bool>().size(), 1);
        assert_eq!(result.stack::<bool>().top(), Ok(&false));
    }

    #[test]
    fn push_adds_value() {
        let result = perform(StackInstruction::push(false));
        assert_eq!(result.stack::<bool>().size(), 3);
        assert_eq!(result.stack::<bool>().top(), Ok(&false));
    }

    #[test]
    fn dup_copies_top() {
        let result = perform(StackInstruction::dup());
        assert_eq!(result.stack::<bool>().size(), 3);
        assert_eq!(result.stack::<bool>().top2(), Ok((&true, &true)));
    }

    #[test]
    fn swap_exchanges_top_two() {
        let result = perform(StackInstruction::swap());
        assert_eq!(result.stack::<bool>().top2(), Ok((&false, &true)));
    }

    #[test]
    fn is_empty_pushes_false_for_non_empty_stack() {
        let result = perform(StackInstruction::is_empty());
        assert_eq!(result.stack::<bool>().size(), 3);
        assert_eq!(result.stack::<bool>().top(), Ok(&false));
    }

    #[test]
    fn stack_depth_pushes_size() {
        let result = perform(StackInstruction::stack_depth());
        assert_eq!(result.stack::<i64>().top(), Ok(&2));
    }

    #[test]
    fn flush_empties_stack() {
        let result = perform(StackInstruction::flush());
        assert_eq!(result.stack::<bool>().size(), 0);
    }
}
