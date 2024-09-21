use derive_optional::Optional;

#[derive(Optional, Debug, PartialEq, Eq, Clone, Copy)]
enum TestType<T: std::fmt::Debug + PartialEq> {
    Something(T),
    Nothing,
}
use TestType::*;

const NOTHING: TestType<usize> = Nothing;

// and
#[test]
fn and() {
    let a = Something(1);
    let b = Something(2);
    assert_eq!(a.and(b), b);
    assert_eq!(a.and(NOTHING), Nothing);
    assert_eq!(NOTHING.and(a), Nothing);
    assert_eq!(NOTHING.and(NOTHING), Nothing);
}

// and_then
#[test]
fn and_then() {
    assert_eq!(Something(1).and_then(|x| Something(x + 1)), Something(2));
    assert_eq!(Something(1).and_then(|_| NOTHING), Nothing);
    assert_eq!(NOTHING.and_then(|x| Something(x + 1)), Nothing);
    assert_eq!(NOTHING.and_then(|_| NOTHING), Nothing);
}

// filter
#[test]
fn filter() {
    assert_eq!(Something(1).filter(|x| *x == 1), Something(1));
    assert_eq!(Something(1).filter(|x| *x == 2), Nothing);
    assert_eq!(NOTHING.filter(|x| *x == 1), Nothing);
}

// or
#[test]
fn or() {
    let a = Something(1);
    let b = Something(2);
    assert_eq!(a.or(b), a);
    assert_eq!(a.or(NOTHING), a);
    assert_eq!(NOTHING.or(a), a);
    assert_eq!(NOTHING.or(NOTHING), Nothing);
}

// or_else
#[test]
fn or_else() {
    let a = Something(1);
    let b = Something(2);
    assert_eq!(a.or_else(|| b), a);
    assert_eq!(a.or_else(|| NOTHING), a);
    assert_eq!(NOTHING.or_else(|| a), a);
    assert_eq!(NOTHING.or_else(|| NOTHING), Nothing);
}

// xor
#[test]
fn xor() {
    let a = Something(1);
    let b = Something(2);
    assert_eq!(a.xor(b), Nothing);
    assert_eq!(a.xor(NOTHING), a);
    assert_eq!(NOTHING.xor(a), a);
    assert_eq!(NOTHING.xor(NOTHING), Nothing);
}
