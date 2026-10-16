use std::{error::Error, fmt::Display};

use miette::Diagnostic;
use rand::Rng;

use super::Composable;
use crate::operator::Operator;

/// A composable [`Operator`] which maps each value inside a collection to
/// another by applying a contained [`Operator`].
///
/// # Example
/// ```
/// # use ec_core::operator::{Operator, constant::Constant, composable::{Map, MapError}};
/// # use std::convert::Infallible;
/// let operator = Map::new(Constant::new(1i32));
///
/// let result = operator.apply(vec![5, 8, 9], &mut rand::rng())?;
/// assert!(result.into_iter().eq([1, 1, 1]));
/// # Ok::<(),MapError<Infallible>>(())
/// ```
#[derive(Copy, Clone, Debug, Composable, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Map<F> {
    f: F,
}

impl<F> Map<F> {
    /// Create a new [`Map`] operator, applying the contained operator to each
    /// value inside a collection.
    ///
    /// # Example
    /// ```
    /// # use ec_core::operator::{Operator, constant::Constant, composable::{Map, MapError}};
    /// # use std::convert::Infallible;
    /// let constant_collection_operator = Map::new(Constant::new(10));
    /// #
    /// # let result = constant_collection_operator.apply(vec![5, 8, 9, 20], &mut rand::rng())?;
    /// # assert!(result.into_iter().eq([10, 10, 10, 10]));
    /// # Ok::<(),MapError<Infallible>>(())
    /// ```
    pub const fn new(f: F) -> Self {
        Self { f }
    }
}

/// Error that can occur when applying the [`Map`] operator.
///
/// Consists of the underlying operator's error as well as the index in the
/// collection where the operator failed.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct MapError<T>(pub T, pub usize);

impl<T> Display for MapError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Error while applying passed operator on the {}-th element of the mapped iterable",
            self.1
        )
    }
}

impl<T> Error for MapError<T>
where
    T: Error + 'static,
{
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.0)
    }
}

impl<T> Diagnostic for MapError<T>
where
    T: Diagnostic + 'static,
{
    fn diagnostic_source(&self) -> Option<&dyn Diagnostic> {
        Some(&self.0)
    }
}

impl<F, Input, const N: usize> Operator<[Input; N]> for Map<F>
where
    F: Operator<Input>,
{
    type Output = [F::Output; N];
    type Error = MapError<F::Error>;

    /// Apply the [`Operator`] this [`Map`] [`Operator`] is based on to each
    /// item of an array.
    ///
    /// # Examples
    /// ```
    /// # use ec_core::operator::{Operator, constant::Constant, composable::{Map, MapError}};
    /// # use std::convert::Infallible;
    /// let operator = Map::new(Constant::new(1i32));
    ///
    /// let result = operator.apply([true, false], &mut rand::rng())?;
    /// assert_eq!(result, [1, 1]);
    /// # Ok::<(),MapError<Infallible>>(())
    /// ```
    /// ```
    /// # use ec_core::operator::{Operator, constant::Constant, composable::{Map, MapError}};
    /// # use std::convert::Infallible;
    /// let operator = Map::new(Constant::new(1i32));
    ///
    /// let result = operator.apply([true, false, true, false], &mut rand::rng())?;
    /// assert_eq!(result, [1, 1, 1, 1]);
    /// # Ok::<(),MapError<Infallible>>(())
    /// ```
    fn apply<R: Rng + ?Sized>(
        &self,
        input: [Input; N],
        rng: &mut R,
    ) -> Result<Self::Output, Self::Error> {
        // FIXME: Check if using MaybeUninit<T> over
        // Option<T> instead creates a measurable performance improvement
        // because of the omitted additional tag information.
        let mut output: [Option<F::Output>; N] = [const { None }; N];

        for (index, (input, output)) in input.into_iter().zip(output.as_mut_slice()).enumerate() {
            output.replace(self.f.apply(input, rng).map_err(|e| MapError(e, index))?);
        }

        Ok(output.map(Option::unwrap))
    }
}

impl<F, Input1, Input2> Operator<(Input1, Input2)> for Map<F>
where
    // Might want to relax the error bound
    // here at some point in time and
    // instead use a different error type
    // below
    F: Operator<Input1> + Operator<Input2, Error = <F as Operator<Input1>>::Error>,
{
    type Output = (
        <F as Operator<Input1>>::Output,
        <F as Operator<Input2>>::Output,
    );

    type Error = MapError<<F as Operator<Input1>>::Error>;

    /// Apply the [`Operator`] this [`Map`] [`Operator`] is based on to each
    /// item of a 2-tuple, possibly consisting of different types.
    ///
    /// # Example
    /// ```
    /// # use ec_core::operator::{Operator, constant::Constant, composable::{Map, MapError}};
    /// # use std::convert::Infallible;
    /// let operator = Map::new(Constant::new(1i32));
    ///
    /// let result = operator.apply(((), 8), &mut rand::rng())?;
    /// assert_eq!(result, (1, 1));
    /// # Ok::<(),MapError<Infallible>>(())
    /// ```
    fn apply<R: Rng + ?Sized>(
        &self,
        (x, y): (Input1, Input2),
        rng: &mut R,
    ) -> Result<Self::Output, Self::Error> {
        let first_result = self.f.apply(x, rng).map_err(|e| MapError(e, 0))?;
        let second_result = self.f.apply(y, rng).map_err(|e| MapError(e, 1))?;
        Ok((first_result, second_result))
    }
}

impl<F, Input> Operator<Vec<Input>> for Map<F>
where
    F: Operator<Input>,
{
    type Output = Vec<F::Output>;
    type Error = MapError<F::Error>;

    /// Apply the [`Operator`] this [`Map`] [`Operator`] is based on to each
    /// item of a vector.
    ///
    /// # Example
    /// ```
    /// # use ec_core::operator::{Operator, constant::Constant, composable::{Map, MapError}};
    /// # use std::convert::Infallible;
    /// let operator = Map::new(Constant::new(1i32));
    ///
    /// let result = operator.apply(vec![1u64, 8u64], &mut rand::rng())?;
    /// assert_eq!(result, vec![1i32, 1]);
    /// # Ok::<(),MapError<Infallible>>(())
    /// ```
    fn apply<R: Rng + ?Sized>(
        &self,
        input: Vec<Input>,
        rng: &mut R,
    ) -> Result<Self::Output, Self::Error> {
        input
            .into_iter()
            .enumerate()
            .map(|(i, x)| self.f.apply(x, rng).map_err(|e| MapError(e, i)))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use rand::Rng;

    use super::{Map, MapError};
    use crate::operator::{Composable, Operator};

    #[derive(Debug, PartialEq, Eq)]
    struct MyError;

    /// An operator that fails whenever its input equals the value it was
    /// constructed with, and otherwise returns the input unchanged.
    struct FailOn(i32);

    impl Composable for FailOn {}

    impl Operator<i32> for FailOn {
        type Output = i32;
        type Error = MyError;

        fn apply<R: Rng + ?Sized>(&self, input: i32, _rng: &mut R) -> Result<i32, MyError> {
            if input == self.0 {
                Err(MyError)
            } else {
                Ok(input)
            }
        }
    }

    #[test]
    fn vector_reports_the_failing_index() {
        let result = Map::new(FailOn(2)).apply(vec![0, 1, 2, 3], &mut rand::rng());
        assert!(matches!(result, Err(MapError(MyError, 2))));
    }

    #[test]
    fn array_reports_the_failing_index() {
        let result = Map::new(FailOn(1)).apply([0, 1, 2], &mut rand::rng());
        assert!(matches!(result, Err(MapError(MyError, 1))));
    }

    #[test]
    fn tuple_reports_the_first_index() {
        let result = Map::new(FailOn(0)).apply((0, 5), &mut rand::rng());
        assert!(matches!(result, Err(MapError(MyError, 0))));
    }

    #[test]
    fn tuple_reports_the_second_index() {
        let result = Map::new(FailOn(5)).apply((0, 5), &mut rand::rng());
        assert!(matches!(result, Err(MapError(MyError, 1))));
    }

    #[test]
    fn empty_containers_map_to_empty_outputs() {
        let empty_vec = Map::new(FailOn(0)).apply(Vec::<i32>::new(), &mut rand::rng());
        assert_eq!(empty_vec.unwrap(), Vec::<i32>::new());

        let empty_array = Map::new(FailOn(0)).apply([0i32; 0], &mut rand::rng());
        assert_eq!(empty_array.unwrap(), [0i32; 0]);
    }

    #[test]
    fn successful_map_preserves_each_value() {
        let result = Map::new(FailOn(99)).apply(vec![1, 2, 3], &mut rand::rng());
        assert_eq!(result.unwrap(), vec![1, 2, 3]);
    }

    #[test]
    fn error_display_reflects_the_index() {
        assert_ne!(
            MapError(MyError, 7).to_string(),
            MapError(MyError, 8).to_string()
        );
    }
}
