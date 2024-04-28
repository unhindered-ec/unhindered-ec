use super::when::When;
use crate::{
    error::{Error, InstructionResult},
    instruction::{instruction_error::PushInstructionError, Instruction, NumOpens},
    push_vm::{
        program::PushProgram,
        stack::{StackDiscard, StackPush},
        HasStack,
    },
};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct IfElse;

impl NumOpens for IfElse {
    fn num_opens(&self) -> usize {
        2
    }
}

impl<S> Instruction<S> for IfElse
where
    S: Clone + HasStack<PushProgram> + HasStack<bool>,
{
    type Error = PushInstructionError;

    fn perform(&self, mut state: S) -> InstructionResult<S, Self::Error> {
        let condition = state.stack::<bool>().top();
        let top_block = state.stack::<PushProgram>().top();
        let top_2_blocks = state.stack::<PushProgram>().top2();

        let r#then = top_2_blocks.as_ref().map(|(a, _)| *a).or(top_block);
        let r#else = top_2_blocks.map(|(_, b)| b);

        match (condition, r#then, r#else) {
            // If there is a boolean that is false and two blocks, discard the boolean
            // and the first (then) block, leaving the second (else) block so that it may be
            // performed next.
            (Ok(false), Ok(_), Ok(_)) => Ok(state)
                .with_stack_discard::<bool>(1)
                .with_stack_discard::<PushProgram>(1),
            // If there is a boolean that is true and two blocks, discard the boolean
            // and the second (else) block, leaving the first (then) block so that it may be
            // performed next.
            (Ok(true), Ok(_), Ok(_)) => {
                if let Err(e) = state.stack_mut::<bool>().pop() {
                    // This should never happen since we just checked that the stack has a boolean.
                    return Err(Error::fatal(state, e));
                }
                let (r#then, _) = match state.stack_mut::<PushProgram>().pop2() {
                    Ok(blocks) => blocks,
                    // This case should never happen since we just checked that there are two blocks
                    // on the exec stack.
                    Err(e) => return Err(Error::fatal(state, e)),
                };
                Ok(state).with_stack_push(r#then)
            }
            // If there is a boolean and only one block, then `IfThen` is equivalent to `When`
            // so we'll just call that.
            (Ok(_), Ok(_), Err(_)) => When.perform(state),
            // If there is no boolean but at one (then) block, discard the then block but keep the
            // rest of the exec stack unchanged. If there is a second (else) block, this makes the
            // logic here consistent with the logic in `Unless`, where we perform that
            // block when there is no boolean.
            (Err(_), Ok(_), Ok(_) | Err(_)) => Ok(state).with_stack_discard::<PushProgram>(1),
            // If there are no blocks, then we want to return some sort of error. Currently we're
            // just returning the error for the "else" block.
            // TODO: This ignores the fact that we underflowed on the boolean stack. We should
            // probably be able to accumulate/merge errors, and then we could merge the error
            // from the boolean stack with the error from the exec stack.
            (Ok(_) | Err(_), Err(_), Err(e)) => Err(Error::recoverable(state, e)),
            // We know that there can't be an "else" block without there also being a "then" block,
            // i.e., this case can't happen, but the compiler doesn't know that so we have to handle
            // it explicitly.
            (_, Err(_), Ok(_)) => {
                unreachable!("There can't be an `else` block without a `then` block")
            }
        }
    }
}

// TODO: Add a `proptest` module and tests

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use crate::{
        instruction::{exec::ifelse::IfElse, ExecInstruction, Instruction},
        push_vm::push_state::PushState,
    };

    #[test]
    fn if_else_is_correct_with_all_empty_stacks() {
        let state = PushState::builder()
            .with_max_stack_size(1000)
            .with_no_program()
            .with_bool_values([])
            .unwrap()
            .build();

        let result_state = IfElse.perform(state.clone()).unwrap();
        assert_eq!(result_state, state);
    }

    // #[test]
    // fn if_else_is_correct() {
    //     let state = PushState::builder()
    //         .with_stack(vec![true, PushProgram::new(vec![]),
    // PushProgram::new(vec![])])         .build();

    //     let state = IfElse.perform(state).unwrap();

    //     assert_eq!(state.stack::<bool>().top(), Ok(true));
    //     assert_eq!(state.stack::<PushProgram>().top().unwrap().len(), 0);
    // }

    // #[test]
    // fn if_else_is_correct_with_false_condition() {
    //     let state = PushState::builder()
    //         .with_stack(vec![false, PushProgram::new(vec![]),
    // PushProgram::new(vec![])])         .build();

    //     let state = IfElse.perform(state).unwrap();

    //     assert_eq!(state.stack::<bool>().top(), Ok(false));
    //     assert_eq!(state.stack::<PushProgram>().top(),
    // Ok(PushProgram::new(vec![]))); }

    // #[test]
    // fn if_else_is_correct_with_no_condition() {
    //     let state = PushState::builder()
    //         .with_stack(vec![PushProgram::new(vec![]),
    // PushProgram::new(vec![])])         .build();

    //     let state = IfElse.perform(state).unwrap();

    //     assert_eq!(state.stack::<bool>().top(), Ok(true));
    //     assert_eq!(state.stack::<PushProgram>().top().unwrap().len(), 0);
    // }

    // #[test]
    // fn if_else_is_correct_with_no_condition_and_no_else() {
    //     let state =
    // PushState::builder().with_stack(vec![PushProgram::new(vec![])]).build();

    //     let state = IfElse.perform(state).unwrap();

    //     assert_eq!(state.stack::<bool>().top(), Ok(true));
    //     assert_eq!(state.stack::<PushProgram>().top().unwrap().len(), 0);
    // }

    // #[test]
    // fn if_else_is_correct_with_no_condition_and_no_then() {
    //     let state =
    // PushState::builder().with_stack(vec![PushProgram::new(vec![])]).build();

    //     let state = IfElse.perform(state).unwrap();

    //     assert_eq!(state.stack::<bool>().top(), Ok(true));
    //     assert_eq!(state.stack::<PushProgram>().top().unwrap().len(), 0);
    // }
}
