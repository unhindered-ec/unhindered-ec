use std::fmt::Display;

use crate::{
    error::InstructionResult,
    instruction::{Instruction, PushInstruction},
    push_vm::variables::{HasInputs, VariableName},
};

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct WithInputInstruction(pub VariableName);

impl Display for WithInputInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl From<VariableName> for WithInputInstruction {
    fn from(var_name: VariableName) -> Self {
        Self(var_name)
    }
}

impl From<&str> for WithInputInstruction {
    fn from(var_name: &str) -> Self {
        Self::from(VariableName::from(var_name))
    }
}

impl From<WithInputInstruction> for PushInstruction {
    fn from(value: WithInputInstruction) -> Self {
        Self::WithInput(value)
    }
}

impl<S> Instruction<S> for WithInputInstruction
where
    S: HasInputs,
{
    type Error = <S::InputInstruction as Instruction<S>>::Error;

    /// # Errors
    ///
    /// This returns an error if performing the input instruction associated
    /// with this variable returns an error.
    ///
    /// # Panics
    ///
    /// This panics if there is no instruction associated with `var_name`, i.e.,
    /// we have not yet added that variable name to the map of names to
    /// instructions.
    fn perform(&self, state: S) -> InstructionResult<S, Self::Error> {
        #[expect(
            clippy::panic,
            reason = "This is legacy and arguably should be changed. Tracked in #172"
        )]
        let instruction = state.get_input_instruction(&self.0).unwrap_or_else(|| {
            panic!(
                "Failed to get an instruction for the input variable '{var_name}' that hadn't \
                 been defined",
                var_name = self.0
            )
        });
        instruction.perform(state)
    }
}
