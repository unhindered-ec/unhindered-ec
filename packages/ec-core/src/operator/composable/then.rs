use anyhow::{Context, Result};
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
{
    type Output = G::Output;

    fn apply(&self, x: A, rng: &mut ThreadRng) -> Result<Self::Output> {
        let f_result = self.f.apply(x, rng).context("f in `Then` failed")?;
        self.g.apply(f_result, rng)
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

        fn apply(&self, input: i32, _: &mut ThreadRng) -> Result<Self::Output> {
            Ok(input + 1)
        }
    }
    impl Composable for Increment {}

    struct Double;
    impl Operator<i32> for Double {
        type Output = i32;

        fn apply(&self, input: i32, _: &mut ThreadRng) -> Result<Self::Output> {
            Ok(input * 2)
        }
    }
    impl Composable for Double {}

    #[test]
    #[allow(clippy::unwrap_used)]
    fn increment_then_double() {
        let combo = Increment.then(Double);
        let result = combo.apply(7, &mut thread_rng()).unwrap();
        assert_eq!(16, result);
    }
}
