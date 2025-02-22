use rand::Rng;

use super::{super::Operator, Composable};

/// A composable [`Operator`] chaining two [`Operator`]'s
///
/// # Example
/// ```
/// # use ec_core::operator::{
/// #     Operator,
/// #     composable::{Map, Then, RepeatWith},
/// #     constant::Constant,
/// #     identity::Identity
/// # };
/// #
/// let chained_operator = Then::new(
///     RepeatWith::<_, 2>::new(Constant::new(1)),
///     Map::new(Identity),
/// );
///
/// assert_eq!(chained_operator.apply((), &mut rand::rng())?, [1; 2]);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[derive(Debug, Composable, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Then<F, G> {
    f: F,
    g: G,
}

impl<F, G> Then<F, G> {
    pub const fn new(f: F, g: G) -> Self {
        Self { f, g }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ThenError<T, U> {
    First(T),
    Second(U),
}

impl<T, U> std::fmt::Display for ThenError<T, U> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::First(_) => f.write_str(
                "Error while applying the first passed operator (`T`) in the `Then<T,_>` Operator",
            ),
            Self::Second(_) => f.write_str(
                "Error while applying the second passed operator (`U`) in the `Then<_,U>` Operator",
            ),
        }
    }
}

impl<T, U> std::error::Error for ThenError<T, U>
where
    T: std::error::Error + 'static,
    U: std::error::Error + 'static,
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::First(t) => Some(t),
            Self::Second(t) => Some(t),
        }
    }
}

impl<T, U> miette::Diagnostic for ThenError<T, U>
where
    T: miette::Diagnostic + 'static,
    U: miette::Diagnostic + 'static,
{
    fn diagnostic_source(&self) -> Option<&dyn miette::Diagnostic> {
        match self {
            Self::First(t) => Some(t),
            Self::Second(t) => Some(t),
        }
    }
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
