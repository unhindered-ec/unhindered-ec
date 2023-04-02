use itertools::Itertools;

use std::iter;

use crate::operator::Operator;
use anyhow::{anyhow, Result};

use super::Composable;

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
{
    type Output = [F::Output; N];

    fn apply(&self, input: Input, rng: &mut rand::rngs::ThreadRng) -> Result<Self::Output> {
        iter::repeat_with(|| self.f.apply(input.clone(), rng))
            .take(N)
            .try_collect::<<F as Operator<Input>>::Output, Vec<<F as Operator<Input>>::Output>, anyhow::Error>()?
            .try_into()
            .map_err(|v: Vec<<F as Operator<Input>>::Output>| {
                anyhow!(
                    "The vector had incorrect length; expected {} and got {}",
                    N,
                    v.len()
                )
            })
    }
}

impl<F, const N: usize> Composable for RepeatWith<F, N> {}

#[cfg(test)]
mod tests {
    use std::ops::Range;

    use rand::{thread_rng, Rng};

    use super::*;

    struct AddOne;
    impl Operator<i32> for AddOne {
        type Output = i32;

        fn apply(&self, input: i32, _: &mut rand::rngs::ThreadRng) -> Result<Self::Output> {
            Ok(input + 1)
        }
    }
    impl Composable for AddOne {}

    #[test]
    fn deterministic() {
        const LENGTH: usize = 5;
        let desired_value = 7;
        let mut rng = thread_rng();
        let repeater: RepeatWith<AddOne, LENGTH> = RepeatWith::new(AddOne);
        #[allow(clippy::unwrap_used)]
        let result = repeater.apply(desired_value, &mut rng).unwrap();
        assert_eq!(LENGTH, result.len());
        result.into_iter().all(|x| x == desired_value);
    }

    struct UniformRange;
    impl Operator<Range<i32>> for UniformRange {
        type Output = i32;

        fn apply(
            &self,
            range: Range<i32>,
            rng: &mut rand::rngs::ThreadRng,
        ) -> Result<Self::Output> {
            Ok(rng.gen_range(range))
        }
    }
    impl Composable for UniformRange {}

    #[test]
    fn stochastic() {
        const LENGTH: usize = 5;
        let range = 0..7;
        let mut rng = thread_rng();
        let repeater: RepeatWith<UniformRange, LENGTH> = RepeatWith::new(UniformRange);
        #[allow(clippy::unwrap_used)]
        let result = repeater.apply(range.clone(), &mut rng).unwrap();
        assert_eq!(LENGTH, result.len());
        result.iter().all(|x| range.contains(x));
    }
}
