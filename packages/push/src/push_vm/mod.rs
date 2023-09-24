use crate::{
    error::{stateful::FatalError, InstructionResult},
    instruction::Instruction,
};

pub mod stack;
pub mod state;

// We'll use a 64-bit integer for our integer types.
pub type PushInteger = i64;

// Need an associated error trait
pub trait Exec<'a>: Sized + 'a {
    type Instruction: Instruction<&'a mut Self>;

    /// # Errors
    ///
    /// Fails if the instruction being performed fails.
    fn perform<'b>(
        &'a mut self,
        instruction: &'b Self::Instruction,
    ) -> InstructionResult<<Self::Instruction as Instruction<&'a mut Self>>::Error> {
        instruction.perform(self)
    }

    /// # Errors
    ///
    /// Fails if any of the performed instructions fails.
    fn run_to_completion(
        &mut self,
    ) -> Result<(), FatalError<<Self::Instruction as Instruction<&'a mut Self>>::Error>>;
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
