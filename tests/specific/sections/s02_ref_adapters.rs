use derive_optional::Optional;

#[derive(Optional, Debug, PartialEq, Eq, Clone, Copy)]
enum TestType {
    Something(usize),
    Nothing,
}
use TestType::*;

// as_ref
// only for generic

// as_mut
// only for generic

// as_pin_ref
// only for generic

// as_pin_mut
// only for generic

// as_slice
#[test]
fn as_slice() {
    let test = Something(1);
    assert_eq!(test.as_slice(), &[1][..]);

    let test = Nothing;
    let rhs: &[usize] = &[]; // can't infer the type on an empty slice since multiple PartialEq impls exist
    assert_eq!(test.as_slice(), rhs);
}
