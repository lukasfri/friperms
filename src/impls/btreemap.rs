//! This module contains implementations of set-traits for `BTreeMap`. Of note is the lack of [`UniversalSet`] and [`Complement`] implementations, since they can't be implemented sensibly. Instead, you can use [`WildcardBTreeMap`] from the [`collections`] module, which has wildcard values which solve the problem of universal sets and complements.

use std::collections::BTreeMap;

use crate::Set;
use crate::operations::{
    DifferenceAssign, DisjunctiveUnionAssign, Intersection, IntersectionAssign, UnionAssign,
};

pub(crate) fn remove_empty_keys<K: Ord, V: Set>(map: &mut BTreeMap<K, V>) {
    map.retain(|_key, value| !value.is_empty());
}

impl<Key: Ord + Eq + Clone, Value> Set for BTreeMap<Key, Value> {
    type Empty = Self;

    fn is_empty(&self) -> bool {
        BTreeMap::is_empty(self)
    }

    fn empty() -> Self::Empty {
        BTreeMap::new()
    }
}

// List A <-> List B implementations
impl<Key: Ord + Eq + Clone, Value: Clone, OtherValue: Clone + Set + Into<Value>>
    UnionAssign<&BTreeMap<Key, OtherValue>> for BTreeMap<Key, Value>
where
    for<'a> Value: UnionAssign<&'a OtherValue>,
{
    fn union_assign(&mut self, other: &BTreeMap<Key, OtherValue>) {
        for (key, other_value) in other.iter() {
            if let Some(self_value) = self.get_mut(key) {
                self_value.union_assign(other_value);
            } else if !other_value.is_empty() {
                self.insert(key.clone(), other_value.clone().into());
            }
        }
    }
}

impl<Key: Ord + Eq + Clone, Value, OtherValue> DifferenceAssign<&BTreeMap<Key, OtherValue>>
    for BTreeMap<Key, Value>
where
    for<'a> Value: DifferenceAssign<&'a OtherValue>,
{
    fn difference_assign(&mut self, other: &BTreeMap<Key, OtherValue>) {
        for (key, other_value) in other.iter() {
            let Some(value) = self.get_mut(key) else {
                continue;
            };

            value.difference_assign(other_value);
        }

        remove_empty_keys(self);
    }
}

impl<Key: Ord + Eq + Clone, Value, OtherValue> IntersectionAssign<&BTreeMap<Key, OtherValue>>
    for BTreeMap<Key, Value>
where
    for<'a> Value: IntersectionAssign<&'a OtherValue>,
{
    fn intersection_assign(&mut self, other: &BTreeMap<Key, OtherValue>) {
        //Remove all that don't exist at all in other
        self.retain(|key, _value| other.get(key).is_some());

        for (key, value) in self.iter_mut() {
            let other_value = other
                .get(key)
                .expect("Removed all keys above that don't exist in other.");

            value.intersection_assign(other_value);
        }
    }
}

impl<Key: Ord + Eq + Clone, Value, OtherValue> Intersection<&BTreeMap<Key, OtherValue>>
    for BTreeMap<Key, Value>
where
    for<'a> Value: IntersectionAssign<&'a OtherValue>,
{
    type Output = Self;
    fn intersection(mut self, other: &BTreeMap<Key, OtherValue>) -> Self::Output {
        self.intersection_assign(other);

        self
    }
}

impl<Key: Ord + Eq + Clone, Value: Clone, OtherValue: Into<Value> + Clone>
    DisjunctiveUnionAssign<&BTreeMap<Key, OtherValue>> for BTreeMap<Key, Value>
where
    for<'a> Value: DisjunctiveUnionAssign<&'a OtherValue>,
{
    fn disjunctive_union_assign(&mut self, rhs: &BTreeMap<Key, OtherValue>) {
        //For all keys that don't exist on self (but exist on rhs), add them.
        for (key, other_value) in rhs.iter() {
            if let Some(value) = self.get_mut(key) {
                value.disjunctive_union_assign(other_value);
            } else {
                self.insert(key.clone(), other_value.clone().into());
            }
        }

        remove_empty_keys(self);
    }
}

#[cfg(test)]
mod tests {
    use core::fmt::Debug;

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
