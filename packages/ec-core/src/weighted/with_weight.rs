/// Types with an associated weight
pub trait WithWeight {
    fn weight(&self) -> u32;
}

static_assertions::assert_obj_safe!(WithWeight);
