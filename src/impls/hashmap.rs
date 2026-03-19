use std::collections::{BTreeMap, HashMap};
pub use std::hash::Hash;

use crate::{Set, impl_map, impl_map_owned_operations, impl_map_ref_operations};

impl_map!(HashMap, Key: Hash + Eq);
impl_map_ref_operations!(HashMap, HashMap, Key: Hash + Eq + Clone);
impl_map_ref_operations!(HashMap, BTreeMap, Key: Hash + Ord + Eq + Clone);
impl_map_owned_operations!(HashMap, HashMap, Key: Hash + Eq);
impl_map_owned_operations!(HashMap, BTreeMap, Key: Hash + Ord + Eq);

#[cfg(feature = "phf")]
mod phf_impl {
    use super::*;
    use phf::{Map as PhfMap, OrderedMap as PhfOrderedMap, PhfHash};
    use phf_shared::PhfBorrow;
    impl_map_ref_operations!(HashMap, PhfMap, Key: (Hash + PhfHash + PhfBorrow<Key> + Eq + Clone), entries);
    impl_map_ref_operations!(HashMap, PhfOrderedMap, Key: (Hash + PhfHash + PhfBorrow<Key> + Eq + Clone), entries);
}

#[cfg(test)]
mod tests {
    use core::fmt::Debug;

    use crate::operations::{DifferenceAssign, UnionAssign};
    use maplit::hashmap;
    use rstest::*;

    #[allow(unused_imports)]
    use super::*;

    #[rstest]
    #[case(hashmap! {
        0 => true,
        1 => true,
    }, hashmap! {
        0 => true,
        1 => true,
    }, hashmap! {
        0 => true,
        1 => true,
    })]
    #[case(hashmap! {
        0 => true,
        1 => true,
    }, hashmap! {
        1 => true,
    }, hashmap! {
        0 => true,
        1 => true,
    })]
    #[case(hashmap! {
        1 => true,
    }, hashmap! {
        0 => true,
    }, hashmap! {
        0 => true,
        1 => true,
    })]
    #[case(hashmap! {
        1 => true,
    }, hashmap! {
        1 => true,
    }, hashmap! {
        1 => true,
    })]
    fn union_list_tests<K, V>(
        #[case] mut list1: HashMap<K, V>,
        #[case] list2: HashMap<K, V>,
        #[case] result: HashMap<K, V>,
    ) where
        K: Hash + Eq + Clone + Debug,
        V: Set + PartialEq + Clone + Debug,
        for<'a> V: UnionAssign<&'a V>,
    {
        list1.union_assign(&list2);

        assert_eq!(list1, result);
    }

    #[rstest]
    #[case(hashmap! {
        0 => true,
        1 => true,
    }, hashmap! {
        0 => true,
        1 => true,
    }, hashmap! {})]
    #[case(hashmap! {
        0 => true,
        1 => true,
    }, hashmap! {
        1 => true,
    }, hashmap! {
        0 => true,
    })]
    #[case(hashmap! {
        1 => true,
    }, hashmap! {
        0 => true,
    }, hashmap! {
        1 => true,
    })]
    #[case(hashmap! {
        1 => true,
    }, hashmap! {
        1 => true,
    }, hashmap! {})]
    fn difference_list_tests<K, V>(
        #[case] mut list1: HashMap<K, V>,
        #[case] list2: HashMap<K, V>,
        #[case] result: HashMap<K, V>,
    ) where
        K: Hash + Eq + Clone + Debug,
        V: Set + PartialEq + Clone + Debug,
        for<'a> V: DifferenceAssign<&'a V>,
    {
        list1.difference_assign(&list2);

        assert_eq!(list1, result);
    }
}
