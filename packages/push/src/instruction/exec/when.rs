use crate::{
    error::{Error, InstructionResult},
    instruction::{instruction_error::PushInstructionError, Instruction, NumOpens},
    push_vm::{
        program::PushProgram,
        stack::{StackDiscard, StackError},
        HasStack,
    },
};

/// A basic `When`/`If` conditional (without `Else`)
///
/// # Inputs
///
/// The `When` instruction takes the following inputs:
///    - `Bool` stack
///      - Zero or one booleans
///    - `Exec` stack
///      - Zero or one code blocks
///
/// # Behavior
///
/// The `When` instruction performs a block of code on the
/// `Exec` stack _when_ the boolean on the top of the `bool` stack is
/// true, ignoring that block otherwise.
///
/// An important feature of `When` is the code block is only executed when:
///    - There is a boolean condition, and
///    - That condition is `true`
///
/// ## Action Table
///
/// The table below indicates the behavior in each of the different
/// cases.
///
///    - The "Boolean stack" column indicates the value of the top of the
///      boolean stack, or whether it exists
///    - The "Code block" column indicates whether there is a code block on the
///      `Exec` stack.
///    - The "Action" columns indicate the action taken on the respective
/// stacks.
///       - "Consumed" means the value on that stack is consumed (i.e., removed)
///       - "Unchanged" means that value was left on the stack unchanged.
///    - The "Success" column indicates whether the instruction succeeds, and if
///      not what kind of error is returned:
///       - ✅: success
///       - ❗: recoverable error, with links to the error kind
///       - ‼️: fatal error, with links to the error kind
///    - The "Note" column briefly summarizes the action state in that case
///
/// | Boolean stack  | Code block | Action (`Bool`) | Action (Code block) | Success| Note |
/// | ------------- | ------------- | ------------- | ------------- | ------------- | ------------- |
/// | `true`  | exists | Consumed | Unchanged | ✅ | The code block is executed |
/// | `false` | exists | Consumed | Consumed | ✅ | The code block is skipped |
/// | missing | exists | Non-existent | Consumed | ✅ | The code block is skipped |
/// | exists | missing | Unchanged | Non-existent | ✅ | State is unchanged |
/// | missing | missing | Non-existent | Non-existent | [❗…](StackError::Underflow) | State is unchanged |
///
/// # Errors
///
/// If either of the stack accesses returns any error other than a
/// [`StackError::Underflow`] then this returns that as a [`Error::Fatal`]
/// error.
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub struct When;

impl NumOpens for When {
    fn num_opens(&self) -> usize {
        1
    }
}

impl<S> Instruction<S> for When
where
    S: Clone + HasStack<PushProgram> + HasStack<bool>,
{
    type Error = PushInstructionError;

    fn perform(&self, state: S) -> InstructionResult<S, Self::Error> {
        let condition = state.stack::<bool>().top();
        let block = state.stack::<PushProgram>().top();
        match (condition, block) {
            // If there is a boolean that is true and a block, discard the boolean
            // and leave the block so that it may be
            // performed next.
            (Ok(true), Ok(_)) => Ok(state).with_stack_discard::<bool>(1),
            // If there is a boolean that is false and a block, discard both the
            // boolean and the block since we don't
            // want to perform that block.
            (Ok(false), Ok(_)) => Ok(state)
                .with_stack_discard::<bool>(1)
                .with_stack_discard::<PushProgram>(1),
            // If there is no boolean but there is a block, discard the block since
            // we only want to perform it if there is a
            // boolean that is true.
            (Err(StackError::Underflow { .. }), Ok(_)) => {
                Ok(state).with_stack_discard::<PushProgram>(1)
            }
            // If there is a boolean but no block, then we just skip this instruction.
            (Ok(_), Err(StackError::Underflow { .. })) => Ok(state),
            // If there is no boolean and no block, then we return an stack underflow
            // error.
            (Err(StackError::Underflow { .. }), Err(e @ StackError::Underflow { .. })) => {
                Err(Error::recoverable(state, e))
            }
            // If some other error occurs (e.g., a fatal error), then we just
            // pass that forward.
            (Err(e), _) | (_, Err(e)) => Err(Error::fatal(state, e)),
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::When;
    use crate::{
        error::into_state::IntoState,
        instruction::{ExecInstruction, Instruction, PushInstructionError},
        push_vm::{push_state::PushState, stack::StackError},
    };

    #[test]
    fn cond_true() {
        let state = PushState::builder()
            .with_max_stack_size(1)
            .with_program([ExecInstruction::noop()])
            .unwrap()
            .with_bool_values([true])
            .unwrap()
            .build();
        let result_state = When.perform(state.clone()).unwrap();
        assert!(result_state.bool.is_empty());
        assert_eq!(result_state.exec, state.exec);
    }

    #[test]
    fn cond_false() {
        let state = PushState::builder()
            .with_max_stack_size(1)
            .with_program([ExecInstruction::noop()])
            .unwrap()
            .with_bool_values([false])
            .unwrap()
            .build();
        let result_state = When.perform(state).unwrap();
        assert!(result_state.bool.is_empty());
        assert!(result_state.exec.is_empty());
    }

    #[test]
    fn cond_true_exec_empty() {
        let state = PushState::builder()
            .with_max_stack_size(1)
            .with_no_program()
            .with_bool_values([true])
            .unwrap()
            .build();
        let result_state = When.perform(state.clone()).unwrap();
        assert_eq!(result_state, state);
    }

    #[test]
    fn cond_false_exec_empty() {
        let state = PushState::builder()
            .with_max_stack_size(1)
            .with_no_program()
            .with_bool_values([false])
            .unwrap()
            .build();
        let result_state = When.perform(state.clone()).unwrap();
        assert_eq!(result_state, state);
    }

    #[test]
    fn cond_missing() {
        let state = PushState::builder()
            .with_max_stack_size(1)
            .with_program([ExecInstruction::noop()])
            .unwrap()
            .build();
        let result_state = When.perform(state).unwrap();
        assert!(result_state.exec.is_empty());
    }

    #[test]
    fn stacks_empty() {
        let state = PushState::builder()
            .with_max_stack_size(0)
            .with_no_program()
            .with_bool_values([])
            .unwrap()
            .build();
        let result_error = When.perform(state.clone()).unwrap_err();
        assert!(result_error.is_recoverable());
        assert!(matches!(
            result_error.error(),
            PushInstructionError::StackError(StackError::Underflow { .. })
        ));
        assert_eq!(result_error.into_state(), state);
    }
}
