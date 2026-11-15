use ordered_float::OrderedFloat;
use strum_macros::EnumIter;

use super::{
    PushInstruction, PushInstructionError, TypedInstruction,
    printing::{Print, PrintLn},
};
use crate::{
    error::{Error, InstructionResult, MapInstructionError},
    instruction::{Instruction, Perform},
    push_vm::{
        HasStack,
        push_io::HasStdout,
        stack::{PushOnto, Stack, StackDiscard, StackError},
    },
};

/// The instructions for the float stack.
pub type FloatInstruction = TypedInstruction<OrderedFloat<f64>, FloatOp>;

/// The float-specific instructions in [`FloatInstruction`].
#[derive(Debug, strum_macros::Display, Copy, Clone, EnumIter, Eq, PartialEq)]
#[non_exhaustive]
#[must_use]
pub enum FloatOp {
    Print(Print<OrderedFloat<f64>>),
    PrintLn(PrintLn<OrderedFloat<f64>>),

    // Arithmetic instructions
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

    FromIntApprox,
}

impl FloatInstruction {
    pub const fn push_f64(value: f64) -> Self {
        Self::push(OrderedFloat(value))
    }
    pub const fn print() -> Self {
        Self::Op(FloatOp::Print(Print::new()))
    }
    pub const fn print_ln() -> Self {
        Self::Op(FloatOp::PrintLn(PrintLn::new()))
    }
    pub const fn add() -> Self {
        Self::Op(FloatOp::Add)
    }
    pub const fn subtract() -> Self {
        Self::Op(FloatOp::Subtract)
    }
    pub const fn multiply() -> Self {
        Self::Op(FloatOp::Multiply)
    }
    pub const fn protected_divide() -> Self {
        Self::Op(FloatOp::ProtectedDivide)
    }
    pub const fn equal() -> Self {
        Self::Op(FloatOp::Equal)
    }
    pub const fn not_equal() -> Self {
        Self::Op(FloatOp::NotEqual)
    }
    pub const fn greater_than() -> Self {
        Self::Op(FloatOp::GreaterThan)
    }
    pub const fn less_than() -> Self {
        Self::Op(FloatOp::LessThan)
    }
    pub const fn greater_than_or_equal() -> Self {
        Self::Op(FloatOp::GreaterThanOrEqual)
    }
    pub const fn less_than_or_equal() -> Self {
        Self::Op(FloatOp::LessThanOrEqual)
    }
    pub const fn from_int_approx() -> Self {
        Self::Op(FloatOp::FromIntApprox)
    }
}

impl From<FloatInstruction> for PushInstruction {
    fn from(instr: FloatInstruction) -> Self {
        Self::FloatInstruction(instr)
    }
}

impl Instruction for FloatOp {}

impl<S> Perform<S> for FloatOp
where
    S: Clone + HasStack<OrderedFloat<f64>> + HasStack<bool> + HasStack<i64> + HasStdout,
{
    type Error = PushInstructionError;

    fn perform(&self, mut state: S) -> InstructionResult<S, Self::Error> {
        match self {
            Self::Print(print) => print.perform(state).map_err_into(),
            Self::PrintLn(println) => println.perform(state).map_err_into(),

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

            Self::FromIntApprox => {
                let int_stack = state.stack_mut::<i64>();
                #[expect(
                    clippy::as_conversions,
                    clippy::cast_precision_loss,
                    reason = "This instruction is meant to be an approximate conversion"
                )]
                int_stack
                    .top()
                    .map_err(PushInstructionError::from)
                    .map(|&i| OrderedFloat(i as f64))
                    .push_onto(state)
                    .with_stack_discard::<i64>(1)
            }
        }
    }
}

impl FloatOp {
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
            let max_size = state.stack::<bool>().max_stack_size();
            return Err(Error::fatal(
                state,
                StackError::Overflow {
                    stack_type: "bool",
                    max_size,
                },
            ));
        }
        let float_stack: &mut Stack<OrderedFloat<f64>> = state.stack_mut::<OrderedFloat<f64>>();
        float_stack
            .top2()
            .map_err(PushInstructionError::from)
            .map(|(x, y)| op(x, y))
            .push_onto(state)
            .with_stack_discard::<OrderedFloat<f64>>(2)
    }
}

#[cfg(test)]
mod test {
    use ordered_float::OrderedFloat;
    use strum::IntoEnumIterator;

    use super::{FloatInstruction, FloatOp};

    // The constructors are in the same order as the `FloatOp` variants.
    #[test]
    fn constructors_wrap_float_ops() {
        assert_eq!(
            vec![
                FloatInstruction::print(),
                FloatInstruction::print_ln(),
                FloatInstruction::add(),
                FloatInstruction::subtract(),
                FloatInstruction::multiply(),
                FloatInstruction::protected_divide(),
                FloatInstruction::equal(),
                FloatInstruction::not_equal(),
                FloatInstruction::greater_than(),
                FloatInstruction::less_than(),
                FloatInstruction::greater_than_or_equal(),
                FloatInstruction::less_than_or_equal(),
                FloatInstruction::from_int_approx(),
            ],
            FloatOp::iter()
                .map(FloatInstruction::Op)
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn push_f64_wraps_value() {
        assert_eq!(
            FloatInstruction::push_f64(1.5),
            FloatInstruction::push(OrderedFloat(1.5))
        );
    }

    #[test]
    fn auto_display() {
        assert_eq!(format!("{}", FloatInstruction::not_equal()), "NotEqual");
    }

    #[test]
    fn manual_push_display() {
        assert_eq!(format!("{}", FloatInstruction::push_f64(1.0)), "Push(1)");
    }
}
