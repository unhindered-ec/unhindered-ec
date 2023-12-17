use crate::instruction::PushInstruction;

use super::NumOpens;

/*
 * exec_if requires a boolean and two (additional) values on the exec stack.
 * If the bool is true, we remove the second of the two exec stack values,
 * and if it's false, we remove the first.
 */

/*
 * exec_while requires a boolean and one additional value on the exec stack.
 * If the bool is true, then you push a copy of the "body" onto the exec, followed
 * by another copy of exec_while.
 */

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ExecInstruction {
    Dup,
    IfElse,
}

impl From<ExecInstruction> for PushInstruction {
    fn from(instr: ExecInstruction) -> Self {
        Self::Exec(instr)
    }
}

impl NumOpens for ExecInstruction {
    fn num_opens(&self) -> usize {
        match self {
            Self::Dup => 1,
            Self::IfElse => 2,
        }
    }
}
