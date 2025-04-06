mod abs;
mod clamp;
mod negate;

use clamp::Clamp;
use miette::Diagnostic;
use strum_macros::EnumIter;

use self::{abs::Abs, negate::Negate};
use super::{
    Instruction, PushInstruction, PushInstructionError,
    common::{
        dup::Dup, flush::Flush, is_empty::IsEmpty, pop::Pop, push_value::PushValue,
        stack_depth::StackDepth, swap::Swap,
    },
    printing::print::{Print, PrintLn},
};
use crate::{
    error::{Error, InstructionResult},
    push_vm::{
        push_io::HasStdout,
        stack::{HasStack, PushOnto, Stack, StackDiscard, StackError},
    },
};

#[derive(Debug, strum_macros::Display, Copy, Clone, PartialEq, Eq, EnumIter)]
#[non_exhaustive]
#[must_use]
pub enum IntInstruction {
    // "Common" instructions specialized for the integer stack
    Pop(Pop<i64>),
    #[strum(to_string = "{0}")]
    Push(PushValue<i64>),
    Dup(Dup<i64>),
    Swap(Swap<i64>),
    IsEmpty(IsEmpty<i64>),
    StackDepth(StackDepth<i64>),
    Flush(Flush<i64>),
    Print(Print<i64>),
    PrintLn(PrintLn<i64>),

    Negate(Negate),
    Abs(Abs),
    Min,
    Max,
    Clamp(Clamp),
    Inc,
    Dec,
    Add,
    Subtract,
    Multiply,
    ProtectedDivide,
    Mod,
    Power,
    Square,
    // We can't easily convert from i64 to f64, etc.,
    // so we might need to do the log(n) integer
    // implementation of sqrt.
    // Sqrt,
    IsZero,
    IsPositive,
    IsNegative,
    IsEven,
    IsOdd,
    Equal,
    NotEqual,
    LessThan,
    LessThanEqual,
    GreaterThan,
    GreaterThanEqual,

    FromBoolean,
}

impl IntInstruction {
    pub const fn pop() -> Self {
        Self::Pop(Pop::new())
    }

    pub const fn push(value: i64) -> Self {
        Self::Push(PushValue(value))
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

    pub const fn negate() -> Self {
        Self::Negate(Negate)
    }

    pub const fn abs() -> Self {
        Self::Abs(Abs)
    }

    pub const fn clamp() -> Self {
        Self::Clamp(Clamp)
    }
}

impl From<IntInstruction> for PushInstruction {
    fn from(instr: IntInstruction) -> Self {
        Self::IntInstruction(instr)
    }
}

impl From<Pop<i64>> for IntInstruction {
    fn from(pop: Pop<i64>) -> Self {
        Self::Pop(pop)
    }
}

impl From<PushValue<i64>> for IntInstruction {
    fn from(push: PushValue<i64>) -> Self {
        Self::Push(push)
    }
}

impl From<Dup<i64>> for IntInstruction {
    fn from(dup: Dup<i64>) -> Self {
        Self::Dup(dup)
    }
}

impl From<Swap<i64>> for IntInstruction {
    fn from(swap: Swap<i64>) -> Self {
        Self::Swap(swap)
    }
}

impl From<IsEmpty<i64>> for IntInstruction {
    fn from(is_empty: IsEmpty<i64>) -> Self {
        Self::IsEmpty(is_empty)
    }
}

impl From<StackDepth<i64>> for IntInstruction {
    fn from(stack_depth: StackDepth<i64>) -> Self {
        Self::StackDepth(stack_depth)
    }
}

impl From<Flush<i64>> for IntInstruction {
    fn from(flush: Flush<i64>) -> Self {
        Self::Flush(flush)
    }
}

#[derive(thiserror::Error, Debug, Eq, PartialEq, Diagnostic)]
pub enum IntInstructionError {
    #[error("Integer arithmetic overflow for instruction {op}")]
    #[diagnostic(
        help = "If you run into this too often you might want to use the saturating or wrapping \
                arithmetic instruction set instead."
    )]
    Overflow {
        op: IntInstruction,
        // I liked the idea of keeping track of the arguments to the instruction
        // that led to the overflow, but that complicated `.perform()` in a variety
        // of ways so I'm removing that for now.
        // args: Vec<i64>,
    },
}

impl<S> Instruction<S> for IntInstruction
where
    S: Clone + HasStack<i64> + HasStack<bool> + HasStdout,
{
    type Error = PushInstructionError;

    #[expect(
        clippy::too_many_lines,
        reason = "This is legacy and arguably should be changed. Tracked in #227."
    )]
    fn perform(&self, mut state: S) -> InstructionResult<S, Self::Error> {
        match self {
            Self::Pop(pop) => pop.perform(state),
            Self::Push(push) => push.perform(state),
            Self::Dup(dup) => dup.perform(state),
            Self::Swap(swap) => swap.perform(state),
            Self::IsEmpty(is_empty) => is_empty.perform(state),
            Self::StackDepth(stack_depth) => stack_depth.perform(state),
            Self::Flush(flush) => flush.perform(state),
            Self::Print(print) => print.perform(state),
            Self::PrintLn(println) => println.perform(state),

            Self::Negate(negate) => negate.perform(state),
            Self::Abs(abs) => abs.perform(state),
            Self::Clamp(clamp) => clamp.perform(state),
            Self::Inc
            | Self::Dec
            | Self::Square
            | Self::Add
            | Self::Subtract
            | Self::Multiply
            | Self::ProtectedDivide
            | Self::Mod
            | Self::Power
            | Self::Min
            | Self::Max => {
                // All these instructions pop at least one value from the integer stack, so
                // we're guaranteed that there will be space for the result.
                // So we don't have to check that
                // any stacks are full before we start.
                let int_stack = state.stack_mut::<i64>();
                match self {
                    // This works, but is going to be nasty after we repeat a lot. There should
                    // perhaps be another trait method somewhere that eliminates a lot of this
                    // boilerplate.
                    Self::Inc => int_stack
                        .top()
                        .map_err(PushInstructionError::from)
                        .map(|&i| i.checked_add(1))
                        .and_then(|v| {
                            v.ok_or(IntInstructionError::Overflow { op: *self })
                                .map_err(Into::into)
                        })
                        .replace_on(1, state),

                    Self::Dec => int_stack
                        .top()
                        .map_err(PushInstructionError::from)
                        .map(|&i| i.checked_sub(1))
                        .and_then(|v| {
                            v.ok_or(IntInstructionError::Overflow { op: *self })
                                .map_err(Into::into)
                        })
                        .replace_on(1, state),

                    Self::Square => int_stack
                        .top()
                        .map_err(PushInstructionError::from)
                        .map(|&x| x.checked_mul(x))
                        .and_then(|v| {
                            v.ok_or(IntInstructionError::Overflow { op: *self })
                                .map_err(Into::into)
                        })
                        .replace_on(1, state),

                    Self::Add => int_stack
                        .top2()
                        .map_err(PushInstructionError::from)
                        .map(|(&x, &y)| x.checked_add(y))
                        .and_then(|v| {
                            v.ok_or(IntInstructionError::Overflow { op: *self })
                                .map_err(Into::into)
                        })
                        .replace_on(2, state),

                    Self::Subtract => int_stack
                        .top2()
                        .map_err(PushInstructionError::from)
                        .map(|(&x, &y)| x.checked_sub(y))
                        .and_then(|v| {
                            v.ok_or(IntInstructionError::Overflow { op: *self })
                                .map_err(Into::into)
                        })
                        .replace_on(2, state),

                    Self::Multiply => int_stack
                        .top2()
                        .map_err(PushInstructionError::from)
                        .map(|(x, y)| (*x).checked_mul(*y))
                        .and_then(|v| {
                            v.ok_or(IntInstructionError::Overflow { op: *self })
                                .map_err(Into::into)
                        })
                        .replace_on(2, state),

                    Self::ProtectedDivide => int_stack
                        .top2()
                        .map_err(PushInstructionError::from)
                        .map(|(&x, &y)| if y == 0 { Some(1) } else { x.checked_div(y) })
                        .and_then(|v| {
                            v.ok_or(IntInstructionError::Overflow { op: *self })
                                .map_err(Into::into)
                        })
                        .replace_on(2, state),

                    Self::Mod => int_stack
                        .top2()
                        .map_err(PushInstructionError::from)
                        .map(|(&x, &y)| if y == 0 { Some(0) } else { x.checked_rem(y) })
                        .and_then(|v| {
                            v.ok_or(IntInstructionError::Overflow { op: *self })
                                .map_err(Into::into)
                        })
                        .replace_on(2, state),

                    Self::Power => int_stack
                        .top2()
                        .map_err(PushInstructionError::from)
                        .map(|(&x, &y)| {
                            let y: u32 = y.try_into().ok()?;
                            x.checked_pow(y)
                        })
                        .and_then(|v| {
                            v.ok_or(IntInstructionError::Overflow { op: *self })
                                .map_err(Into::into)
                        })
                        .replace_on(2, state),

                    Self::Min => int_stack
                        .top2()
                        .map_err(PushInstructionError::from)
                        .map(|(&x, &y)| x.min(y))
                        .replace_on(2, state),

                    Self::Max => int_stack
                        .top2()
                        .map_err(PushInstructionError::from)
                        .map(|(&x, &y)| x.max(y))
                        .replace_on(2, state),
                    _ => {
                        unreachable!("We failed to handle an arithmetic Int instruction: {self:?}")
                    }
                }
            }
            Self::IsZero
            | Self::IsPositive
            | Self::IsNegative
            | Self::IsEven
            | Self::IsOdd
            | Self::Equal
            | Self::NotEqual
            | Self::LessThan
            | Self::LessThanEqual
            | Self::GreaterThan
            | Self::GreaterThanEqual => {
                // None of these instructions pop anything off the boolean stack, but
                // they will push a result onto that stack. Thus before we start performing
                // the instruction, we need to check for the case that the boolean stack is
                // already full, and return an `Overflow` error if it is.
                if state.stack::<bool>().is_full() {
                    return Err(Error::fatal(
                        state,
                        StackError::Overflow { stack_type: "bool" },
                    ));
                }
                let int_stack: &mut Stack<i64> = state.stack_mut::<i64>();
                match self {
                    Self::IsZero => int_stack
                        .top()
                        .map_err(PushInstructionError::from)
                        .map(|&x| x == 0)
                        .push_onto(state)
                        .with_stack_discard::<i64>(1),
                    Self::IsPositive => int_stack
                        .top()
                        .map_err(PushInstructionError::from)
                        .map(|&x| x > 0)
                        .push_onto(state)
                        .with_stack_discard::<i64>(1),
                    Self::IsNegative => int_stack
                        .top()
                        .map_err(PushInstructionError::from)
                        .map(|&x| x < 0)
                        .push_onto(state)
                        .with_stack_discard::<i64>(1),
                    // TODO: Write a test for IsEven that makes sure
                    // all the stack manipulation is correct.
                    Self::IsEven => int_stack
                        .top()
                        .map_err(PushInstructionError::from)
                        .map(|&x| x % 2 == 0)
                        .push_onto(state)
                        .with_stack_discard::<i64>(1),

                    Self::IsOdd => int_stack
                        .top()
                        .map_err(PushInstructionError::from)
                        .map(|&x| x % 2 == 1)
                        .push_onto(state)
                        .with_stack_discard::<i64>(1),

                    Self::Equal => int_stack
                        .top2()
                        .map_err(PushInstructionError::from)
                        .map(|(&x, &y)| x == y)
                        .push_onto(state)
                        .with_stack_discard::<i64>(1),

                    Self::NotEqual => int_stack
                        .top2()
                        .map_err(PushInstructionError::from)
                        .map(|(&x, &y)| x != y)
                        .push_onto(state)
                        .with_stack_discard::<i64>(1),

                    Self::LessThan => int_stack
                        .top2()
                        .map_err(PushInstructionError::from)
                        .map(|(&x, &y)| x < y)
                        .push_onto(state)
                        .with_stack_discard::<i64>(1),

                    Self::LessThanEqual => int_stack
                        .top2()
                        .map_err(PushInstructionError::from)
                        .map(|(&x, &y)| x <= y)
                        .push_onto(state)
                        .with_stack_discard::<i64>(1),

                    Self::GreaterThan => int_stack
                        .top2()
                        .map_err(PushInstructionError::from)
                        .map(|(&x, &y)| x > y)
                        .push_onto(state)
                        .with_stack_discard::<i64>(1),

                    Self::GreaterThanEqual => int_stack
                        .top2()
                        .map_err(PushInstructionError::from)
                        .map(|(&x, &y)| x >= y)
                        .push_onto(state)
                        .with_stack_discard::<i64>(1),
                    _ => unreachable!(
                        "We failed to implement a boolean-valued operation on integers: {self:?}"
                    ),
                }
            }
            Self::FromBoolean => {
                let bool_stack = state.stack_mut::<bool>();
                bool_stack
                    .top()
                    .map_err(PushInstructionError::from)
                    .map(|&b| i64::from(b))
                    .push_onto(state)
                    .with_stack_discard::<bool>(1)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::IntInstruction;

    #[test]
    fn auto_display() {
        assert_eq!(format!("{}", IntInstruction::IsZero), "IsZero");
    }

    #[test]
    fn manual_push_display() {
        assert_eq!(format!("{}", IntInstruction::push(1)), "Push(1)");
    }
}
