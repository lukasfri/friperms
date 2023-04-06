use friperms::{DifferenceInPlace, IntersectionInPlace, Set, UnionInPlace};

#[derive(Set, UnionInPlace, DifferenceInPlace, IntersectionInPlace, PartialEq, Debug)]
struct Test1 {
    field1: bool,
    field2: bool,
}

#[test]
fn derive_test_is_empty() {
    let value1 = Test1 {
        field1: false,
        field2: false,
    };
    assert!(value1.is_empty());

    let value2 = Test1 {
        field1: true,
        field2: false,
    };
    assert!(!value2.is_empty());
}

#[test]
fn derive_test_empty() {
    let value1 = Test1 {
        field1: false,
        field2: false,
    };
    assert_eq!(Test1::empty(), value1);
}

#[test]
fn derive_test() {
    let mut value1 = Test1 {
        field1: false,
        field2: true,
    };
    let value2 = Test1 {
        field1: true,
        field2: false,
    };

    value1.union_in_place(&value2);

    let result = Test1 {
        field1: true,
        field2: true,
    };
    assert_eq!(value1, result);
}
