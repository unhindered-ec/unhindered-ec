use std::convert::Infallible;

use rand::Rng;

use super::{Composable, Operator};

/// An [`Operator`] that always returns its input value.
///
/// # See also
///
/// See [`Operator`] and [`Composable`].
///
/// # Examples
///
/// ```
/// # use ec_core::operator::{Operator, identity::Identity};
/// #
/// // This will always return the value that is passed in.
/// let identity = Identity;
///
/// assert_eq!(identity.apply(3, &mut rand::rng())?, 3);
/// assert_eq!(identity.apply("string", &mut rand::rng())?, "string");
/// assert_eq!(identity.apply(Some(7), &mut rand::rng())?, Some(7));
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[derive(Debug, Composable, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Identity;

impl<T> Operator<T> for Identity {
    /// The output type (and value) of this [`Operator`] is the same as that of
    /// whatever input is passed in.
    type Output = T;
    /// This [`Operator`] can't fail
    type Error = Infallible;

    /// Apply this [`Identity`] [`Operator`], always returning the input value.
    ///
    /// ```
    /// # use ec_core::operator::{Operator, identity::Identity};
    /// #
    /// // This will always return the value that is passed in.
    /// let identity = Identity;
    ///
    /// let result = identity.apply("Hello, World!", &mut rand::rng())?;
    ///
    /// assert_eq!(result, "Hello, World!");
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    fn apply<R: Rng + ?Sized>(&self, input: T, _: &mut R) -> Result<Self::Output, Self::Error> {
        Ok(input)
    }
}

#[cfg(test)]
mod tests {
    use rand::rng;

    use crate::operator::{Operator, identity::Identity};

    #[test]
    fn returns_input() {
        let mut rng = rng();
        // This should always return its input.
        let identity = Identity;

        assert_eq!(identity.apply(3, &mut rng).unwrap(), 3);
        assert_eq!(identity.apply("string", &mut rng).unwrap(), "string");
        assert_eq!(identity.apply(Some(7), &mut rng).unwrap(), Some(7));
    }
}
