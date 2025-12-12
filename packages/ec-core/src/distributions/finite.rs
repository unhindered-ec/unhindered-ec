use std::num::NonZeroUsize;

use rand::distr::slice::Choose;

/// A distribution which is finite, i.e. has a finite sample space
///
/// This can be useful to for example determine the probability of elements in a
/// uniform distribution.
///
/// # Example
/// ```
/// use std::num::NonZero;
///
/// use ec_core::distributions::finite::Finite;
/// use rand::distr::slice::Choose;
///
/// assert_eq!(
///     Finite::sample_space_size(&Choose::new(&[1, 2, 3]).unwrap()),
///     NonZero::new(3).unwrap()
/// );
/// ```
pub trait Finite {
    /// The size of this distributions sample space.
    ///
    /// # Example
    /// ```
    /// use std::num::NonZero;
    ///
    /// use ec_core::distributions::finite::Finite;
    /// use rand::distr::slice::Choose;
    ///
    /// assert_eq!(
    ///     Finite::sample_space_size(&Choose::new(&[1, 2, 3]).unwrap()),
    ///     NonZero::new(3).unwrap()
    /// );
    /// ```
    fn sample_space_size(&self) -> NonZeroUsize;
}

static_assertions::assert_obj_safe!(Finite);

impl<T> Finite for Choose<'_, T> {
    /// Sample space size of this Choose distribution. This corresponds to the
    /// [`Choose::num_choices`].
    ///
    ///  # Example
    /// ```
    /// use std::num::NonZero;
    ///
    /// use ec_core::distributions::finite::Finite;
    /// use rand::distr::slice::Choose;
    ///
    /// assert_eq!(
    ///     Finite::sample_space_size(&Choose::new(&[1, 2, 3]).unwrap()),
    ///     NonZero::new(3).unwrap()
    /// );
    /// ```
    fn sample_space_size(&self) -> NonZeroUsize {
        self.num_choices()
    }
}

impl<T> Finite for &T
where
    T: Finite + ?Sized,
{
    fn sample_space_size(&self) -> NonZeroUsize {
        (**self).sample_space_size()
    }
}

impl<T> Finite for &mut T
where
    T: Finite + ?Sized,
{
    fn sample_space_size(&self) -> NonZeroUsize {
        (**self).sample_space_size()
    }
}
