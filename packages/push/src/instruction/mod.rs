/*
 * exec_if requires a boolean and two (additional) values on the exec stack.
 * If the bool is true, we remove the second of the two exec stack values,
 * and if it's false, we remove the first.
 */

/*
 * exec_while requires a boolean and one additional value on the exec stack.
 * If the bool is true, then you push a copy of the "body" onto the exec, followed
 * by another copy of exec_while.
 */

/*
 * Instructions that are generic over stacks:
 *
 * - push
 * - pop
 * - dup (int_dup, exec_dup, bool_dup, ...)
 */

pub trait Instruction<S> {
    fn perform(&self, state: &mut S);
}

impl<S> Instruction<S> for Box<dyn Instruction<S>> {
    fn perform(&self, state: &mut S) {
        self.as_ref().perform(state);
    }
}

// impl<F> Instruction for F
// where
//     F: Fn(dyn State) -> dyn State
// {}
