pub use std::hash::Hash;
use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

use crate::{
    DifferenceInPlace, DisjunctiveUnionInPlace, Intersection, IntersectionInPlace, Set,
    UnionInPlace,
};

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Set-variation of a key-value list. Does operations on sub-values based on key.
pub struct KVListSet<Key: Hash + Eq + Clone, Value>(HashMap<Key, Value>);

impl<Key: Hash + Eq + Clone, Value> KVListSet<Key, Value> {
    pub fn new() -> Self {
        KVListSet(HashMap::new())
    }
}

impl<Key: Hash + Eq + Clone, Value: Set> KVListSet<Key, Value> {
    pub(crate) fn remove_empty_keys(&mut self) {
        self.retain(|_key, value| !value.is_empty());
    }
}

impl<Key: Hash + Eq + Clone, Value> Default for KVListSet<Key, Value> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Key: Hash + Eq + Clone, Value> Set for KVListSet<Key, Value> {
    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    fn empty() -> Self {
        KVListSet::new()
    }
}

// List A <-> List B implementations
impl<Key: Hash + Eq + Clone, Value: Clone, OtherValue: Clone + Set + Into<Value>>
    UnionInPlace<&KVListSet<Key, OtherValue>> for KVListSet<Key, Value>
where
    for<'a> Value: UnionInPlace<&'a OtherValue>,
{
    fn union_in_place(&mut self, other: &KVListSet<Key, OtherValue>) {
        for (key, other_value) in other.iter() {
            if let Some(self_value) = self.get_mut(key) {
                self_value.union_in_place(other_value);
            } else if !other_value.is_empty() {
                self.insert(key.clone(), other_value.clone().into());
            }
        }
    }
}

impl<Key: Hash + Eq + Clone, Value, OtherValue> DifferenceInPlace<&KVListSet<Key, OtherValue>>
    for KVListSet<Key, Value>
where
    for<'a> Value: DifferenceInPlace<&'a OtherValue>,
{
    fn difference_in_place(&mut self, other: &KVListSet<Key, OtherValue>) {
        for (key, other_value) in other.iter() {
            let Some(value) = self.get_mut(key) else {
                continue;
            };

            value.difference_in_place(other_value);
        }

        self.remove_empty_keys()
    }
}

impl<Key: Hash + Eq + Clone, Value, OtherValue> IntersectionInPlace<&KVListSet<Key, OtherValue>>
    for KVListSet<Key, Value>
where
    for<'a> Value: IntersectionInPlace<&'a OtherValue>,
{
    fn intersection_in_place(&mut self, other: &KVListSet<Key, OtherValue>) {
        //Remove all that don't exist at all in other
        self.retain(|key, _value| other.get(key).is_some());

        for (key, value) in self.iter_mut() {
            let other_value = other
                .get(key)
                .expect("Removed all keys above that don't exist in other.");

            value.intersection_in_place(other_value);
        }
    }
}

impl<Key: Hash + Eq + Clone, Value, OtherValue> Intersection<&KVListSet<Key, OtherValue>>
    for KVListSet<Key, Value>
where
    for<'a> Value: IntersectionInPlace<&'a OtherValue>,
{
    type Output = Self;
    fn intersection(mut self, other: &KVListSet<Key, OtherValue>) -> Self::Output {
        self.intersection_in_place(other);

        self
    }
}

impl<Key: Hash + Eq + Clone, Value: Clone, OtherValue: Into<Value> + Clone>
    DisjunctiveUnionInPlace<&KVListSet<Key, OtherValue>> for KVListSet<Key, Value>
where
    for<'a> Value: DisjunctiveUnionInPlace<&'a OtherValue>,
{
    fn disjunctive_union_in_place(&mut self, rhs: &KVListSet<Key, OtherValue>) {
        for (key, value) in self.iter_mut() {
            let Some(other_value) = rhs.get(key) else {
                continue;
            };

            value.disjunctive_union_in_place(other_value);
        }

        //For all keys that don't exist on self (but exist on rhs), add them.
        for (key, value) in rhs.iter() {
            if self.contains_key(key) {
                continue;
            }

            self.insert(key.clone(), value.clone().into());
        }

        self.remove_empty_keys();
    }
}

impl<Key: Hash + Eq + Clone, Value> From<HashMap<Key, Value>> for KVListSet<Key, Value> {
    fn from(list: HashMap<Key, Value>) -> Self {
        KVListSet(list)
    }
}

impl<Key: Hash + Eq + Clone, Value> Deref for KVListSet<Key, Value> {
    type Target = HashMap<Key, Value>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<Key: Hash + Eq + Clone, Value> DerefMut for KVListSet<Key, Value> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[macro_export]
macro_rules! kv_list_set {
    ($($key:expr => $val:expr),* ,) => (
        $crate::kv_list_set!($($key => $val),*)
    );
    ($($key:expr => $val:expr),*) => ({
        #[allow(unused_mut)]
        let mut map = $crate::KVListSet::new();
        $( map.insert($key.to_owned(), $val); )*
        map
    });
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use rstest::*;

    #[allow(unused_imports)]
    use super::*;

    #[rstest]
    #[case(kv_list_set! {
        0 => true,
        1 => true,
    }, kv_list_set! {
        0 => true,
        1 => true,
    }, kv_list_set! {
        0 => true,
        1 => true,
    })]
    #[case(kv_list_set! {
        0 => true,
        1 => true,
    }, kv_list_set! {
        1 => true,
    }, kv_list_set! {
        0 => true,
        1 => true,
    })]
    #[case(kv_list_set! {
        1 => true,
    }, kv_list_set! {
        0 => true,
    }, kv_list_set! {
        0 => true,
        1 => true,
    })]
    #[case(kv_list_set! {
        1 => true,
    }, kv_list_set! {
        1 => true,
    }, kv_list_set! {
        1 => true,
    })]
    fn union_list_tests<K: Hash + Eq + Clone + Debug, V: Set + PartialEq + Clone + Debug>(
        #[case] mut list1: KVListSet<K, V>,
        #[case] list2: KVListSet<K, V>,
        #[case] result: KVListSet<K, V>,
    ) where
        for<'a> V: UnionInPlace<&'a V>,
    {
        list1.union_in_place(&list2);

        assert_eq!(list1, result);
    }

    #[rstest]
    #[case(kv_list_set! {
        0 => true,
        1 => true,
    }, kv_list_set! {
        0 => true,
        1 => true,
    }, kv_list_set! {})]
    #[case(kv_list_set! {
        0 => true,
        1 => true,
    }, kv_list_set! {
        1 => true,
    }, kv_list_set! {
        0 => true,
    })]
    #[case(kv_list_set! {
        1 => true,
    }, kv_list_set! {
        0 => true,
    }, kv_list_set! {
        1 => true,
    })]
    #[case(kv_list_set! {
        1 => true,
    }, kv_list_set! {
        1 => true,
    }, kv_list_set! {})]
    fn difference_list_tests<K: Hash + Eq + Clone + Debug, V: Set + PartialEq + Clone + Debug>(
        #[case] mut list1: KVListSet<K, V>,
        #[case] list2: KVListSet<K, V>,
        #[case] result: KVListSet<K, V>,
    ) where
        for<'a> V: DifferenceInPlace<&'a V>,
    {
        list1.difference_in_place(&list2);

        assert_eq!(list1, result);
    }
}
