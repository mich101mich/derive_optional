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
// unstable

// is_none
#[test]
fn is_none() {
    let test = Something(5);
    assert!(!test.is_nothing());

    let test = Nothing;
    assert!(test.is_nothing());
}
