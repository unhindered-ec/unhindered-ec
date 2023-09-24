pub trait State {
    type State;
    fn state(&self) -> &Self::State;
}

pub trait StateMut: State {
    fn state_mut(&mut self) -> &mut Self::State;
}

impl<'a, T> State for &'a mut T
where
    T: State,
{
    type State = T::State;

    fn state(&self) -> &Self::State {
        (**self).state()
    }
}

impl<'a, T> StateMut for &'a mut T
where
    T: StateMut,
{
    fn state_mut(&mut self) -> &mut Self::State {
        (**self).state_mut()
    }
}

impl<'a, T> State for &'a T
where
    T: State,
{
    type State = T::State;

    fn state(&self) -> &Self::State {
        (**self).state()
    }
}
