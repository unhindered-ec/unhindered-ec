use super::{push_state::PushState, stack::StackError, HasStack};
use crate::{
    error::{Error, InstructionResult},
    genome::plushy::{Plushy, PushGene},
    instruction::{
        instruction_error::PushInstructionError, Instruction, NumOpens, PushInstruction,
    },
};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PushProgram {
    Instruction(PushInstruction),
    Block(Vec<PushProgram>),
}

impl From<Plushy> for Vec<PushProgram> {
    fn from(plushy: Plushy) -> Self {
        let mut genes = plushy.into_iter();
        let mut program = Self::new();
        PushProgram::parse_from_plushy(true, &mut genes, &mut program);
        program
    }
}

impl<T> From<T> for PushProgram
where
    T: Into<PushInstruction>,
{
    fn from(instruction: T) -> Self {
        Self::Instruction(instruction.into())
    }
}

impl PushProgram {
    // Take a vector of genes, parse out the next complete Push program and
    // return that program and the remaining slice of genes.
    fn parse_from_plushy(
        is_top_level: bool,
        genes: &mut impl Iterator<Item = PushGene>,
        program: &mut Vec<Self>,
    ) {
        while let Some(gene) = genes.next() {
            match gene {
                PushGene::Close => {
                    if !is_top_level {
                        // This closes a block, so return up to the caller.
                        return;
                    } // Otherwise ignore the `Close` and continue on to the next instruction
                }
                PushGene::Instruction(i) => {
                    let num_opens = i.num_opens();
                    program.push(Self::Instruction(i));
                    for _ in 0..num_opens {
                        let mut block = Vec::new();
                        Self::parse_from_plushy(false, genes, &mut block);
                        program.push(Self::Block(block));
                    }
                }
            }
        }
    }
}

// This is for "performing" an instruction that is in
// fact a block of instructions. To perform this instruction
// we need to push all the instructions in the block onto
// the stack in the correct order, i.e., the first instruction
// in the block should be the top instruction on the exec
// stack after all the pushing is done.
impl<S, I> Instruction<S> for Vec<I>
where
    S: HasStack<I>,
    I: Instruction<S> + Clone,
    I::Error: From<StackError>,
{
    type Error = I::Error;

    fn perform(&self, mut state: S) -> InstructionResult<S, Self::Error> {
        // If the size of the block + the size of the exec stack exceed the max stack
        // size then we generate a fatal error.
        if let Err(err) = state.stack_mut::<I>().try_extend(self.iter().cloned()) {
            return Err(Error::fatal(state, err));
        }
        Ok(state)
    }
}

impl Instruction<PushState> for PushProgram {
    type Error = PushInstructionError;

    fn perform(&self, state: PushState) -> InstructionResult<PushState, Self::Error> {
        match self {
            Self::Instruction(i) => i.perform(state),
            Self::Block(block) => block.perform(state),
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod test {
    use super::PushProgram;
    use crate::{
        genome::plushy::{Plushy, PushGene},
        instruction::{
            BoolInstruction, ExecInstruction, FloatInstruction, Instruction, IntInstruction,
        },
        list_into::{arr_into, vec_into},
        push_vm::{push_state::PushState, HasStack},
    };

    #[test]
    fn conversion() {
        let genes = arr_into![
            IntInstruction::Add,
            ExecInstruction::if_else(),
            IntInstruction::Multiply,
            PushGene::Close,
            ExecInstruction::dup_block(),
            IntInstruction::Subtract,
        ];
        let plushy: Plushy = genes.into_iter().collect();
        let program: Vec<PushProgram> = plushy.into();
        // [Instruction(Int-Add), Instruction(Exec-IfElse),
        // Block([Instruction(Int-Multiply)]), Block([Instruction(Exec-Dup),
        // Block([Instruction(Int-Subtract)])])]
        assert_eq!(
            program,
            vec_into![
                IntInstruction::Add,
                ExecInstruction::if_else(),
                PushProgram::Block(vec_into![IntInstruction::Multiply]),
                PushProgram::Block(vec_into![
                    ExecInstruction::dup_block(),
                    PushProgram::Block(vec_into![IntInstruction::Subtract])
                ])
            ]
        );
    }

    #[test]
    fn block() {
        let instructions = vec_into![
            IntInstruction::Add,
            FloatInstruction::Multiply,
            BoolInstruction::And,
        ];
        let block = PushProgram::Block(instructions);
        let state = PushState::builder()
            .with_max_stack_size(3)
            .with_no_program()
            .build();
        let mut result = block.perform(state).unwrap();
        let exec_stack = result.stack_mut::<PushProgram>();
        assert_eq!(exec_stack.size(), 3);
        assert_eq!(exec_stack.pop().unwrap(), IntInstruction::Add.into());
        assert_eq!(exec_stack.pop().unwrap(), FloatInstruction::Multiply.into());
        assert_eq!(exec_stack.pop().unwrap(), BoolInstruction::And.into());
        assert_eq!(exec_stack.size(), 0);
    }

    #[test]
    fn block_overflows() {
        let instructions = vec_into![
            IntInstruction::Add,
            FloatInstruction::Multiply,
            BoolInstruction::And,
        ];
        let block = PushProgram::Block(instructions);
        let state = PushState::builder()
            // Set the max stack size to 2, so when we execute the block it overflows
            .with_max_stack_size(0)
            .with_no_program()
            .build();

        assert!(
            block.perform(state).is_err(),
            "Performing the block didn't generate an overflow error"
        );
    }
}
