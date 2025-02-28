use rand::{distr::slice::Choose, prelude::Distribution};

use super::wrappers::{
    choose_cloning::{ChooseCloning, EmptySlice},
    owned::OneOfCloning,
};

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

impl<U> IntoDistribution<U> for Vec<U>
where
    U: Clone,
{
    type Distribution = OneOfCloning<Self, U>;
    type Error = EmptySlice;

    fn into_distribution(self) -> Result<Self::Distribution, Self::Error> {
        OneOfCloning::new(self)
    }
}

impl<'a, U> IntoDistribution<&'a U> for &'a Vec<U> {
    type Distribution = Choose<'a, U>;

    type Error = EmptySlice;

    fn into_distribution(self) -> Result<Self::Distribution, Self::Error> {
        self.to_distribution()
    }
}

impl<'a, U> IntoDistribution<U> for &'a Vec<U>
where
    U: Clone,
{
    type Distribution = ChooseCloning<'a, U>;

    type Error = EmptySlice;

    fn into_distribution(self) -> Result<Self::Distribution, Self::Error> {
        ToDistribution::<U>::to_distribution(self)
    }
}

impl<'a, U> ToDistribution<'a, U> for Vec<U>
where
    U: 'a + Clone,
{
    type Distribution = ChooseCloning<'a, U>;
    type Error = EmptySlice;

    fn to_distribution(&'a self) -> Result<Self::Distribution, Self::Error> {
        ChooseCloning::new(self)
    }
}

impl<'a, U> ToDistribution<'a, &'a U> for Vec<U>
where
    U: 'a,
{
    type Distribution = Choose<'a, U>;
    type Error = EmptySlice;

    fn to_distribution(&'a self) -> Result<Self::Distribution, Self::Error> {
        Choose::new(self).map_err(|_| EmptySlice)
    }
}

impl<U, const N: usize> IntoDistribution<U> for [U; N]
where
    U: Clone,
{
    type Distribution = OneOfCloning<Self, U>;
    type Error = EmptySlice;

    fn into_distribution(self) -> Result<Self::Distribution, Self::Error> {
        OneOfCloning::new(self)
    }
}

impl<'a, U, const N: usize> IntoDistribution<&'a U> for &'a [U; N] {
    type Distribution = Choose<'a, U>;

    type Error = EmptySlice;

    fn into_distribution(self) -> Result<Self::Distribution, Self::Error> {
        self.to_distribution()
    }
}

impl<'a, U, const N: usize> IntoDistribution<U> for &'a [U; N]
where
    U: Clone,
{
    type Distribution = ChooseCloning<'a, U>;

    type Error = EmptySlice;

    fn into_distribution(self) -> Result<Self::Distribution, Self::Error> {
        ToDistribution::<U>::to_distribution(self)
    }
}

impl<'a, U, const N: usize> ToDistribution<'a, U> for [U; N]
where
    U: 'a + Clone,
{
    type Distribution = ChooseCloning<'a, U>;
    type Error = EmptySlice;

    fn to_distribution(&'a self) -> Result<Self::Distribution, Self::Error> {
        ChooseCloning::new(self)
    }
}

impl<'a, U, const N: usize> ToDistribution<'a, &'a U> for [U; N]
where
    U: 'a,
{
    type Distribution = Choose<'a, U>;
    type Error = EmptySlice;

    fn to_distribution(&'a self) -> Result<Self::Distribution, Self::Error> {
        Choose::new(self).map_err(|_| EmptySlice)
    }
}

impl<'a, T> IntoDistribution<&'a T> for &'a [T] {
    type Distribution = Choose<'a, T>;

    type Error = EmptySlice;

    fn into_distribution(self) -> Result<Self::Distribution, Self::Error> {
        Choose::new(self).map_err(|_| EmptySlice)
    }
}

impl<'a, T> IntoDistribution<T> for &'a [T]
where
    T: Clone,
{
    type Distribution = ChooseCloning<'a, T>;

    type Error = EmptySlice;

    fn into_distribution(self) -> Result<Self::Distribution, Self::Error> {
        ChooseCloning::new(self)
    }
}

impl<'a, T> ToDistribution<'a, &'a T> for [T] {
    type Distribution = Choose<'a, T>;

    type Error = EmptySlice;

    fn to_distribution(&'a self) -> Result<Self::Distribution, Self::Error> {
        Choose::new(self).map_err(|_| EmptySlice)
    }
}

impl<'a, T> ToDistribution<'a, T> for [T]
where
    T: Clone + 'a,
{
    type Distribution = ChooseCloning<'a, T>;

    type Error = EmptySlice;

    fn to_distribution(&'a self) -> Result<Self::Distribution, Self::Error> {
        ChooseCloning::new(self)
    }
}
