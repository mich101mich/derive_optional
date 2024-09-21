use derive_optional::Optional;

#[derive(Optional, Debug, PartialEq, Eq, Clone, Copy)]
enum TestType {
    Something(usize),
    Nothing,
}
use TestType::*;

// map
// only for generic

// inspect
#[test]
fn inspect() {
    let x = Something(1);
    let mut y = 0;
    assert_eq!(x.inspect(|x| y = *x), Something(1));
    assert_eq!(y, 1);

    let x = Nothing;
    let mut y = 0;
    assert_eq!(x.inspect(|x| y = *x), Nothing);
    assert_eq!(y, 0);
}

// map_or
#[test]
fn map_or() {
    let x = Something(1);
    assert_eq!(x.map_or(0.0, |x| x as f32), 1.0);

    let x = Nothing;
    assert_eq!(x.map_or(0.0, |x| x as f32), 0.0);
}

// map_or_else
#[test]
fn map_or_else() {
    let x = Something(1);
    assert_eq!(x.map_or_else(|| 0.0, |x| x as f32), 1.0);

    let x = Nothing;
    assert_eq!(x.map_or_else(|| 0.0, |x| x as f32), 0.0);
}

// ok_or
#[test]
fn ok_or() {
    let x = Something(1);
    assert_eq!(x.ok_or("error"), Ok(1));

    let x = Nothing;
    assert_eq!(x.ok_or("error"), Err("error"));
}

// ok_or_else
#[test]
fn ok_or_else() {
    let x = Something(1);
    assert_eq!(x.ok_or_else(|| "error"), Ok(1));

    let x = Nothing;
    assert_eq!(x.ok_or_else(|| "error"), Err("error"));
}

// as_deref
// only for generic

// as_deref_mut
// only for generic
