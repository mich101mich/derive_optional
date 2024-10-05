use derive_optional::Optional;

#[derive(Optional, Debug, PartialEq, Eq, Clone, Copy)]
enum TestType<T: std::fmt::Debug + PartialEq> {
    Something(T),
    Nothing,
}
use TestType::*;

// unzip on TestType<(T, U)>
#[test]
fn test_unzip() {
    let x: TestType<(i32, i32)> = Something((1, 2));
    assert_eq!(x.unzip(), (Something(1), Something(2)));
    let x: TestType<(i32, i32)> = Nothing;
    assert_eq!(x.unzip(), (Nothing, Nothing));
}

// copied on #name<&#some_ty>
#[test]
fn test_copied_ref() {
    let x: TestType<&i32> = Something(&1);
    assert_eq!(x.copied(), Something(1));
    let x: TestType<&i32> = Nothing;
    assert_eq!(x.copied(), Nothing);
}

// cloned on #name<&#some_ty>
#[test]
fn test_cloned_ref() {
    let x: TestType<&i32> = Something(&1);
    assert_eq!(x.cloned(), Something(1));
    let x: TestType<&i32> = Nothing;
    assert_eq!(x.cloned(), Nothing);
}

// copied on #name<&mut #some_ty>
#[test]
fn test_copied_mut() {
    let mut input = 1;
    let x: TestType<&mut i32> = Something(&mut input);
    assert_eq!(x.copied(), Something(1));
    let x: TestType<&mut i32> = Nothing;
    assert_eq!(x.copied(), Nothing);
}

// cloned on #name<&mut #some_ty>
#[test]
fn test_cloned_mut() {
    let mut input = 1;
    let x: TestType<&mut i32> = Something(&mut input);
    assert_eq!(x.cloned(), Something(1));
    let x: TestType<&mut i32> = Nothing;
    assert_eq!(x.cloned(), Nothing);
}

// transpose on #name<Result<#some_ty, E>>
#[test]
fn test_transpose() {
    let x: TestType<Result<i32, &'static str>> = Something(Ok(1));
    assert_eq!(x.transpose(), Ok(Something(1)));
    let x: TestType<Result<i32, &'static str>> = Something(Err("error"));
    assert_eq!(x.transpose(), Err("error"));
    let x: TestType<Result<i32, &'static str>> = Nothing;
    assert_eq!(x.transpose(), Ok(Nothing));
}

// flatten on #name<#name<#some_ty>>
#[test]
fn test_flatten() {
    let x: TestType<TestType<i32>> = Something(Something(1));
    assert_eq!(x.flatten(), Something(1));
    let x: TestType<TestType<i32>> = Something(Nothing);
    assert_eq!(x.flatten(), Nothing);
    let x: TestType<TestType<i32>> = Nothing;
    assert_eq!(x.flatten(), Nothing);
}

// flatten on #name<Option<#some_ty>>
#[test]
fn test_flatten_option() {
    let x: TestType<Option<i32>> = Something(Some(1));
    assert_eq!(x.flatten(), Something(1));
    let x: TestType<Option<i32>> = Something(None);
    assert_eq!(x.flatten(), Nothing);
    let x: TestType<Option<i32>> = Nothing;
    assert_eq!(x.flatten(), Nothing);
}
