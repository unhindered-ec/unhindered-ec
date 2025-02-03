use rand::Rng;

use super::Composable;
use crate::operator::Operator;

#[derive(Composable)]
pub struct Map<F> {
    f: F,
}

impl<F> Map<F> {
    pub const fn new(f: F) -> Self {
        Self { f }
    }
}

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
#[error("Error while applying passed operator on the {1}-th element of the mapped iterable")]
pub struct MapError<T>(
    #[diagnostic_source]
    #[source]
    T,
    usize,
);

// I think I can parameterize over the 2 here to make this more general?
impl<F, Input> Operator<[Input; 2]> for Map<F>
where
    F: Operator<Input>,
{
    type Output = [F::Output; 2];
    type Error = MapError<F::Error>;

    fn apply<R: Rng + ?Sized>(
        &self,
        [x, y]: [Input; 2],
        rng: &mut R,
    ) -> Result<Self::Output, Self::Error> {
        let first_result = self.f.apply(x, rng).map_err(|e| MapError(e, 0))?;
        let second_result = self.f.apply(y, rng).map_err(|e| MapError(e, 1))?;
        Ok([first_result, second_result])
    }
}

impl<F, Input> Operator<(Input, Input)> for Map<F>
where
    F: Operator<Input>,
{
    type Output = (F::Output, F::Output);
    type Error = MapError<F::Error>;

    fn apply<R: Rng + ?Sized>(
        &self,
        (x, y): (Input, Input),
        rng: &mut R,
    ) -> Result<Self::Output, Self::Error> {
        let first_result = self.f.apply(x, rng).map_err(|e| MapError(e, 0))?;
        let second_result = self.f.apply(y, rng).map_err(|e| MapError(e, 1))?;
        Ok((first_result, second_result))
    }
}

impl<F, Input> Operator<Vec<Input>> for Map<F>
where
    F: Operator<Input>,
{
    type Output = Vec<F::Output>;
    type Error = MapError<F::Error>;

    fn apply<R: Rng + ?Sized>(
        &self,
        input: Vec<Input>,
        rng: &mut R,
    ) -> Result<Self::Output, Self::Error> {
        input
            .into_iter()
            .enumerate()
            .map(|(i, x)| self.f.apply(x, rng).map_err(|e| MapError(e, i)))
            .collect()
    }
}
