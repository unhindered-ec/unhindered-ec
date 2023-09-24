pub trait IntoState<S> {
    fn into_state(self) -> S;
}

impl<S> IntoState<S> for S {
    fn into_state(self) -> S {
        self
    }
}
