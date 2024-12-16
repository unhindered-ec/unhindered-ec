use std::iter;

use super::Composable;
use crate::operator::Operator;

/// An `Operator` that applies the encapsulated `Operator`
/// `N` times on the given input, returning an array of
/// the `N` results.
pub struct RepeatWith<F, const N: usize> {
    f: F,
}

impl<F, const N: usize> RepeatWith<F, N> {
    pub const fn new(f: F) -> Self {
        Self { f }
    }
}

impl<F, Input, const N: usize> Operator<Input> for RepeatWith<F, N>
where
    Input: Clone,
    F: Operator<Input>,
    anyhow::Error: From<F::Error>,
{
    type Output = [F::Output; N];
    type Error = anyhow::Error;

    fn apply(
        &self,
        input: Input,
        rng: &mut rand::rngs::ThreadRng,
    ) -> Result<Self::Output, Self::Error> {
        Ok(iter::repeat_with(|| self.f.apply(input.clone(), rng))
            .take(N)
            .collect::<Result<Vec<_>, _>>()?
            .try_into()
            .unwrap_or_else(|v: Vec<_>| {
                unreachable!(
                    "The vector had incorrect length; expected {} and got {}",
                    N,
                    v.len()
                )
            }))
    }
}

impl<F, const N: usize> Composable for RepeatWith<F, N> {}

#[cfg(test)]
#[expect(
    clippy::arithmetic_side_effects,
    reason = "The tradeoff safety <> ease of writing arguably lies on the ease of writing side \
              for test code."
)]
#[expect(
    clippy::unwrap_used,
    reason = "Panicking is the best way to deal with errors in unit tests"
)]
mod tests {
    use std::{convert::Infallible, ops::Range};

    use rand::{Rng, rng};

    use super::*;

    struct AddOne;
    impl Operator<i32> for AddOne {
        type Output = i32;
        type Error = Infallible;

        fn apply(
            &self,
            input: i32,
            _: &mut rand::rngs::ThreadRng,
        ) -> Result<Self::Output, Self::Error> {
            Ok(input + 1)
        }
    }
    impl Composable for AddOne {}

    #[test]
    fn deterministic() {
        const LENGTH: usize = 5;
        let desired_value = 7;
        let mut rng = rng();
        let repeater: RepeatWith<AddOne, LENGTH> = RepeatWith::new(AddOne);
        let result = repeater.apply(desired_value, &mut rng).unwrap();
        assert_eq!(LENGTH, result.len());
        result.into_iter().all(|x| x == desired_value);
    }

    struct UniformRange;
    impl Operator<Range<i32>> for UniformRange {
        type Output = i32;
        type Error = Infallible;

        fn apply(
            &self,
            range: Range<i32>,
            rng: &mut rand::rngs::ThreadRng,
        ) -> Result<Self::Output, Self::Error> {
            Ok(rng.random_range(range))
        }
    }
    impl Composable for UniformRange {}

    #[test]
    fn stochastic() {
        const LENGTH: usize = 5;
        let range = 0..7;
        let mut rng = rng();
        let repeater: RepeatWith<UniformRange, LENGTH> = RepeatWith::new(UniformRange);
        let result = repeater.apply(range.clone(), &mut rng).unwrap();
        assert_eq!(LENGTH, result.len());
        result.iter().all(|x| range.contains(x));
    }
}
