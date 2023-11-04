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
// unstable

// map_or
// only for generic

// map_or_else
// only for generic

// ok_or
#[test]
fn ok_or() {
    let x = Something(1);
    assert_eq!(x.ok_or("error"), Ok(1));

    let x = TestType::Nothing;
    assert_eq!(x.ok_or("error"), Err("error"));
}

// ok_or_else
#[test]
fn ok_or_else() {
    let x = Something(1);
    assert_eq!(x.ok_or_else(|| "error"), Ok(1));

    let x = TestType::Nothing;
    assert_eq!(x.ok_or_else(|| "error"), Err("error"));
}

// as_deref
// only for generic

// as_deref_mut
// only for generic
