use derive_optional::Optional;

#[derive(Optional, Debug, PartialEq, Eq, Clone, Copy)]
enum TestType<T: std::fmt::Debug + PartialEq> {
    Something(T),
    Nothing,
}
use TestType::*;

// as_ref
#[test]
fn as_ref() {
    let test = Something(1);
    if let Something(x) = test.as_ref() {
        assert_eq!(x, &1);
    }
    assert_eq!(test.as_ref(), Something(&1));

    let test: TestType<usize> = Nothing;
    if let Something(_) = test.as_ref() {
        panic!("This should not be executed");
    }
    assert_eq!(test.as_ref(), Nothing);
}

// as_mut
#[test]
fn as_mut() {
    let mut test = Something(1);
    if let Something(x) = test.as_mut() {
        *x = 2;
    }
    assert_eq!(test, Something(2));

    let mut test: TestType<usize> = Nothing;
    if let Something(_) = test.as_mut() {
        panic!("This should not be executed");
    }
    assert_eq!(test, Nothing);
}

// as_pin_ref
#[test]
fn as_pin_ref() {
    let val = Something(1);
    let test = std::pin::Pin::new(&val);
    if let Something(x) = test.as_pin_ref() {
        assert_eq!(*x, 1);
    } else {
        panic!("This should not be executed");
    }

    let val: TestType<usize> = Nothing;
    let test = std::pin::Pin::new(&val);
    if let Something(_) = test.as_pin_ref() {
        panic!("This should not be executed");
    }
}

// as_pin_mut
#[test]
fn as_pin_mut() {
    let mut val = Something(1);
    let test = std::pin::Pin::new(&mut val);
    if let Something(mut x) = test.as_pin_mut() {
        *x = 2;
    }
    assert_eq!(val, Something(2));

    let mut val: TestType<usize> = Nothing;
    let test = std::pin::Pin::new(&mut val);
    if let Something(_) = test.as_pin_mut() {
        panic!("This should not be executed");
    }
    assert_eq!(val, Nothing);
}
