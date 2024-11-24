use std::convert::Infallible;

use rand::rngs::ThreadRng;

use super::{Composable, Operator};

pub struct Identity;

impl<T> Operator<T> for Identity {
    type Output = T;
    type Error = Infallible;

    fn apply(&self, input: T, _: &mut ThreadRng) -> Result<Self::Output, Self::Error> {
        Ok(input)
    }
}

impl Composable for Identity {}
