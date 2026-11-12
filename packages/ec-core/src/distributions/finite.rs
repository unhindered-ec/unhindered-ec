use std::num::NonZeroUsize;

use rand::distr::slice::Choose;

/// A distribution which is finite, i.e. has a finite sample space
///
/// This can be useful, for example, when determining
/// the probability of choosing any particular element
/// from a uniform distribution.
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
    /// The size of this distribution's sample space.
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

#[cfg(test)]
mod tests {
    use std::num::NonZeroUsize;

    use rand::distr::slice::Choose;

    use super::Finite;

    #[test]
    fn choose_sample_space_size_is_the_number_of_choices() {
        let choose = Choose::new(&[1, 2, 3]).unwrap();
        assert_eq!(choose.sample_space_size(), NonZeroUsize::new(3).unwrap());
    }

    #[test]
    fn references_report_the_underlying_sample_space_size() {
        let choose = Choose::new(&[1, 2, 3, 4]).unwrap();
        let shared = &choose;
        assert_eq!(
            Finite::sample_space_size(&shared),
            NonZeroUsize::new(4).unwrap()
        );

        let mut choose = Choose::new(&[1, 2]).unwrap();
        let exclusive = &mut choose;
        assert_eq!(
            Finite::sample_space_size(&exclusive),
            NonZeroUsize::new(2).unwrap()
        );
    }
}
