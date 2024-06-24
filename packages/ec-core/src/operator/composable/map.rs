use anyhow::Context;

use super::Composable;
use crate::operator::Operator;

pub struct Map<F> {
    f: F,
}

impl<F> Map<F> {
    pub const fn new(f: F) -> Self {
        Self { f }
    }
}

// I think I can parameterize over the 2 here to make this more general?
impl<F, Input> Operator<[Input; 2]> for Map<F>
where
    F: Operator<Input>,
    anyhow::Error: From<F::Error>,
{
    type Output = [F::Output; 2];
    type Error = anyhow::Error;

    fn apply(
        &self,
        [x, y]: [Input; 2],
        rng: &mut rand::rngs::ThreadRng,
    ) -> Result<Self::Output, Self::Error> {
        let first_result = self
            .f
            .apply(x, rng)
            .map_err(anyhow::Error::from)
            .context("Calling f with {x} in `Map` failed")?;
        let second_result = self
            .f
            .apply(y, rng)
            .map_err(anyhow::Error::from)
            .context("Calling f with {y} in `Map` failed")?;
        Ok([first_result, second_result])
    }
}

impl<F, Input> Operator<(Input, Input)> for Map<F>
where
    F: Operator<Input>,
    anyhow::Error: From<F::Error>,
{
    type Output = (F::Output, F::Output);
    type Error = anyhow::Error;

    fn apply(
        &self,
        (x, y): (Input, Input),
        rng: &mut rand::rngs::ThreadRng,
    ) -> Result<Self::Output, Self::Error> {
        let first_result = self
            .f
            .apply(x, rng)
            .map_err(anyhow::Error::from)
            .context("Calling f with {x} in `Map` failed")?;
        let second_result = self
            .f
            .apply(y, rng)
            .map_err(anyhow::Error::from)
            .context("Calling f with {y} in `Map` failed")?;
        Ok((first_result, second_result))
    }
}

impl<F, Input> Operator<Vec<Input>> for Map<F>
where
    F: Operator<Input>,
    anyhow::Error: From<F::Error>,
{
    type Output = Vec<F::Output>;
    type Error = anyhow::Error;

    fn apply(
        &self,
        input: Vec<Input>,
        rng: &mut rand::rngs::ThreadRng,
    ) -> Result<Self::Output, Self::Error> {
        input
            .into_iter()
            .map(|x| {
                self.f
                    .apply(x, rng)
                    .map_err(anyhow::Error::from)
                    .context("Applying f to {x} in `Map` failed")
            })
            .collect()
    }
}

// TODO: Impl `Map` over iterators.

impl<F> Composable for Map<F> {}
