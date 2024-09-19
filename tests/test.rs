mod generic {}

mod specific {
    mod external {}
    mod sections {
        mod s01_querying;
        mod s02_ref_adapters;
        mod s03_get_contained;
        mod s04_transformers;
        mod s05_bool_ops;
        mod s05_iters;
        mod s06_entry_ops;
        mod s07_misc;
        mod s99_additions;
    }
}

use derive_optional::Optional;
#[derive(Optional, Clone, Copy)]
enum HasChanged<T> {
    Changed(T),
    NoChange,
}

#[test]
fn happy_path() {
    // Querying the contained values
    let mut change = HasChanged::Changed(1);
    assert!(change.is_changed());
    assert!(!change.is_no_change());

    change = HasChanged::NoChange;
    assert!(!change.is_changed());
    assert!(change.is_no_change());

    // Adapter for working with references
    let mut change = HasChanged::Changed(11);

    let inner = change.as_ref().unwrap();
    assert_eq!(inner, &11);

    let inner = change.as_mut().unwrap();
    *inner = 12;
    assert_eq!(change.unwrap(), 12);

    // Getting to contained values
    let inner = HasChanged::Changed(21).unwrap();
    assert_eq!(inner, 21);

    let inner = HasChanged::Changed(22).expect("unreachable message");
    assert_eq!(inner, 22);

    // From/Into conversions
    let op: Option<usize> = HasChanged::NoChange.into();
    assert_eq!(op, None);

    let op: Option<_> = HasChanged::Changed(31i64).into();
    assert_eq!(op, Some(31i64));

    let change: HasChanged<usize> = Some(32).into();
    assert_eq!(change.unwrap(), 32);

    let change: HasChanged<usize> = None.into();
    assert!(change.is_no_change());
}

#[test]
#[should_panic(expected = "called `HasChanged::unwrap()` on a `NoChange` value")]
fn test_unwrap_panic() {
    HasChanged::<usize>::NoChange.unwrap();
}

#[test]
#[should_panic(expected = "my custom message")]
fn test_expect_panic() {
    HasChanged::<usize>::NoChange.expect("my custom message");
}

// TODO: check generic, lifetime, and where clause

// #[test]
// fn test() {
//     let t = trybuild::TestCases::new();
//     t.pass("passing/*.rs");
// }
