#[derive(Debug)]
pub struct Individual<T> {
    pub genome: T,
    pub fitness: f64,
}

