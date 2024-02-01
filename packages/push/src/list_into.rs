/// Create a new vector of a specified type, calling `.into()` on all elements.
/// Similar to the [`vec![...]`](vec) macro in [`std`], optionally allowing to
/// specify the type using `vec_into![<T> ...]`.
#[macro_railroad_annotation::generate_railroad]
///
/// # Examples
/// ```rs
/// let vec = vec_into![];
/// let vec_2 = vec_into![<i64>];
/// let vec_3: Vec<i64> = vec_into![3i32;5];
/// let vec_4: Vec<i64> = vec_into![3i32,10i32,500i32];
/// let vec_5 = vec_into![<i64> 3i32;5];
/// let vec_6 = vec_into![<i64> 3i32,10i32,500i32];
/// ```
#[macro_export]
macro_rules! vec_into {
    (<$output_type:ty>) => {
        Vec::<$output_type>::new()
    };
    (<$output_type:ty>$item:expr; $repeat_times:expr) => {
         vec![<$output_type>::from($item); $repeat_times]
    };
    (<$output_type:ty>$($items:expr),+ $(,)?) => {
         vec![$(<$output_type>::from($items)),+]
    };
    () => {
         Vec::new()
    };
    ($item:expr; $repeat_times:expr) => {
         vec![($item).into(); $repeat_times]
    };
    ($($items:expr),+ $(,)?) => {
         vec![$(($items).into()),+]
    };
}

pub use vec_into;

/// Create a new array of a specified type, calling `.into()` on all elements.
/// Similar to the `[...]` syntax of rust optionally allowing to specify the
/// type using `arr_into![<T> ...]`.
#[macro_railroad_annotation::generate_railroad]
///
/// # Examples
/// ```rs
/// let arr = arr_into![];
/// let arr_2 = arr_into![<i64>];
/// let arr_3: [i64;5] = arr_into![3i32;5];
/// let arr_4: [i64;3] = arr_into![3i32,10i32,500i32];
/// let arr_5 = arr_into![<i64> 3i32;5];
/// let arr_6 = arr_into![<i64> 3i32,10i32,500i32];
/// ```
#[macro_export]
macro_rules! arr_into {
     (<$output_type:ty>) => {
          {
               let a: [$output_type; 1] = [];
               a
          }
     };
     (<$output_type:ty>$item:expr; $repeat_times:expr) => {
          [<$output_type>::from($item); $repeat_times]
     };
     (<$output_type:ty>$($items:expr),+ $(,)?) => {
          [$(<$output_type>::from($items)),+]
     };
     () => {
          Vec::new()
     };
     ($item:expr; $repeat_times:expr) => {
          [($item).into(); $repeat_times]
     };
     ($($items:expr),+ $(,)?) => {
          [$(($items).into()),+]
     };
}

pub use arr_into;
