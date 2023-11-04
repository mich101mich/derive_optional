use derive_optional::Optional;

#[derive(Optional, Debug, PartialEq, Eq, Clone, Copy)]
enum TestType {
    Something(usize),
    Nothing,
}
use TestType::*;

// take
#[test]
fn take() {
    let mut test = Something(1);
    assert_eq!(test.take(), Something(1));
    assert_eq!(test, Nothing);

    let mut test = Nothing;
    assert_eq!(test.take(), Nothing);
    assert_eq!(test, Nothing);
}

// replace
#[test]
fn replace() {
    let mut test = Something(1);
    assert_eq!(test.replace(2), Something(1));
    assert_eq!(test, Something(2));

    let mut test = Nothing;
    assert_eq!(test.replace(2), Nothing);
    assert_eq!(test, Something(2));
}

// contains
#[test]
fn contains() {
    let test = Something(1);
    assert!(test.contains(&1));
    assert!(!test.contains(&2));

    let test = Nothing;
    assert!(!test.contains(&1));
}

// zip
// only for generic

// zip_with
#[test]
fn zip_with() {
    let a = Something(1);
    let b = Something(2);
    let n = Nothing;

    assert_eq!(a.zip_with(b, |x, y| x + y), Something(3));
    assert_eq!(a.zip_with(n, |x, y| x + y), Nothing);
    assert_eq!(n.zip_with(b, |x, y| x + y), Nothing);
    assert_eq!(n.zip_with(n, |x, y| x + y), Nothing);
}
