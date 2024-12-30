use rand::rngs::ThreadRng;

use super::{Composable, Operator};
use crate::population::Population;

pub mod best;
pub mod dyn_weighted;
pub mod error;
pub mod lexicase;
pub mod random;
pub mod tournament;
pub mod worst;

pub use error::EmptyPopulation;

/// Select an individual from a `Population`.
///
/// See [`Select`] for a wrapper that converts a `Selector` into an
/// [`Operator`], allowing selectors to be used in chains of operators.
///
/// # Examples
///
/// In this example we use the `[Best]` selector to choose the
/// "best" (maximal) value from a list.
///
/// ```
/// # use ec_core::operator::selector::{best::Best, Selector};
/// # use rand::rng;
/// #
/// let population = vec![5, 8, 9, 2, 3, 6];
/// let winner = Best.select(&population, &mut rng())?;
/// assert_eq!(*winner, 9);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// Here we implement a `First` selector that always returns the first
/// element in a vector.
///
/// ```
/// # use rand::{rngs::ThreadRng, rng};
/// # use ec_core::operator::selector::{error::EmptyPopulation, Selector};
/// #
/// struct First;
///
/// impl Selector<Vec<u8>> for First {
///     type Error = EmptyPopulation;
///
///     fn select<'pop>(
///         &self,
///         population: &'pop Vec<u8>,
///         _: &mut ThreadRng,
///     ) -> Result<&'pop u8, Self::Error> {
///         population.first().ok_or(EmptyPopulation)
///     }
/// }
///
/// let population = vec![5, 8, 9];
/// let choice = First.select(&population, &mut rng())?;
/// assert_eq!(*choice, 5);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub trait Selector<P>
where
    P: Population,
{
    type Error;

    /// Select an individual from the given `population`
    ///
    /// # Errors
    ///
    /// This will return an error if there's some problem selecting. That will
    /// usually be because the population is empty or not large enough for
    /// the desired selector.
    fn select<'pop>(
        &self,
        population: &'pop P,
        rng: &mut ThreadRng,
    ) -> Result<&'pop P::Individual, Self::Error>;
}

/// A wrapper that converts a `Selector` into an `Operator`.
///
/// This allows the inclusion of `Selector`s in chains of operators.
///
/// See [the `operator` module docs](crate::operator#wrappers) for more on the
/// design decisions behind using wrappers.
///
/// # Examples
///
/// Here we illustrate the implementation of a simple [`Selector`],
/// `First`, which returns the first item in a vector. We then wrap that in a
/// [`Select`] to create an operator. Then calling [`Operator::apply`] on the
/// operator is the same as calling [`Selector::select`] directly on the
/// selector.
///
/// ```
/// # use rand::{rngs::ThreadRng, rng};
/// #
/// # use ec_core::operator::{Operator, selector::{error::EmptyPopulation, Select, Selector}};
/// #
/// struct First; // Simple selector that always returns the first element in a vector.
///
/// impl<T> Selector<Vec<T>> for First {
///     type Error = EmptyPopulation;
///
///     fn select<'pop>(
///         &self,
///         population: &'pop Vec<T>,
///         _: &mut ThreadRng,
///     ) -> Result<&'pop T, Self::Error> {
///         population.first().ok_or(EmptyPopulation)
///     }
/// }
///
/// // Create a `Select` operator from the `First` selector
/// let select = Select::new(First);
/// let population: Vec<u8> = vec![5, 8, 9];
///
/// // Selectors return references to the individuals they choose, so we
/// // get a `&u8` back from `select` and `apply`.
/// let selector_result: &u8 = First.select(&population, &mut rng())?;
/// assert_eq!(*selector_result, 5);
///
/// let operator_result: &u8 = select.apply(&population, &mut rng())?;
/// assert_eq!(selector_result, operator_result);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// Because both [`Select`] and [`Operator`] are [`Composable`], we can chain
/// them using the [`Composable::then()`] method, and then [`Operator::apply`]
/// the resulting chain to an input. In the examples below we apply the chain
/// to a "population" that is a vector of `String`s. The `First` selector will
/// return the first element of the vector, and then the `StrLen` operator will
/// return the length of that string.
///
/// ```
/// # use rand::{rngs::ThreadRng, rng};
/// # use std::convert::Infallible;
/// #
/// # use ec_core::operator::{Composable, Operator, selector::{error::EmptyPopulation, Select, Selector}};
/// #
/// # struct First; // Simple selector that always returns the first element in a vector.
/// #
/// # impl<T> Selector<Vec<T>> for First {
/// #    type Error = EmptyPopulation;
/// #
/// #    fn select<'pop>(
/// #        &self,
/// #        population: &'pop Vec<T>,
/// #        _: &mut ThreadRng,
/// #    ) -> Result<&'pop T, Self::Error> {
/// #        population
/// #            .first()
/// #            .ok_or(EmptyPopulation)
/// #    }
/// # }
/// #
/// struct StrLen; // A simple `Operator` that takes a `&String` and returns its length.
///
/// impl Operator<&String> for StrLen {
///     type Output = usize;
///     type Error = Infallible;
///
///     // The argument is a reference to a `String` because `Selector`s return
///     // references to the individuals they choose.
///     fn apply(&self, input: &String, _: &mut ThreadRng) -> Result<usize, Self::Error> {
///         Ok(input.len())
///     }
/// }
/// // All `Operators` have to implement `Composable` so we can chain them.
/// // The default implementations of all the `Composable` methods are fine,
/// // though, so we don't have to do anything.
/// impl Composable for StrLen {}
///
/// let select = Select::new(First);
/// let chain = select.then(StrLen);
/// let population: Vec<String> = vec!["Hello".to_string(), "World".to_string()];
///
/// // The `StrLen` operator will take the `&String` returned by the `First`
/// // selector and return its length.
/// let choice_length: usize = chain.apply(&population, &mut rng())?;
/// assert_eq!(choice_length, 5);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// We can also pass a _reference_ to a [`Selector`] (i.e., `&Selector`) to
/// [`Select`], allowing us to pass references to selectors into chains of
/// operators without requiring or giving up ownership of the selector.
///
/// ```
/// # use std::convert::Infallible;
/// #
/// # use rand::{rngs::ThreadRng, rng};
/// #
/// # use ec_core::operator::{Composable, Operator, selector::{error::EmptyPopulation, Select, Selector}};
/// #
/// # struct First; // Simple selector that always returns the first element in a vector.
/// #
/// # impl<T> Selector<Vec<T>> for First {
/// #    type Error = EmptyPopulation;
/// #
/// #    fn select<'pop>(
/// #        &self,
/// #        population: &'pop Vec<T>,
/// #        _: &mut ThreadRng,
/// #    ) -> Result<&'pop T, Self::Error> {
/// #        population
/// #            .first()
/// #            .ok_or(EmptyPopulation)
/// #    }
/// # }
/// #
/// # struct StrLen; // A simple `Operator` that takes a `&String` and returns its length.
/// #
/// # impl Operator<&String> for StrLen {
/// #    type Output = usize;
/// #    type Error = Infallible;
/// #
/// #    fn apply(&self, input: &String, _: &mut ThreadRng) -> Result<usize, Self::Error> {
/// #        Ok(input.len())
/// #    }
/// # }
/// # impl Composable for StrLen {}
/// #
/// let ref_select = Select::new(&First);
/// let chain = ref_select.then(StrLen);
/// let population: Vec<String> = vec!["Hello".to_string(), "World".to_string()];
/// let choice_length: usize = chain.apply(&population, &mut rng())?;
/// assert_eq!(choice_length, 5);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[derive(Clone)]
pub struct Select<S> {
    /// The wrapped [`Selector`] that this [`Select`] will apply
    selector: S,
}

impl<S> Select<S> {
    pub const fn new(selector: S) -> Self {
        Self { selector }
    }
}

impl<'pop, P, S> Operator<&'pop P> for Select<S>
where
    P: Population,
    S: Selector<P>,
{
    /// The `Output` is a _reference_ to the selected individual
    type Output = &'pop P::Individual;
    type Error = S::Error;

    /// Apply this `Selector` as an `Operator`
    fn apply(&self, population: &'pop P, rng: &mut ThreadRng) -> Result<Self::Output, Self::Error> {
        self.selector.select(population, rng)
    }
}
impl<S> Composable for Select<S> {}

/// Implement [`Selector`] for a reference to a [`Selector`].
/// This allows us to wrap a reference to a selector in a [`Select`] operator
/// and use it in chains of operators.
impl<S, P> Selector<P> for &S
where
    P: Population,
    S: Selector<P>,
{
    type Error = S::Error;

    fn select<'pop>(
        &self,
        population: &'pop P,
        rng: &mut ThreadRng,
    ) -> Result<&'pop P::Individual, Self::Error> {
        (**self).select(population, rng)
    }
}

#[cfg(test)]
mod tests {
    use std::convert::Infallible;

    use rand::{rng, rngs::ThreadRng};

    use super::{Select, Selector, error::EmptyPopulation};
    use crate::operator::{Composable, Operator};

    // A simple `Selector`` that always returns the first item in a vector.
    struct First;

    impl<T> Selector<Vec<T>> for First {
        type Error = EmptyPopulation;

        fn select<'pop>(
            &self,
            population: &'pop Vec<T>,
            _: &mut ThreadRng,
        ) -> Result<&'pop T, Self::Error> {
            population.first().ok_or(EmptyPopulation)
        }
    }

    // A simple `Operator` that takes a `&str` (or anything that can be treated as
    // a `&str`, like `&String) and returns its length.
    struct StrLen;

    impl<T> Operator<T> for StrLen
    where
        T: AsRef<str>,
    {
        type Output = usize;
        type Error = Infallible;

        fn apply(&self, input: T, _: &mut ThreadRng) -> Result<usize, Self::Error> {
            Ok(input.as_ref().len())
        }
    }
    impl Composable for StrLen {}

    #[test]
    fn can_implement_simple_selector() {
        let population = vec![5, 8, 9];
        let choice = First.select(&population, &mut rng()).unwrap();
        assert_eq!(*choice, 5);
    }

    #[test]
    fn can_wrap_selector() {
        let select = Select::new(First);
        let population = vec![5, 8, 9];
        let choice = select.apply(&population, &mut rng()).unwrap();
        assert_eq!(*choice, 5);
    }

    #[test]
    fn can_wrap_selector_reference() {
        // Make sure that we can pass a reference to a selector to `Select` and
        // still successfully select values.
        let select = Select::new(&First);
        let population = vec![5, 8, 9];
        let choice = select.apply(&population, &mut rng()).unwrap();
        assert_eq!(*choice, 5);
    }

    #[test]
    fn can_chain_selector_and_operator() {
        let select = Select::new(First);
        let double = StrLen;
        let chain = select.then(double);
        let population: Vec<String> = vec!["Hello".to_string(), "World!".to_string()];
        let length_of_choice: usize = chain.apply(&population, &mut rng()).unwrap();
        assert_eq!(length_of_choice, 5);
    }

    #[test]
    fn can_chain_with_selector_reference() {
        // Use a _reference_ to a selector in the chain instead of an
        // owned selector.
        let select = Select::new(&First);
        let double = StrLen;
        let chain = select.then(double);
        let population: Vec<String> = vec!["Hello".to_string(), "World!".to_string()];
        let length_of_choice: usize = chain.apply(&population, &mut rng()).unwrap();
        assert_eq!(length_of_choice, 5);
    }
}
