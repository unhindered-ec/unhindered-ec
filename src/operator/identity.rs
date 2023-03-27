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

    fn apply(&self, _: (), _: &mut ThreadRng) -> Self::Output {
        self.value.clone()
    }
}
impl<T> Composable for Identity<T> {}
