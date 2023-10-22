use anyhow::Result;
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

    fn apply(&self, (): (), _: &mut ThreadRng) -> Result<Self::Output> {
        Ok(self.value.clone())
    }
}
impl<T> Composable for Identity<T> {}
