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
///
/// ```
/// # use ec_core::operator::{Operator, constant::Constant};
/// # use rand::rng;
/// #
/// let mut rng = rng();
/// // This will always return 5 regardless of the input.
/// let constant_five = Constant::new(5);
///
/// assert_eq!(constant_five.apply(3, &mut rng).unwrap(), 5);
/// assert_eq!(constant_five.apply("string", &mut rng).unwrap(), 5);
/// assert_eq!(constant_five.apply(true, &mut rng).unwrap(), 5);
/// ```
#[derive(Composable)]
pub struct Constant<T> {
    /// The value that this operator will always return.
    value: T,
}

impl<T> Constant<T> {
    /// Return the value stored in this [`Operator`].
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

    /// Always return the value stored in the [`Operator`] regardless of the
    /// input value (of type `S`).
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
