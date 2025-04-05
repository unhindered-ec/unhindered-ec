use ordered_float::OrderedFloat;
use strum_macros::EnumIter;

use super::{
    Instruction, PushInstruction, PushInstructionError,
    common::{
        dup::Dup, flush::Flush, is_empty::IsEmpty, pop::Pop, push_value::PushValue,
        stack_depth::StackDepth, swap::Swap,
    },
    printing::print::Print,
};
use crate::{
    error::{Error, InstructionResult},
    push_vm::{
        HasStack,
        push_io::HasStdout,
        stack::{PushOnto, Stack, StackDiscard, StackError},
    },
};

#[derive(Debug, strum_macros::Display, Copy, Clone, EnumIter, Eq, PartialEq)]
#[non_exhaustive]
pub enum FloatInstruction {
    // "Common" instructions specialized for the integer stack
    Pop(Pop<OrderedFloat<f64>>),
    #[strum(to_string = "{0}")]
    Push(PushValue<OrderedFloat<f64>>),
    Dup(Dup<OrderedFloat<f64>>),
    Swap(Swap<OrderedFloat<f64>>),
    IsEmpty(IsEmpty<OrderedFloat<f64>>),
    StackDepth(StackDepth<OrderedFloat<f64>>),
    Flush(Flush<OrderedFloat<f64>>),
    Print(Print<OrderedFloat<f64>>),

    Add,
    Subtract,
    Multiply,
    ProtectedDivide,
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
}

impl FloatInstruction {
    #[must_use]
    pub const fn pop() -> Self {
        Self::Pop(Pop::new())
    }

    #[must_use]
    pub const fn push(value: f64) -> Self {
        Self::Push(PushValue(OrderedFloat(value)))
    }

    #[must_use]
    pub const fn push_ordered_float(value: OrderedFloat<f64>) -> Self {
        Self::Push(PushValue(value))
    }

    #[must_use]
    pub const fn dup() -> Self {
        Self::Dup(Dup::new())
    }

    #[must_use]
    pub const fn swap() -> Self {
        Self::Swap(Swap::new())
    }

    #[must_use]
    pub const fn is_empty() -> Self {
        Self::IsEmpty(IsEmpty::new())
    }

    #[must_use]
    pub const fn stack_depth() -> Self {
        Self::StackDepth(StackDepth::new())
    }

    #[must_use]
    pub const fn flush() -> Self {
        Self::Flush(Flush::new())
    }
}

impl From<FloatInstruction> for PushInstruction {
    fn from(instr: FloatInstruction) -> Self {
        Self::FloatInstruction(instr)
    }
}

impl From<Pop<OrderedFloat<f64>>> for FloatInstruction {
    fn from(pop: Pop<OrderedFloat<f64>>) -> Self {
        Self::Pop(pop)
    }
}

impl From<PushValue<OrderedFloat<f64>>> for FloatInstruction {
    fn from(push: PushValue<OrderedFloat<f64>>) -> Self {
        Self::Push(push)
    }
}

impl From<Dup<OrderedFloat<f64>>> for FloatInstruction {
    fn from(dup: Dup<OrderedFloat<f64>>) -> Self {
        Self::Dup(dup)
    }
}

impl From<Swap<OrderedFloat<f64>>> for FloatInstruction {
    fn from(swap: Swap<OrderedFloat<f64>>) -> Self {
        Self::Swap(swap)
    }
}

impl From<IsEmpty<OrderedFloat<f64>>> for FloatInstruction {
    fn from(is_empty: IsEmpty<OrderedFloat<f64>>) -> Self {
        Self::IsEmpty(is_empty)
    }
}

impl From<StackDepth<OrderedFloat<f64>>> for FloatInstruction {
    fn from(stack_depth: StackDepth<OrderedFloat<f64>>) -> Self {
        Self::StackDepth(stack_depth)
    }
}

impl From<Flush<OrderedFloat<f64>>> for FloatInstruction {
    fn from(flush: Flush<OrderedFloat<f64>>) -> Self {
        Self::Flush(flush)
    }
}

impl<S> Instruction<S> for FloatInstruction
where
    S: Clone + HasStack<OrderedFloat<f64>> + HasStack<bool> + HasStack<i64> + HasStdout,
{
    type Error = PushInstructionError;

    fn perform(&self, state: S) -> InstructionResult<S, Self::Error> {
        match self {
            Self::Pop(pop) => pop.perform(state),
            Self::Push(push) => push.perform(state),
            Self::Dup(dup) => dup.perform(state),
            Self::Swap(swap) => swap.perform(state),
            Self::IsEmpty(is_empty) => is_empty.perform(state),
            Self::StackDepth(stack_depth) => stack_depth.perform(state),
            Self::Flush(flush) => flush.perform(state),
            Self::Print(print) => print.perform(state),

            // All these instructions pop at least one value from the float stack, so we're
            // guaranteed that there will be space for the result. So we don't have to check that
            // any stacks are full before we start.
            Self::Add => Self::binary_arithmetic(state, std::ops::Add::add),
            Self::Subtract => Self::binary_arithmetic(state, std::ops::Sub::sub),
            Self::Multiply => Self::binary_arithmetic(state, std::ops::Mul::mul),
            Self::ProtectedDivide => Self::binary_arithmetic(state, |x, y| {
                #[expect(
                    clippy::arithmetic_side_effects,
                    reason = "Dividing floats won't overflow"
                )]
                if y == 0.0 { OrderedFloat(1.0) } else { x / y }
            }),

            // None of these instructions pop anything off the boolean stack, but
            // they will push a result onto that stack. Thus before we start performing
            // the instruction, we need to check for the case that the boolean stack is
            // already full, and return an `Overflow` error if it is.
            Self::Equal => Self::binary_predicate(state, std::cmp::PartialEq::eq),
            Self::NotEqual => Self::binary_predicate(state, std::cmp::PartialEq::ne),
            Self::GreaterThan => Self::binary_predicate(state, std::cmp::PartialOrd::gt),
            Self::LessThan => Self::binary_predicate(state, std::cmp::PartialOrd::lt),
            Self::GreaterThanOrEqual => Self::binary_predicate(state, std::cmp::PartialOrd::ge),
            Self::LessThanOrEqual => Self::binary_predicate(state, std::cmp::PartialOrd::le),
        }
    }
}

impl FloatInstruction {
    fn binary_arithmetic<S>(
        mut state: S,
        op: impl FnOnce(OrderedFloat<f64>, OrderedFloat<f64>) -> OrderedFloat<f64>,
    ) -> Result<S, Error<S, PushInstructionError>>
    where
        S: Clone + HasStack<OrderedFloat<f64>>,
    {
        let float_stack = state.stack_mut::<OrderedFloat<f64>>();
        float_stack
            .top2()
            .map_err(PushInstructionError::from)
            .map(|(&x, &y)| op(x, y))
            .replace_on(2, state)
    }

    fn binary_predicate<S>(
        mut state: S,
        op: impl FnOnce(&OrderedFloat<f64>, &OrderedFloat<f64>) -> bool,
    ) -> Result<S, Error<S, PushInstructionError>>
    where
        S: Clone + HasStack<OrderedFloat<f64>> + HasStack<bool>,
    {
        if state.stack::<bool>().is_full() {
            return Err(Error::fatal(
                state,
                StackError::Overflow { stack_type: "bool" },
            ));
        }
        let float_stack: &mut Stack<OrderedFloat<f64>> = state.stack_mut::<OrderedFloat<f64>>();
        float_stack
            .top2()
            .map_err(PushInstructionError::from)
            .map(|(x, y)| op(x, y))
            .push_onto(state)
            .with_stack_discard::<OrderedFloat<f64>>(1)
    }
}

#[cfg(test)]
mod test {
    use super::FloatInstruction;

    #[test]
    fn auto_display() {
        assert_eq!(format!("{}", FloatInstruction::NotEqual), "NotEqual");
    }

    #[test]
    fn manual_push_display() {
        assert_eq!(format!("{}", FloatInstruction::push(1.0)), "Push(1)");
    }
}
