use miette::Diagnostic;
use rand::Rng;

use super::{super::Operator, Composable};

#[derive(Composable)]
pub struct Then<F, G> {
    f: F,
    g: G,
}

impl<F, G> Then<F, G> {
    pub const fn new(f: F, g: G) -> Self {
        Self { f, g }
    }
}

#[derive(Debug, thiserror::Error, Diagnostic)]
pub enum ThenError<T, U> {
    #[error("Error while applying the first passed operator (`T`) in the `Then<T,>` Operator")]
    First(
        #[diagnostic_source]
        #[source]
        T,
    ),
    #[error("Error while applying the second passed operator (`U`) in the `Then<,U>` Operator")]
    Second(
        #[diagnostic_source]
        #[source]
        U,
    ),
}

impl<A, F, G> Operator<A> for Then<F, G>
where
    F: Operator<A>,
    G: Operator<F::Output>,
{
    type Output = G::Output;
    type Error = ThenError<F::Error, G::Error>;

    fn apply<R: Rng + ?Sized>(&self, x: A, rng: &mut R) -> Result<Self::Output, Self::Error> {
        let f_result = self.f.apply(x, rng).map_err(ThenError::First)?;
        self.g.apply(f_result, rng).map_err(ThenError::Second)
    }
}

#[cfg(test)]
#[expect(
    clippy::arithmetic_side_effects,
    reason = "The tradeoff safety <> ease of writing arguably lies on the ease of writing side \
              for test code."
)]
pub mod tests {
    use std::convert::Infallible;

    use rand::rng;

    use super::*;

    #[derive(Composable)]
    struct Increment;

    impl Operator<i32> for Increment {
        type Output = i32;
        type Error = Infallible;

        fn apply<R: Rng + ?Sized>(
            &self,
            input: i32,
            _: &mut R,
        ) -> Result<Self::Output, Self::Error> {
            Ok(input + 1)
        }
    }

    #[derive(Composable)]
    struct Double;

    impl Operator<i32> for Double {
        type Output = i32;
        type Error = Infallible;

        fn apply<R: Rng + ?Sized>(
            &self,
            input: i32,
            _: &mut R,
        ) -> Result<Self::Output, Self::Error> {
            Ok(input * 2)
        }
    }

    #[test]
    fn increment_then_double() {
        let combo = Increment.then(Double);
        let result = combo.apply(7, &mut rng()).unwrap();
        assert_eq!(16, result);
    }
}
