use crate::Set;
use crate::operations::{Difference, DifferenceAssign, IntersectionAssign, Union, UnionAssign};
use std::{collections::HashMap, hash::Hash, ops::Deref};

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Set-variation of a key-value list with optional wildcard. Does operations on sub-values based on key.
pub struct WildcardHashMap<Key: Hash + Eq + Clone, Value: Set> {
    wildcard_exceptions: HashMap<Key, Value>,
    wildcard_value: Box<Value>,
    rest_list: HashMap<Key, Value>,
}

impl<Key: Hash + Eq + Clone, Value: Set> WildcardHashMap<Key, Value> {
    pub fn new(wildcard_value: Value) -> Self {
        Self {
            wildcard_exceptions: HashMap::empty(),
            wildcard_value: Box::new(wildcard_value),
            rest_list: HashMap::empty(),
        }
    }
}

impl<Key: Hash + Eq + Clone, Value: Set<Empty = Value>> Set for WildcardHashMap<Key, Value> {
    type Empty = Self;

    fn is_empty(&self) -> bool {
        self.rest_list.is_empty() && self.wildcard_value.is_empty()
    }

    fn empty() -> Self::Empty {
        WildcardHashMap {
            wildcard_exceptions: HashMap::empty(),
            wildcard_value: Box::new(Value::empty()),
            rest_list: HashMap::empty(),
        }
    }
}

impl<Key: Hash + Eq + Clone, Value: Set<Empty = Value>> Default for WildcardHashMap<Key, Value> {
    fn default() -> Self {
        Self::empty()
    }
}

impl<Key: Hash + Eq + Clone, Value: Set<Empty = Value>> From<HashMap<Key, Value>>
    for WildcardHashMap<Key, Value>
{
    fn from(rest_list: HashMap<Key, Value>) -> Self {
        WildcardHashMap {
            rest_list,
            ..Default::default()
        }
    }
}

// WildcardList A <-> List B
impl<Key: Hash + Eq + Clone, Value: Set<Empty = Value>, OtherValue: Clone>
    UnionAssign<&HashMap<Key, OtherValue>> for WildcardHashMap<Key, Value>
where
    for<'a> Value: DifferenceAssign<&'a Value>
        + IntersectionAssign<&'a OtherValue>
        + UnionAssign<&'a OtherValue>,
    for<'a> OtherValue: DifferenceAssign<&'a Value>,
{
    fn union_assign(&mut self, rhs: &HashMap<Key, OtherValue>) {
        for (key, value) in rhs.iter() {
            //For each key, for the intersection that is covered by the wildcard and value, remove it from the exceptions for this key.
            //For the rest (that is not part of the intersection), add it to the rest list.
            self.wildcard_value.intersection_assign(value);

            let mut rest = value.clone();

            rest.difference_assign(&self.wildcard_value);

            if !self.wildcard_value.is_empty() {
                let mut remove: bool = false;
                if let Some(val) = self.wildcard_exceptions.get_mut(key) {
                    val.difference_assign(&self.wildcard_value);

                    remove = val.is_empty();
                };

                if remove {
                    self.wildcard_exceptions.remove(key);
                };
            }

            if !rest.is_empty() {
                self.rest_list
                    .entry(key.clone())
                    .or_insert_with(Value::empty)
                    .union_assign(&rest);
            }
        }
    }
}

impl<Key: Hash + Eq + Clone, Value: Set<Empty = Value>, OtherValue: Clone>
    Union<&HashMap<Key, OtherValue>> for WildcardHashMap<Key, Value>
where
    for<'a> Value: DifferenceAssign<&'a Value>
        + IntersectionAssign<&'a OtherValue>
        + UnionAssign<&'a OtherValue>,
    for<'a> OtherValue: DifferenceAssign<&'a Value>,
{
    type Output = Self;

    fn union(mut self, rhs: &HashMap<Key, OtherValue>) -> Self::Output {
        self.union_assign(rhs);
        self
    }
}

impl<Key: Hash + Eq + Clone, Value: Set<Empty = Value>, OtherValue: Set<Empty = OtherValue>>
    DifferenceAssign<&HashMap<Key, OtherValue>> for WildcardHashMap<Key, Value>
where
    for<'a> Value: DifferenceAssign<&'a OtherValue>
        + IntersectionAssign<&'a OtherValue>
        + UnionAssign<&'a Value>,
{
    fn difference_assign(&mut self, rhs: &HashMap<Key, OtherValue>) {
        self.rest_list.difference_assign(rhs);

        for (key, value) in rhs.iter() {
            self.wildcard_value.intersection_assign(value);

            // Whatever intersection exists between the wildcard and the value of a key should be inserted as an exception on that key.
            if !self.wildcard_value.is_empty() {
                self.wildcard_exceptions
                    .entry(key.clone())
                    .or_insert_with(Value::empty)
                    .union_assign(&self.wildcard_value);
            }
        }
    }
}

impl<Key: Hash + Eq + Clone, Value: Set<Empty = Value>, OtherValue: Set<Empty = OtherValue>>
    Difference<&HashMap<Key, OtherValue>> for WildcardHashMap<Key, Value>
where
    for<'a> Value: DifferenceAssign<&'a OtherValue>
        + IntersectionAssign<&'a OtherValue>
        + UnionAssign<&'a Value>,
{
    type Output = Self;

    fn difference(mut self, rhs: &HashMap<Key, OtherValue>) -> Self::Output {
        self.difference_assign(rhs);
        self
    }
}

// WildcardList A <-> WildcardList B
impl<Key, Value, OtherValue> UnionAssign<&WildcardHashMap<Key, OtherValue>>
    for WildcardHashMap<Key, Value>
where
    Key: Hash + Eq + Clone,
    Value: Set<Empty = Value> + Clone,
    for<'a> Value: DifferenceAssign<&'a Value>
        + DifferenceAssign<&'a OtherValue>
        + UnionAssign<&'a OtherValue>,
    OtherValue: Set<Empty = OtherValue> + Clone,
    for<'a> OtherValue: DifferenceAssign<&'a Value> + DifferenceAssign<&'a OtherValue>,
    for<'a> HashMap<Key, Value>: UnionAssign<&'a HashMap<Key, OtherValue>>,
{
    fn union_assign(&mut self, rhs: &WildcardHashMap<Key, OtherValue>) {
        let mut cleaned_rhs_wildcard_exceptions = rhs.wildcard_exceptions.clone();

        /// This function removes covered exceptions from a wildcard value (and it's associated exceptions).
        fn remove_covered_values<Key, Value, OtherValue>(
            exceptions: &mut HashMap<Key, OtherValue>,
            wildcard_value: &Value,
            wildcard_exceptions: &HashMap<Key, Value>,
        ) where
            Key: Hash + Eq + Clone,
            Value: Set<Empty = Value> + Clone,
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

            crate::impls::hashmap::remove_empty_keys(exceptions);
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
        self.wildcard_value.union_assign(rhs.wildcard_value.deref());

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

impl<Key, Value, OtherValue> Union<&WildcardHashMap<Key, OtherValue>>
    for WildcardHashMap<Key, Value>
where
    Key: Hash + Eq + Clone,
    Value: Set<Empty = Value> + Clone,
    for<'a> Value: DifferenceAssign<&'a Value>
        + DifferenceAssign<&'a OtherValue>
        + UnionAssign<&'a OtherValue>,
    OtherValue: Set<Empty = OtherValue> + Clone,
    for<'a> OtherValue: DifferenceAssign<&'a Value> + DifferenceAssign<&'a OtherValue>,
    for<'a> HashMap<Key, Value>: UnionAssign<&'a HashMap<Key, OtherValue>>,
{
    type Output = Self;

    fn union(mut self, rhs: &WildcardHashMap<Key, OtherValue>) -> Self::Output {
        self.union_assign(rhs);
        self
    }
}

impl<Key, Value, OtherValue> DifferenceAssign<&WildcardHashMap<Key, OtherValue>>
    for WildcardHashMap<Key, Value>
where
    Key: Hash + Eq + Clone,
    Value: Set<Empty = Value> + Clone,
    for<'a> Value: DifferenceAssign<&'a Value>
        + DifferenceAssign<&'a OtherValue>
        + IntersectionAssign<&'a OtherValue>
        + UnionAssign<&'a OtherValue>
        + UnionAssign<&'a Value>,
    OtherValue: Set<Empty = OtherValue> + Clone,
    for<'a> OtherValue: IntersectionAssign<&'a Value>,
{
    fn difference_assign(&mut self, rhs: &WildcardHashMap<Key, OtherValue>) {
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

            self.wildcard_exceptions
                .entry(key.clone())
                .or_insert_with(Value::empty)
                .union_assign(&value);
        }

        self.rest_list.difference_assign(&rhs.rest_list);
    }
}

impl<Key, Value, OtherValue> Difference<&WildcardHashMap<Key, OtherValue>>
    for WildcardHashMap<Key, Value>
where
    Key: Hash + Eq + Clone,
    Value: Set<Empty = Value> + Clone,
    for<'a> Value: DifferenceAssign<&'a Value>
        + DifferenceAssign<&'a OtherValue>
        + IntersectionAssign<&'a OtherValue>
        + UnionAssign<&'a OtherValue>
        + UnionAssign<&'a Value>,
    OtherValue: Set<Empty = OtherValue> + Clone,
    for<'a> OtherValue: IntersectionAssign<&'a Value>,
{
    type Output = Self;

    fn difference(mut self, rhs: &WildcardHashMap<Key, OtherValue>) -> Self::Output {
        self.difference_assign(rhs);
        self
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use rstest::*;

    #[allow(unused_imports)]
    use super::*;

    use maplit::hashmap;

    #[rstest]
    // WildcardList A <-> List B
    #[case(WildcardHashMap {
        wildcard_value: Box::new(false),
        wildcard_exceptions: hashmap! {},
        rest_list: hashmap! {
            0 => true,
        },
    }, hashmap! {
        1 => true,
    }, WildcardHashMap {
        wildcard_value: Box::new(false),
        wildcard_exceptions: hashmap! {},
        rest_list: hashmap! {
            0 => true,
            1 => true,
        }
    })]
    #[case(WildcardHashMap {
        wildcard_value: Box::new(true),
        wildcard_exceptions: hashmap! {},
        rest_list: hashmap! {},
    }, hashmap! {
        1 => true,
    }, WildcardHashMap {
        wildcard_value: Box::new(true),
        wildcard_exceptions: hashmap! {},
        rest_list: hashmap! {}
    })]
    #[case(WildcardHashMap {
        wildcard_value: Box::new(true),
        wildcard_exceptions: hashmap! {
            1 => true
        },
        rest_list: hashmap! {},
    }, hashmap! {
        1 => true,
    }, WildcardHashMap {
        wildcard_value: Box::new(true),
        wildcard_exceptions: hashmap! {},
        rest_list: hashmap! {}
    })]
    #[case(WildcardHashMap {
        wildcard_value: Box::new(true),
        wildcard_exceptions: hashmap! {
            1 => true,
            2 => true,
        },
        rest_list: hashmap! {},
    }, hashmap! {
        1 => true,
    }, WildcardHashMap {
        wildcard_value: Box::new(true),
        wildcard_exceptions: hashmap! {
            2 => true,
        },
        rest_list: hashmap! {}
    })]
    // WildcardList A <-> WildcardList B
    #[case(WildcardHashMap {
        wildcard_value: Box::new(true),
        wildcard_exceptions: hashmap! {
            2 => true,
        },
        rest_list: hashmap! {},
    }, WildcardHashMap {
        wildcard_value: Box::new(true),
        wildcard_exceptions: hashmap! {
            1 => true,
        },
        rest_list: hashmap! {},
    }, WildcardHashMap {
        wildcard_value: Box::new(true),
        wildcard_exceptions: hashmap! {},
        rest_list: hashmap! {}
    })]
    #[case(WildcardHashMap {
        wildcard_value: Box::new(true),
        wildcard_exceptions: hashmap! {
            1 => true,
            2 => true,
        },
        rest_list: hashmap! {},
    }, WildcardHashMap {
        wildcard_value: Box::new(true),
        wildcard_exceptions: hashmap! {
            1 => true,
        },
        rest_list: hashmap! {},
    }, WildcardHashMap {
        wildcard_value: Box::new(true),
        wildcard_exceptions: hashmap! {
            1 => true,
        },
        rest_list: hashmap! {}
    })]
    #[case(WildcardHashMap {
        wildcard_value: Box::new(hashmap! {
            1 => true
        }),
        wildcard_exceptions: hashmap! {
            2 => hashmap! {
                1 => true
            }
        },
        rest_list: hashmap! {},
    }, WildcardHashMap {
        wildcard_value: Box::new(hashmap! {
            2 => true
        }),
        wildcard_exceptions: hashmap! {
            1 => hashmap! {
                2 => true
            }
        },
        rest_list: hashmap! {
            2 => hashmap! {
                1 => true
            }
        },
    }, WildcardHashMap {
        wildcard_value: Box::new(hashmap! {
            1 => true,
            2 => true
        }),
        wildcard_exceptions: hashmap! {
            1 => hashmap! {
                2 => true
            }
        },
        rest_list: hashmap! {}
    })]
    fn union_list_tests<I1, I2, R>(#[case] mut list1: I1, #[case] list2: I2, #[case] result: R)
    where
        I1: PartialEq<R> + Debug,
        I2: Debug,
        R: Debug,
        for<'a> I1: UnionAssign<&'a I2>,
    {
        list1.union_assign(&list2);

        assert_eq!(list1, result);
    }

    #[rstest]
    #[case(WildcardHashMap {
        wildcard_value: Box::new(false),
        wildcard_exceptions: hashmap! {},
        rest_list: hashmap! {
            0 => true,
        },
    }, hashmap! {
        1 => true,
    }, WildcardHashMap {
        wildcard_value: Box::new(false),
        wildcard_exceptions: hashmap! {},
        rest_list: hashmap! {
            0 => true,
        }
    })]
    #[case(WildcardHashMap {
        wildcard_value: Box::new(true),
        wildcard_exceptions: hashmap! {},
        rest_list: hashmap! {},
    }, hashmap! {
        1 => true,
    }, WildcardHashMap {
        wildcard_value: Box::new(true),
        wildcard_exceptions: hashmap! {
            1 => true,
        },
        rest_list: hashmap! {}
    })]
    #[case(WildcardHashMap {
        wildcard_value: Box::new(true),
        wildcard_exceptions: hashmap! {
            1 => true
        },
        rest_list: hashmap! {},
    }, hashmap! {
        1 => true,
    }, WildcardHashMap {
        wildcard_value: Box::new(true),
        wildcard_exceptions: hashmap! {
            1 => true,
        },
        rest_list: hashmap! {}
    })]
    #[case(WildcardHashMap {
        wildcard_value: Box::new(true),
        wildcard_exceptions: hashmap! {
            2 => true,
        },
        rest_list: hashmap! {
        },
    }, hashmap! {
        1 => true,
    }, WildcardHashMap {
        wildcard_value: Box::new(true),
        wildcard_exceptions: hashmap! {
            2 => true,
            1 => true,
        },
        rest_list: hashmap! {}
    })]
    // WildcardList A <-> WildcardList B
    #[case(WildcardHashMap::<i32, bool> {
        wildcard_value: Box::new(true),
        wildcard_exceptions: hashmap! {},
        rest_list: hashmap! {},
    }, WildcardHashMap {
        wildcard_value: Box::new(true),
        wildcard_exceptions: hashmap! {},
        rest_list: hashmap! {},
    }, WildcardHashMap {
        wildcard_value: Box::new(false),
        wildcard_exceptions: hashmap! {},
        rest_list: hashmap! {}
    })]
    #[case(WildcardHashMap {
        wildcard_value: Box::new(true),
        wildcard_exceptions: hashmap! {},
        rest_list: hashmap! {},
    }, WildcardHashMap {
        wildcard_value: Box::new(true),
        wildcard_exceptions: hashmap! {
            1 => true,
        },
        rest_list: hashmap! {},
    }, WildcardHashMap {
        wildcard_value: Box::new(false),
        wildcard_exceptions: hashmap! {},
        rest_list: hashmap! {
            1 => true,
        }
    })]
    #[case(WildcardHashMap {
        wildcard_value: Box::new(true),
        wildcard_exceptions: hashmap! {},
        rest_list: hashmap! {},
    }, WildcardHashMap {
        wildcard_value: Box::new(false),
        wildcard_exceptions: hashmap! {},
        rest_list: hashmap! {
            1 => true,
        },
    }, WildcardHashMap {
        wildcard_value: Box::new(true),
        wildcard_exceptions: hashmap! {
            1 => true,
        },
        rest_list: hashmap! {}
    })]
    #[case(WildcardHashMap {
        wildcard_value: Box::new(hashmap! {
            1 => true,
            2 => true
        }),
        wildcard_exceptions: hashmap! {},
        rest_list: hashmap! {},
    }, WildcardHashMap {
        wildcard_value: Box::new(hashmap! {
            1 => true
        }),
        wildcard_exceptions: hashmap! {
            2 => hashmap! {
                1 => true
            }
        },
        rest_list: hashmap! {},
    }, WildcardHashMap {
        wildcard_value: Box::new(hashmap! {
            2 => true
        }),
        wildcard_exceptions: hashmap! {},
        rest_list: hashmap! {
            2 => hashmap! {
                1 => true
            }
        }
    })]
    fn difference_list_tests<I1, I2, R>(#[case] mut list1: I1, #[case] list2: I2, #[case] result: R)
    where
        I1: PartialEq<R> + Debug,
        I2: Debug,
        R: Debug,
        for<'a> I1: DifferenceAssign<&'a I2>,
    {
        list1.difference_assign(&list2);

        assert_eq!(list1, result);
    }

    #[test]
    fn test_add() {
        let tree_1 = hashmap! {
            1 => WildcardHashMap {
                rest_list: hashmap! {},
                wildcard_exceptions: hashmap! {},
                wildcard_value: Box::new(hashmap! {
                    15 => true,
                })
            }
        };

        let tree_2 = hashmap! {
            1 => hashmap! {
                5 => hashmap! {
                    15 => true,
                    5 => true,
                },
            },
        };

        let mut tree_1_minus_2 = tree_1.clone();
        tree_1_minus_2.difference_assign(&tree_2);

        let result = hashmap! {
          1 => WildcardHashMap {
            rest_list: hashmap! {},
            wildcard_exceptions: hashmap! {
                5 => hashmap! {
                    15 => true,
                },
            },
            wildcard_value: Box::new(hashmap! {
                15 => true,
            }),
          }
        };

        assert_eq!(tree_1_minus_2, result);

        tree_1_minus_2.union_assign(&tree_2);
        //Does not equal tree_1 because 1.5.5 has been added.
        assert_ne!(tree_1, tree_1_minus_2);
    }
}
