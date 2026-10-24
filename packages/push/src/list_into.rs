/// Create a new vector of a specified type, calling `.into()` on all elements.
/// Similar to the [`vec![...]`](vec) macro in [`std`], optionally allowing to
/// specify the type using `vec_into![<T> ...]`.
///
/// ![Railroad diagram for the `vec_into` macro][ref_text]
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
#[macro_railroad_annotation::generate_railroad("ref_text")]
#[macro_export]
macro_rules! vec_into {
    (<$output_type:ty>) => {
        ::std::vec::Vec::<$output_type>::new()
    };
    (<$output_type:ty>$item:expr; $repeat_times:expr) => {
         ::std::vec![::std::convert::Into::<$output_type>::into($item); $repeat_times]
    };
    (<$output_type:ty>$($items:expr),+ $(,)?) => {
         ::std::vec![$(::std::convert::Into::<$output_type>::into($items)),+]
    };
    () => {
         ::std::vec::Vec::new()
    };
    ($item:expr; $repeat_times:expr) => {
         ::std::vec![::std::convert::Into::into($item); $repeat_times]
    };
    ($($items:expr),+ $(,)?) => {
         ::std::vec![$(::std::convert::Into::into($items)),+]
    };
}

pub use vec_into;

/// Create a new array of a specified type, calling `.into()` on all elements.
/// Similar to the `[...]` syntax of rust optionally allowing to specify the
/// type using `arr_into![<T> ...]`.
///
/// ![Railroad diagram for the `arr_into` macro][ref_text]
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
#[macro_railroad_annotation::generate_railroad("ref_text")]
#[macro_export]
macro_rules! arr_into {
     (<$output_type:ty>) => {
          {
               let a: [$output_type; 0] = [];
               a
          }
     };
     (<$output_type:ty>$item:expr; $repeat_times:expr) => {
          [::std::convert::Into::<$output_type>::into($item); $repeat_times]
     };
     (<$output_type:ty>$($items:expr),+ $(,)?) => {
          [$(::std::convert::Into::<$output_type>::into($items)),+]
     };
     () => {
          []
     };
     ($item:expr; $repeat_times:expr) => {
          [::std::convert::Into::into($item); $repeat_times]
     };
     ($($items:expr),+ $(,)?) => {
          [$(::std::convert::Into::into($items)),+]
     };
}

pub use arr_into;

#[cfg(test)]
mod test {
    #[test]
    fn vec_empty_given_type() {
        assert_eq!(vec_into![<bool>], Vec::<bool>::new());
    }

    #[test]
    fn vec_repeat_given_type() {
        assert_eq!(vec_into![<u64> 4u32;2], vec![4u64, 4u64]);
    }

    #[test]
    fn vec_explicit_given_type() {
        assert_eq!(vec_into![<u64> 4u32, 3u32], vec![4u64, 3u64]);
    }

    #[test]
    fn vec_empty_inferred_type() {
        let vec: Vec<bool> = vec_into![];
        assert_eq!(vec, Vec::<bool>::new());
    }

    #[test]
    fn vec_repeat_inferred_type() {
        let vec: Vec<u64> = vec_into![4u32;2];
        assert_eq!(vec, vec![4u64, 4u64]);
    }

    #[test]
    fn vec_explicit_inferred_type() {
        let vec: Vec<u64> = vec_into![4u32, 3u32];
        assert_eq!(vec, vec![4u64, 3u64]);
    }

    #[test]
    fn arr_empty_given_type() {
        assert_eq!(arr_into![<bool>], [true; 0]);
    }

    #[test]
    fn arr_repeat_given_type() {
        assert_eq!(arr_into![<u64> 4u32;2], [4u64, 4u64]);
    }

    #[test]
    fn arr_explicit_given_type() {
        assert_eq!(arr_into![<u64> 4u32, 3u32], [4u64, 3u64]);
    }

    #[test]
    fn arr_empty_inferred_type() {
        let arr: [bool; 0] = arr_into![];
        assert_eq!(arr, [true; 0]);
    }

    #[test]
    fn arr_repeat_inferred_type() {
        let arr: [u64; 2] = arr_into![4u32;2];
        assert_eq!(arr, [4u64, 4u64]);
    }

    #[test]
    fn arr_explicit_inferred_type() {
        let arr: [u64; 2] = arr_into![4u32, 3u32];
        assert_eq!(arr, [4u64, 3u64]);
    }
}
