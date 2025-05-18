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
