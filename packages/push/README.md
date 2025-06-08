# push

This crate is a part of the [unhindered-ec](https://unhindered.ec) project.

There are essentially two components to this package (that
probably should be split into separate packages): The Push
interpreter and the algorithms that allow us to evolve Push
programs.

There has been a _lot_ of discussion of the design of the Push
interpreter, and that would absolutely be considered WIP and
liable to change without notice.

The current interpreter has support for integer, floating point,
and boolean instructions, and some work has been done on
character instructions. A significant weakness at the moment
is that you can't add instructions without modifying the package
directly.

## Examples

The [examples directory](examples/) has several examples for
evolving solutions to both symbolic regression problems, as
well as software synthesis problems from
[the PSB](https://doi.org/10.1145/2739480.2754769) and
[the PSB2](https://doi.org/10.1145/3449639.3459285)
benchmark suites.

These include:

- [median](examples/median/main.rs), which finds the median of three integers
- [number_io](examples/number_io/main.rs), which converts an integer (`i64`)
  to a float (`f64`), and then adds it to a second float
- [smallest](examples/smallest/main.rs) which finds the smallest of four
  integers (`i64`)

To run an example:

```bash
cargo run --release --example <name> -- <parameter settings>
```

where `name` is replaced by the name of the example (e.g., `median`).
Problems provide a set of optional parameter settings that allow you to
set things like the population size and the maximum number of generations.
One parameter to consider is `max-initial-instructions`. This defaults to
100, but runs will be faster and evolved solutions will typically be easier
to understand, if you start with a small number, perhaps even 1.

To see the available parameters for a given example:

```bash
cargo run --release --example <name> -- --help
```
