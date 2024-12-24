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

#[derive(Debug)]
pub struct MapError<T>(T, usize);

impl<T> std::fmt::Display for MapError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Error while applying passed operator on the {}-th element of the mapped iterable",
            self.1
        )
    }
}

impl<T> std::error::Error for MapError<T>
where
    T: std::error::Error + 'static,
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.0)
    }
}

impl<T> miette::Diagnostic for MapError<T>
where
    T: miette::Diagnostic + 'static,
{
    fn diagnostic_source(&self) -> Option<&dyn miette::Diagnostic> {
        Some(&self.0)
    }
}

// I think I can parameterize over the 2 here to make this more general?
impl<F, Input> Operator<[Input; 2]> for Map<F>
where
    F: Operator<Input>,
{
    type Output = [F::Output; 2];
    type Error = MapError<F::Error>;

    fn apply(
        &self,
        [x, y]: [Input; 2],
        rng: &mut rand::rngs::ThreadRng,
    ) -> Result<Self::Output, Self::Error> {
        let first_result = self.f.apply(x, rng).map_err(|e| MapError(e, 0))?;
        let second_result = self.f.apply(y, rng).map_err(|e| MapError(e, 0))?;
        Ok([first_result, second_result])
    }
}

impl<F, Input> Operator<(Input, Input)> for Map<F>
where
    F: Operator<Input>,
{
    type Output = (F::Output, F::Output);
    type Error = MapError<F::Error>;

    fn apply(
        &self,
        (x, y): (Input, Input),
        rng: &mut rand::rngs::ThreadRng,
    ) -> Result<Self::Output, Self::Error> {
        let first_result = self.f.apply(x, rng).map_err(|e| MapError(e, 0))?;
        let second_result = self.f.apply(y, rng).map_err(|e| MapError(e, 0))?;
        Ok((first_result, second_result))
    }
}

impl<F, Input> Operator<Vec<Input>> for Map<F>
where
    F: Operator<Input>,
{
    type Output = Vec<F::Output>;
    type Error = MapError<F::Error>;

    fn apply(
        &self,
        input: Vec<Input>,
        rng: &mut rand::rngs::ThreadRng,
    ) -> Result<Self::Output, Self::Error> {
        input
            .into_iter()
            .enumerate()
            .map(|(i, x)| self.f.apply(x, rng).map_err(|e| MapError(e, i)))
            .collect()
    }
}

// TODO: Impl `Map` over iterators.

impl<F> Composable for Map<F> {}
