#![expect(
    clippy::arithmetic_side_effects,
    reason = "The tradeoff safety <> ease of writing arguably lies on the ease of writing side \
              for test code."
)]
use std::convert::Infallible;

use rand::Rng;

use super::{super::Operator, Composable};

#[derive(Composable)]
pub(super) struct Increment;

impl Operator<i32> for Increment {
    type Output = i32;
    type Error = Infallible;

    fn apply<R: Rng + ?Sized>(&self, input: i32, _: &mut R) -> Result<Self::Output, Self::Error> {
        Ok(input + 1)
    }
}

#[derive(Composable)]
pub(super) struct Double;

impl Operator<i32> for Double {
    type Output = i32;
    type Error = Infallible;

    fn apply<R: Rng + ?Sized>(&self, input: i32, _: &mut R) -> Result<Self::Output, Self::Error> {
        Ok(input * 2)
    }
}
