# ec-linear

This crate is a part of the [unhindered-ec](https://unhindered.ec) project.

This crate provides implementations for basic tools for evolving
linear structures such as bitstrings and Plushy genomes (see `push`
crate). There is support for both fixed-length and variable-length
linear genomes.

## Mutation operators

This crate includes a few mutation operators such as:

- `Umad` (uniform mutation through addition and deletion), which adds and deletes elements.
- `WithRate` which mutates elements with a given mutation rate.
- `WithOneOverLength` which mutates elements with the mutation rate $1/L$ where, $L$ is the length of the genome.

The `Umad` mutation operator can change the length of the genome and so can only be used with variable-length linear genomes. The other two
mutation operators maintain the length of the genome and can be used
with both fixed-length and variable-length genomes.

## Recombination operators

This crate includes a few recombination operators based on the
`Crossover` trait.

### `UniformXo`: Uniform crossover

`UniformXo` implements uniform crossover, choosing genes uniformly from two parents as illustrated below:

![Illustration of uniform crossover](../../images/UniformCrossover.excalidraw.svg)

### `TwoPointXo`: Two-point crossover

`TwoPointXo` implements two-point crossover, choosing two random locations along the parent genomes, and swapping sections as illustrated below.


 <figure>

 ![Illustration of two-point crossover](../../images/Two_point_crossover.svg)

  <figcaption>

  *Illustration of two-point crossover. Alternatively see
  [this animation](../../images/TwoPointCrossover.avif).*

  </figcaption>
</figure>

---

## Examples

The [examples directory](examples/) has two examples of performing
evolution on bitstrings:

- [`count_ones`](examples/count_ones/main.rs), an implementation of
  [the classic OneMax problem](https://schlosserpg.github.io/Heuristic/benchmark.html#onemax-problem),
  where the goal is to maximize the number of 1s in a bitstring
- [`hiff`](examples/hiff/main.rs), based on [Watson's Hierarchical If-and-only-if problem](https://doi.org/10.1109/CEC.1999.782647)

To run an example:

```bash
cargo run --release --example <name> -- <parameter settings>
```

where `<name>` is replaced by the name of the example (e.g., `hiff`).
Problems provide a set of optional parameter settings that allow you to
set things like the population size and the maximum number of generations.

To see the available parameters for a given example:

```bash
cargo run --release --example <name> -- --help
```

## Creating your own project

If you want to create your own Rust project that has `unhindered-ec`
as a dependency, you need to:

- Create a new Rust project with

  ```bash
  cargo init <my-project>
  ```

  (where you replace `<my-project>` with the name of your project).
- To enter your newly created project directory:

  ```bash
  cd <my-project>
  ```

- Add the `ec-core` and `ec-linear` packages from `unhindered-ec`
  as dependencies with

  ```bash
  cargo add --git https://github.com/unhindered-ec/unhindered-ec.git ec-core ec-linear
  ```

- Add the `rand` crate as a dependency with

  ```bash
  cargo add rand
  ```

  The `rand` crate provides random number generation and selection
  tools, which are necessary to do things like generate random initial
  populations.

- Copy [the starter code from this repository](./examples/count_ones_basic/main.rs) into `src/main.rs`, replacing the
  `main.rs` created by `cargo init`.

At this point you should be able to compile and run the sample project
with

```bash
cargo run --release
```

You can then edit the sample project in `main.rs` to, for example,
use a different scoring function, or change the selection or
recombination/mutation operators.

If, for example, you change the scoring function to:

```rust
pub fn count_zeroes(bits: &[bool]) -> Score<i64> {
    bits.iter().copied().map(Not::not).map(i64::from).sum()
}
```

then you'll negate all the bits before summing them up, which will try to
maximize the number of zeroes instead of the number of ones.

## Using `miette` for better error messages

`unhindered-ec` aims to provide help messages for each error. You
might want to consider using `miette` on your end to pretty-print
errors and get these help messages. See [the `count_ones` (not basic)
example](./examples/count_ones/main.rs) for how this might be done.

The tl;dr is:

```rust
  // change the return type of main to miette::Result<()>
  fn main() -> miette::Result<()> {
    // in library code which supports miette (unhindered-ec crates
    // do) just keep using the `?` operator
    call_some_ec_linear_function()?
    // in libraries which don't support miette (e.g. std) use
    // `.into_diagnostic()?`
    call_some_std_function().into_diagnostic()?
  }
```
