use push::{
    error::Error,
    instruction::{
        BoolInstruction, FloatInstruction, Instruction, IntInstruction, PushInstruction,
    },
    push_vm::{push_state::PushState, HasStack},
};

#[test]
fn block() {
    let instructions = vec![
        IntInstruction::Add.into(),
        FloatInstruction::Multiply.into(),
        BoolInstruction::And.into(),
    ];
    let block = dbg!(PushInstruction::Block(instructions));
    let state = PushState::builder()
        .with_max_stack_size(100)
        .with_program([])
        .unwrap()
        .build();
    let mut result = block.perform(state).unwrap();
    let exec_stack = result.stack_mut::<PushInstruction>();
    assert_eq!(exec_stack.size(), 3);
    assert_eq!(exec_stack.pop().unwrap(), IntInstruction::Add.into());
    assert_eq!(exec_stack.pop().unwrap(), FloatInstruction::Multiply.into());
    assert_eq!(exec_stack.pop().unwrap(), BoolInstruction::And.into());
    assert_eq!(exec_stack.size(), 0);
}

#[test]
fn block_overflows() {
    let instructions = vec![
        IntInstruction::Add.into(),
        FloatInstruction::Multiply.into(),
        BoolInstruction::And.into(),
    ];
    let block = dbg!(PushInstruction::Block(instructions));
    let state = PushState::builder()
        // Set the max stack size to 2, so when we execute the block it overflows
        .with_max_stack_size(2)
        .with_program([])
        .unwrap()
        .build();
    let Error::Fatal(_) = block.perform(state).unwrap_err() else {
        panic!("Performing the block didn't generate an overflow error");
    };
}
