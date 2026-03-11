//! This module contains implementations of set-traits for `HashMap`. Of note is the lack of [`UniversalSet`] and [`Complement`] implementations, since they can't be implemented sensibly. Instead, you can use [`WildcardHashMap`] from the [`collections`] module, which has wildcard values which solve the problem of universal sets and complements.

use std::collections::HashMap;
pub use std::hash::Hash;

use crate::Set;
use crate::operations::{
    DifferenceAssign, DisjunctiveUnionAssign, Intersection, IntersectionAssign, UnionAssign,
};

pub(crate) fn remove_empty_keys<K, V: Set>(map: &mut HashMap<K, V>) {
    map.retain(|_key, value| !value.is_empty());
}

impl<Key: Hash + Eq + Clone, Value> Set for HashMap<Key, Value> {
    type Empty = Self;

    fn is_empty(&self) -> bool {
        HashMap::is_empty(self)
    }

    fn empty() -> Self::Empty {
        HashMap::new()
    }
}

// List A <-> List B implementations
impl<Key: Hash + Eq + Clone, Value: Clone, OtherValue: Clone + Set + Into<Value>>
    UnionAssign<&HashMap<Key, OtherValue>> for HashMap<Key, Value>
where
    for<'a> Value: UnionAssign<&'a OtherValue>,
{
    fn union_assign(&mut self, other: &HashMap<Key, OtherValue>) {
        for (key, other_value) in other.iter() {
            if let Some(self_value) = self.get_mut(key) {
                self_value.union_assign(other_value);
            } else if !other_value.is_empty() {
                self.insert(key.clone(), other_value.clone().into());
            }
        }
    }
}

impl<Key: Hash + Eq + Clone, Value, OtherValue> DifferenceAssign<&HashMap<Key, OtherValue>>
    for HashMap<Key, Value>
where
    for<'a> Value: DifferenceAssign<&'a OtherValue>,
{
    fn difference_assign(&mut self, other: &HashMap<Key, OtherValue>) {
        for (key, other_value) in other.iter() {
            let Some(value) = self.get_mut(key) else {
                continue;
            };

            value.difference_assign(other_value);
        }

        remove_empty_keys(self);
    }
}

impl<Key: Hash + Eq + Clone, Value, OtherValue> IntersectionAssign<&HashMap<Key, OtherValue>>
    for HashMap<Key, Value>
where
    for<'a> Value: IntersectionAssign<&'a OtherValue>,
{
    fn intersection_assign(&mut self, other: &HashMap<Key, OtherValue>) {
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

impl<Key: Hash + Eq + Clone, Value, OtherValue> Intersection<&HashMap<Key, OtherValue>>
    for HashMap<Key, Value>
where
    for<'a> Value: IntersectionAssign<&'a OtherValue>,
{
    type Output = Self;
    fn intersection(mut self, other: &HashMap<Key, OtherValue>) -> Self::Output {
        self.intersection_assign(other);

        self
    }
}

impl<Key: Hash + Eq + Clone, Value: Clone, OtherValue: Into<Value> + Clone>
    DisjunctiveUnionAssign<&HashMap<Key, OtherValue>> for HashMap<Key, Value>
where
    for<'a> Value: DisjunctiveUnionAssign<&'a OtherValue>,
{
    fn disjunctive_union_assign(&mut self, rhs: &HashMap<Key, OtherValue>) {
        for (key, value) in self.iter_mut() {
            let Some(other_value) = rhs.get(key) else {
                continue;
            };

            value.disjunctive_union_assign(other_value);
        }

        //For all keys that don't exist on self (but exist on rhs), add them.
        for (key, value) in rhs.iter() {
            if self.contains_key(key) {
                continue;
            }

            self.insert(key.clone(), value.clone().into());
        }

        remove_empty_keys(self);
    }
}

#[cfg(test)]
mod tests {
    use core::fmt::Debug;

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
