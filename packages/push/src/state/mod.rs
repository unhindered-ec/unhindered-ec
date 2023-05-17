use crate::instruction::Instruction;

pub mod push_state;

pub trait State: Sized {
    type Instruction: Instruction<Self>;

    fn perform(&mut self, instruction: &Self::Instruction) {
        instruction.perform(self);
    }

    fn run_to_completion(&mut self);
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
