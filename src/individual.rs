use rand::rngs::ThreadRng;

#[derive(Debug)]
pub struct Individual<T> {
    pub genome: T,
    pub fitness: i64,
}

impl<T> Individual<T> {
    pub fn new(
            mut make_genome: impl FnMut(&mut ThreadRng) -> T, 
            compute_fitness: impl Fn(&T) -> i64,
            rng: &mut ThreadRng) 
        -> Individual<T>
    {
        let genome = make_genome(rng);
        let fitness = compute_fitness(&genome);
        Individual {
            genome,
            fitness,
        }
    }
}
