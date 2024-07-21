/// Create a Distribution of a specified type, calling `.into()` on all
/// elements. Similar to the `[...]` syntax of rust optionally allowing the
/// specification of the target type using `uniform_distribution_of![<T> ...]`.
/// To sample from the Distribution, the elements are cloned.
///
/// This is equivalent to `arr_into![].into_distribution().unwrap()` without the
/// need for an `unwrap()`.
///
/// ![Railroad diagram for the `uniform_distribution_of!` macro][ref_text]
///
/// # Examples
///
/// Here we create a distribution of `i32`. Since the provided values
/// are `i32`, we don't need to specify the target type of the distribution.
/// ```ignore
/// let distrb = uniform_distribution_of![
///    1i32,
///    2i32
/// ];
/// ```
///
/// Here we convert a list of `i32` into a distribution of `i64`s.
/// ```ignore
/// let distrb: impl Distribution<i64> = uniform_distribution_of![<i64>
///     1i32,
///     2i32
/// ];
/// ```
///
/// A common usage is in generating a uniform distribution of instructions where
/// the target type is `<PushInstruction>`. Using the macro means we don't have
/// to explicitly convert the various instruction types into `PushInstruction`,
/// and allows us to have a variety of types in the list (e.g.,
/// `IntInstruction`, `BoolInstruction`, and `FloatInstruction`).
/// ```ignore
/// let instruction_set = uniform_distribution_of![<PushInstruction>
///     IntInstruction::Add,
///     IntInstruction::Subtract,
///     BoolInstruction::And,
///     FloatInstruction::Multiply,
/// ];
/// ```
#[macro_railroad_annotation::generate_railroad("ref_text")]
#[macro_export]
macro_rules! uniform_distribution_of {
     (<$output_type:ty>  $($items:expr),+ $(,)?) => {
          // FIXME: is unwrap_unchecked a performance gain here?
          {
               // The macro pattern guarantees that there is at least one item ($(...)+) and into_distribution for all types only errors if no element is present.
               let val = ::std::result::Result::unwrap(
                    $crate::distributions::conversion
                         ::IntoDistribution::<$output_type>::into_distribution([
                         $(::std::convert::Into::<$output_type>::into($items)),+
                    ])
               );

               val
          }
     };
     ($($items:expr),+ $(,)?) => {
          // FIXME: is unwrap_unchecked a performance gain here?
          {
               // The macro pattern guarantees that there is at least one item ($(...)+) and into_distribution for all types only errors if no element is present.
               let val = ::std::result::Result::unwrap(
                    $crate::distributions::conversion::IntoDistribution::into_distribution([
                         $($items),+
                    ])
               );

               val
          }
     };
}

#[cfg(test)]
#[rustversion::attr(before(1.81), allow(clippy::unwrap_used))]
#[rustversion::attr(
    since(1.81),
    expect(
        clippy::unwrap_used,
        reason = "Panicking is the best way to deal with errors in unit tests"
    )
)]
mod test {
    use crate::distributions::conversion::IntoDistribution;

    #[test]
    fn distr_into_owned() {
        let distr = uniform_distribution_of![<i64>
             1i32,
             3i32
        ];

        assert_eq!(distr, [1i64, 3i64].into_distribution().unwrap());
    }

    #[test]
    fn distr_inferred_owned() {
        let distr = uniform_distribution_of![1i32, 3i32];

        assert_eq!(distr, [1i32, 3i32].into_distribution().unwrap());
    }
}
