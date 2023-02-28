use self::then::Then;

mod then;

// TODO: Rationalize the naming of module files. I think I want
//   to go back to using `mod.rs` inside sub-folders. At a minimum
//   I want to be consistent, which I am current not.
pub trait Composable {
    fn then<Op>(self, op: Op) -> Then<Self, Op>
    where
        Self: Sized,
    {
        Then::new(self, op)
    }

    // fn then_mutate<M>(self, mutator: M) -> Then<Self, Mutate<M>> {
    //     Then::new(self, Mutate::new(mutator))
    // }

    // fn and_select<S>(self, selector: S) -> Then<Self, Select<S>> {
    //     Then::new(self, Select::new(selector))
    // }
}
