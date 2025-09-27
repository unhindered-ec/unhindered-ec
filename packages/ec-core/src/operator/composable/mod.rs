mod and;
mod map;
mod repeat_with;
mod then;

pub use and::*;
// derive macro
pub use ec_macros::Composable;
pub use map::*;
pub use repeat_with::*;
pub use then::*;
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

    fn apply_twice(self) -> RepeatWith<Self, 2>
    where
        Self: Sized,
    {
        RepeatWith::new(self)
    }

    fn apply_n_times<const N: usize>(self) -> RepeatWith<Self, N>
    where
        Self: Sized,
    {
        RepeatWith::new(self)
    }

    fn map<Op>(self, op: Op) -> Map<Op>
    where
        Self: Sized,
    {
        Map::new(op)
    }

    fn wrap<T>(self, context: T::Context) -> T
    where
        T: Wrappable<Self>,
        Self: Sized,
    {
        T::construct(self, context)
    }

    // fn and_select<S>(self, selector: S) -> Then<Self, Select<S>> {
    //     Then::new(self, Select::new(selector))
    // }
}

static_assertions::assert_obj_safe!(Composable);

pub trait Wrappable<T> {
    type Context;

    fn construct(wrapped: T, context: Self::Context) -> Self;
}
