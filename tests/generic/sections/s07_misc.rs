use derive_optional::Optional;

#[derive(Optional, Debug, PartialEq, Eq, Clone, Copy)]
enum TestType<T: std::fmt::Debug + PartialEq> {
    Something(T),
    Nothing,
}
use TestType::*;

// take
#[test]
fn take() {
    let mut test = Something(1);
    assert_eq!(test.take(), Something(1));
    assert_eq!(test, Nothing);

    let mut test: TestType<usize> = Nothing;
    assert_eq!(test.take(), Nothing);
    assert_eq!(test, Nothing);
}

// replace
#[test]
fn replace() {
    let mut test = Something(1);
    assert_eq!(test.replace(2), Something(1));
    assert_eq!(test, Something(2));

    let mut test: TestType<usize> = Nothing;
    assert_eq!(test.replace(2), Nothing);
    assert_eq!(test, Something(2));
}

// contains
#[test]
fn contains() {
    let test = Something(1);
    assert!(test.contains(&1));
    assert!(!test.contains(&2));

    let test: TestType<usize> = Nothing;
    assert!(!test.contains(&1));
}

// zip
#[test]
fn zip() {
    let a = Something(1);
    let b = Something("hi");
    let n: TestType<f32> = Nothing;

    assert_eq!(a.zip(b), Something((1, "hi")));
    assert_eq!(b.zip(a), Something(("hi", 1)));
    assert_eq!(a.zip(n), Nothing);
    assert_eq!(n.zip(b), Nothing);
    assert_eq!(n.zip(n), Nothing);
}

// zip_with
#[test]
fn zip_with() {
    let a = Something(1);
    let b = Something("hi");
    let n: TestType<f32> = Nothing;

    assert_eq!(
        a.zip_with(b, |x, y| format!("{} {}", x, y)),
        Something(String::from("1 hi"))
    );
    assert_eq!(
        b.zip_with(a, |x, y| format!("{} {}", x, y)),
        Something(String::from("hi 1"))
    );
    assert_eq!(a.zip_with(n, |x, y| format!("{} {}", x, y)), Nothing);
    assert_eq!(n.zip_with(b, |x, y| format!("{} {}", x, y)), Nothing);
    assert_eq!(n.zip_with(n, |x, y| format!("{} {}", x, y)), Nothing);
}
