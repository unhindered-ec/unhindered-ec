pub trait Genome {
    type Gene;
}
static_assertions::assert_obj_safe!(Genome<Gene = ()>);
