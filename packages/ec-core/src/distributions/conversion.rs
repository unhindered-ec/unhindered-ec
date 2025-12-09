use rand::{
    distr::slice::{Choose, Empty},
    prelude::Distribution,
};

use super::wrappers::{choose_cloning::ChooseCloning, choose_cloning_owning::ChooseCloningOwning};

pub trait IntoDistribution<Element> {
    type Distribution: Distribution<Element>;
    type Error;

    /// Converts some type into a Distribution (fallible)
    ///
    /// # Errors
    /// If the conversion to a distribution failed, then [`Self::Error`] is
    /// returned
    fn into_distribution(self) -> Result<Self::Distribution, Self::Error>;
}

static_assertions::assert_obj_safe!(IntoDistribution<(), Distribution = (), Error = ()>);

pub trait ToDistribution<'a, Element> {
    type Distribution: Distribution<Element>;
    type Error;

    /// Creates a Distribution from a reference to self (fallible)
    ///
    /// # Errors
    /// If the creation of a distribution failed, then [`Self::Error`] is
    /// returned
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

    fn into_distribution(self) -> Result<Self::Distribution, Self::Error> {
        ChooseCloningOwning::new(self)
    }
}

impl<'a, U> IntoDistribution<&'a U> for &'a Vec<U> {
    type Distribution = Choose<'a, U>;

    type Error = Empty;

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

    fn into_distribution(self) -> Result<Self::Distribution, Self::Error> {
        ChooseCloningOwning::new(self)
    }
}

impl<'a, U, const N: usize> IntoDistribution<&'a U> for &'a [U; N] {
    type Distribution = Choose<'a, U>;

    type Error = Empty;

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

    fn into_distribution(self) -> Result<Self::Distribution, Self::Error> {
        ChooseCloning::new(self)
    }
}

impl<'a, T> IntoDistribution<&'a T> for &'a [T] {
    type Distribution = Choose<'a, T>;

    type Error = Empty;

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

    fn into_distribution(self) -> Result<Self::Distribution, Self::Error> {
        ChooseCloning::new(self)
    }
}
