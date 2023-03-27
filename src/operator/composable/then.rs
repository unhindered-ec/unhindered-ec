use rand::rngs::ThreadRng;

use super::super::Operator;

use super::Composable;

pub struct Then<F, G> {
    f: F,
    g: G,
}

impl<F, G> Then<F, G> {
    pub const fn new(f: F, g: G) -> Self {
        Self { f, g }
    }
}

impl<A, F, G> Operator<A> for Then<F, G>
where
    F: Operator<A>,
    G: Operator<F::Output>,
{
    type Output = G::Output;

    fn apply(&self, x: A, rng: &mut ThreadRng) -> Self::Output {
        self.g.apply(self.f.apply(x, rng), rng)
    }
}
impl<F, G> Composable for Then<F, G> {}

#[cfg(test)]
pub mod tests {
    use rand::thread_rng;

    use super::*;

    struct Increment;
    impl Operator<i32> for Increment {
        type Output = i32;

        fn apply(&self, input: i32, _: &mut ThreadRng) -> Self::Output {
            input + 1
        }
    }
    impl Composable for Increment {}

    struct Double;
    impl Operator<i32> for Double {
        type Output = i32;

        fn apply(&self, input: i32, _: &mut ThreadRng) -> Self::Output {
            input * 2
        }
    }
    impl Composable for Double {}

    #[test]
    fn increment_then_double() {
        let combo = Increment.then(Double);
        let result = combo.apply(7, &mut thread_rng());
        assert_eq!(16, result);
    }
}

// // Essentially our function composition; takes two operators and
// // it's `apply` method performs the first and sends that output as
// // the input to the second.
// // @esitsu: So Then<A, B> is an Operator where A is Operator and B is Operator<A::Output>
// pub struct Then<A, B> {
//     first: A,
//     second: B,
// }

// impl<A, B> Then<A, B> {
//     fn new(first: A, second: B) -> Self {
//         Then { first, second }
//     }
// }

// impl<A, B, Input, P> Operator<Input, P> for Then<A, B>
// where
//     A: Operator<Input, P>,
//     B: Operator<A::Output, P>,
// {
//     type Output = B::Output;

//     fn apply(&self, input: Input, p: &P, rng: &mut ThreadRng) -> Self::Output {
//         self.second.apply(self.first.apply(input, p, rng), p, rng)
//     }
// }
