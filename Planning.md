# Rust-GA live stream planning <!-- omit in toc -->

I'm going to use this to do some planning and documentation for the
live stream. Not sure how well that will work out, but just putting
`TODO`s in the code doesn't always provide any kind of "big picture".

- [Issues to address](#issues-to-address)
- [Wednesday, 21 Sep 2022 (7-9pm)](#wednesday-21-sep-2022-7-9pm)
  - [What actually happened](#what-actually-happened)

## Issues to address

Some things that we could/should deal with include:

- I think we have the parent selection thing in a reasonable
  place, but still we need to work out pipelines of mutation
  and recombination operators.
  - The addition of a closure that captures most of this pipeline
    in the construction of a `Generation` type may essentially
    resolve this issue, or at least suggests a way to approach it.
- Extend `PopulationSelector` to a `WeightedParentSelector` that
  is essentially a wrapper around `rand::distributions::WeightedChoice`
  so we can provide weights on the different selectors.
- Implement a non-threaded `Generation::next()` method that
  creates a new generation without using the Rayon `into_par_iter()`.
  - If we did this we could also benchmark both parallel and serial
    versions of the system to see how much faster things run in
    parallel.
  - It would probably be reasonable to add a `parallel` feature
    to the project so people could leave out parallelism (& Rayon)
    if they didn't want any of that.
- Should the `scorer` be inside the generation so we don't have to
  keep capturing it and passing it around?
  - Or should there actually be a `Run` type (or a `RunParams` type)
    that holds all this stuff and is used to make them available to
    types like `Generation` and `Population`?

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
  pipelining mutation and recombination operators.

### What actually happened

`esitsu@Twitch` suggested moving the `ParentSelector` logic into a new
`Generation` type, which would then have a `next()` method to generate
the next generation from the current one. We then spent pretty much the
entire stream implementing this (good) idea.

Most of it was pretty straightforward, but we got really bogged down at
the end because of a problem with the Rust compiler's understanding of a
closure's types.

`esitsu` also had a really cool idea of extracting the bit string specific
parts of the `next` generation logic into a closure that we'd pass in when
we construct the generation. This worked, but I didn't explicitly type the
arguments to the closure initially, and that let to all kinds of weird
confusion. We flailed pretty hard trying to resolve the issues, doing all
sorts of things with lifetimes. In the end it turned out that all we
_really_ needed to do was explicitly type that closure, and once we did
that everything worked.

After the stream ended I removed the `next_generation()` logic and
`ParentSelector` type from `Population`, moving the former into
`Generation` and deleting the latter altogether.
