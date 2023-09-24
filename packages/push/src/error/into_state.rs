pub trait IntoState<S> {
    fn into_state(self) -> S;
}
