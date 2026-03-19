use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;

use crate::{Set, impl_map_owned_operations, impl_map_ref_operations};

use crate::impl_map;

impl_map!(BTreeMap, Key: Ord + Eq);
impl_map_ref_operations!(BTreeMap, BTreeMap, Key: Ord + Eq + Clone);
impl_map_ref_operations!(BTreeMap, HashMap, Key: Hash + Ord + Eq + Clone);
impl_map_owned_operations!(BTreeMap, BTreeMap, Key: Ord + Eq);
impl_map_owned_operations!(BTreeMap, HashMap, Key: Hash + Ord + Eq);

#[cfg(test)]
mod tests {
    use core::fmt::Debug;

    use crate::operations::{DifferenceAssign, UnionAssign};
    use maplit::btreemap;
    use rstest::*;

    #[allow(unused_imports)]
    use super::*;

    #[rstest]
    #[case(btreemap! {
        0 => true,
        1 => true,
    }, btreemap! {
        0 => true,
        1 => true,
    }, btreemap! {
        0 => true,
        1 => true,
    })]
    #[case(btreemap! {
        0 => true,
        1 => true,
    }, btreemap! {
        1 => true,
    }, btreemap! {
        0 => true,
        1 => true,
    })]
    #[case(btreemap! {
        1 => true,
    }, btreemap! {
        0 => true,
    }, btreemap! {
        0 => true,
        1 => true,
    })]
    #[case(btreemap! {
        1 => true,
    }, btreemap! {
        1 => true,
    }, btreemap! {
        1 => true,
    })]
    fn union_list_tests<K, V>(
        #[case] mut list1: BTreeMap<K, V>,
        #[case] list2: BTreeMap<K, V>,
        #[case] result: BTreeMap<K, V>,
    ) where
        K: Ord + Eq + Clone + Debug,
        V: Set + PartialEq + Clone + Debug,
        for<'a> V: UnionAssign<&'a V>,
    {
        list1.union_assign(&list2);

        assert_eq!(list1, result);
    }

    #[rstest]
    #[case(btreemap! {
        0 => true,
        1 => true,
    }, btreemap! {
        0 => true,
        1 => true,
    }, btreemap! {})]
    #[case(btreemap! {
        0 => true,
        1 => true,
    }, btreemap! {
        1 => true,
    }, btreemap! {
        0 => true,
    })]
    #[case(btreemap! {
        1 => true,
    }, btreemap! {
        0 => true,
    }, btreemap! {
        1 => true,
    })]
    #[case(btreemap! {
        1 => true,
    }, btreemap! {
        1 => true,
    }, btreemap! {})]
    fn difference_list_tests<K, V>(
        #[case] mut list1: BTreeMap<K, V>,
        #[case] list2: BTreeMap<K, V>,
        #[case] result: BTreeMap<K, V>,
    ) where
        K: Ord + Eq + Clone + Debug,
        V: Set + PartialEq + Clone + Debug,
        for<'a> V: DifferenceAssign<&'a V>,
    {
        list1.difference_assign(&list2);

        assert_eq!(list1, result);
    }
}
