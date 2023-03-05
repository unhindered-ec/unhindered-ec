// TODO: I had to make this `pub use` to get the imports in `TwoPointXoMutate` to work
//   and I'm really not sure why. I should do homework on this.
use self::{and::And, map::Map, then::Then};

mod and;
pub(crate) mod map;
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

    fn then_map<Op>(self, op: Op) -> Then<Self, Map<Op>>
    where
        Self: Sized,
    {
        Then::new(self, Map::new(op))
    }

    fn and<Op>(self, op: Op) -> And<Self, Op>
    where
        Self: Sized,
    {
        And::new(self, op)
    }

    fn apply_twice<Op>(self) -> And<Self, Self>
    where
        Self: Sized + Clone
    {
        And::new(self.clone(), self)
    }

    fn map<Op>(self, op: Op) -> Map<Op>
    where
        Self: Sized,
    {
        Map::new(op)
    }

    // fn and_select<S>(self, selector: S) -> Then<Self, Select<S>> {
    //     Then::new(self, Select::new(selector))
    // }
}
