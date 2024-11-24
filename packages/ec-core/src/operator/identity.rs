use std::convert::Infallible;

use rand::rngs::ThreadRng;

use super::{Composable, Operator};

pub struct Identity<T> {
    value: T,
}

impl<T> Identity<T> {
    pub const fn new(value: T) -> Self {
        Self { value }
    }
}

impl<T> Operator<()> for Identity<T>
where
    T: Clone,
{
    type Output = T;
    type Error = Infallible;

    fn apply(&self, (): (), _: &mut ThreadRng) -> Result<Self::Output, Self::Error> {
        Ok(self.value.clone())
    }
}
impl<T> Composable for Identity<T> {}
