pub mod ec;
pub mod scorer;

pub trait Individual {
    type Genome;
    type TestResults;

    fn genome(&self) -> &Self::Genome;
    fn test_results(&self) -> &Self::TestResults;
}

static_assertions::assert_obj_safe!(Individual<Genome = (), TestResults = ()>);
