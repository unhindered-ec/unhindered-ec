use rand::{
    distr::slice::{Choose, Empty},
    prelude::Distribution,
};

use super::wrappers::{choose_cloning::ChooseCloning, choose_cloning_owning::ChooseCloningOwning};

/// Conversion into a [`Distribution`].
///
/// # Notable implementations
/// - `[T; N]`
/// - `&'a [T]`
/// - `&'a [T; N]`
///
/// These implementations allow you to easily convert a list of elements into a
/// [`Distribution`] you can sample one from. Usually, these implementations are
/// meant to turn types into Uniform distributions.
///
/// # Example
/// ```
/// # use ec_core::distributions::conversion::IntoDistribution;
/// # use rand::{rng, distr::{Distribution, slice::Empty}};
/// #
/// # fn main() -> Result<(),Empty> {
/// let my_collection = [1i32, 2, 3, 4, 5];
/// let my_distribution = my_collection.into_distribution()?;
///
/// let my_sample = my_distribution.sample(&mut rng());
/// # let _ = my_sample;
/// # Ok(())
/// # }
/// ```
pub trait IntoDistribution<Element> {
    type Distribution: Distribution<Element>;
    type Error;

    /// Converts self into a [`Distribution`] by moving.
    ///
    /// This is usually implemented for collections and then allows uniformly
    /// sampling from them.
    ///
    /// # Example
    /// ```
    /// # use ec_core::distributions::conversion::IntoDistribution;
    /// # use rand::{rng, distr::{Distribution, slice::Empty}};
    /// #
    /// # fn main() -> Result<(),Empty> {
    /// let my_collection = [1i32, 2, 3, 4, 5];
    /// let my_distribution = my_collection.into_distribution()?;
    ///
    /// let my_sample = my_distribution.sample(&mut rng());
    /// # let _ = my_sample;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    /// - [`Self::Error`] if the creation of the distribution fails.
    fn into_distribution(self) -> Result<Self::Distribution, Self::Error>;
}

static_assertions::assert_obj_safe!(IntoDistribution<(), Distribution = (), Error = ()>);

/// Helper trait for conversion into a [`Distribution`].
///
/// This helper trait is not meant to be implemented directly, implement
/// [`IntoDistribution`] for `&T` instead.
///
/// Since this trait's conversion method [`ToDistribution::to_distribution`]
/// takes self by reference instead of moving self, this can be used to avoid
/// moving `Self` in a method chain.
///
/// # Example
/// ```
/// # use ec_core::distributions::conversion::ToDistribution;
/// # use rand::{rng, distr::{Distribution, slice::Empty}};
/// #
/// # fn main() -> Result<(),Empty> {
/// let my_collection = [1, 2, 3, 4, 5];
/// let my_distribution = ToDistribution::<&i32>::to_distribution(&my_collection)?;
///
/// let my_sample: &i32 = my_distribution.sample(&mut rng());
/// # let _ = my_sample;
///
/// // not moved, still usable
/// let first_element = my_collection.first();
/// # let _ = first_element;
/// # Ok(())
/// # }
/// ```
pub trait ToDistribution<'a, Element> {
    type Distribution: Distribution<Element>;
    type Error;

    /// Creates a [`Distribution`] referencing self.
    ///
    /// This method is blanket-implemented for all `&T: IntoDistribution`.
    ///
    /// # Example
    /// ```
    /// # use ec_core::distributions::conversion::ToDistribution;
    /// # use rand::{rng, distr::{Distribution, slice::Empty}};
    /// #
    /// # fn main() -> Result<(),Empty> {
    /// let my_collection = [1, 2, 3, 4, 5];
    /// let my_distribution = ToDistribution::<&i32>::to_distribution(&my_collection)?;
    ///
    /// let my_sample: &i32 = my_distribution.sample(&mut rng());
    /// # let _ = my_sample;
    ///
    /// // not moved, still usable
    /// let first_element = my_collection.first();
    /// # let _ = first_element;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    /// - [`Self::Error`] if the creation of the distribution fails.
    fn to_distribution(&'a self) -> Result<Self::Distribution, Self::Error>;
}

static_assertions::assert_obj_safe!(ToDistribution<(), Distribution = (), Error = ()>);

impl<'a, T: 'a, E> ToDistribution<'a, E> for T
where
    &'a T: IntoDistribution<E>,
{
    type Distribution = <&'a T as IntoDistribution<E>>::Distribution;

    type Error = <&'a T as IntoDistribution<E>>::Error;

    fn to_distribution(&'a self) -> Result<Self::Distribution, Self::Error> {
        (*self).into_distribution()
    }
}

impl<U> IntoDistribution<U> for Vec<U>
where
    U: Clone,
{
    type Distribution = ChooseCloningOwning<Self, U>;
    type Error = Empty;

    /// Creates a [`Distribution`] which uniformly samples a single element from
    /// a [`Vec<T>`](Vec) and clones it.
    ///
    /// # Example
    /// ```
    /// # use ec_core::distributions::conversion::IntoDistribution;
    /// # use rand::{rng, distr::{Distribution, slice::Empty}};
    /// #
    /// # fn main() -> Result<(),Empty> {
    /// let my_vec = vec![0i32, 1, 2];
    /// let my_distr = my_vec.into_distribution()?;
    ///
    /// let my_value: i32 = my_distr.sample(&mut rng());
    /// # let _ = my_value;
    /// # Ok(())
    /// # }
    /// ```
    fn into_distribution(self) -> Result<Self::Distribution, Self::Error> {
        ChooseCloningOwning::new(self)
    }
}

impl<'a, U> IntoDistribution<&'a U> for &'a Vec<U> {
    type Distribution = Choose<'a, U>;

    type Error = Empty;

    /// Creates a [`Distribution`] which uniformly samples a single element from
    /// a [`&Vec<T>`](Vec) and returns a reference to it.
    ///
    /// # Example
    /// ```
    /// # use ec_core::distributions::conversion::IntoDistribution;
    /// # use rand::{rng, distr::{Distribution, slice::Empty}};
    /// #
    /// # fn main() -> Result<(),Empty> {
    /// let my_vec = vec![0i32, 1, 2];
    /// let my_distr = IntoDistribution::<&i32>::into_distribution(&my_vec)?;
    ///
    /// let my_value: &i32 = my_distr.sample(&mut rng());
    /// # let _ = my_value;
    /// # Ok(())
    /// # }
    /// ```
    fn into_distribution(self) -> Result<Self::Distribution, Self::Error> {
        Choose::new(self).map_err(|_: Empty| Empty)
    }
}

impl<'a, U> IntoDistribution<U> for &'a Vec<U>
where
    U: Clone,
{
    type Distribution = ChooseCloning<'a, U>;

    type Error = Empty;

    /// Creates a [`Distribution`] which uniformly samples a single element from
    /// a [`&Vec<T>`](Vec) and clones it.
    ///
    /// # Example
    /// ```
    /// # use ec_core::distributions::conversion::IntoDistribution;
    /// # use rand::{rng, distr::{Distribution, slice::Empty}};
    /// #
    /// # fn main() -> Result<(),Empty> {
    /// let my_vec = vec![0i32, 1, 2];
    /// let my_distr = IntoDistribution::<i32>::into_distribution(&my_vec)?;
    ///
    /// let my_value: i32 = my_distr.sample(&mut rng());
    /// # let _ = my_value;
    /// # Ok(())
    /// # }
    /// ```
    fn into_distribution(self) -> Result<Self::Distribution, Self::Error> {
        ChooseCloning::new(self)
    }
}

impl<U, const N: usize> IntoDistribution<U> for [U; N]
where
    U: Clone,
{
    type Distribution = ChooseCloningOwning<Self, U>;
    type Error = Empty;

    /// Creates a [`Distribution`] which uniformly samples a single element from
    /// a [`[U; N]`](primitive@array) and clones it.
    ///
    /// # Example
    /// ```
    /// # use ec_core::distributions::conversion::IntoDistribution;
    /// # use rand::{rng, distr::{Distribution, slice::Empty}};
    /// #
    /// # fn main() -> Result<(),Empty> {
    /// let my_array: [i32; 3] = [0, 1, 2];
    /// let my_distr = my_array.into_distribution()?;
    ///
    /// let my_value: i32 = my_distr.sample(&mut rng());
    /// # let _ = my_value;
    /// # Ok(())
    /// # }
    /// ```
    fn into_distribution(self) -> Result<Self::Distribution, Self::Error> {
        ChooseCloningOwning::new(self)
    }
}

impl<'a, U, const N: usize> IntoDistribution<&'a U> for &'a [U; N] {
    type Distribution = Choose<'a, U>;

    type Error = Empty;

    /// Creates a [`Distribution`] which uniformly samples a single element from
    /// a [`&[U; N]`](primitive@array) and returns a reference to it.
    ///
    /// # Example
    /// ```
    /// # use ec_core::distributions::conversion::IntoDistribution;
    /// # use rand::{rng, distr::{Distribution, slice::Empty}};
    /// #
    /// # fn main() -> Result<(),Empty> {
    /// let my_array: [i32; 3] = [0, 1, 2];
    /// let my_distr = IntoDistribution::<&i32>::into_distribution(&my_array)?;
    ///
    /// let my_value: &i32 = my_distr.sample(&mut rng());
    /// # let _ = my_value;
    /// # Ok(())
    /// # }
    /// ```
    fn into_distribution(self) -> Result<Self::Distribution, Self::Error> {
        Choose::new(self).map_err(|_: Empty| Empty)
    }
}

impl<'a, U, const N: usize> IntoDistribution<U> for &'a [U; N]
where
    U: Clone,
{
    type Distribution = ChooseCloning<'a, U>;

    type Error = Empty;

    /// Creates a [`Distribution`] which uniformly samples a single element from
    /// a [`&[T; N]`](primitive@array) and clones it.
    ///
    /// # Example
    /// ```
    /// # use ec_core::distributions::conversion::IntoDistribution;
    /// # use rand::{rng, distr::{Distribution, slice::Empty}};
    /// #
    /// # fn main() -> Result<(),Empty> {
    /// let my_array: [i32; 3] = [0, 1, 2];
    /// let my_distr = IntoDistribution::<i32>::into_distribution(&my_array)?;
    ///
    /// let my_value: i32 = my_distr.sample(&mut rng());
    /// # let _ = my_value;
    /// # Ok(())
    /// # }
    /// ```
    fn into_distribution(self) -> Result<Self::Distribution, Self::Error> {
        ChooseCloning::new(self)
    }
}

impl<'a, T> IntoDistribution<&'a T> for &'a [T] {
    type Distribution = Choose<'a, T>;

    type Error = Empty;

    /// Creates a [`Distribution`] which uniformly samples a single element from
    /// a [`&[U]`](primitive@slice) and returns a reference to it.
    ///
    /// # Example
    /// ```
    /// # use ec_core::distributions::conversion::IntoDistribution;
    /// # use rand::{rng, distr::{Distribution, slice::Empty}};
    /// #
    /// # fn main() -> Result<(),Empty> {
    /// let my_slice: &[i32] = &[0, 1, 2];
    /// let my_distr = IntoDistribution::<&i32>::into_distribution(my_slice)?;
    ///
    /// let my_value: &i32 = my_distr.sample(&mut rng());
    /// # let _ = my_value;
    /// # Ok(())
    /// # }
    /// ```
    fn into_distribution(self) -> Result<Self::Distribution, Self::Error> {
        Choose::new(self).map_err(|_| Empty)
    }
}

impl<'a, T> IntoDistribution<T> for &'a [T]
where
    T: Clone,
{
    type Distribution = ChooseCloning<'a, T>;

    type Error = Empty;

    /// Creates a [`Distribution`] which uniformly samples a single element from
    /// a [`&[T]`](primitive@slice) and clones it.
    ///
    /// # Example
    /// ```
    /// # use ec_core::distributions::conversion::IntoDistribution;
    /// # use rand::{rng, distr::{Distribution, slice::Empty}};
    /// #
    /// # fn main() -> Result<(),Empty> {
    /// let my_slice: &[i32] = &[0, 1, 2];
    /// let my_distr = IntoDistribution::<i32>::into_distribution(my_slice)?;
    ///
    /// let my_value: i32 = my_distr.sample(&mut rng());
    /// # let _ = my_value;
    /// # Ok(())
    /// # }
    /// ```
    fn into_distribution(self) -> Result<Self::Distribution, Self::Error> {
        ChooseCloning::new(self)
    }
}
