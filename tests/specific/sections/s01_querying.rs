use derive_optional::Optional;

#[derive(Optional, Debug, PartialEq, Eq, Clone, Copy)]
enum TestType {
    Something(usize),
    Nothing,
}
use TestType::*;

// is_some
#[test]
fn is_some() {
    let test = Something(5);
    assert!(test.is_something());

    let test = Nothing;
    assert!(!test.is_something());
}

// is_some_and
#[test]
fn is_some_and() {
    let test = Something(5);
    assert!(test.is_something_and(|x| x == 5));

    let test = Something(5);
    assert!(!test.is_something_and(|x| x == 6));

    let test = Nothing;
    assert!(!test.is_something_and(|x| x == 5));
}

// is_none
#[test]
fn is_none() {
    let test = Something(5);
    assert!(!test.is_nothing());

    let test = Nothing;
    assert!(test.is_nothing());
}

// is_none_or
#[test]
fn is_none_or() {
    let test = Something(5);
    assert!(test.is_nothing_or(|x| x == 5));

    let test = Something(5);
    assert!(!test.is_nothing_or(|x| x == 6));

    let test = Nothing;
    assert!(test.is_nothing_or(|x| x == 5));
}
