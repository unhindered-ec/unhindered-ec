use std::ops::Not;

use strum::IntoEnumIterator;

use super::{
    PushInstruction, PushInstructionError,
    common::{Dup, Flush, IsEmpty, Pop, PushValue, StackDepth, StackInstruction, Swap},
    printing::{Print, PrintLn},
};
use crate::{
    error::{InstructionResult, MapInstructionError},
    instruction::{Instruction, Perform},
    push_vm::{
        push_io::HasStdout,
        stack::{HasStack, PushOnto},
    },
};

// `IntoEnumIterator` is written by hand below, because the `EnumIter`
// derive gives only one instruction for the `Stack` variant.
#[derive(Debug, strum_macros::Display, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[must_use]
pub enum BoolInstruction {
    // The common stack instructions, for the boolean stack
    #[strum(to_string = "{0}")]
    Stack(StackInstruction<bool>),
    Print(Print<bool>),
    Println(PrintLn<bool>),

    Not,
    Or,
    And,
    Xor,
    Implies,
    // Do we really want either of these? Do they get used?
    // BooleanInvertFirstThenAnd,
    // BoolInvertSecondThenAnd,
    FromInt,
    // BoolFromFloat,
}

impl BoolInstruction {
    pub const fn pop() -> Self {
        Self::Stack(StackInstruction::pop())
    }
    pub const fn push(value: bool) -> Self {
        Self::Stack(StackInstruction::push(value))
    }
    pub const fn dup() -> Self {
        Self::Stack(StackInstruction::dup())
    }
    pub const fn swap() -> Self {
        Self::Stack(StackInstruction::swap())
    }
    pub const fn is_empty() -> Self {
        Self::Stack(StackInstruction::is_empty())
    }
    pub const fn stack_depth() -> Self {
        Self::Stack(StackInstruction::stack_depth())
    }
    pub const fn flush() -> Self {
        Self::Stack(StackInstruction::flush())
    }
    pub const fn print() -> Self {
        Self::Print(Print::<bool>::new())
    }
    pub const fn println() -> Self {
        Self::Println(PrintLn::<bool>::new())
    }
    pub const fn not() -> Self {
        Self::Not
    }
    pub const fn or() -> Self {
        Self::Or
    }
    pub const fn and() -> Self {
        Self::And
    }
    pub const fn xor() -> Self {
        Self::Xor
    }
    pub const fn implies() -> Self {
        Self::Implies
    }
    pub const fn from_int() -> Self {
        Self::FromInt
    }
}

// Keep this list in step with the variants of `BoolInstruction`.
impl IntoEnumIterator for BoolInstruction {
    type Iterator = std::vec::IntoIter<Self>;

    fn iter() -> Self::Iterator {
        StackInstruction::iter()
            .map(Self::Stack)
            .chain([
                Self::Print(Print::new()),
                Self::Println(PrintLn::new()),
                Self::Not,
                Self::Or,
                Self::And,
                Self::Xor,
                Self::Implies,
                Self::FromInt,
            ])
            .collect::<Vec<_>>()
            .into_iter()
    }
}

impl From<BoolInstruction> for PushInstruction {
    fn from(instr: BoolInstruction) -> Self {
        Self::BoolInstruction(instr)
    }
}

impl From<Pop<bool>> for BoolInstruction {
    fn from(pop: Pop<bool>) -> Self {
        Self::Stack(StackInstruction::Pop(pop))
    }
}

impl From<PushValue<bool>> for BoolInstruction {
    fn from(push: PushValue<bool>) -> Self {
        Self::Stack(StackInstruction::Push(push))
    }
}

impl From<Dup<bool>> for BoolInstruction {
    fn from(dup: Dup<bool>) -> Self {
        Self::Stack(StackInstruction::Dup(dup))
    }
}

impl From<Swap<bool>> for BoolInstruction {
    fn from(swap: Swap<bool>) -> Self {
        Self::Stack(StackInstruction::Swap(swap))
    }
}

impl From<IsEmpty<bool>> for BoolInstruction {
    fn from(is_empty: IsEmpty<bool>) -> Self {
        Self::Stack(StackInstruction::IsEmpty(is_empty))
    }
}

impl From<StackDepth<bool>> for BoolInstruction {
    fn from(stack_depth: StackDepth<bool>) -> Self {
        Self::Stack(StackInstruction::StackDepth(stack_depth))
    }
}

impl From<Flush<bool>> for BoolInstruction {
    fn from(flush: Flush<bool>) -> Self {
        Self::Stack(StackInstruction::Flush(flush))
    }
}

impl Instruction for BoolInstruction {}

impl<S> Perform<S> for BoolInstruction
where
    S: Clone + HasStack<bool> + HasStack<i64> + HasStdout,
{
    type Error = PushInstructionError;

    // TODO: This only "works" because all the stack operations are
    // "transactional", i.e., things like `pop2()` either completely succeed
    // or return an error without modifying the (mutable) state. (This is
    // done by checking that the size of the relevant stack is big enough
    // before removing any elements.) If any stack operations were _not_
    // "transactional" then we could end up passing an inconsistent state to
    // the call to `Error::recoverable_error()`, which would be bad. Because
    // the `pop` and `push` calls aren't together, we can still have
    // inconsistent states in the call to `Error::fatal_error()`. For
    // example, if the boolean is full and the instruction is `BoolFromInt`,
    // we could pop off an integer before we realize there's no room to push
    // on the new boolean. We can special case that, but the burden lies on
    // the programmer, with no help from the type system.

    /*
    // Get the nth character from a string and push it on the char stack.
    //   - n comes from the int stack
    //   - string comes from the string stack
    //   - result goes on the char stack
    let transaction: Transaction<PushState> = state.transaction();

    // This version has the transaction be mutable.
    let string = transaction.try_pop<String>()?;
    // This version returns a "new" transaction.
    let (string, transaction) = transaction.try_pop<String>()?;

    let (index, transaction) = transaction.try_pop<i64>()?;
    let c = string.chars.nth(index)?;
    let transaction = transaction.try_push<char>(c)?;
    let new_state = transaction.close()?; // Can closing actually fail?
     */

    // [pop string] then [pop integer] contains a closure with a tuple of
    // (string, int)

    // state.transaction().pop::<String>().with_min_length(1)
    //     .and_pop::<Integer>().then_push::<Char>(|(s, i)| s.chars.nth(i))
    // state.transaction().pop::<String>().with_min_length(1)
    //     .and_pop::<Integer>().charAt().then_push::<Char>()
    // state.transaction().pop::<String>().with_min_length(1)
    //     .and_pop::<Integer>().map::<Char>(|(s, i)|
    // s.chars.nth(i)).then_push::<Char>() Then you wouldn't be able to chain on
    // that and query what you would push onto the stack so maybe not ideal.

    // Options:
    //   '- Make operations reversible (undo/redo)
    //   '- Hold operations in some kind of queue and apply the at the end when
    //      we know they'll all work

    fn perform(&self, mut state: S) -> InstructionResult<S, Self::Error> {
        let bool_stack = state.stack_mut::<bool>();
        match self {
            Self::Stack(instruction) => instruction.perform(state).map_err_into(),
            Self::Print(print) => print.perform(state).map_err_into(),
            Self::Println(println) => println.perform(state).map_err_into(),

            Self::Not => bool_stack.pop().map(Not::not).push_onto(state),
            Self::And => bool_stack.pop2().map(|(x, y)| x && y).push_onto(state),
            Self::Or => bool_stack.pop2().map(|(x, y)| x || y).push_onto(state),
            Self::Xor => bool_stack.pop2().map(|(x, y)| x != y).push_onto(state),
            Self::Implies => bool_stack.pop2().map(|(x, y)| !x || y).push_onto(state),
            Self::FromInt => {
                let mut state = state.not_full::<bool>().map_err_into()?;
                state
                    .stack_mut::<i64>()
                    .pop()
                    .map(|i| i != 0)
                    .push_onto(state)
            }
        }
    }
}

#[cfg(test)]
mod property_tests {
    use proptest::prop_assert_eq;
    use strum::IntoEnumIterator;
    use test_strategy::proptest;

    use crate::{
        instruction::{BoolInstruction, Perform},
        push_vm::push_state::PushState,
    };

    fn all_instructions() -> Vec<BoolInstruction> {
        BoolInstruction::iter().collect()
    }

    #[proptest]
    fn ops_do_not_crash(
        #[strategy(proptest::sample::select(all_instructions()))] instr: BoolInstruction,
        #[any] x: bool,
        #[any] y: bool,
        #[any] i: i64,
    ) {
        let state = PushState::builder()
            .with_max_stack_size(3)
            .with_no_program()
            .with_bool_values([x, y])
            .unwrap()
            .with_int_values([i])
            .unwrap()
            .with_instruction_step_limit(1000)
            .build();

        instr.perform(state).unwrap();
    }

    #[proptest]
    fn and_is_correct(#[any] x: bool, #[any] y: bool) {
        let state = PushState::builder()
            .with_max_stack_size(2)
            .with_no_program()
            .with_bool_values([x, y])
            .unwrap()
            .with_instruction_step_limit(1000)
            .build();
        let result_state = BoolInstruction::And.perform(state).unwrap();

        prop_assert_eq!(result_state.bool.size(), 1);
        prop_assert_eq!(*result_state.bool.top().unwrap(), x && y);
    }

    #[proptest]
    fn implies_is_correct(#[any] x: bool, #[any] y: bool) {
        let state = PushState::builder()
            .with_max_stack_size(2)
            .with_no_program()
            .with_bool_values([x, y])
            .unwrap()
            .with_instruction_step_limit(1000)
            .build();
        let result_state = BoolInstruction::Implies.perform(state).unwrap();

        prop_assert_eq!(result_state.bool.size(), 1);
        prop_assert_eq!(*result_state.bool.top().unwrap(), !x || y);
    }
}

#[cfg(test)]
mod test {
    use super::BoolInstruction;

    #[test]
    fn from_wraps_each_stack_instruction() {
        use crate::instruction::common::{Dup, Flush, IsEmpty, Pop, PushValue, StackDepth, Swap};

        assert_eq!(
            [
                BoolInstruction::from(Pop::new()),
                BoolInstruction::from(PushValue::new(true)),
                BoolInstruction::from(Dup::new()),
                BoolInstruction::from(Swap::new()),
                BoolInstruction::from(IsEmpty::new()),
                BoolInstruction::from(StackDepth::new()),
                BoolInstruction::from(Flush::new()),
            ],
            [
                BoolInstruction::pop(),
                BoolInstruction::push(true),
                BoolInstruction::dup(),
                BoolInstruction::swap(),
                BoolInstruction::is_empty(),
                BoolInstruction::stack_depth(),
                BoolInstruction::flush(),
            ]
        );
    }

    #[test]
    fn auto_display() {
        assert_eq!(format!("{}", BoolInstruction::Not), "Not");
    }

    #[test]
    fn manual_push_display() {
        assert_eq!(format!("{}", BoolInstruction::push(true)), "Push(true)");
    }
}
