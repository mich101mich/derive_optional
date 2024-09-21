use derive_optional::Optional;

#[derive(Optional, Debug, PartialEq, Eq, Clone, Copy)]
enum TestType<T: std::fmt::Debug + PartialEq> {
    Something(T),
    Nothing,
}
use TestType::*;

// expect
#[test]
fn expect_non_panicking() {
    let test = Something(1);
    assert_eq!(test.expect("test"), 1);
}
#[test]
#[should_panic(expected = "my_test_message")]
fn expect_panicking() {
    let test: TestType<usize> = Nothing;
    test.expect("my_test_message");
}

// unwrap
#[test]
fn unwrap_non_panicking() {
    let test = Something(1);
    assert_eq!(test.unwrap(), 1);
}
#[test]
#[should_panic(expected = "called `TestType::unwrap()` on a `Nothing` value")]
fn unwrap_panicking() {
    let test: TestType<usize> = Nothing;
    test.unwrap();
}

// unwrap_or
#[test]
fn unwrap_or() {
    let test = Something(1);
    assert_eq!(test.unwrap_or(2), 1);

    let test: TestType<usize> = Nothing;
    assert_eq!(test.unwrap_or(2), 2);
}

// unwrap_or_else
#[test]
fn unwrap_or_else() {
    let test = Something(1);
    assert_eq!(test.unwrap_or_else(|| 2), 1);

    let test: TestType<usize> = Nothing;
    assert_eq!(test.unwrap_or_else(|| 2), 2);
}

// unwrap_or_default
#[test]
fn unwrap_or_default() {
    let test = Something(1);
    assert_eq!(test.unwrap_or_default(), 1);

    let test: TestType<usize> = Nothing;
    assert_eq!(test.unwrap_or_default(), 0);
}

// unwrap_unchecked
#[test]
fn unwrap_unchecked() {
    let test = Something(1);
    assert_eq!(unsafe { test.unwrap_unchecked() }, 1);

    // no test for Nothing, because it would be UB
}
