mod unless;
mod when;

use self::{unless::Unless, when::When};
use super::{instruction_error::PushInstructionError, Instruction, NumOpens, PushInstruction};
use crate::{
    error::InstructionResult,
    push_vm::{program::PushProgram, HasStack},
};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ExecInstruction {
    Noop, /* Maybe use `Noop(())` instead of `Noop(Noop)` when we get around to this. See
           * 2024-03-31 chat for more. */
    Dup,
    IfElse,
    When(When),
    Unless(Unless),
}

impl ExecInstruction {
    #[must_use]
    pub const fn when() -> Self {
        Self::When(When)
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
            Self::Dup | Self::When(_) | Self::Unless(_) => 1,
            Self::IfElse => 2,
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
            Self::When(w) => w.perform(state),
            Self::Unless(u) => u.perform(state),
            Self::IfElse => match self {
                Self::IfElse => todo!(),
                _ => {
                    unreachable!("We failed to handle an Exec instruction: {self:?}")
                }
            },
            Self::Dup => todo!(), // Could overflow exec
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
            .with_max_stack_size(1000)
            .with_no_program()
            .build();
        let result_state = ExecInstruction::Noop.perform(state.clone()).unwrap();
        assert_eq!(result_state, state);
    }
}
