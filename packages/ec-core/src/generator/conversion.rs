use rand::{distributions::Slice, prelude::Distribution};

use super::wrappers::{
    owned::OneOfCloning,
    slice_cloning::{EmptySlice, SliceCloning},
};

pub trait IntoDistribution<Element> {
    type Distribution: Distribution<Element>;
    type Error;

    fn into_distribution(self) -> Result<Self::Distribution, Self::Error>;
}

pub trait AsDistribution<'a, Element> {
    type Distribution: Distribution<Element>;
    type Error;

    fn as_distribution(&'a self) -> Result<Self::Distribution, Self::Error>;
}

impl<U> IntoDistribution<U> for Vec<U>
where
    U: Clone,
{
    type Distribution = OneOfCloning<Self>;
    type Error = EmptySlice;

    fn into_distribution(self) -> Result<Self::Distribution, Self::Error> {
        OneOfCloning::new(self)
    }
}

impl<'a, U> IntoDistribution<&'a U> for &'a Vec<U> {
    type Distribution = Slice<'a, U>;

    type Error = EmptySlice;

    fn into_distribution(self) -> Result<Self::Distribution, Self::Error> {
        self.as_distribution()
    }
}

impl<'a, U> IntoDistribution<U> for &'a Vec<U>
where
    U: Clone,
{
    type Distribution = SliceCloning<'a, U>;

    type Error = EmptySlice;

    fn into_distribution(self) -> Result<Self::Distribution, Self::Error> {
        AsDistribution::<U>::as_distribution(self)
    }
}

impl<'a, U> AsDistribution<'a, U> for Vec<U>
where
    U: 'a + Clone,
{
    type Distribution = SliceCloning<'a, U>;
    type Error = EmptySlice;

    fn as_distribution(&'a self) -> Result<Self::Distribution, Self::Error> {
        SliceCloning::new(self)
    }
}

impl<'a, U> AsDistribution<'a, &'a U> for Vec<U>
where
    U: 'a,
{
    type Distribution = Slice<'a, U>;
    type Error = EmptySlice;

    fn as_distribution(&'a self) -> Result<Self::Distribution, Self::Error> {
        Slice::new(self).map_err(|_| EmptySlice)
    }
}

impl<U, const N: usize> IntoDistribution<U> for [U; N]
where
    U: Clone,
{
    type Distribution = OneOfCloning<Self>;
    type Error = EmptySlice;

    fn into_distribution(self) -> Result<Self::Distribution, Self::Error> {
        OneOfCloning::new(self)
    }
}

impl<'a, U, const N: usize> IntoDistribution<&'a U> for &'a [U; N] {
    type Distribution = Slice<'a, U>;

    type Error = EmptySlice;

    fn into_distribution(self) -> Result<Self::Distribution, Self::Error> {
        self.as_distribution()
    }
}

impl<'a, U, const N: usize> IntoDistribution<U> for &'a [U; N]
where
    U: Clone,
{
    type Distribution = SliceCloning<'a, U>;

    type Error = EmptySlice;

    fn into_distribution(self) -> Result<Self::Distribution, Self::Error> {
        AsDistribution::<U>::as_distribution(self)
    }
}

impl<'a, U, const N: usize> AsDistribution<'a, U> for [U; N]
where
    U: 'a + Clone,
{
    type Distribution = SliceCloning<'a, U>;
    type Error = EmptySlice;

    fn as_distribution(&'a self) -> Result<Self::Distribution, Self::Error> {
        SliceCloning::new(self)
    }
}

impl<'a, U, const N: usize> AsDistribution<'a, &'a U> for [U; N]
where
    U: 'a,
{
    type Distribution = Slice<'a, U>;
    type Error = EmptySlice;

    fn as_distribution(&'a self) -> Result<Self::Distribution, Self::Error> {
        Slice::new(self).map_err(|_| EmptySlice)
    }
}

impl<'a, T> IntoDistribution<&'a T> for &'a [T] {
    type Distribution = Slice<'a, T>;

    type Error = EmptySlice;

    fn into_distribution(self) -> Result<Self::Distribution, Self::Error> {
        Slice::new(self).map_err(|_| EmptySlice)
    }
}

impl<'a, T> IntoDistribution<T> for &'a [T]
where
    T: Clone,
{
    type Distribution = SliceCloning<'a, T>;

    type Error = EmptySlice;

    fn into_distribution(self) -> Result<Self::Distribution, Self::Error> {
        SliceCloning::new(self)
    }
}

impl<'a, T> AsDistribution<'a, &'a T> for [T] {
    type Distribution = Slice<'a, T>;

    type Error = EmptySlice;

    fn as_distribution(&'a self) -> Result<Self::Distribution, Self::Error> {
        Slice::new(self).map_err(|_| EmptySlice)
    }
}

impl<'a, T> AsDistribution<'a, T> for [T]
where
    T: Clone + 'a,
{
    type Distribution = SliceCloning<'a, T>;

    type Error = EmptySlice;

    fn as_distribution(&'a self) -> Result<Self::Distribution, Self::Error> {
        SliceCloning::new(self)
    }
}
