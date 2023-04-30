use rand::{rngs::ThreadRng, Rng};

pub trait Generator<T, Context> {
    fn generate(&mut self, context: &Context) -> T;
}

impl Generator<bool, f64> for ThreadRng {
    fn generate(&mut self, probability: &f64) -> bool {
        self.gen_bool(*probability)
    }
}
