fn compose<A, B, C, F, G>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(A) -> B,
    G: Fn(B) -> C,
{
    move |x| g(f(x))
}

fn increment(x: i32) -> i32 {
    x + 1
}

fn first(vals: &[i32]) -> i32 {
    vals[0]
}

fn first_plus_one_fn(vals: &[i32]) -> i32 {
    compose(first, increment)(vals)
}

fn main() {
    let vals = vec![5, 8, 9];
    // let first_plus_one = compose(first, increment);

    let result = first_plus_one_fn(&vals);
    assert_eq!(6, result);
    println!("The result was {result}.");
}

fn _main() {
    let increment = |x: i32| x + 1;
    let double = |x: i32| x * 2;

    let increment_then_double = compose(increment, double);
    let result = increment_then_double(3);
    assert_eq!(8, result);

    // let first = |vals: &[i32]| vals[0];
    // let first_plus_one = compose(first, increment);

    // NOTE: If we put the declaration of `vals` before the declaration of
    // `combo3`, everything is good. We just don't want to be forced to define
    // them in that order.
    let vals = vec![5, 8, 9];

    // If you replace `first_plus_one_fn` with `first_plus_one` (no `_fn`)
    // below, you end up with a compile error, despite the fact that the
    // function and closure seem (to me) to be logically equivalent.
    let result = first_plus_one_fn(&vals);
    assert_eq!(6, result);
    println!("The result was {result}");
}
