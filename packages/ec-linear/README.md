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
`Crossover` trait:

- `Uniform`, which chooses elements uniformly from
  two parents
- `TwoPointXo`, which chooses two random locations
  along the parent genomes, and swaps sections as illustrated
  below.

![Illustration of two-point crossover over](../../images/Two_point_crossover.svg)
