use derive_optional::Optional;

#[derive(Optional, Debug, PartialEq, Eq, Clone, Copy)]
enum TestType {
    Something(usize),
    Nothing,
}
use TestType::*;

// Self: From<#some_ty>
#[test]
fn self_from_some_ty() {
    let test = 1usize;
    assert_eq!(TestType::from(test), Something(1));
}

// Self: From<Option>
#[test]
fn self_from_option() {
    let test = Some(1usize);
    let result: TestType = test.into();
    assert_eq!(result, Something(1));

    let test = None;
    let result: TestType = test.into();
    assert_eq!(result, Nothing);
}

// Option: From<Self>
#[test]
fn option_from_self() {
    let test = Something(1);
    let result: Option<usize> = test.into();
    assert_eq!(result, Some(1));

    let test = Nothing;
    let result: Option<usize> = test.into();
    assert_eq!(result, None);
}

// Self: Default
#[test]
fn self_default() {
    let test: TestType = Default::default();
    assert_eq!(test, Nothing);
}

// Self: IntoIterator
#[test]
fn self_into_iter() {
    let test = Something(1);
    let mut iter = test.into_iter();
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), None);

    let test = Nothing;
    let mut iter = test.into_iter();
    assert_eq!(iter.next(), None);
}

// Self: std::ops::Try
// unstable
#[cfg(feature = "try_op")]
#[test]
fn self_try() {
    let mut y = 0;
    let res = || -> TestType {
        y = 1;
        Something(1)?;
        y = 2;
        Nothing?;
        y = 3;
        Something(2)
    }();
    assert_eq!(res, Nothing);
    assert_eq!(y, 2);

    y = 0;
    let res = || -> Option<usize> {
        y = 1;
        Something(1)?;
        y = 2;
        Nothing?;
        y = 3;
        Some(2)
    }();
    assert_eq!(res, None);
    assert_eq!(y, 2);

    y = 0;
    let res = || -> TestType {
        y = 1;
        Some(1)?;
        y = 2;
        None?;
        y = 3;
        Something(2)
    }();
    assert_eq!(res, Nothing);
    assert_eq!(y, 2);
}
