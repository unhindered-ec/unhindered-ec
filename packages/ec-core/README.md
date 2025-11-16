# ec-core

This crate is a part of the [unhindered-ec](https://unhindered.ec) project.

## Structure

The diagram below illustrates an example of a common evolutionary
process.

![Diagram of basic evolutionary computation process](../../images/Evolutionary_computation_flowchart.excalidraw.svg)

We start by generating a random initial `Population` of
`Individual`s. These `Individual`s are then `Score`d, and those
`Score`s are used to select the "best" individuals to be used as
parents. Those parent individuals are modified using mutation and
recombination operators to form a new population made up of child individuals.
This process repeats until some exit condition is reached, e.g., we have
found an optimal score or the maximum number of generations is reached, etc.

See [`ec-linear/examples`](../ec-linear/examples/) for implementations of
simple bit-string evolutionary processes. See [`push/examples`](../push/examples/)
for implementations of evolutionary processes using PushGP to evolve Push
programs.

## Tools

This crate provides core evolutionary computation functionality, such as interfaces for

- `Genome`, the component of `Individual`s that evolves over time
- `Individual` (a trait) and `EcIndividual` (a simple struct implementing the `Individual` trait)[^subject-to-change]
- `Population`, which is typically a collection of `Individual`s
- `Generation`,[^subject-to-change] which represents a `Population`
  at a given point in time during the evolutionary process

## Operators

This includes support for `Operator`s, which encapsulate logic that maps from one
type to another. This includes:

- `Scorer`, which maps from an `Individual` to a score, which is then used by `Selector`s in
  the selection process.
- `Selector`, which maps from a `Population` of `Individual`s to a single, selected individual. This
  is typically how parents are selected, preferring individuals with better scores.
- `Mutator`, which maps from one `Genome` to a (presumably modified) `Genome` via some kind of mutation operator
- `Recombinator`, which maps from one or more `Genome`s to a (child) `Genome` via some kind of recombination operator like uniform crossover
- Compositional operators such as `And` and `Then`.

## Generating random values

We use the `rand` crate's `Distribution` trait to generate random instances of numerous types such as
primitive values, `Genome`s, and `Individual`s. We also provide `collection::Generator`, to
generate fixed size collections of randomly generated values of an arbitrary type.

## Scoring

`ec-core` provides two score types which can be useful for scoring: `Score<T: Ord>`, where larger is better, and
`Error<T: Ord>`, where smaller is better. You can also implement
your own scoring type as needed.

The `TestResults<R>`[^subject-to-change] type encapsulates a collection of scores, keeping them separate (instead of aggregating
them) for use with selectors such as `Lexicase`.

[^subject-to-change]: This type is likely to change in the future.

---

## Examples / How to get started

To see examples of how to use these tools, and guides for how to get started,
see:

- [The README for `ec-linear` package](../ec-linear/README.md#examples)
- [The README for the `push` package](../push/README.md#examples)
