#[derive(Debug)]
pub struct Individual<T> {
    pub genome: T,
    pub fitness: f64,
}

impl<T> Individual<T> {
    pub fn new(mut make_genome: impl FnMut() -> T, compute_fitness: impl Fn(&T) -> f64) -> Individual<T>
    {
        let genome = make_genome();
        let fitness = compute_fitness(&genome);
        Individual {
            genome,
            fitness,
        }
    }
}
