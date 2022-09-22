# Rust-GA live stream planning

I'm going to use this to do some planning and documentation for the
live stream. Not sure how well that will work out, but just putting
`TODO`s in the code doesn't always provide any kind of "big picture".

## Issues to address

Some things that we could/should deal with include:

- I think we have the parent selection thing in a reasonable
  place, but still we need to work out pipelines of mutation
  and recombination operators.

## Wednesday, 21 Sep 2022 (7-9pm)

Two things that would be good to do that would follow up on work
we did last week:

- Should the new `PopulationSelector` type just become part of
  the `Population` type?
  We could provide a set of selectors in the constructor (or
  a builder) for `Population`, and then just have a `get_parent()`
  method there.
- Extend `PopulationSelector` to a `WeightedParentSelector` that
  is essentially a wrapper around `rand::distributions::WeightedChoice`
  so we can provide weights on the different selectors.
- If those get done quickly, then we can look at the problem of
  pipelining mutation and recominbation operators.
  