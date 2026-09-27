pub trait AsState<S> {
    /// Borrows the state embedded in the error
    fn as_state(&self) -> &S;
}

static_assertions::assert_obj_safe!(AsState<()>);
