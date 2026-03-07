use crate::{DifferenceAssign, Intersection, IntersectionAssign, KVListSet, Set, UnionAssign};
use std::{hash::Hash, ops::Deref};

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Set-variation of a key-value list with optional wildcard. Does operations on sub-values based on key.
pub struct KVListWithWildcardSet<Key: Hash + Eq + Clone, Value: Set> {
    pub wildcard_exceptions: KVListSet<Key, Value>,
    pub wildcard_value: Box<Value>,
    pub rest_list: KVListSet<Key, Value>,
}

impl<Key: Hash + Eq + Clone, Value: Set> KVListWithWildcardSet<Key, Value> {}

impl<Key: Hash + Eq + Clone, Value: Set> Set for KVListWithWildcardSet<Key, Value> {
    fn is_empty(&self) -> bool {
        self.rest_list.is_empty() && self.wildcard_value.is_empty()
    }

    fn empty() -> Self {
        KVListWithWildcardSet {
            wildcard_exceptions: KVListSet::empty(),
            wildcard_value: Box::new(Value::empty()),
            rest_list: KVListSet::empty(),
        }
    }
}

impl<Key: Hash + Eq + Clone, Value: Set> Default for KVListWithWildcardSet<Key, Value> {
    fn default() -> Self {
        Self::empty()
    }
}

impl<Key: Hash + Eq + Clone, Value: Set> From<KVListSet<Key, Value>>
    for KVListWithWildcardSet<Key, Value>
{
    fn from(rest_list: KVListSet<Key, Value>) -> Self {
        KVListWithWildcardSet {
            rest_list,
            ..Default::default()
        }
    }
}

// WildcardList A <-> List B
impl<Key: Hash + Eq + Clone, Value: Set + Clone, OtherValue: Clone>
    UnionAssign<&KVListSet<Key, OtherValue>> for KVListWithWildcardSet<Key, Value>
where
    for<'a> Value: DifferenceAssign<&'a Value>
        + Intersection<&'a OtherValue, Output = Value>
        + UnionAssign<&'a OtherValue>
        + From<OtherValue>,
    for<'a> OtherValue: DifferenceAssign<&'a Value>,
    for<'a> KVListSet<Key, Value>: Intersection<&'a KVListSet<Key, OtherValue>>,
{
    fn union_assign(&mut self, rhs: &KVListSet<Key, OtherValue>) {
        for (key, value) in rhs.iter() {
            //For each key, for the intersection that is covered by the wildcard and value, remove it from the exceptions for this key.
            //For the rest (that is not part of the intersection), add it to the rest list.
            let wildcard_value = self.wildcard_value.clone().intersection(value);

            let mut rest = value.clone();

            rest.difference_assign(&wildcard_value);

            if !wildcard_value.is_empty() {
                let mut remove: bool = false;
                if let Some(val) = self.wildcard_exceptions.get_mut(key) {
                    val.difference_assign(&wildcard_value);

                    remove = val.is_empty();
                };

                if remove {
                    self.wildcard_exceptions.remove(key);
                };
            }

            if !rest.is_empty() {
                self.rest_list
                    .entry(key.clone())
                    .and_modify(|entry| entry.union_assign(&rest))
                    .or_insert(rest.into());
            }
        }
    }
}

impl<Key: Hash + Eq + Clone, Value: Set + Clone, OtherValue>
    DifferenceAssign<&KVListSet<Key, OtherValue>> for KVListWithWildcardSet<Key, Value>
where
    for<'a> Value: DifferenceAssign<&'a OtherValue>
        + Intersection<&'a OtherValue, Output = Value>
        + UnionAssign<&'a Value>,
{
    fn difference_assign(&mut self, rhs: &KVListSet<Key, OtherValue>) {
        self.rest_list.difference_assign(rhs);

        for (key, value) in rhs.iter() {
            let wildcard_value = self.wildcard_value.clone().intersection(value);

            // Whatever intersection exists between the wildcard and the value of a key should be inserted as an exception on that key.
            if !wildcard_value.is_empty() {
                self.wildcard_exceptions
                    .entry(key.clone())
                    .and_modify(|entry| entry.union_assign(&wildcard_value))
                    .or_insert(wildcard_value);
            }
        }
    }
}

// WildcardList A <-> WildcardList B
impl<Key: Hash + Eq + Clone, Value: Set + Clone, OtherValue: Set + Clone>
    UnionAssign<&KVListWithWildcardSet<Key, OtherValue>> for KVListWithWildcardSet<Key, Value>
where
    for<'a> Value: DifferenceAssign<&'a Value>
        + DifferenceAssign<&'a OtherValue>
        + UnionAssign<&'a OtherValue>,
    for<'a> OtherValue: DifferenceAssign<&'a Value> + DifferenceAssign<&'a OtherValue>,
    for<'a> KVListSet<Key, Value>: UnionAssign<&'a KVListSet<Key, OtherValue>>,
{
    fn union_assign(&mut self, rhs: &KVListWithWildcardSet<Key, OtherValue>) {
        let mut cleaned_rhs_wildcard_exceptions = rhs.wildcard_exceptions.clone();

        /// This function removes covered exceptions from a wildcard value (and it's associated exceptions).
        fn remove_covered_values<Key: Hash + Eq + Clone, Value: Set + Clone, OtherValue>(
            exceptions: &mut KVListSet<Key, OtherValue>,
            wildcard_value: &Value,
            wildcard_exceptions: &KVListSet<Key, Value>,
        ) where
            for<'a> Value: DifferenceAssign<&'a Value>, // + DifferenceAssign<&'a OtherValue>,
            for<'a> OtherValue: DifferenceAssign<&'a Value>, // + DifferenceAssign<&'a OtherValue>,
        {
            for (key, other_exception) in exceptions.iter_mut() {
                if let Some(exception) = wildcard_exceptions.get(key) {
                    let mut wildcard_value = wildcard_value.clone();
                    wildcard_value.difference_assign(exception);
                    other_exception.difference_assign(&wildcard_value);
                } else {
                    other_exception.difference_assign(wildcard_value);
                };
            }

            exceptions.remove_empty_keys();
        }

        // Remove exceptions in rhs covered by selfs wildcard.
        remove_covered_values(
            &mut cleaned_rhs_wildcard_exceptions,
            self.wildcard_value.as_ref(),
            &self.wildcard_exceptions,
        );
        cleaned_rhs_wildcard_exceptions.difference_assign(&self.rest_list);

        // Remove exceptions in self covered by rhs' wildcard.
        remove_covered_values(
            &mut self.wildcard_exceptions,
            rhs.wildcard_value.as_ref(),
            &rhs.wildcard_exceptions,
        );
        self.wildcard_exceptions.difference_assign(&rhs.rest_list);

        // Merge the exception lists and the wildcards.
        self.wildcard_exceptions
            .union_assign(&cleaned_rhs_wildcard_exceptions);
        self.wildcard_value.union_assign(&rhs.wildcard_value);

        // Merge rest lists.
        self.rest_list.union_assign(&rhs.rest_list);

        // Remove values in rest list covered by new wildcard.
        remove_covered_values(
            &mut self.rest_list,
            self.wildcard_value.as_ref(),
            &self.wildcard_exceptions,
        );
    }
}

impl<Key: Hash + Eq + Clone, Value: Set + Clone, OtherValue: Set + Clone>
    DifferenceAssign<&KVListWithWildcardSet<Key, OtherValue>> for KVListWithWildcardSet<Key, Value>
where
    for<'a> Value: DifferenceAssign<&'a Value>
        + DifferenceAssign<&'a OtherValue>
        + IntersectionAssign<&'a OtherValue>
        + UnionAssign<&'a OtherValue>
        + UnionAssign<&'a Value>
        + From<OtherValue>,
    for<'a> OtherValue: IntersectionAssign<&'a Value>,
{
    fn difference_assign(&mut self, rhs: &KVListWithWildcardSet<Key, OtherValue>) {
        //If exception exists for X key, that value should not be removed for that key.
        //That means, if there is an intersection between that exception and the wildcard value, it should be added to the rest list.
        for (key, other_exception) in rhs.wildcard_exceptions.iter() {
            let mut value = self.wildcard_value.deref().clone();

            if let Some(exception) = self.wildcard_exceptions.get(key) {
                value.difference_assign(exception);
            }

            value.intersection_assign(other_exception);

            if value.is_empty() {
                continue;
            }

            if let Some(rest_value) = self.rest_list.get_mut(key) {
                rest_value.union_assign(&value);
            } else {
                self.rest_list.insert(key.clone(), value);
            }
        }
        //Remove rhs wildcard from self wildcard.
        self.wildcard_value
            .difference_assign(rhs.wildcard_value.as_ref());

        // If any rest list items in rhs intersect with the self wildcard, add them to the exceptions.
        // Subtract any rest list items in self with rhs.

        for (key, value) in rhs.rest_list.iter() {
            let mut value = value.clone();

            value.intersection_assign(&self.wildcard_value);

            if value.is_empty() {
                continue;
            }

            if let Some(exception) = self.wildcard_exceptions.get_mut(key) {
                exception.union_assign(&value);
            } else {
                self.wildcard_exceptions.insert(key.clone(), value.into());
            }
        }

        self.rest_list.difference_assign(&rhs.rest_list);
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use rstest::*;

    #[allow(unused_imports)]
    use super::*;

    use crate::kv_list_set;

    #[rstest]
    // WildcardList A <-> List B
    #[case(KVListWithWildcardSet {
        wildcard_value: Box::new( false),
        wildcard_exceptions: kv_list_set! {},
        rest_list: kv_list_set! {
            0 => true,
        },
    }, kv_list_set! {
        1 => true,
    }, KVListWithWildcardSet {
        wildcard_value: Box::new( false),
        wildcard_exceptions: kv_list_set! {},
        rest_list: kv_list_set! {
            0 => true,
            1 => true,
        }
    })]
    #[case(KVListWithWildcardSet {
        wildcard_value: Box::new( true),
        wildcard_exceptions: kv_list_set! {},
        rest_list: kv_list_set! {},
    }, kv_list_set! {
        1 => true,
    }, KVListWithWildcardSet {
        wildcard_value: Box::new( true),
        wildcard_exceptions: kv_list_set! {},
        rest_list: kv_list_set! {}
    })]
    #[case(KVListWithWildcardSet {
        wildcard_value: Box::new( true),
        wildcard_exceptions: kv_list_set! {
            1 => true
        },
        rest_list: kv_list_set! {},
    }, kv_list_set! {
        1 => true,
    }, KVListWithWildcardSet {
        wildcard_value: Box::new( true),
        wildcard_exceptions: kv_list_set! {},
        rest_list: kv_list_set! {}
    })]
    #[case(KVListWithWildcardSet {
        wildcard_value: Box::new( true),
        wildcard_exceptions: kv_list_set! {
            1 => true,
            2 => true,
        },
        rest_list: kv_list_set! {},
    }, kv_list_set! {
        1 => true,
    }, KVListWithWildcardSet {
        wildcard_value: Box::new( true),
        wildcard_exceptions: kv_list_set! {
            2 => true,
        },
        rest_list: kv_list_set! {}
    })]
    // WildcardList A <-> WildcardList B
    #[case(KVListWithWildcardSet {
        wildcard_value: Box::new( true),
        wildcard_exceptions: kv_list_set! {
            2 => true,
        },
        rest_list: kv_list_set! {},
    }, KVListWithWildcardSet {
        wildcard_value: Box::new( true),
        wildcard_exceptions: kv_list_set! {
            1 => true,
        },
        rest_list: kv_list_set! {},
    }, KVListWithWildcardSet {
        wildcard_value: Box::new( true),
        wildcard_exceptions: kv_list_set! {},
        rest_list: kv_list_set! {}
    })]
    #[case(KVListWithWildcardSet {
        wildcard_value: Box::new( true),
        wildcard_exceptions: kv_list_set! {
            1 => true,
            2 => true,
        },
        rest_list: kv_list_set! {},
    }, KVListWithWildcardSet {
        wildcard_value: Box::new( true),
        wildcard_exceptions: kv_list_set! {
            1 => true,
        },
        rest_list: kv_list_set! {},
    }, KVListWithWildcardSet {
        wildcard_value: Box::new( true),
        wildcard_exceptions: kv_list_set! {
            1 => true,
        },
        rest_list: kv_list_set! {}
    })]
    #[case(KVListWithWildcardSet {
        wildcard_value: Box::new(kv_list_set! {
            1 => true
        }),
        wildcard_exceptions: kv_list_set! {
            2 => kv_list_set! {
                1 => true
            }
        },
        rest_list: kv_list_set! {},
    }, KVListWithWildcardSet {
        wildcard_value: Box::new(kv_list_set! {
            2 => true
        }),
        wildcard_exceptions: kv_list_set! {
            1 => kv_list_set! {
                2 => true
            }
        },
        rest_list: kv_list_set! {
            2 => kv_list_set! {
                1 => true
            }
        },
    }, KVListWithWildcardSet {
        wildcard_value: Box::new(kv_list_set! {
            1 => true,
            2 => true
        }),
        wildcard_exceptions: kv_list_set! {
            1 => kv_list_set! {
                2 => true
            }
        },
        rest_list: kv_list_set! {}
    })]
    fn union_list_tests<I1: PartialEq<R> + Debug, I2: Debug, R: Debug>(
        #[case] mut list1: I1,
        #[case] list2: I2,
        #[case] result: R,
    ) where
        for<'a> I1: UnionAssign<&'a I2>,
    {
        list1.union_assign(&list2);

        assert_eq!(list1, result);
    }

    #[rstest]
    #[case(KVListWithWildcardSet {
        wildcard_value: Box::new( false),
        wildcard_exceptions: kv_list_set! {},
        rest_list: kv_list_set! {
            0 => true,
        },
    }, kv_list_set! {
        1 => true,
    }, KVListWithWildcardSet {
        wildcard_value: Box::new( false),
        wildcard_exceptions: kv_list_set! {},
        rest_list: kv_list_set! {
            0 => true,
        }
    })]
    #[case(KVListWithWildcardSet {
        wildcard_value: Box::new( true),
        wildcard_exceptions: kv_list_set! {},
        rest_list: kv_list_set! {},
    }, kv_list_set! {
        1 => true,
    }, KVListWithWildcardSet {
        wildcard_value: Box::new( true),
        wildcard_exceptions: kv_list_set! {
            1 => true,
        },
        rest_list: kv_list_set! {}
    })]
    #[case(KVListWithWildcardSet {
        wildcard_value: Box::new( true),
        wildcard_exceptions: kv_list_set! {
            1 => true
        },
        rest_list: kv_list_set! {},
    }, kv_list_set! {
        1 => true,
    }, KVListWithWildcardSet {
        wildcard_value: Box::new( true),
        wildcard_exceptions: kv_list_set! {
            1 => true,
        },
        rest_list: kv_list_set! {}
    })]
    #[case(KVListWithWildcardSet {
        wildcard_value: Box::new( true),
        wildcard_exceptions: kv_list_set! {
            2 => true,
        },
        rest_list: kv_list_set! {
        },
    }, kv_list_set! {
        1 => true,
    }, KVListWithWildcardSet {
        wildcard_value: Box::new( true),
        wildcard_exceptions: kv_list_set! {
            2 => true,
            1 => true,
        },
        rest_list: kv_list_set! {}
    })]
    // WildcardList A <-> WildcardList B
    #[case(KVListWithWildcardSet::<i32, bool> {
        wildcard_value: Box::new(true),
        wildcard_exceptions: kv_list_set! {},
        rest_list: kv_list_set! {},
    }, KVListWithWildcardSet {
        wildcard_value: Box::new(true),
        wildcard_exceptions: kv_list_set! {},
        rest_list: kv_list_set! {},
    }, KVListWithWildcardSet {
        wildcard_value: Box::new(false),
        wildcard_exceptions: kv_list_set! {},
        rest_list: kv_list_set! {}
    })]
    #[case(KVListWithWildcardSet {
        wildcard_value: Box::new(true),
        wildcard_exceptions: kv_list_set! {},
        rest_list: kv_list_set! {},
    }, KVListWithWildcardSet {
        wildcard_value: Box::new(true),
        wildcard_exceptions: kv_list_set! {
            1 => true,
        },
        rest_list: kv_list_set! {},
    }, KVListWithWildcardSet {
        wildcard_value: Box::new(false),
        wildcard_exceptions: kv_list_set! {},
        rest_list: kv_list_set! {
            1 => true,
        }
    })]
    #[case(KVListWithWildcardSet {
        wildcard_value: Box::new(true),
        wildcard_exceptions: kv_list_set! {},
        rest_list: kv_list_set! {},
    }, KVListWithWildcardSet {
        wildcard_value: Box::new(false),
        wildcard_exceptions: kv_list_set! {},
        rest_list: kv_list_set! {
            1 => true,
        },
    }, KVListWithWildcardSet {
        wildcard_value: Box::new(true),
        wildcard_exceptions: kv_list_set! {
            1 => true,
        },
        rest_list: kv_list_set! {}
    })]
    #[case(KVListWithWildcardSet {
        wildcard_value: Box::new(kv_list_set! {
            1 => true,
            2 => true
        }),
        wildcard_exceptions: kv_list_set! {},
        rest_list: kv_list_set! {},
    }, KVListWithWildcardSet {
        wildcard_value: Box::new(kv_list_set! {
            1 => true
        }),
        wildcard_exceptions: kv_list_set! {
            2 => kv_list_set! {
                1 => true
            }
        },
        rest_list: kv_list_set! {},
    }, KVListWithWildcardSet {
        wildcard_value: Box::new(kv_list_set! {
            2 => true
        }),
        wildcard_exceptions: kv_list_set! {},
        rest_list: kv_list_set! {
            2 => kv_list_set! {
                1 => true
            }
        }
    })]
    fn difference_list_tests<I1: PartialEq<R> + Debug, I2: Debug, R: Debug>(
        #[case] mut list1: I1,
        #[case] list2: I2,
        #[case] result: R,
    ) where
        for<'a> I1: DifferenceAssign<&'a I2>,
    {
        list1.difference_assign(&list2);

        assert_eq!(list1, result);
    }
}
