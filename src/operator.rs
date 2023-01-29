// The Operator trait would have a single method that takes an input,
// population and rng.

use rand::rngs::ThreadRng;

use crate::population::Population;

// G is the genome type
trait Operator<Input, P> {
    type Output;

    fn apply(&self, input: Input, p: &P, rng: &mut ThreadRng) -> Self::Output;
}

// Essentially our function composition; takes two operators and
// it's `apply` method performs the first and sends that output as
// the input to the second.
// @esitsu: So Then<A, B> is an Operator where A is Operator and B is Operator<A::Output>
pub struct Then<A, B> {
    first: A,
    second: B,
}

impl<A, B> Then<A, B> {
    fn new(first: A, second: B) -> Self {
        Then { first, second }
    }
}

impl<A, B, Input, P> Operator<Input, P> for Then<A, B>
where
    A: Operator<Input, P>,
    B: Operator<A::Output, P>,
{
    type Output = B::Output;

    fn apply(&self, input: Input, p: &P, rng: &mut ThreadRng) -> Self::Output {
        self.second.apply(self.first.apply(input, p, rng), p, rng)
    }
}

pub struct Mutate<M> {
    mutator: M,
}

impl<M> Mutate<M> {
    fn new(mutator: M) -> Self {
        Self { mutator }
    }
}

// -sel&clone-> a -mut-> b -mut-> c -mut-> d

pub trait Mutator<Genome> {
    // This is "my" way, but may involve making more Genomes in long pipelines.
    fn mutate(input: &Genome, rng: &mut ThreadRng) -> Genome;

    // The next two are from @esitsu and would involve less copying,
    // but require that the initial select.apply() step clone the genome so that
    // subsequent steps in the pipeline can mutate it safely.
    //
    // However…
    // I just thought of a possible problem with passing &mut Genome everywhere. 
    // Imagine I have a single "parent" genome g and I want to mutate it twice
    // and then recombine the resulting genomes to get the output. Mutating g the
    // first time will change g if we pass it as &mut, so the second mutation will
    // be a mutation of the mutated version of g and overwrite the initial mutation
    // of g. That's gonna be a problem, right?
    fn mutate0(input: Genome, rng: &mut ThreadRng) -> Genome {
        input
    }
    fn mutate1(input: &mut Genome, rng: &mut ThreadRng);
}

impl<Input, P, M> Operator<Input, P> for Mutate<M>
where
    M: Mutator,
{
    type Output = Input;

    fn apply(&self, input: Input, _: &P, rng: &mut ThreadRng) -> Self::Output {
        self.mutator.mutate(input, &mut rng)
    }
}

pub struct Select<S> {
    selector: S,
}

impl<S> Select<S> {
    fn new(selector: S) -> Self {
        Self { selector }
    }
}

pub trait Compose: Sized {
    fn then_mutate<M>(self, mutator: M) -> Then<Self, Mutate<M>> {
        Then::new(self, Mutate::new(mutator))
    }

    fn and_select<S>(self, selector: S) -> Then<Self, Select<S>> {
        Then::new(self, Select::new(selector))
    }
}

impl<T> Compose for T {}

#[cfg(test)]
pub mod compose_tests {
    use rand::thread_rng;

    use super::*;

    #[test]
    fn then_mutate_smoke_test() {
        let selector = 0;
        let mutator = 0;
        let combo = Select::new(selector).then_mutate(mutator);
        // combo.apply(0, 0, thread_rng());
    }
}
