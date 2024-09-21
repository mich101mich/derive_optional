use derive_optional::Optional;

#[derive(Optional, Debug, PartialEq, Eq, Clone, Copy)]
enum TestType<T: std::fmt::Debug + PartialEq> {
    Something(T),
    Nothing,
}
use TestType::*;

// iter
#[test]
fn iter() {
    let test = Something(1);
    let mut iter = test.iter();
    assert_eq!(iter.size_hint(), (1, Some(1)));
    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), None);

    let test: TestType<usize> = Nothing;
    let mut iter = test.iter();
    assert_eq!(iter.size_hint(), (0, Some(0)));
    assert_eq!(iter.next(), None);
}

// iter_mut
#[test]
fn iter_mut() {
    let mut test = Something(1);
    let mut iter = test.iter_mut();
    assert_eq!(iter.size_hint(), (1, Some(1)));
    assert_eq!(iter.next(), Some(&mut 1));
    assert_eq!(iter.next(), None);

    let mut test: TestType<usize> = Nothing;
    let mut iter = test.iter_mut();
    assert_eq!(iter.size_hint(), (0, Some(0)));
    assert_eq!(iter.next(), None);
}
