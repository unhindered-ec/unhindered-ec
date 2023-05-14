use crate::instruction::Instruction;

mod push_state;

pub trait State: Sized {
    type Instruction: Instruction<Self>;

    fn perform(&mut self, instruction: &Self::Instruction) {
        instruction.perform(self);
    }

    fn run<'a, P>(&mut self, program: P) 
    where 
        P: IntoIterator<Item = &'a Self::Instruction>,
        Self::Instruction: 'a,
    {
        for instruction in program {
            self.perform(instruction);
        }
    }
}

#[cfg(test)]
mod simple_check {
    use crate::state::push_state::{PushInstruction, PushState};

    use super::*;

    #[test]
    fn run_simple_program() {
        use PushInstruction::*;

        let program = vec![
            Int(5),
            Int(8),
            Int(9),
            IntAdd,
            Int(6),
            IntIsEven,
            BoolOr
        ];
        let mut state = PushState::default();
        state.run(&program);
        assert!(state.exec().is_empty());
        assert_eq!(state.int(), &vec![5, 17]);
        assert_eq!(state.bool(), &vec![true]);
    }
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
