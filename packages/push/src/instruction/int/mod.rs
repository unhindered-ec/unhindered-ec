mod abs;
mod clamp;
mod negate;

use clamp::Clamp;
use miette::Diagnostic;
use ordered_float::OrderedFloat;
use strum_macros::EnumIter;

use self::{abs::Abs, negate::Negate};
use super::{
    PushInstruction, PushInstructionError, TypedInstruction,
    printing::{Print, PrintLn},
    typed::op_constants,
};
use crate::{
    error::{Error, InstructionResult, MapInstructionError},
    instruction::{Instruction, Perform},
    push_vm::{
        push_io::HasStdout,
        stack::{HasStack, PushOnto, Stack, StackDiscard, StackError},
    },
};

/// The instructions for the integer stack.
pub type IntInstruction = TypedInstruction<i64, IntOp>;

/// The integer-specific instructions in [`IntInstruction`].
#[derive(Debug, strum_macros::Display, Copy, Clone, PartialEq, Eq, EnumIter)]
#[non_exhaustive]
#[must_use]
pub enum IntOp {
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
    FromFloatApprox,
}

impl IntInstruction {
    pub const fn print() -> Self {
        Self::Op(IntOp::Print(Print::new()))
    }
    pub const fn print_ln() -> Self {
        Self::Op(IntOp::PrintLn(PrintLn::new()))
    }
    pub const fn negate() -> Self {
        Self::Op(IntOp::Negate(Negate::new()))
    }
    pub const fn abs() -> Self {
        Self::Op(IntOp::Abs(Abs::new()))
    }
    pub const fn min() -> Self {
        Self::Op(IntOp::Min)
    }
    pub const fn max() -> Self {
        Self::Op(IntOp::Max)
    }
    pub const fn clamp() -> Self {
        Self::Op(IntOp::Clamp(Clamp::new()))
    }
    pub const fn inc() -> Self {
        Self::Op(IntOp::Inc)
    }
    pub const fn dec() -> Self {
        Self::Op(IntOp::Dec)
    }
    pub const fn add() -> Self {
        Self::Op(IntOp::Add)
    }
    pub const fn subtract() -> Self {
        Self::Op(IntOp::Subtract)
    }
    pub const fn multiply() -> Self {
        Self::Op(IntOp::Multiply)
    }
    pub const fn protected_divide() -> Self {
        Self::Op(IntOp::ProtectedDivide)
    }
    pub const fn r#mod() -> Self {
        Self::Op(IntOp::Mod)
    }
    pub const fn power() -> Self {
        Self::Op(IntOp::Power)
    }
    pub const fn square() -> Self {
        Self::Op(IntOp::Square)
    }
    pub const fn is_zero() -> Self {
        Self::Op(IntOp::IsZero)
    }
    pub const fn is_positive() -> Self {
        Self::Op(IntOp::IsPositive)
    }
    pub const fn is_negative() -> Self {
        Self::Op(IntOp::IsNegative)
    }
    pub const fn is_even() -> Self {
        Self::Op(IntOp::IsEven)
    }
    pub const fn is_odd() -> Self {
        Self::Op(IntOp::IsOdd)
    }
    pub const fn equal() -> Self {
        Self::Op(IntOp::Equal)
    }
    pub const fn not_equal() -> Self {
        Self::Op(IntOp::NotEqual)
    }
    pub const fn less_than() -> Self {
        Self::Op(IntOp::LessThan)
    }
    pub const fn less_than_equal() -> Self {
        Self::Op(IntOp::LessThanEqual)
    }
    pub const fn greater_than() -> Self {
        Self::Op(IntOp::GreaterThan)
    }
    pub const fn greater_than_equal() -> Self {
        Self::Op(IntOp::GreaterThanEqual)
    }
    pub const fn from_boolean() -> Self {
        Self::Op(IntOp::FromBoolean)
    }
    pub const fn from_float_approx() -> Self {
        Self::Op(IntOp::FromFloatApprox)
    }
}

op_constants!(IntInstruction, IntOp {
    Negate = IntOp::Negate(Negate::new()),
    Abs = IntOp::Abs(Abs::new()),
    Min,
    Max,
    Clamp = IntOp::Clamp(Clamp::new()),
    Inc,
    Dec,
    Add,
    Subtract,
    Multiply,
    ProtectedDivide,
    Mod,
    Power,
    Square,
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
    FromFloatApprox,
});

/// Functions named like the old `Print` and `PrintLn` variants, so that code
/// such as `IntInstruction::Print(Print::default())` still compiles.
#[expect(
    non_snake_case,
    reason = "The names match the variants of the old typed enums."
)]
impl IntInstruction {
    pub const fn Print(print: Print<i64>) -> Self {
        Self::Op(IntOp::Print(print))
    }
    pub const fn PrintLn(println: PrintLn<i64>) -> Self {
        Self::Op(IntOp::PrintLn(println))
    }
}

impl From<IntInstruction> for PushInstruction {
    fn from(instr: IntInstruction) -> Self {
        Self::IntInstruction(instr)
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

impl Instruction for IntOp {}

impl<S> Perform<S> for IntOp
where
    S: Clone + HasStack<i64> + HasStack<bool> + HasStack<OrderedFloat<f64>> + HasStdout,
{
    type Error = PushInstructionError;

    #[expect(
        clippy::too_many_lines,
        reason = "This is legacy and arguably should be changed. Tracked in #227."
    )]
    fn perform(&self, mut state: S) -> InstructionResult<S, Self::Error> {
        match self {
            Self::Print(print) => print.perform(state).map_err_into(),
            Self::PrintLn(println) => println.perform(state).map_err_into(),

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
                // All these instructions pop at least one value from the
                // integer stack, so we're guaranteed that there
                // will be space for the result. So we don't
                // have to check that any stacks are full before
                // we start.
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
                            v.ok_or(IntInstructionError::Overflow {
                                op: IntInstruction::Op(*self),
                            })
                            .map_err(Into::into)
                        })
                        .replace_on(1, state),

                    Self::Dec => int_stack
                        .top()
                        .map_err(PushInstructionError::from)
                        .map(|&i| i.checked_sub(1))
                        .and_then(|v| {
                            v.ok_or(IntInstructionError::Overflow {
                                op: IntInstruction::Op(*self),
                            })
                            .map_err(Into::into)
                        })
                        .replace_on(1, state),

                    Self::Square => int_stack
                        .top()
                        .map_err(PushInstructionError::from)
                        .map(|&x| x.checked_mul(x))
                        .and_then(|v| {
                            v.ok_or(IntInstructionError::Overflow {
                                op: IntInstruction::Op(*self),
                            })
                            .map_err(Into::into)
                        })
                        .replace_on(1, state),

                    Self::Add => int_stack
                        .top2()
                        .map_err(PushInstructionError::from)
                        .map(|(&x, &y)| x.checked_add(y))
                        .and_then(|v| {
                            v.ok_or(IntInstructionError::Overflow {
                                op: IntInstruction::Op(*self),
                            })
                            .map_err(Into::into)
                        })
                        .replace_on(2, state),

                    Self::Subtract => int_stack
                        .top2()
                        .map_err(PushInstructionError::from)
                        .map(|(&x, &y)| x.checked_sub(y))
                        .and_then(|v| {
                            v.ok_or(IntInstructionError::Overflow {
                                op: IntInstruction::Op(*self),
                            })
                            .map_err(Into::into)
                        })
                        .replace_on(2, state),

                    Self::Multiply => int_stack
                        .top2()
                        .map_err(PushInstructionError::from)
                        .map(|(x, y)| (*x).checked_mul(*y))
                        .and_then(|v| {
                            v.ok_or(IntInstructionError::Overflow {
                                op: IntInstruction::Op(*self),
                            })
                            .map_err(Into::into)
                        })
                        .replace_on(2, state),

                    Self::ProtectedDivide => int_stack
                        .top2()
                        .map_err(PushInstructionError::from)
                        .map(|(&x, &y)| if y == 0 { Some(1) } else { x.checked_div(y) })
                        .and_then(|v| {
                            v.ok_or(IntInstructionError::Overflow {
                                op: IntInstruction::Op(*self),
                            })
                            .map_err(Into::into)
                        })
                        .replace_on(2, state),

                    Self::Mod => int_stack
                        .top2()
                        .map_err(PushInstructionError::from)
                        .map(|(&x, &y)| if y == 0 { Some(0) } else { x.checked_rem(y) })
                        .and_then(|v| {
                            v.ok_or(IntInstructionError::Overflow {
                                op: IntInstruction::Op(*self),
                            })
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
                            v.ok_or(IntInstructionError::Overflow {
                                op: IntInstruction::Op(*self),
                            })
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
                // None of these instructions pop anything off the boolean
                // stack, but they will push a result onto that
                // stack. Thus before we start performing
                // the instruction, we need to check for the case that the
                // boolean stack is already full, and return an
                // `Overflow` error if it is.
                if state.stack::<bool>().is_full() {
                    let max_size = state.stack::<bool>().max_stack_size();
                    return Err(Error::fatal(
                        state,
                        StackError::Overflow {
                            stack_type: "bool",
                            max_size,
                        },
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
                        .with_stack_discard::<i64>(2),

                    Self::NotEqual => int_stack
                        .top2()
                        .map_err(PushInstructionError::from)
                        .map(|(&x, &y)| x != y)
                        .push_onto(state)
                        .with_stack_discard::<i64>(2),

                    Self::LessThan => int_stack
                        .top2()
                        .map_err(PushInstructionError::from)
                        .map(|(&x, &y)| x < y)
                        .push_onto(state)
                        .with_stack_discard::<i64>(2),

                    Self::LessThanEqual => int_stack
                        .top2()
                        .map_err(PushInstructionError::from)
                        .map(|(&x, &y)| x <= y)
                        .push_onto(state)
                        .with_stack_discard::<i64>(2),

                    Self::GreaterThan => int_stack
                        .top2()
                        .map_err(PushInstructionError::from)
                        .map(|(&x, &y)| x > y)
                        .push_onto(state)
                        .with_stack_discard::<i64>(2),

                    Self::GreaterThanEqual => int_stack
                        .top2()
                        .map_err(PushInstructionError::from)
                        .map(|(&x, &y)| x >= y)
                        .push_onto(state)
                        .with_stack_discard::<i64>(2),
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
            Self::FromFloatApprox => {
                let float_stack = state.stack_mut::<OrderedFloat<f64>>();
                #[expect(
                    clippy::as_conversions,
                    clippy::cast_possible_truncation,
                    reason = "This instruction is meant to be an approximate conversion"
                )]
                float_stack
                    .top()
                    .map_err(PushInstructionError::from)
                    .map(|&OrderedFloat(f)| f as i64)
                    .push_onto(state)
                    .with_stack_discard::<OrderedFloat<f64>>(1)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use strum::IntoEnumIterator;

    use super::{IntInstruction, IntOp};

    // The constructors are in the same order as the `IntOp` variants.
    #[test]
    fn constructors_wrap_int_ops() {
        assert_eq!(
            vec![
                IntInstruction::print(),
                IntInstruction::print_ln(),
                IntInstruction::negate(),
                IntInstruction::abs(),
                IntInstruction::min(),
                IntInstruction::max(),
                IntInstruction::clamp(),
                IntInstruction::inc(),
                IntInstruction::dec(),
                IntInstruction::add(),
                IntInstruction::subtract(),
                IntInstruction::multiply(),
                IntInstruction::protected_divide(),
                IntInstruction::r#mod(),
                IntInstruction::power(),
                IntInstruction::square(),
                IntInstruction::is_zero(),
                IntInstruction::is_positive(),
                IntInstruction::is_negative(),
                IntInstruction::is_even(),
                IntInstruction::is_odd(),
                IntInstruction::equal(),
                IntInstruction::not_equal(),
                IntInstruction::less_than(),
                IntInstruction::less_than_equal(),
                IntInstruction::greater_than(),
                IntInstruction::greater_than_equal(),
                IntInstruction::from_boolean(),
                IntInstruction::from_float_approx(),
            ],
            IntOp::iter().map(IntInstruction::Op).collect::<Vec<_>>()
        );
    }

    #[test]
    fn auto_display() {
        assert_eq!(format!("{}", IntInstruction::is_zero()), "IsZero");
    }

    #[test]
    fn manual_push_display() {
        assert_eq!(format!("{}", IntInstruction::push(1)), "Push(1)");
    }
}
