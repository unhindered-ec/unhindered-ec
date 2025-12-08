pub trait Genome {
    /// Type of each individual gene in this genome
    type Gene;
}
static_assertions::assert_obj_safe!(Genome<Gene = ()>);

impl<T> Genome for Vec<T> {
    /// For linear genomes (`Vec<T>`) each genome is of the type `T`.
    type Gene = T;
}
