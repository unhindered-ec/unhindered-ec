use self::then::Then;

mod then;

// esitsu@Twitch used this blanket implementation so every type in the universe
// implements `Composable`. I'm going to comment it out for now and see where we
// actually _need_ to implement `Composable`. We'll presumably come back to the
// blanket implementation down the road, but I was to see why we need it.
// impl<T> Composable for T {}

// TODO: Do some homework on the role of `Sized` here. It used to be
//  on the trait `Composable`, but moving into the function fixed
//  some things in `Weighted`. This is related to the whole object
//  safety thing that I don't understand terribly well.
pub trait Composable {
    fn then<Op>(self, op: Op) -> Then<Self, Op> 
    where
        Self: Sized
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
