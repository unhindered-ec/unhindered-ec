use anyhow::Context;
use rand::rngs::ThreadRng;

use super::{super::Operator, Composable};

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
    anyhow::Error: From<F::Error> + From<G::Error>,
{
    type Output = G::Output;
    type Error = anyhow::Error;

    fn apply(&self, x: A, rng: &mut ThreadRng) -> Result<Self::Output, Self::Error> {
        let f_result = self
            .f
            .apply(x, rng)
            .map_err(anyhow::Error::from)
            .context("f in `Then` failed")?;
        self.g.apply(f_result, rng).map_err(anyhow::Error::from)
    }
}
impl<F, G> Composable for Then<F, G> {}

#[cfg(test)]
#[rustversion::attr(before(1.81), allow(clippy::arithmetic_side_effects))]
#[rustversion::attr(
    since(1.81),
    expect(
        clippy::arithmetic_side_effects,
        reason = "The tradeoff safety <> ease of writing arguably lies on the ease of writing \
                  side for test code."
    )
)]
#[rustversion::attr(before(1.81), allow(clippy::unwrap_used))]
#[rustversion::attr(
    since(1.81),
    expect(
        clippy::unwrap_used,
        reason = "Panicking is the best way to deal with errors in unit tests"
    )
)]
pub mod tests {
    use std::convert::Infallible;

    use rand::thread_rng;

    use super::*;

    struct Increment;
    impl Operator<i32> for Increment {
        type Output = i32;
        type Error = Infallible;

        fn apply(&self, input: i32, _: &mut ThreadRng) -> Result<Self::Output, Self::Error> {
            Ok(input + 1)
        }
    }
    impl Composable for Increment {}

    struct Double;
    impl Operator<i32> for Double {
        type Output = i32;
        type Error = Infallible;

        fn apply(&self, input: i32, _: &mut ThreadRng) -> Result<Self::Output, Self::Error> {
            Ok(input * 2)
        }
    }
    impl Composable for Double {}

    #[test]
    fn increment_then_double() {
        let combo = Increment.then(Double);
        let result = combo.apply(7, &mut thread_rng()).unwrap();
        assert_eq!(16, result);
    }
}
