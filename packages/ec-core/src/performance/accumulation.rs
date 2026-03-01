use std::num::Saturating;

// Justus suggested a macro like this to reduce the boilerplate below. Esitsu
// even thought it was a reasonable idea. Gemini's code review also suggested
// that it would be useful! So I think we're in.

macro_rules! default_behavior {
    ($t: ty => $accumulator: ty) => {
        impl DefaultAccumulator for $t { type Accumulator = $accumulator; }
    };
    ($($t: ty => $accumulator: ty),* $(,)?) => {
        $(default_behavior!($t => $accumulator);)*
    }
}

#[diagnostic::on_unimplemented(
    message = "No default summation behavior specified for type {Self}",
    label = "explicit summation type required here",
    note = "If you are trying to use TestResults<{Self}>, use TestResults<{Self}, MyTotalType> \
            instead,\nwhere MyTotalType is used for the total test result."
)]
/// Specifies the default type to use when summing a set of values.
///
/// # Examples
///
/// ```
/// use ec_core::performance::accumulation::{AccumulateInto, DefaultAccumulator};
///
/// fn sum_smart<T>(values: &[T]) -> T::Accumulator
/// where
///     T: DefaultAccumulator + ToOwned,
///     <T as ToOwned>::Owned: AccumulateInto<T::Accumulator>,
///     T::Accumulator: std::iter::Sum,
/// {
///     values
///         .iter()
///         .map(|v| v.to_owned())
///         .map(|v| v.accumulate_into())
///         .sum()
/// }
///
/// let data = [100i8, 100i8];
/// // i8 accumulates into i16 by default
/// let sum: i16 = sum_smart(&data);
/// assert_eq!(sum, 200);
/// ```
pub trait DefaultAccumulator {
    type Accumulator;
}

/// A trait for types that can be accumulated into a target type `T`.
///
/// This is similar to `Into<T>`, but allows for specific behaviors like
/// saturation or promotion that might not be covered by standard `From`/`Into`
/// implementations (e.g., `Saturating<T>` does not implement `From<T>`).
pub trait AccumulateInto<T> {
    fn accumulate_into(self) -> T;
}

impl<T> AccumulateInto<T> for T {
    fn accumulate_into(self) -> T {
        self
    }
}

impl AccumulateInto<i16> for i8 {
    fn accumulate_into(self) -> i16 {
        self.into()
    }
}

impl AccumulateInto<i32> for i16 {
    fn accumulate_into(self) -> i32 {
        self.into()
    }
}

impl AccumulateInto<i64> for i32 {
    fn accumulate_into(self) -> i64 {
        self.into()
    }
}

impl AccumulateInto<i128> for i64 {
    fn accumulate_into(self) -> i128 {
        self.into()
    }
}

impl<T> AccumulateInto<Saturating<T>> for T {
    fn accumulate_into(self) -> Saturating<T> {
        Saturating(self)
    }
}

default_behavior! {
    i8 => i16,
    i16 => i32,
    i32 => i64,
    i64 => i128,
    i128 => i128,
    u8 => Saturating<u8>,
    u16 => Saturating<u16>,
    u32 => Saturating<u32>,
    u64 => Saturating<u64>,
    u128 => Saturating<u128>,
    f32 => f32,
    f64 => f64,
}
