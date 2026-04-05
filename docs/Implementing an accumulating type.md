_I'm not sure where this ultimately goes, but I'll start it here._

# Implementing an accumulating type

if you want to implement an _accumulating type_, i.e., a type where you can accumulate a collection
of values into a single value, you can use the `accumulate` package. This provides generic implementations
of basic accumulation strategies. THere are two types of accumulator strategies defined in the `accumulate` package:

- "base" accumulators, which specify the specific accumulation strategy
- "adapters", which modify the behavior of wrapped accumulators

The current base accumulators include:

- `Sum`, adds up the values. This is auto implemented for all types `T: AddAssign + Sum + Default`
- `WrappingSum` adds the values with wrapping. This is auto implemented for all types `Wrapping<T>: AddAssign + Sum` and `T: Default`
- `SaturatingSum` adds the values with saturation. For technical reasons, this must be implemented separately for each type.
- `StoreResults` does not add up the values, but instead stores them in a `Vec` for later access. If you want to store _and_ sum
  you can use `Combine` or the helper type `KeepResults`.

> [!NOTE]
> There are generic implementations for all of these except `SaturatingSum`. Because we can't currently
> specify the necessary features in the type, if you want saturating accumulation for a new item type you will
> have to implement a number of traits. See below.

The current adapters include:

- `Widen<T, Strategy>`, first converts the values to type `T` (which is presumably a "wider" type), and then
  uses a provided `Strategy` to accumulate these widened values
- `Combine<IndividualStrategy, TotalStrategy>` allows you to use different strategies to (a) combine the individuals and (b) create the total

`KeepResults<T>` is an alias for `Combine<StoreResults, T>`. This allows you to store all the individual results, while also
combining them into a final collected value of type `T` (e.g., a sum).

## Implementing saturating accumulation

If you want to implement a _saturating accumulating type_, i.e., a type where you can accumulate a collection of values
into a single value using saturation, you will need to implement a number of traits by hand. See the implementations of
`ScoreValue` and `ErrorValue` for examples.

To implement this strategy for a new type `T`, you need implement most or all of these traits for  `T`:

- `AccumulateStrategy<T> for SaturatingSum`
- `TotalResult<T> for SaturatingSum`
- `IndividualResults<T> for SaturatingSum`
- `IndexResults<T> for SaturatingSum`

In most cases it is probably fine to skip implementing `IndividualResults` and `IndexResults` and
rely on `KeepResults<SaturatingSum>` if you need access to individual results.

If in your specific implementation of `SaturatingSum` you need to keep all results to calculate a
total anyways, it might make sense to also implement the `IndividualResults` and `IndexResults` traits
to avoid needing a `KeepResults` and storing them twice.

In particular, if you have a general wrapper type that forwards its accumulate implementation to an inner type,
you also want to implement all the traits to support a base item type as discussed above.

## Defaults

You may also wish to specify default behaviors for accumulation, especially if you have concerns
about something like the sum overflowing the base type.

```rust
unhindered_accumulate::default_to! {
    ScoreValue<u8> => KeepResults<SaturatingSum>,
    ScoreValue<u16> => KeepResults<SaturatingSum>,
    ScoreValue<u32> => KeepResults<SaturatingSum>,
    ScoreValue<u64> => KeepResults<SaturatingSum>,
    ScoreValue<u128> => KeepResults<SaturatingSum>,
    ScoreValue<usize> => KeepResults<SaturatingSum>,
```
