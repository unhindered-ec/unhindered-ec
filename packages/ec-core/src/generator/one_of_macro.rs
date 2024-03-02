/// Create a Distribution of a specified type, calling `.into()` on all
/// elements. Similar to the `[...]` syntax of rust optionally allowing to
/// specify the type using `uniform_distribution_of![<T> ...]`. To sample from
/// the Distribution, the elements are cloned. If you wish to avoid that, use
/// `uniform_distribution_of![ref <T> ...]` to return references instead.
///
/// This is equivalent to `arr_into![].into_distribution().unwrap()` without the
/// need for an `unwrap()`.
///
/// ![Railroad diagram for the `uniform_distribution_of!` macro][ref_text]
///
/// # Examples
/// ```rs
/// let distrb_1: impl Distribution<&i64> = uniform_distribution_of![ref <i64>
///     1i32,
///     2i32
/// ];
/// let distrb_2: impl Distribution<i64> = uniform_distribution_of![<i64>
///     1i32,
///     2i32
/// ];
/// ```
#[macro_railroad_annotation::generate_railroad("ref_text")]
#[macro_export]
macro_rules! uniform_distribution_of {
     (<$output_type:ty>  $($items:expr),+ $(,)?) => {
          unsafe {
               ::std::result::Result::unwrap_unchecked(
                    $crate::generator::conversion
                         ::IntoDistribution::<$output_type>::into_distribution([
                         $(::std::convert::Into::<$output_type>::into($items)),+
                    ])
               )
          }
     };
     ($($items:expr),+ $(,)?) => {
          unsafe {
               ::std::result::Result::unwrap_unchecked(
                    $crate::generator::conversion::IntoDistribution::into_distribution([
                         $($items),+
                    ])
               )
          }
     };
     (ref <$output_type:ty> $($items:expr),+ $(,)?) => {
          {
               // this is needed cuz we can't impl IntoDistribution<&T> for Vec<T> cuz of lifetime issues
               let arr = [
                         $(::std::convert::Into::<$output_type>::into($items)),+
                    ];

               unsafe {
                    ::std::result::Result::unwrap_unchecked(
                         $crate::generator::conversion
                              ::ToDistribution::<&$output_type>::to_distribution(&arr)
                    )
               }
          }
     };
     (ref  $($items:expr),+ $(,)?) => {
          {
               let arr = [
                         $($items),+
                    ];
               unsafe {
                    ::std::result::Result::unwrap_unchecked(
                         $crate::generator::conversion::ToDistribution::<&_>::to_distribution(&arr)
                    )
               }
          }
     };
}
