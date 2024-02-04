#[macro_export]
macro_rules! vec_into {
    (<$t:ty>) => { Vec::<$t>::new() };
    (<$t:ty>$elem:expr; $n:expr) => { vec![<$t>::from($elem); $n] };
    (<$t:ty>$($x:expr),+ $(,)?) => { vec![$(<$t>::from($x)),+] };
    () => { Vec::new() };
    ($elem:expr; $n:expr) => { vec![($elem).into(); $n] };
    ($($x:expr),+ $(,)?) => { vec![$(($x).into()),+] };
}

pub use vec_into;

#[macro_export]
macro_rules! arr_into {
    (<$t:ty>) => { {let a: [$t; 1] = []; a} };
    (<$t:ty>$elem:expr; $n:expr) => { [<$t>::from($elem); $n] };
    (<$t:ty>$($x:expr),+ $(,)?) => { [$(<$t>::from($x)),+] };
    () => { Vec::new() };
    ($elem:expr; $n:expr) => { [($elem).into(); $n] };
    ($($x:expr),+ $(,)?) => { [$(($x).into()),+] };
}

pub use arr_into;
