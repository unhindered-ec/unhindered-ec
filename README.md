# unhindered-ec

A prototype of an evolutionary computation library in Rust. The
current implementation focuses on genetic algorithms and genetic programming,
but the design is hopefully flexible enough to incorporate other evolutionary systems.

A key goal of this library is to improve the speed of the evolutionary processes,
especially when compared to similar systems implemented in languages like Clojure
(such as [Clojush](https://github.com/lspector/Clojush) or [Propeller](https://github.com/lspector/propeller/))
or Python (such as [PyshGP](https://github.com/erp12/pyshgp) or [DEAP](https://github.com/DEAP/deap)).
The hope is to make research and experimentation
less hardware intensive and more accessible. This is especially important in PushGP, where
the interpretation of [Push programs](https://erp12.github.io/push-redux/pages/intro_to_push/)
can be very time consuming.

## Project structure

This project is split into several sub-packages:

- [`ec-core`](packages/ec-core/README.md)

  Definitions for traits and structs for key concepts such as `Genome`, `Individual`,
  `Population`, and `Generation`. It also defines the notion of an `Operator`, which
  encapsulates logic transforming one type to another. This is used for things like selection
  (which transforms a population into a selected individual), scoring (which transforms
  an individual into a score), and mutation and recombination operators (which transform
  one or more genomes into a new child genome).

- [`ec-linear`](packages/ec-linear/README.md)

  Defines an implementation of the key concepts from `ec-core` specifically for
  linear genomes. This includes `BitString`s (as frequently used in genetic algorithms)
  and both fixed-length and variable-length `Vector`s of simple types.
  Common associated mutation and recombination operators for these types are also provided.

- [`push`](packages/push/README.md) (WIP)

  Provides both an implementation of the [Push programming language](https://erp12.github.io/push-redux/pages/intro_to_push/)
  and tools for evolving Push programs in a PushGP system.

There are also macro packages ([`ec-macros`](packages/ec-macros/README.md) and [`push-macros`](packages/push-macros/README.md))
that provide support for the other packages. These should typically not be used directly; instead use the re-exports from
[`ec-core`](packages/ec-core/README.md) and [`push`](packages/push/README.md).

## Aspirations/goals

We would ultimately like to be able to replicate key PushGP research
using the PSB1 and PSB2 benchmark suites. This requires:

- A more complete set of Push types and instructions
  - In particular, we would need to add or complete the `char`, `String`,
    and `Vector` types
- A larger collection of operators (e.g., selection, mutation, and recombination)
  - In particular, we don't yet provide epsilon-lexicase,
    down-sampled lexicase, and related selection operators.
- Better support for downloading remote training data and
    loading training data from files

It would be valuable to support other genetic programming representations such as [grammatical evolution](https://en.wikipedia.org/wiki/Grammatical_evolution) [^ge_archive], [linear GP](https://en.wikipedia.org/wiki/Linear_genetic_programming) [^linear_gp_archive],
and [tree-based GP](https://archive.org/details/AFieldGuideToGeneticProgramming).

It would also be useful if we could create Python wrappers around
the appropriate parts of these libraries so that researchers familiar
with Python could benefit from the performance of
these libraries without having to learn Rust.

[^ge_archive]: Archived on [Internet Archive at 2025-10-09](https://web.archive.org/web/20251009050335/https://en.wikipedia.org/wiki/Grammatical_evolution).
[^linear_gp_archive]: Archived on [Internet Archive at 2025-08-27](https://web.archive.org/web/20250827032953/https://en.wikipedia.org/wiki/Linear_genetic_programming).

It would be nice to have more detailed "Getting started" documentation
that walked through the creation of a simple evolutionary experiment,
explaining all the key steps.

If you have other ideas or applications feel free to reach out.

---

## How to get started

To see evolution in action:

- Install Rust
- Run `cargo run --release --example count_ones`

### Pre-requisites

To use this library you will need to have [the latest stable
version of Rust installed](https://rust-lang.org/tools/install/).
We strongly encourage the use of `rustup` to install the latest
version, since versions provided by your system package manager
might be out of date.

There are three other tools that you might need to install depending
on your system:

- To install `rustup` you will need the `curl` utility
- To download and interact with the `unhindered-ec` code,
  you'll probably want to use `git`
- To build the system, the Rust compiler requires `cc`, so you will
  a C toolchain like `gcc` or `clang`.

### Running the system

Probably the first question is whether you're evolving fixed-length
structures like bitstrings as used in genetic algorithms, or you're
evolving variable-length structures as used in genetic programming.

If you're evolving fixed-length structures, then you'll probably want
to use [the `ec-linear` package](packages/ec-linear/README.md), which provides both that
representation and several basic operators. See [the
`ec-linear/examples` folder](packages/ec-linear/examples) for several examples showing how to
set up evolution of fixed-length structures. [The `ec-linear`
README](packages/ec-linear/README.md#Examples) shows how to
compile and run some of those examples.

If you're evolving variable-length structures, we currently support
variable length linear structures in [the `push` package](packages/push/README.md), focusing
primarily on evolving Push programs. See [the `push/examples`
folder](packages/push/examples) for several examples showing how to
set up evolution of Push programs. [The `push`
README](packages/push/README.md#Examples) shows how to
compile and run some of those examples.

---

## Socials

To contact us [join our Discord server](https://discord.gg/2drhrXQkRr), or open a GitHub issue, pull request, or discussion.

---

## License

Licensed under either of

- Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

See [LICENSE-APACHE](LICENSE-APACHE), [LICENSE-MIT](LICENSE-MIT), and [COPYRIGHT](COPYRIGHT) for details.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
