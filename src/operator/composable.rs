use self::then::Then;

mod then;

// esitsu@Twitch used this blanket implementation so every type in the universe
// implements `Composable`. I'm going to comment it out for now and see where we
// actually _need_ to implement `Composable`. We'll presumably come back to the
// blanket implementation down the road, but I was to see why we need it.
// impl<T> Composable for T {}

pub trait Composable: Sized {
    fn then<Op>(self, op: Op) -> Then<Self, Op> {
        Then::new(self, op)
    }

    // fn then_mutate<M>(self, mutator: M) -> Then<Self, Mutate<M>> {
    //     Then::new(self, Mutate::new(mutator))
    // }

    // fn and_select<S>(self, selector: S) -> Then<Self, Select<S>> {
    //     Then::new(self, Select::new(selector))
    // }
}
