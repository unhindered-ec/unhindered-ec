/// Types with an associated weight
pub trait WithWeight {
    fn weight(&self) -> u32;
}
