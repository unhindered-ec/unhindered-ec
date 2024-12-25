pub trait IntoState<S> {
    fn into_state(self) -> S;
}

static_assertions::assert_obj_safe!(IntoState<()>);
