#![cfg(test)]
use std::ops::Not;

use push::{
    evaluation::cases::{Case, Cases, WithTargetFn},
    vec_into,
};

#[test]
fn test_from_inputs() {
    let cases = Cases::from_inputs(0..10, |x| x * 2);
    assert_eq!(
        cases.into_iter().collect::<Vec<_>>(),
        (0..10).map(|x| Case::new(x, x * 2)).collect::<Vec<_>>()
    );
}

#[test]
fn test_from_inputs_different_types() {
    let cases = Cases::from_inputs(0..10, ToString::to_string);
    assert_eq!(
        cases.into_iter().collect::<Vec<_>>(),
        (0..10)
            .map(|x| Case::new(x, x.to_string()))
            .collect::<Vec<_>>()
    );
}

#[test]
fn test_add_case() {
    let mut cases = Cases::new();
    cases.add_case((1, 2));
    cases.add_case((3, 6));
    assert_eq!(
        cases.into_iter().collect::<Vec<_>>(),
        vec_into![(1, 2), (3, 6)]
    );
}

#[test]
fn test_with_case() {
    let cases = Cases::new().with_case((1, 2)).with_case((3, 6));
    assert_eq!(
        cases.into_iter().collect::<Vec<_>>(),
        vec_into![(1, 2), (3, 6)]
    );
}

#[test]
fn test_len() {
    let mut cases = Cases::default();
    assert!(cases.is_empty());
    assert_eq!(cases.len(), 0);
    cases.add_case((1, 2));
    assert!(cases.is_empty().not());
    assert_eq!(cases.len(), 1);
    cases.add_case((3, 6));
    assert!(cases.is_empty().not());
    assert_eq!(cases.len(), 2);
}

#[test]
fn test_with_target() {
    let inputs = 0..10;
    let cases = inputs.with_target_fn(|x| x * 2);
    assert_eq!(
        cases.into_iter().collect::<Vec<_>>(),
        (0..10).map(|x| Case::new(x, x * 2)).collect::<Vec<_>>()
    );
}
