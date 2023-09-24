pub trait IntoState<S> {
    fn into_state(self) -> S;
}

impl<'a, T, S> IntoState<&'a mut S> for &'a mut T {
    fn into_state(self) -> &'a mut S {
        &mut (*self).into_state()
    }
}

impl<'a, T, S> IntoState<&'a S> for &'a T {
    fn into_state(self) -> &'a S {
        &mut (*self).into_state()
    }
}
