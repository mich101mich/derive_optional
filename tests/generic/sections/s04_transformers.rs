use derive_optional::Optional;

#[derive(Optional, Debug, PartialEq, Eq, Clone, Copy)]
enum TestType<T: std::fmt::Debug + PartialEq> {
    Something(T),
    Nothing,
}
use TestType::*;

// map
#[test]
fn map() {
    let x = Something(1);
    assert_eq!(x.map(|x| x as f32), Something(1.0));

    let x: TestType<usize> = Nothing;
    assert_eq!(x.map(|x| x as f32), Nothing);
}

// inspect
#[test]
fn inspect() {
    let x = Something(1);
    let mut y = 0;
    assert_eq!(x.inspect(|x| y = *x), Something(1));
    assert_eq!(y, 1);

    let x: TestType<usize> = Nothing;
    let mut y = 0;
    assert_eq!(x.inspect(|x| y = *x), Nothing);
    assert_eq!(y, 0);
}

// map_or
#[test]
fn map_or() {
    let x = Something(1);
    assert_eq!(x.map_or(0.0, |x| x as f32), 1.0);

    let x: TestType<usize> = Nothing;
    assert_eq!(x.map_or(0.0, |x| x as f32), 0.0);
}

// map_or_else
#[test]
fn map_or_else() {
    let x = Something(1);
    assert_eq!(x.map_or_else(|| 0.0, |x| x as f32), 1.0);

    let x: TestType<usize> = Nothing;
    assert_eq!(x.map_or_else(|| 0.0, |x| x as f32), 0.0);
}

// ok_or
#[test]
fn ok_or() {
    let x = Something(1);
    assert_eq!(x.ok_or("error"), Ok(1));

    let x: TestType<usize> = Nothing;
    assert_eq!(x.ok_or("error"), Err("error"));
}

// ok_or_else
#[test]
fn ok_or_else() {
    let x = Something(1);
    assert_eq!(x.ok_or_else(|| "error"), Ok(1));

    let x: TestType<usize> = Nothing;
    assert_eq!(x.ok_or_else(|| "error"), Err("error"));
}

// as_deref
#[test]
fn as_deref() {
    let x = Something(String::from("1"));
    assert_eq!(x.as_deref(), Something("1"));

    let x: TestType<String> = Nothing;
    assert_eq!(x.as_deref(), Nothing);
}

// as_deref_mut
#[test]
fn as_deref_mut() {
    let mut x = Something(vec![0]);
    if let Something(x) = x.as_deref_mut() {
        x[0] = 1;
    }
    assert_eq!(x, Something(vec![1]));

    let mut x: TestType<Vec<usize>> = Nothing;
    if let Something(_) = x.as_deref_mut() {
        panic!("This should not be executed");
    }
    assert_eq!(x, Nothing);
}
