pub trait AccumulateStrategy<Item>
where
    Self: Sized,
{
    type Error;
    type State;

    fn initialize() -> Self::State;

    fn accululate_into<I>(state: &mut Self::State, iter: I) -> Result<(), Self::Error>
    where
        I: Iterator<Item = Item>;

    fn accumulate<I>(iter: I) -> Result<Self::State, Self::Error>
    where
        I: Iterator<Item = Item>,
    {
        let mut state = Self::initialize();
        Self::accululate_into(&mut state, iter)?;
        Ok(state)
    }
}
