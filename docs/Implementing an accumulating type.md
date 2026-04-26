_I'm not sure where this ultimately goes, but I'll start it here._

# Implementing an accumulating type

if you want to implement an _accumulating type_, i.e., a type where you can accumulate a collection
of values into a single value, you can use the `accumulate` package. This provides generic implementations
of basic accumulation strategies. There are two types of accumulator strategies defined in the `accumulate` package:

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

To implement this strategy for a new type `T`, you need to implement most or all of these traits for  `T`:

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

The `default_to!` macro allows you to
specify default accumulation behaviors for specific types. The syntax for `default_to!` is:

```rust
unhindered_accumulae::default_to! {
  type_to_accumulate => accumulation_strategy,
}
```

Here `type_to_accumulate` (e.g., `u8`) is the type of the values to be accumulated,
and `accumulation_strategy` is the strategy used to perform the accumulation (e.g., `KeepResults<SaturatingSum>`).

### Example of defaults

In the example:

```rust
unhindered_accumulate::default_to! {
  u8 => KeepResults<SaturatingSum>
}
```

the items being accumulated have type `u8`. The strategy being used is
`SaturatingSum` wrapped in `KeepResults`.

The `SaturatingSum` strategy will sum the
score values, saturating as necessary. It also provides a `.total()` method on the accumulation result
which we can use to
access to final sum.

The `KeepResults` wrapper keeps all the individual scores, and
provides (among other things):

- a `result()` method on the accumulation result which gives us an iterator over the individual scores, and
- a `.get()` method on the accumulation result which we can use to access individual scores.

```rust
let scores: [u8; 7] = [5, 8, 9, 6, 3, 2, 0];
// If we don't specify a second generic in `Accumulate<T>`,
// the second generic defaults to the default accumulation strategy.
// Since `T = u8` here, we use the default strategy for `u8`,
// which is `KeepResults<SaturatingSum>`, so the expanded type
// becomes `Accumulate<u8, KeepResults<SaturatingSum>>`. Because
// `KeepResults` is a type alias, which is actually
// `Accumulate<u8, Combine<StoreResults, SaturatingSum>>`.
//                       \/ - note how we didn't specify an
//                            accumulation strategy here
let result: Accumulated<u8> = scores.into_iter().accumulate().unwrap();
// `SaturatingSum` ensures we have the `.total()` method.
assert_eq!(result.total(), 33);
// `StoreResults` ensures that we have the `.get()` method.
assert_eq!(result.get(2), Some(&9));
```

---

In the example below, we specify
that all the unsigned integer types default to `KeepResults<SaturatingSum>`:

```rust
unhindered_accumulate::default_to! {
    ScoreValue<u8> => KeepResults<SaturatingSum>,
    ScoreValue<u16> => KeepResults<SaturatingSum>,
    ScoreValue<u32> => KeepResults<SaturatingSum>,
    ScoreValue<u64> => KeepResults<SaturatingSum>,
    ScoreValue<u128> => KeepResults<SaturatingSum>,
    ScoreValue<usize> => KeepResults<SaturatingSum>,
}
```

### Choosing good defaults
