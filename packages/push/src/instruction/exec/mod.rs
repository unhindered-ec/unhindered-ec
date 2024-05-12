mod dup;
mod ifelse;
mod unless;
mod when;

use self::{dup::Dup, ifelse::IfElse, unless::Unless, when::When};
use super::{instruction_error::PushInstructionError, Instruction, NumOpens, PushInstruction};
use crate::{
    error::InstructionResult,
    push_vm::{program::PushProgram, HasStack},
};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ExecInstruction {
    Noop, /* Maybe use `Noop(())` instead of `Noop(Noop)` when we get around to this. See
           * 2024-03-31 stream chat for more. */
    Dup(Dup),
    When(When),
    Unless(Unless),
    IfElse(IfElse),
}

impl ExecInstruction {
    #[must_use]
    pub const fn dup() -> Self {
        Self::Dup(Dup)
    }

    #[must_use]
    pub const fn when() -> Self {
        Self::When(When)
    }

    #[must_use]
    pub const fn unless() -> Self {
        Self::Unless(Unless)
    }

    #[must_use]
    pub const fn if_else() -> Self {
        Self::IfElse(IfElse)
    }
}
impl From<ExecInstruction> for PushInstruction {
    fn from(instr: ExecInstruction) -> Self {
        Self::Exec(instr)
    }
}

impl NumOpens for ExecInstruction {
    fn num_opens(&self) -> usize {
        match self {
            Self::Noop => 0,
            Self::Dup(dup) => dup.num_opens(),
            Self::When(when) => when.num_opens(),
            Self::Unless(unless) => unless.num_opens(),
            Self::IfElse(if_else) => if_else.num_opens(),
        }
    }
}

impl<S> Instruction<S> for ExecInstruction
where
    S: Clone + HasStack<PushProgram> + HasStack<bool>,
{
    type Error = PushInstructionError;

    fn perform(&self, state: S) -> InstructionResult<S, Self::Error> {
        match self {
            Self::Noop => Ok(state),
            Self::When(when) => when.perform(state),
            Self::Unless(unless) => unless.perform(state),
            Self::IfElse(if_else) => if_else.perform(state),
            Self::Dup(dup) => dup.perform(state),
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::ExecInstruction;
    use crate::{instruction::Instruction, push_vm::push_state::PushState};

    #[test]
    fn noop_is_correct() {
        let state = PushState::builder()
            .with_max_stack_size(0)
            .with_no_program()
            .build();
        let result_state = ExecInstruction::Noop.perform(state.clone()).unwrap();
        assert_eq!(result_state, state);
    }
}
