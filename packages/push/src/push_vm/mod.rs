use crate::{
    error::{InstructionResult, stateful::FatalError},
    instruction::Perform,
};

pub mod program;
pub mod push_io;
pub mod push_state;
pub mod stack;
pub mod variables;

pub use self::stack::HasStack;

pub trait State: Sized {
    type Instruction: Perform<Self>;

    /// # Errors
    ///
    /// Fails if the instruction being performed fails.
    fn perform(
        self,
        instruction: &Self::Instruction,
    ) -> InstructionResult<Self, <Self::Instruction as Perform<Self>>::Error> {
        instruction.perform(self)
    }

    /// # Errors
    ///
    /// Fails if any of the performed instructions fails.
    fn run_to_completion(
        self,
    ) -> Result<Self, FatalError<Self, <Self::Instruction as Perform<Self>>::Error>>;
}

/*
 * exec: 5 8 9 int_plus 6 int_is_even bool_or
 * int: <empty>
 * bool: <empty>
 *
 * 5 : Push 5 on the integer stack
 * 8
 * 9
 * int_add : Pop 8 and 9, add them, and push 17 on the integer stack
 * 6
 * int_is_even: Pop 6 and push true on the boolean stack
 * bool_or: Be ignored because there's only one value on the boolean stack
 *
 * exec: <empty> (after performing all the instructions)
 * int: 5 17
 * bool: true
 */
