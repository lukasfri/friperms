#[macro_export]
macro_rules! impl_map {
    ($map:ident, Key: $($bounds:tt)*) => {
        pub(crate) fn remove_empty_keys<K: $($bounds)*, V: $crate::Set>(map: &mut $map<K, V>) {
            map.retain(|_key, value| !value.is_empty());
        }

        impl<Key: $($bounds)*, Value> $crate::Set for $map<Key, Value> {
            type Empty = Self;

            fn is_empty(&self) -> bool {
                $map::is_empty(self)
            }

            fn empty() -> Self::Empty {
                $map::new()
            }
        }
    }
}

#[macro_export]
macro_rules! impl_map_owned_operations {
    ($map:ident, $rhs_map:ident, Key: $($bounds:tt)*) => {
        impl<Key, Value, OtherValue>
            $crate::operations::UnionAssign<$rhs_map<Key, OtherValue>> for $map<Key, Value>
        where
            Key: $($bounds)*,
            Value: $crate::operations::UnionAssign<OtherValue> + Clone,
            OtherValue: Clone + Set + Into<Value>
        {
            fn union_assign(&mut self, other: $rhs_map<Key, OtherValue>) {
                for (key, other_value) in other.into_iter() {
                    if let Some(self_value) = self.get_mut(&key) {
                        $crate::operations::UnionAssign::union_assign(self_value, other_value);
                    } else if !other_value.is_empty() {
                        self.insert(key, other_value.into());
                    }
                }
            }
        }

        impl<Key, Value, OtherValue>
            $crate::operations::Union<$rhs_map<Key, OtherValue>> for $map<Key, Value>
        where
            Key: $($bounds)*,
            Value: $crate::operations::UnionAssign<OtherValue> + Clone,
            OtherValue: Clone + Set + Into<Value>
        {
            type Output = Self;
            fn union(mut self, other: $rhs_map<Key, OtherValue>) -> Self::Output {
                $crate::operations::UnionAssign::union_assign(&mut self, other);

                self
            }
        }

        impl<Key, Value, OtherValue> $crate::operations::DifferenceAssign<$rhs_map<Key, OtherValue>>
            for $map<Key, Value>
        where
            Key: $($bounds)*,
            Value: $crate::operations::DifferenceAssign< OtherValue>,
        {
            fn difference_assign(&mut self, other: $rhs_map<Key, OtherValue>) {
                for (key, other_value) in other.into_iter() {
                    let Some(value) = self.get_mut(&key) else {
                        continue;
                    };

                    $crate::operations::DifferenceAssign::difference_assign(value, other_value);
                }

                remove_empty_keys(self);
            }
        }

        impl<Key, Value, OtherValue> $crate::operations::Difference<$rhs_map<Key, OtherValue>> for $map<Key, Value>
        where
            Key: $($bounds)*,
            Value: $crate::operations::DifferenceAssign< OtherValue>,
        {
            type Output = Self;
            fn difference(mut self, other: $rhs_map<Key, OtherValue>) -> Self::Output {
                $crate::operations::DifferenceAssign::difference_assign(&mut self, other);

                self
            }
        }

        impl<Key, Value, OtherValue> $crate::operations::IntersectionAssign<$rhs_map<Key, OtherValue>>
            for $map<Key, Value>
        where
            Key: $($bounds)*,
            Value: $crate::operations::IntersectionAssign<OtherValue>,
        {
            fn intersection_assign(&mut self, mut other: $rhs_map<Key, OtherValue>) {
                //Remove all that don't exist at all in other
                self.retain(|key, _value| other.get(key).is_some());

                for (key, value) in self.iter_mut() {
                    let other_value = other
                        .remove(key)
                        .expect("Removed all keys above that don't exist in other.");

                    $crate::operations::IntersectionAssign::intersection_assign(value, other_value);
                }
            }
        }

        impl<Key, Value, OtherValue> $crate::operations::Intersection<$rhs_map<Key, OtherValue>>
            for $map<Key, Value>
        where
            Key: $($bounds)*,
            Value: $crate::operations::IntersectionAssign<OtherValue>,
        {
            type Output = Self;
            fn intersection(mut self, other: $rhs_map<Key, OtherValue>) -> Self::Output {
                $crate::operations::IntersectionAssign::intersection_assign(&mut self, other);

                self
            }
        }

        impl<Key, Value, OtherValue>
            $crate::operations::DisjunctiveUnionAssign<$rhs_map<Key, OtherValue>> for $map<Key, Value>
        where
            Key: $($bounds)*,
            Value: $crate::operations::DisjunctiveUnionAssign<OtherValue>,
            OtherValue: Into<Value>,
        {
            fn disjunctive_union_assign(&mut self, rhs: $rhs_map<Key, OtherValue>) {
                //For all keys that don't exist on self (but exist on rhs), add them.
                for (key, other_value) in rhs.into_iter() {
                    if let Some(value) = self.get_mut(&key) {
                        $crate::operations::DisjunctiveUnionAssign::disjunctive_union_assign(value, other_value);
                    } else {
                        self.insert(key, other_value.into());
                    }
                }

                remove_empty_keys(self);
            }
        }

        impl<Key, Value, OtherValue> $crate::operations::DisjunctiveUnion<$rhs_map<Key, OtherValue>> for $map<Key, Value>
        where
            Key: $($bounds)*,
            Value: $crate::operations::DisjunctiveUnionAssign<OtherValue>,
            OtherValue: Into<Value>,
        {
            type Output = Self;
            fn disjunctive_union(mut self, rhs: $rhs_map<Key, OtherValue>) -> Self::Output {
                $crate::operations::DisjunctiveUnionAssign::disjunctive_union_assign(&mut self, rhs);

                self
            }
        }
    };
}

#[macro_export]
macro_rules! impl_map_ref_operations {
    ($map:ident, $rhs_map:ident, Key: ($($bounds:tt)*)) => {
      impl_map_ref_operations!($map, $rhs_map, Key: ($($bounds)*), iter);
    };
    ($map:ident, $rhs_map:ident, Key: ($($bounds:tt)*), $rhs_iter_func:ident) => {
        impl<Key, Value, OtherValue>
            $crate::operations::UnionAssign<&$rhs_map<Key, OtherValue>> for $map<Key, Value>
        where
            Key: $($bounds)*,
            Value: Clone,
            for<'a> Value: $crate::operations::UnionAssign<&'a OtherValue>,
            OtherValue: Clone + Set + Into<Value>,
        {
            fn union_assign(&mut self, other: &$rhs_map<Key, OtherValue>) {
                for (key, other_value) in other.$rhs_iter_func() {
                    if let Some(self_value) = self.get_mut(key) {
                        $crate::operations::UnionAssign::union_assign(self_value, other_value);
                    } else if !other_value.is_empty() {
                        self.insert(key.clone(), other_value.clone().into());
                    }
                }
            }
        }

        impl<Key, Value, OtherValue>
            $crate::operations::Union<&$rhs_map<Key, OtherValue>> for $map<Key, Value>
        where
            Key: $($bounds)*,
            Value: Clone,
            for<'a> Value: $crate::operations::UnionAssign<&'a OtherValue>,
            OtherValue: Clone + Set + Into<Value>,
        {
            type Output = Self;
            fn union(mut self, other: &$rhs_map<Key, OtherValue>) -> Self::Output {
                $crate::operations::UnionAssign::union_assign(&mut self, other);

                self
            }
        }

        impl<Key, Value, OtherValue> $crate::operations::DifferenceAssign<&$rhs_map<Key, OtherValue>>
            for $map<Key, Value>
        where
            Key: $($bounds)*,
            for<'a> Value: $crate::operations::DifferenceAssign<&'a OtherValue>,
        {
            fn difference_assign(&mut self, other: &$rhs_map<Key, OtherValue>) {
                for (key, other_value) in other.$rhs_iter_func() {
                    let Some(value) = self.get_mut(key) else {
                        continue;
                    };

                    $crate::operations::DifferenceAssign::difference_assign(value, other_value);
                }

                remove_empty_keys(self);
            }
        }

        impl<Key, Value, OtherValue> $crate::operations::Difference<&$rhs_map<Key, OtherValue>> for $map<Key, Value>
        where
            Key: $($bounds)*,
            for<'a> Value: $crate::operations::DifferenceAssign<&'a OtherValue>,
        {
            type Output = Self;
            fn difference(mut self, other: &$rhs_map<Key, OtherValue>) -> Self::Output {
                $crate::operations::DifferenceAssign::difference_assign(&mut self, other);

                self
            }
        }

        impl<Key, Value, OtherValue> $crate::operations::IntersectionAssign<&$rhs_map<Key, OtherValue>>
            for $map<Key, Value>
        where
            Key: $($bounds)*,
            for<'a> Value: $crate::operations::IntersectionAssign<&'a OtherValue>,
        {
            fn intersection_assign(&mut self, other: &$rhs_map<Key, OtherValue>) {
                //Remove all that don't exist at all in other
                self.retain(|key, _value| other.get(key).is_some());

                for (key, value) in self.iter_mut() {
                    let other_value = other
                        .get(key)
                        .expect("Removed all keys above that don't exist in other.");

                    $crate::operations::IntersectionAssign::intersection_assign(value, other_value);
                }
            }
        }

        impl<Key, Value, OtherValue> $crate::operations::Intersection<&$rhs_map<Key, OtherValue>>
            for $map<Key, Value>
        where
            Key: $($bounds)*,
            for<'a> Value: $crate::operations::IntersectionAssign<&'a OtherValue>,
        {
            type Output = Self;
            fn intersection(mut self, other: &$rhs_map<Key, OtherValue>) -> Self::Output {
                $crate::operations::IntersectionAssign::intersection_assign(&mut self, other);

                self
            }
        }

        impl<Key, Value, OtherValue>
            $crate::operations::DisjunctiveUnionAssign<&$rhs_map<Key, OtherValue>> for $map<Key, Value>
        where
            Key: $($bounds)*,
            Value: Clone,
            for<'a> Value: $crate::operations::DisjunctiveUnionAssign<&'a OtherValue>,
            OtherValue: Into<Value> + Clone,
        {
            fn disjunctive_union_assign(&mut self, rhs: &$rhs_map<Key, OtherValue>) {
                //For all keys that don't exist on self (but exist on rhs), add them.
                for (key, other_value) in rhs.$rhs_iter_func() {
                    if let Some(value) = self.get_mut(key) {
                        $crate::operations::DisjunctiveUnionAssign::disjunctive_union_assign(value, other_value);
                    } else {
                        self.insert(key.clone(), other_value.clone().into());
                    }
                }

                remove_empty_keys(self);
            }
        }

        impl<Key, Value, OtherValue> $crate::operations::DisjunctiveUnion<&$rhs_map<Key, OtherValue>> for $map<Key, Value>
        where
            Key: $($bounds)*,
            Value: Clone,
            for<'a> Value: $crate::operations::DisjunctiveUnionAssign<&'a OtherValue>,
            OtherValue: Into<Value> + Clone,
        {
            type Output = Self;
            fn disjunctive_union(mut self, rhs: &$rhs_map<Key, OtherValue>) -> Self::Output {
                $crate::operations::DisjunctiveUnionAssign::disjunctive_union_assign(&mut self, rhs);

                self
            }
        }
    };
    ($map:ident, $rhs_map:ident, Key: $($bounds:tt)*) => {
      impl_map_ref_operations!($map, $rhs_map, Key: ($($bounds)*));
    };
}

#[macro_export]
macro_rules! impl_map_comparisons {
    ($map:ident, $rhs_map:ident, Key: ($($bounds:tt)*)) => {
      impl_map_comparisons!($map, $rhs_map, Key: ($($bounds)*), iter, keys);
    };
    ($map:ident, $rhs_map:ident, Key: ($($bounds:tt)*), $self_iter_func:ident, $rhs_key_func:ident) => {
        impl<Key, Value, OtherValue>
            $crate::comparisons::SetEq<$rhs_map<Key, OtherValue>> for $map<Key, Value>
        where
            Key: $($bounds)*,
            Value: $crate::comparisons::SetEq<OtherValue>,
        {
            fn set_eq(&self, rhs: &$rhs_map<Key, OtherValue>) -> bool {
                rhs.$rhs_key_func().all(|k| self.get(k).is_some()) &&
                self.$self_iter_func()
                    .all(|(k, v)| rhs.get(k).is_some_and(|rhs_v| v.set_eq(rhs_v)))
            }
        }

        impl<Key, Value, OtherValue>
            $crate::comparisons::SubsetOf<$rhs_map<Key, OtherValue>> for $map<Key, Value>
        where
            Key: $($bounds)*,
            Value: $crate::comparisons::SubsetOf<OtherValue>,
        {
            fn subset_of(&self, rhs: &$rhs_map<Key, OtherValue>) -> bool {
                self.$self_iter_func()
                    .all(|(k, v)| rhs.get(k).is_some_and(|rhs_v| v.subset_of(rhs_v)))
            }
        }
    };
    ($map:ident, $rhs_map:ident, Key: $($bounds:tt)*) => {
      impl_map_comparisons!($map, $rhs_map, Key: ($($bounds)*));
    };
}
