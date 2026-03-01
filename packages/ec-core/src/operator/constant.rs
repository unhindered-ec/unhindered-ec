use std::convert::Infallible;

use rand::Rng;

use super::{Composable, Operator};

/// An [`Operator`] that always returns the same value regardless
/// of the input.
///
/// # See also
///
/// See [`Operator`] and [`Composable`].
///
/// # Examples
/// ```
/// # use ec_core::operator::{Operator, constant::Constant};
/// #
/// // This will always return 5 regardless of the input.
/// let constant_five = Constant::new(5);
///
/// assert_eq!(constant_five.apply(3, &mut rand::rng())?, 5);
/// assert_eq!(constant_five.apply("string", &mut rand::rng())?, 5);
/// assert_eq!(constant_five.apply(true, &mut rand::rng())?, 5);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[derive(Debug, Composable, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Constant<T> {
    /// The value that this operator will always return.
    value: T,
}

impl<T> From<T> for Constant<T> {
    /// Convert a `T` into a [`Operator`] returning a constant `T`.
    ///
    /// # Example
    /// ```
    /// # use ec_core::operator::{constant::Constant, Operator};
    /// # use rand::rng;
    /// #
    /// let my_constant_operator: Constant<_> = 5.into();
    ///
    /// let Ok(sample_value) = my_constant_operator.apply((), &mut rng());
    /// assert_eq!(sample_value, 5);
    /// ```
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl<T> Constant<T> {
    /// Create a new [`Constant`] [`Operator`], always returning it's value.
    ///
    /// # Examples
    /// ```
    /// # use ec_core::operator::{Operator, constant::Constant};
    /// #
    /// // This will always return 5 regardless of the input.
    /// let constant_five = Constant::new(5);
    /// #
    /// # assert_eq!(constant_five.apply(3, &mut rand::rng())?, 5);
    /// # assert_eq!(constant_five.apply("string", &mut rand::rng())?, 5);
    /// # assert_eq!(constant_five.apply(true, &mut rand::rng())?, 5);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub const fn new(value: T) -> Self {
        Self { value }
    }
}

impl<S, T> Operator<S> for Constant<T>
where
    T: Clone,
{
    /// The output type of this [`Operator`] is the type of the value
    /// stored in the [`Operator`].
    type Output = T;
    /// This [`Operator`] can't fail
    type Error = Infallible;

    /// Apply this [`Constant`] [`Operator`], always returning a [`Clone`] of
    /// it's stored value, regardless of the input.
    ///
    /// # Example
    /// ```
    /// # use ec_core::operator::{Operator, constant::Constant};
    /// #
    /// // This will always return 5 regardless of the input.
    /// let constant_five = Constant::new(5);
    ///
    /// let result = constant_five.apply("Hello, World!", &mut rand::rng())?;
    ///
    /// assert_eq!(result, 5);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    fn apply<R: Rng + ?Sized>(&self, _: S, _: &mut R) -> Result<Self::Output, Self::Error> {
        Ok(self.value.clone())
    }
}

#[cfg(test)]
mod tests {
    use rand::rng;

    use crate::operator::{Operator, constant::Constant};

    #[test]
    fn is_constant() {
        let mut rng = rng();
        // This should always return 5 regardless of the input.
        let constant_five = Constant::new(5);

        assert_eq!(constant_five.apply(3, &mut rng).unwrap(), 5);
        assert_eq!(constant_five.apply("string", &mut rng).unwrap(), 5);
        assert_eq!(constant_five.apply(true, &mut rng).unwrap(), 5);
    }
}
