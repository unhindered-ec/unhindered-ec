// The Operator trait would have a single method that takes an input,
// population and rng.

use rand::rngs::ThreadRng;

mod composable;

pub use composable::Composable;

pub trait Operator<Input>: Composable {
    type Output;

    fn apply(&self, input: Input, rng: &mut ThreadRng) -> Self::Output;
}

// trait Operator<Input, P> {
//     type Output;

//     fn apply(&self, input: Input, p: &P, rng: &mut ThreadRng) -> Self::Output;
// }

// pub struct Mutate<M> {
//     mutator: M,
// }

// impl<M> Mutate<M> {
//     fn new(mutator: M) -> Self {
//         Self { mutator }
//     }
// }

// // fn frogs() {
// //     let child_genome = select(Best)
// //         .and(select(Lexicase))
// //         .then_recombine(UniformXo)
// //         .then_mutate(OneOverLength);
// // }

// // -sel&clone-> a -mut-> b -mut-> c -mut-> d

// pub trait Mutator<Genome> {
//     // This is "my" way, but may involve making more Genomes in long pipelines.
//     fn mutate(input: &Genome, rng: &mut ThreadRng) -> Genome;

//     // The next two are from @esitsu and would involve less copying,
//     // but require that the initial select.apply() step clone the genome so that
//     // subsequent steps in the pipeline can mutate it safely.
//     //
//     // However…
//     // I just thought of a possible problem with passing &mut Genome everywhere.
//     // Imagine I have a single "parent" genome g and I want to mutate it twice
//     // and then recombine the resulting genomes to get the output. Mutating g the
//     // first time will change g if we pass it as &mut, so the second mutation will
//     // be a mutation of the mutated version of g and overwrite the initial mutation
//     // of g. That's gonna be a problem, right?
//     fn mutate0(input: Genome, rng: &mut ThreadRng) -> Genome {
//         input
//     }
//     fn mutate1(input: &mut Genome, rng: &mut ThreadRng);
// }

// impl<Input, P, M> Operator<Input, P> for Mutate<M>
// where
//     M: Mutator,
// {
//     type Output = Input;

//     fn apply(&self, input: Input, _: &P, rng: &mut ThreadRng) -> Self::Output {
//         self.mutator.mutate(input, &mut rng)
//     }
// }

// pub struct Select<S> {
//     selector: S,
// }

// impl<S> Select<S> {
//     fn new(selector: S) -> Self {
//         Self { selector }
//     }
// }
