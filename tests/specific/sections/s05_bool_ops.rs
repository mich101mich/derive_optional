use derive_optional::Optional;

#[derive(Optional, Debug, PartialEq, Eq, Clone, Copy)]
enum TestType {
    Something(usize),
    Nothing,
}
use TestType::*;

// and
#[test]
fn and() {
    let a = Something(1);
    let b = Something(2);
    assert_eq!(a.and(b), b);
    assert_eq!(a.and(Nothing), Nothing);
    assert_eq!(Nothing.and(a), Nothing);
    assert_eq!(Nothing.and(Nothing), Nothing);
}

// and_then
#[test]
fn and_then() {
    assert_eq!(Something(1).and_then(|x| Something(x + 1)), Something(2));
    assert_eq!(Something(1).and_then(|_| Nothing), Nothing);
    assert_eq!(Nothing.and_then(|x| Something(x + 1)), Nothing);
    assert_eq!(Nothing.and_then(|_| Nothing), Nothing);
}

// filter
#[test]
fn filter() {
    assert_eq!(Something(1).filter(|x| *x == 1), Something(1));
    assert_eq!(Something(1).filter(|x| *x == 2), Nothing);
    assert_eq!(Nothing.filter(|x| *x == 1), Nothing);
}

// or
#[test]
fn or() {
    let a = Something(1);
    let b = Something(2);
    assert_eq!(a.or(b), a);
    assert_eq!(a.or(Nothing), a);
    assert_eq!(Nothing.or(a), a);
    assert_eq!(Nothing.or(Nothing), Nothing);
}

// or_else
#[test]
fn or_else() {
    let a = Something(1);
    let b = Something(2);
    assert_eq!(a.or_else(|| b), a);
    assert_eq!(a.or_else(|| Nothing), a);
    assert_eq!(Nothing.or_else(|| a), a);
    assert_eq!(Nothing.or_else(|| Nothing), Nothing);
}

// xor
#[test]
fn xor() {
    let a = Something(1);
    let b = Something(2);
    assert_eq!(a.xor(b), Nothing);
    assert_eq!(a.xor(Nothing), a);
    assert_eq!(Nothing.xor(a), a);
    assert_eq!(Nothing.xor(Nothing), Nothing);
}
