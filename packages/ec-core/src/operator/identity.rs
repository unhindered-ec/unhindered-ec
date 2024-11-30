use std::convert::Infallible;

use rand::rngs::ThreadRng;

use super::{Composable, Operator};

/// An [`Operator`] that always returns it's input value.
///
/// # See also
///
/// See [`Operator`] and [`Composable`].
///
/// # Examples
///
/// ```
/// # use ec_core::operator::{Operator, identity::Identity};
/// # use rand::thread_rng;
/// #
/// let mut rng = thread_rng();
/// // This will always return the value that is passed in.
/// let identity = Identity;
///
/// assert_eq!(identity.apply(3, &mut rng).unwrap(), 3);
/// assert_eq!(identity.apply("string", &mut rng).unwrap(), "string");
/// assert_eq!(identity.apply(Some(7), &mut rng).unwrap(), Some(7));
/// ```
pub struct Identity;

impl<T> Operator<T> for Identity {
    type Output = T;
    type Error = Infallible;

    fn apply(&self, input: T, _: &mut ThreadRng) -> Result<Self::Output, Self::Error> {
        Ok(input)
    }
}

impl Composable for Identity {}

#[cfg(test)]
mod tests {
    use rand::thread_rng;

    use crate::operator::{Operator, identity::Identity};

    #[test]
    fn is_constant() {
        let mut rng = thread_rng();
        // This should always return 5 regardless of the input.
        let identity = Identity;

        assert_eq!(identity.apply(3, &mut rng).unwrap(), 3);
        assert_eq!(identity.apply("string", &mut rng).unwrap(), "string");
        assert_eq!(identity.apply(Some(7), &mut rng).unwrap(), Some(7));
    }
}
