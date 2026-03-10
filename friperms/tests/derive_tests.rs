use friperms::{Set, operations::UnionAssign};
use friperms_derive::{DifferenceAssign, IntersectionAssign, Set, UnionAssign};

#[derive(Set, UnionAssign, DifferenceAssign, IntersectionAssign, PartialEq, Debug)]
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

#[rstest::rstest]
#[case::both_false(Test1 {
    field1: false,
    field2: false,
}, Test1 {
    field1: false,
    field2: false,
}, Test1 {
    field1: false,
    field2: false,
})]
#[case::self_empty(Test1 {
    field1: false,
    field2: false,
}, Test1 {
    field1: true,
    field2: true,
}, Test1 {
    field1: true,
    field2: true,
})]
#[case::rhs_empty(Test1 {
    field1: true,
    field2: true,
}, Test1 {
    field1: false,
    field2: false,
}, Test1 {
    field1: true,
    field2: true,
})]
#[case::both_true(Test1 {
    field1: true,
    field2: true,
}, Test1 {
    field1: true,
    field2: true,
}, Test1 {
    field1: true,
    field2: true,
})]
#[case::combination(Test1 {
    field1: true,
    field2: false,
}, Test1 {
    field1: false,
    field2: true,
}, Test1 {
    field1: true,
    field2: true,
})]
fn derive_test(#[case] mut a: Test1, #[case] b: Test1, #[case] c: Test1) {
    a.union_assign(&b);
    assert_eq!(a, c);
}
