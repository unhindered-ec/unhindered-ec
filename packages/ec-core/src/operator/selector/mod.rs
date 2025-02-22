use rand::Rng;

use super::{Composable, Operator};
use crate::population::Population;

#[cfg(feature = "erased")]
mod erased;
#[cfg(feature = "erased")]
pub use erased::*;

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
/// # [dyn-compatibility](https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility)
///
/// This trait is **not** dyn-compatible. As such please
/// try to avoid the need for trait objects whenever you can.
///
/// If you can't get around the usage of trait objects, you can
/// use the [`DynSelector`] trait, which is available if you compile
/// this crate with the `erased` feature.
///
/// Please see its documentation for further details on its usage.
///
/// # Usage
/// Also of note is that similarly to the `Read` and `Write` traits in the
/// standard library, the [`Selector`] trait is implemented for references to
/// selectors. That means, if you don't wish to consume a selector and want to
/// re-use it later, you can pass a reference to that selector to any function
/// expecting a [`Selector`] instead.
///
/// # Examples
///
/// In this example we use the
/// [`Best`](ec_core::operator::selector::best::Best) selector to choose the
/// "best" (maximal) value from a list.
///
/// ```
/// # use ec_core::operator::selector::{best::Best, Selector};
/// # use rand::rng;
/// #
/// let population = [5, 8, 9, 2, 3, 6];
/// let winner = Best.select(&population, &mut rng())?;
/// assert_eq!(*winner, 9);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// Here we implement a `First` selector that always returns the first
/// element in an array.
///
/// ```
/// # use rand::{Rng, rng};
/// # use ec_core::operator::selector::{error::EmptyPopulation, Selector};
/// #
/// struct First;
///
/// impl<const N: usize> Selector<[u8; N]> for First {
///     type Error = EmptyPopulation;
///
///     fn select<'pop, R: Rng + ?Sized>(
///         &self,
///         population: &'pop [u8; N],
///         _: &mut R,
///     ) -> Result<&'pop u8, Self::Error> {
///         population.first().ok_or(EmptyPopulation)
///     }
/// }
///
/// let population = [5, 8, 9];
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
    fn select<'pop, R: Rng + ?Sized>(
        &self,
        population: &'pop P,
        rng: &mut R,
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
/// # use rand::{Rng, rng};
/// #
/// # use ec_core::operator::{Operator, selector::{error::EmptyPopulation, Select, Selector}};
/// #
/// struct First; // Simple selector that always returns the first element in a vector.
///
/// impl<T, const N: usize> Selector<[T; N]> for First {
///     type Error = EmptyPopulation;
///
///     fn select<'pop, R: Rng + ?Sized>(
///         &self,
///         population: &'pop [T; N],
///         _: &mut R,
///     ) -> Result<&'pop T, Self::Error> {
///         population.first().ok_or(EmptyPopulation)
///     }
/// }
///
/// // Create a `Select` operator from the `First` selector
/// let select = Select::new(First);
/// let population = [5u8, 8, 9];
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
/// # use rand::{Rng, rng};
/// # use std::convert::Infallible;
/// #
/// # use ec_core::operator::{Composable, Operator, selector::{error::EmptyPopulation, Select, Selector}};
/// #
/// # struct First; // Simple selector that always returns the first element in a vector.
/// #
/// # impl<T, const N: usize> Selector<[T;N]> for First {
/// #    type Error = EmptyPopulation;
/// #
/// #    fn select<'pop, R: Rng + ?Sized>(
/// #        &self,
/// #        population: &'pop [T;N],
/// #        _: &mut R,
/// #    ) -> Result<&'pop T, Self::Error> {
/// #        population
/// #            .first()
/// #            .ok_or(EmptyPopulation)
/// #    }
/// # }
/// #
/// // All `Operators` have to implement `Composable` so we can chain them.
/// // The default implementations of all the `Composable` methods are fine,
/// // though, so we don't have to do anything other than add a derive.
/// #[derive(Composable)]
/// struct StrLen; // A simple `Operator` that takes a `&String` and returns its length.
///
/// impl Operator<&String> for StrLen {
///     type Output = usize;
///     type Error = Infallible;
///
///     // The argument is a reference to a `String` because `Selector`s return
///     // references to the individuals they choose.
///     fn apply<R: Rng + ?Sized>(&self, input: &String, _: &mut R) -> Result<usize, Self::Error> {
///         Ok(input.len())
///     }
/// }
///
/// let select = Select::new(First);
/// let chain = select.then(StrLen);
/// let population = ["Hello".to_string(), "World".to_string()];
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
/// # use rand::{Rng, rng};
/// #
/// # use ec_core::operator::{Composable, Operator, selector::{error::EmptyPopulation, Select, Selector}};
/// #
/// # struct First; // Simple selector that always returns the first element in a vector.
/// #
/// # impl<T, const N:usize> Selector<[T;N]> for First {
/// #    type Error = EmptyPopulation;
/// #
/// #    fn select<'pop, R: Rng + ?Sized>(
/// #        &self,
/// #        population: &'pop [T;N],
/// #        _: &mut R,
/// #    ) -> Result<&'pop T, Self::Error> {
/// #        population
/// #            .first()
/// #            .ok_or(EmptyPopulation)
/// #    }
/// # }
/// #
/// # #[derive(Composable)]
/// # struct StrLen; // A simple `Operator` that takes a `&String` and returns its length.
/// #
/// # impl Operator<&String> for StrLen {
/// #    type Output = usize;
/// #    type Error = Infallible;
/// #
/// #    fn apply<R: Rng + ?Sized>(&self, input: &String, _: &mut R) -> Result<usize, Self::Error>{
/// #        Ok(input.len())
/// #    }
/// # }
/// #
/// let ref_select = Select::new(&First);
/// let chain = ref_select.then(StrLen);
/// let population = ["Hello".to_string(), "World".to_string()];
/// let choice_length: usize = chain.apply(&population, &mut rng())?;
/// assert_eq!(choice_length, 5);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[derive(Debug, Clone, Composable, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Select<S> {
    /// The wrapped [`Selector`] that this [`Select`] will apply
    selector: S,
}

impl<S> Select<S> {
    /// Create a new [`Select`] [`Operator`] from the given [`Selector`]
    ///
    /// # Example
    /// ```
    /// # use ec_core::operator::selector::{Select, best::Best};
    /// let operator = Select::new(Best);
    /// # let _ = operator;
    /// ```
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
    ///
    /// # Example
    /// ```
    /// # use ec_core::operator::{Operator, selector::{Select, best::Best, error::EmptyPopulation}};
    /// let population = [2, 3, 5];
    ///
    /// let operator = Select::new(Best);
    /// let selected = operator.apply(&population, &mut rand::rng())?;
    ///
    /// assert_eq!(*selected, 5);
    /// # Ok::<(), EmptyPopulation>(())
    /// ```
    fn apply<R: Rng + ?Sized>(
        &self,
        population: &'pop P,
        rng: &mut R,
    ) -> Result<Self::Output, Self::Error> {
        self.selector.select(population, rng)
    }
}

/// Implement [`Selector`] for a reference to a [`Selector`].
/// This allows us to wrap a reference to a selector in a [`Select`] operator
/// and use it in chains of operators.
impl<S, P> Selector<P> for &S
where
    P: Population,
    S: Selector<P>,
{
    type Error = S::Error;

    /// Select from the [`Selector`] behind this reference, using the selector
    /// by reference rather than by value.
    ///
    /// # Example
    /// ```
    /// # use ec_core::operator::{Operator, selector::{Select, best::Best, error::EmptyPopulation}};
    /// let population = [2, 3, 5];
    ///
    /// let selector = Best;
    /// let operator = Select::new(&selector); // <- by reference
    /// let selected = operator.apply(&population, &mut rand::rng())?;
    ///
    /// assert_eq!(*selected, 5);
    ///
    /// // can re-use the selector
    /// let operator_2 = Select::new(selector);
    /// # let _ = operator_2;
    /// # Ok::<(), EmptyPopulation>(())
    /// ```
    fn select<'pop, R: Rng + ?Sized>(
        &self,
        population: &'pop P,
        rng: &mut R,
    ) -> Result<&'pop P::Individual, Self::Error> {
        (**self).select(population, rng)
    }
}

#[cfg(test)]
mod tests {
    use std::convert::Infallible;

    use rand::{Rng, rng};

    use super::{Select, Selector, error::EmptyPopulation};
    use crate::operator::{Composable, Operator};

    // A simple `Selector`` that always returns the first item in a vector.
    struct First;

    impl<T, const N: usize> Selector<[T; N]> for First {
        type Error = EmptyPopulation;

        fn select<'pop, R: Rng + ?Sized>(
            &self,
            population: &'pop [T; N],
            _: &mut R,
        ) -> Result<&'pop T, Self::Error> {
            population.first().ok_or(EmptyPopulation)
        }
    }

    // A simple `Operator` that takes a `&str` (or anything that can be treated as
    // a `&str`, like `&String) and returns its length.
    #[derive(Composable)]
    struct StrLen;

    impl<T> Operator<T> for StrLen
    where
        T: AsRef<str>,
    {
        type Output = usize;
        type Error = Infallible;

        fn apply<R: Rng + ?Sized>(&self, input: T, _: &mut R) -> Result<usize, Self::Error> {
            Ok(input.as_ref().len())
        }
    }

    #[test]
    fn can_implement_simple_selector() {
        let population = [5, 8, 9];
        let choice = First.select(&population, &mut rng()).unwrap();
        assert_eq!(*choice, 5);
    }

    #[test]
    fn can_wrap_selector() {
        let select = Select::new(First);
        let population = [5, 8, 9];
        let choice = select.apply(&population, &mut rng()).unwrap();
        assert_eq!(*choice, 5);
    }

    #[test]
    fn can_wrap_selector_reference() {
        // Make sure that we can pass a reference to a selector to `Select` and
        // still successfully select values.
        let select = Select::new(&First);
        let population = [5, 8, 9];
        let choice = select.apply(&population, &mut rng()).unwrap();
        assert_eq!(*choice, 5);
    }

    #[test]
    fn can_chain_selector_and_operator() {
        let select = Select::new(First);
        let double = StrLen;
        let chain = select.then(double);
        let population = ["Hello".to_string(), "World!".to_string()];
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
        let population = ["Hello".to_string(), "World!".to_string()];
        let length_of_choice: usize = chain.apply(&population, &mut rng()).unwrap();
        assert_eq!(length_of_choice, 5);
    }
}
