use rand::seq::SliceRandom;
use rand::thread_rng;

// `compose!` and `compose_two` taken from
// https://functional.works-hub.com/learn/functional-programming-jargon-in-rust-1b555

// macro_rules! compose {
//     ( $last:expr ) => { $last };
//     ( $head:expr, $($tail:expr), +) => {
//         compose_two($head, compose!($($tail),+))
//     };
// }

// fn compose_two<A, B, C, F, G>(f: F, g: G) -> impl Fn(A) -> C
// where
//     F: Fn(A) -> B,
//     G: Fn(B) -> C,
// {
//     move |x| g(f(x))
// }

// `and` and `then` based on suggestions from esitsu@Twitch, also
// influenced by https://www.reddit.com/r/rust/comments/hswzps/readable_function_composition/

// TODO: Change name to `Composable`
trait Compose: Sized {
    fn then<Op>(self, op: Op) -> Then<Self, Op> {
        Then::new(self, op)
    }
}

trait Operator<Input>: Compose {
    type Output;

    fn apply(&self, input: Input) -> Self::Output;
}

struct Increment;
impl Operator<i32> for Increment {
    type Output = i32;

    fn apply(&self, x: i32) -> Self::Output {
        x + 1
    }
}
impl Compose for Increment {}

struct Multiply;
impl Operator<(i32, i32)> for Multiply {
    type Output = i32;

    fn apply(&self, (x, y): (i32, i32)) -> Self::Output {
        x * y
    }
}
impl Compose for Multiply {}

struct RandomItem;

impl Operator<&[i32]> for RandomItem {
    type Output = i32;

    fn apply(&self, vals: &[i32]) -> Self::Output {
        // TODO: Do something about calling `thread_rng()` here?
        *(vals.choose(&mut thread_rng()).unwrap())
    }
}
impl Compose for RandomItem {}

impl Operator<&mut [i32]> for RandomItem {
    type Output = i32;

    fn apply(&self, vals: &mut [i32]) -> Self::Output {
        // TODO: Do something about calling `thread_rng()` here?
        *(vals.choose(&mut thread_rng()).unwrap())
    }
}

struct IncrementAll;

impl<'a> Operator<&'a mut [i32]> for IncrementAll {
    type Output = &'a mut [i32];

    fn apply(&self, vals: &'a mut [i32]) -> Self::Output {
        for v in vals.iter_mut() {
            *v += 1;
        }
        vals
    }
}
impl Compose for IncrementAll {}

struct Then<F, G> {
    f: F,
    g: G,
}

impl<F, G> Then<F, G> {
    fn new(f: F, g: G) -> Self {
        Then { f, g }
    }
}

impl<A, F, G> Operator<A> for Then<F, G>
where
    F: Operator<A>,
    G: Operator<F::Output>,
{
    type Output = G::Output;

    fn apply(&self, x: A) -> Self::Output {
        self.g.apply(self.f.apply(x))
    }
}
impl<F, G> Compose for Then<F, G> {}

// fn increment(x: i32) -> i32 { x+1 }
// fn multiply((x, y): (i32, i32)) -> i32 { x * y }

// Sadly, this can't be a closure because of the lifetime
// capture problem.
fn rand_then_increment(vals: &[i32]) -> i32 {
    RandomItem.then(Increment).apply(vals)
}

fn increment_then_choose(vals: &mut [i32]) -> i32 {
    IncrementAll.then(RandomItem).apply(vals)
}

// fn and<A, B, C, D, F, G>(f: F, g: G) -> impl Fn((A, C)) -> (B, D)
// where
//     F: Fn(A) -> B,
//     G: Fn(C) -> D,
// {
//     move |(x, y)| (f(x), g(y))
// }

// fn cloning_and<A, B, C, F, G>(f: F, g: G) -> impl Fn(A) -> (B, C)
// where
//     A: Clone,
//     F: Fn(A) -> B,
//     G: Fn(A) -> C,
// {
//     move |x| (f(x.clone()), g(x))
// }

// fn vec_and<A, B, F, G>(f: F, g: G) -> impl Fn(A) -> Vec<B>
// where
//     A: Clone,
//     F: Fn(A) -> B,
//     G: Fn(A) -> B,
// {
//     move |x: A| vec![f(x.clone()), g(x)]
// }

// // TODO: Make a version of `and` that returns an iterator.

// fn then<A, B, C, G, F>(f: F, g: G) -> impl Fn(A) -> C
// where
//     F: Fn(A) -> B,
//     G: Fn(B) -> C,
// {
//     move |x| g(f(x))
// }

// fn then_ref<A, B, C, G, F>(f: F, g: G) -> impl Fn(&A) -> C
// where
//     A: ?Sized,
//     F: Fn(&A) -> B,
//     G: Fn(B) -> C,
// {
//     move |x: &A| g(f(x))
// }

// fn random_item(vals: &[i32]) -> i32 {
//     (vals.choose(&mut thread_rng()).unwrap()).clone()
// }

fn main() {
    let vals = vec![1, 3, 5, 7, 9];

    let result = rand_then_increment(&vals);
    println!("The result was {result}");

    let mut vals = vec![1, 3, 5, 7, 9];
    let result = increment_then_choose(&mut vals);
    println!("The next result was {result}");

    // // 3 --\
    // //      * (15) -- +1 (16)
    // // 5 --/
    // let combo_then = then(multiply, increment);
    // let result = combo_then((3, 5));
    // println!("The first result using then was {result}");

    // // 3 -- +1 (4) --\
    // //                * (24)
    // // 5 -- +1 (6) --/
    // let inc_both = and(increment, increment);
    // let combo2 = then(inc_both, multiply);
    // let result2 = combo2((3, 5));
    // println!("The second result using and and then was {result2}");

    // let inc_both = cloning_and(increment, increment);
    // let combo2 = then(inc_both, multiply);
    // let result2 = combo2(3);
    // println!("The second result using cloning_and and then was {result2}");

    // let inc_both = vec_and(increment, increment);
    // let vec_mult = | vals: Vec<i32> | vals[0] * vals[1];
    // let combo2 = then(inc_both, vec_mult);
    // let result2 = combo2(3);
    // println!("The second result using vec_and and then was {result2}");

    // let always = | n: i32 | (move |()| n);
    // let always_3 = always(3);
    // let always_5 = always(5);
    // println!("Should be (3, 5): ({}, {})", always_3(()), always_5(()));

    // // 3 -- +1 (4) --\
    // //                * (24)
    // // 5 -- +1 (6) --/
    // let combo3 = compose!(always_3, increment);
    // let combo4 = compose!(always_5, increment);
    // let combo5 = then(and(combo3, combo4), multiply);
    // // What do we need to do so we can write this as
    // //   combo3.and(combo4).then(multiply)
    // // or
    // //   pipeline! { always_3.then(increment).and(always_5.then(increment)).then(multiply) }
    // // esitsu's idea is to do this through lots of traits and such.

    // println!("Third result was {:?}", combo5(((), ())));

    // let combo3 = then_ref(random_item, increment);
    // // let combo4 = compose!(random_item, increment);
    // // let combo5 = then(and(combo3, combo4), multiply);

    // let vals = vec![1, 3, 5, 7, 9];
    // // let vals2 = vec![10, 11, 12];
    // let result = combo3(&vals);
    // println!("{:?}", result);

    // let result = combo3(&vals2);
    // println!("{:?}", result);
    // [1, 2, 3, 4] -- random_item (x?) -- +1 (x+1) --\
    //                                                 * ((x+1)*(y+1))
    // [10, 11, 12] -- random_item (y?) -- +1 (y+1) --/
    // let input = (&vals, &vals2);
    // let result = combo5(input);
    // println!("Fourth result was {:?}", result);
}
