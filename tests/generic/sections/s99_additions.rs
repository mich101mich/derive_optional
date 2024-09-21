use derive_optional::Optional;

#[derive(Optional, Debug, PartialEq, Eq, Clone, Copy)]
enum TestType<T: std::fmt::Debug + PartialEq> {
    Something(T),
    Nothing,
}
use TestType::*;

// as_option_ref
#[test]
fn as_option_ref() {
    let test = Something(0);
    assert_eq!(test.as_option_ref(), Some(&0));

    let test: TestType<usize> = Nothing;
    assert_eq!(test.as_option_ref(), None);
}

// as_option_mut
#[test]
fn as_option_mut() {
    let mut test = Something(0);
    {
        let ref_mut = test.as_option_mut();
        assert_eq!(ref_mut, Some(&mut 0));
        *ref_mut.unwrap() = 1;
    }
    assert_eq!(test, Something(1));

    let mut test: TestType<usize> = Nothing;
    assert_eq!(test.as_option_mut(), None);
}
