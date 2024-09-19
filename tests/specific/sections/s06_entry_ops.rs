use derive_optional::Optional;

#[derive(Optional, Debug, PartialEq, Eq, Clone, Copy)]
enum TestType {
    Something(usize),
    Nothing,
}
use TestType::*;

// insert
#[test]
fn insert() {
    let mut test = Something(1);
    assert_eq!(test.insert(2), &mut 2);
    assert_eq!(test, Something(2));

    let mut test = Nothing;
    assert_eq!(test.insert(2), &mut 2);
    assert_eq!(test, Something(2));
}

// get_or_insert
#[test]
fn get_or_insert() {
    let mut test = Something(1);
    assert_eq!(test.get_or_insert(2), &mut 1);
    assert_eq!(test, Something(1));

    let mut test = Nothing;
    assert_eq!(test.get_or_insert(2), &mut 2);
    assert_eq!(test, Something(2));
}

// get_or_insert_default
#[test]
fn get_or_insert_default() {
    let mut test = Something(1);
    assert_eq!(test.get_or_insert_default(), &mut 1);
    assert_eq!(test, Something(1));

    let mut test = Nothing;
    assert_eq!(test.get_or_insert_default(), &mut 0);
    assert_eq!(test, Something(0));
}

// get_or_insert_with
#[test]
fn get_or_insert_with() {
    let mut test = Something(1);
    assert_eq!(test.get_or_insert_with(|| 2), &mut 1);
    assert_eq!(test, Something(1));

    let mut test = Nothing;
    assert_eq!(test.get_or_insert_with(|| 2), &mut 2);
    assert_eq!(test, Something(2));
}
