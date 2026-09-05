mod error;

use std::fmt::Display;

pub use error::WithInputInstructionError;

use crate::{
    error::{Error, InstructionResult, MapInstructionError},
    instruction::{Instruction, PushInstruction},
    push_vm::variables::{HasInputs, UnknownVariableError, VariableName},
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
    type Error = WithInputInstructionError<<S::InputInstruction as Instruction<S>>::Error>;

    /// # Errors
    ///
    /// - Returns an error if performing the input instruction associated with
    ///   this variable returns an error.
    /// - Returns a fatal [`UnknownVariableError`] if there is an attempt to
    ///   lookup the value of a variable that isn't registered with the state.
    fn perform(&self, state: S) -> InstructionResult<S, Self::Error> {
        let Some(instruction) = state.get_input_instruction(&self.0) else {
            return Err(Error::fatal(state, UnknownVariableError(self.0.clone())));
        };

        instruction
            .perform(state)
            .map_inner_err(WithInputInstructionError::Instruction)
    }
}

#[cfg(test)]
mod tests {
    use std::assert_matches;

    use crate::{
        error::Error,
        instruction::{
            Instruction,
            with_input::{WithInputInstruction, WithInputInstructionError},
        },
        push_vm::{
            push_state::PushState,
            variables::{UnknownVariableError, VariableName},
        },
    };

    #[test]
    fn variable_exists() {
        let s: PushState = PushState::builder()
            .with_max_stack_size(1)
            .with_no_program()
            .with_instruction_step_limit(1)
            .with_int_input("x", 7)
            .build();

        let i = WithInputInstruction::from("x");
        // This will fail if the instruct fails (which it shouldn't).
        let new_state = i.perform(s).unwrap();
        assert_matches!(new_state.int.top(), Ok(&7));
        assert_eq!(1, new_state.int.size());
    }

    #[test]
    fn unknown_variable() {
        let s: PushState = PushState::builder()
            .with_max_stack_size(1)
            .with_no_program()
            .with_instruction_step_limit(1)
            .build();

        let i = WithInputInstruction::from("x");
        // This should fail, returning a fatal `UnknownVariableError`.
        let error = i.perform(s.clone()).unwrap_err();
        assert_eq!(
            error,
            Error::fatal(
                s,
                WithInputInstructionError::UnknownVariable(UnknownVariableError(
                    VariableName::from("x")
                ))
            )
        );
    }
}
