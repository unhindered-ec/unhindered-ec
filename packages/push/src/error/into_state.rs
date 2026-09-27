pub trait IntoState<S> {
    /// Convert the error into its embedded state.
    fn into_state(self) -> S;

    /// Borrows the state embedded in the error
    fn as_state(&self) -> &S;
}

static_assertions::assert_obj_safe!(IntoState<()>);
