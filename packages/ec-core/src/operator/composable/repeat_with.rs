use std::iter;

use rand::Rng;

use super::Composable;
use crate::operator::Operator;

/// A composition [`Operator`] that applies the encapsulated [`Operator`]
/// a constant `N` times on the given input, returning an array of
/// the `N` results.
///
/// Since the contained operator is applied multiple times, the Input to this
/// operator needs to implement [`Clone`].
///
/// # Example
/// ```
/// # use ec_core::operator::{Operator, constant::Constant, composable::RepeatWith};
/// # use std::convert::Infallible;
/// let operator = RepeatWith::<_, 5>::new(Constant::new(1));
///
/// let value = operator.apply((), &mut rand::rng())?;
///
/// assert_eq!(value, [1; 5]);
/// # Ok::<(), Infallible>(())
/// ```
#[derive(Debug, Composable, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct RepeatWith<F, const N: usize> {
    f: F,
}

impl<F, const N: usize> From<F> for RepeatWith<F, N> {
    /// Convert `Self` into a repeated [`Operator`].
    ///
    /// The count of repetitions is provided by the `N` generic on
    /// [`RepeatWith`]. This is identical to [`RepeatWith::new`]
    ///
    /// # Examples
    /// ```
    /// # use ec_core::operator::{Operator, constant::Constant, composable::RepeatWith};
    /// # use std::convert::Infallible;
    /// let repeated_operator = RepeatWith::<_, 5>::from(Constant::new(1));
    /// #
    /// # let value = repeated_operator.apply((), &mut rand::rng())?;
    /// # assert_eq!(value, [1; 5]);
    /// # Ok::<(), Infallible>(())
    /// ```
    /// ```
    /// # use ec_core::operator::{Operator, constant::Constant, composable::RepeatWith};
    /// # use std::convert::Infallible;
    /// let repeated_operator: RepeatWith<_, 5> = Constant::new(1).into();
    /// #
    /// # let value = repeated_operator.apply((), &mut rand::rng())?;
    /// # assert_eq!(value, [1; 5]);
    /// # Ok::<(), Infallible>(())
    /// ```
    fn from(value: F) -> Self {
        Self::new(value)
    }
}

impl<F, const N: usize> RepeatWith<F, N> {
    /// Create a [`RepeatWith`] from another [`Operator`].
    ///
    /// The count of repetitions is provided by the `N` generic on
    /// [`RepeatWith`]. This is identical to [`RepeatWith::from`]
    ///
    /// # Example
    /// ```
    /// # use ec_core::operator::{Operator, constant::Constant, composable::RepeatWith};
    /// # use std::convert::Infallible;
    /// let repeated_operator = RepeatWith::<_, 5>::new(Constant::new(1));
    /// #
    /// # let value = repeated_operator.apply((), &mut rand::rng())?;
    /// # assert_eq!(value, [1; 5]);
    /// # Ok::<(), Infallible>(())
    /// ```
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
    type Error = F::Error;

    /// Apply this [`RepeatWith`] [`Operator`] to the input, applying the
    /// contained [`Operator`] N times. This requires [`Input:
    /// Clone`](Clone).
    ///
    /// Returns an array of the outputs of length `N`.
    ///
    /// # Errors
    /// - `F::Error` (The underlying [`Operator`]'s error) if any of the `N`
    ///   applications fail.
    ///
    /// # Example
    /// ```
    /// # use ec_core::operator::{Operator, constant::Constant, composable::RepeatWith};
    /// # use std::convert::Infallible;
    /// let operator = RepeatWith::<_, 5>::new(Constant::new(10));
    ///
    /// let value = operator.apply((), &mut rand::rng())?;
    ///
    /// assert_eq!(value, [10; 5]);
    /// # Ok::<(), Infallible>(())
    /// ```
    fn apply<R: Rng + ?Sized>(
        &self,
        input: Input,
        rng: &mut R,
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

#[cfg(test)]
#[expect(
    clippy::arithmetic_side_effects,
    reason = "The tradeoff safety <> ease of writing arguably lies on the ease of writing side \
              for test code."
)]
mod tests {
    use std::{convert::Infallible, ops::Range};

    use rand::{Rng, rng};

    use super::*;

    #[derive(Composable)]
    struct AddOne;

    impl Operator<i32> for AddOne {
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

    #[test]
    fn deterministic() {
        const LENGTH: usize = 5;
        let desired_value = 7;
        let mut rng = rng();
        let repeater: RepeatWith<AddOne, LENGTH> = RepeatWith::new(AddOne);
        // This is fine since the compiler can use static analysis to
        // verify that the Err variant of the result enum is uninhabited (can't be
        // constructed, is of type '!') and as such the Pattern `Ok()` becomes
        // irrefutable here.
        //
        // If it wasn't, then we would either need a match block or a else clause
        // instead and the compiler would complain.
        let Ok(result) = repeater.apply(desired_value, &mut rng);
        assert_eq!(LENGTH, result.len());
        result.into_iter().all(|x| x == desired_value);
    }

    #[derive(Composable)]
    struct UniformRange;

    impl Operator<Range<i32>> for UniformRange {
        type Output = i32;
        type Error = Infallible;

        fn apply<R: Rng + ?Sized>(
            &self,
            range: Range<i32>,
            rng: &mut R,
        ) -> Result<Self::Output, Self::Error> {
            Ok(rng.random_range(range))
        }
    }

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
