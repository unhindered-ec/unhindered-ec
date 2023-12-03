use push::{
    instruction::{BoolInstruction, FloatInstruction, IntInstruction, PushInstruction},
    push_vm::{push_state::PushState, HasStack, State},
};

#[test]
fn block() {
    let instructions = vec![
        IntInstruction::Add.into(),
        FloatInstruction::Multiply.into(),
        BoolInstruction::And.into(),
    ];
    let block = PushInstruction::Block(instructions);
    let state = PushState::builder()
        .with_max_stack_size(100)
        .with_program([])
        .unwrap()
        .build();
    let mut result = state.perform(&block).unwrap();
    let exec_stack = result.stack_mut::<PushInstruction>();
    assert_eq!(exec_stack.size(), 3);
    assert_eq!(exec_stack.pop().unwrap(), IntInstruction::Add.into());
    assert_eq!(exec_stack.pop().unwrap(), FloatInstruction::Multiply.into());
    assert_eq!(exec_stack.pop().unwrap(), BoolInstruction::And.into());
    assert_eq!(exec_stack.size(), 0);
}
