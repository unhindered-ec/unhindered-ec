use ordered_float::OrderedFloat;
use proptest::{prop_assert_eq, proptest};
use push::{
    instruction::{FloatInstruction, Instruction},
    push_vm::{push_state::PushState, HasStack},
};

#[test]
fn add() {
    let x = OrderedFloat(409.37);
    let y = OrderedFloat(512.825);
    let mut state = PushState::builder()
        .with_max_stack_size(100)
        .with_program([])
        .unwrap()
        .build();
    state.stack_mut::<OrderedFloat<f64>>().push(y).unwrap();
    state.stack_mut::<OrderedFloat<f64>>().push(x).unwrap();
    let result = FloatInstruction::Add.perform(state).unwrap();
    assert_eq!(result.stack::<OrderedFloat<f64>>().size(), 1);
    assert_eq!(*result.stack::<OrderedFloat<f64>>().top().unwrap(), x + y);
}

proptest! {
    #![proptest_config(proptest::prelude::ProptestConfig::with_cases(1_000))]

    #[test]
    fn add_prop(x in proptest::num::f64::ANY, y in proptest::num::f64::ANY) {
        let expected_result = OrderedFloat(x + y);
        let mut state = PushState::builder()
            .with_max_stack_size(100)
            .with_program([])
            .unwrap()
            .build();
        state.stack_mut::<OrderedFloat<f64>>().push(OrderedFloat(y)).unwrap();
        state.stack_mut::<OrderedFloat<f64>>().push(OrderedFloat(x)).unwrap();
        let result = FloatInstruction::Add.perform(state).unwrap();
        let output = result.stack::<OrderedFloat<f64>>().top().unwrap();
        prop_assert_eq!(*output, expected_result);
    }

    #[test]
    fn subtract_prop(x in proptest::num::f64::ANY, y in proptest::num::f64::ANY) {
        let expected_result = OrderedFloat(x - y);
        let mut state = PushState::builder()
            .with_max_stack_size(100)
            .with_program([])
            .unwrap()
            .build();
        state.stack_mut::<OrderedFloat<f64>>().push(OrderedFloat(y)).unwrap();
        state.stack_mut::<OrderedFloat<f64>>().push(OrderedFloat(x)).unwrap();
        let result = FloatInstruction::Subtract.perform(state).unwrap();
        let output = result.stack::<OrderedFloat<f64>>().top().unwrap();
        prop_assert_eq!(*output, expected_result);
    }

    #[test]
    fn multiply_prop(x in proptest::num::f64::ANY, y in proptest::num::f64::ANY) {
        let expected_result = OrderedFloat(x * y);
        let mut state = PushState::builder()
            .with_max_stack_size(100)
            .with_program([])
            .unwrap()
            .build();
        state.stack_mut::<OrderedFloat<f64>>().push(OrderedFloat(y)).unwrap();
        state.stack_mut::<OrderedFloat<f64>>().push(OrderedFloat(x)).unwrap();
        let result = FloatInstruction::Multiply.perform(state).unwrap();
        let output = result.stack::<OrderedFloat<f64>>().top().unwrap();
        prop_assert_eq!(*output, expected_result);
    }

    #[test]
    fn protected_divide_prop(x in proptest::num::f64::ANY, y in proptest::num::f64::ANY) {
        let expected_result = if y == 0.0 { OrderedFloat(1.0) } else { OrderedFloat(x / y) };
        let mut state = PushState::builder()
            .with_max_stack_size(100)
            .with_program([])
            .unwrap()
            .build();
        state.stack_mut::<OrderedFloat<f64>>().push(OrderedFloat(y)).unwrap();
        state.stack_mut::<OrderedFloat<f64>>().push(OrderedFloat(x)).unwrap();
        let result = FloatInstruction::ProtectedDivide.perform(state).unwrap();
        let output = result.stack::<OrderedFloat<f64>>().top().unwrap();
        prop_assert_eq!(*output, expected_result);
    }
}
