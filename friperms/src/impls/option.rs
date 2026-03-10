use crate::{
    DifferenceAssign, DisjunctiveUnionAssign, Intersection, IntersectionAssign, Set, UnionAssign,
};

impl<Value: Set> Set for Option<Value> {
    type Empty = Self;

    fn is_empty(&self) -> bool {
        self.as_ref().is_none_or(|value| value.is_empty())
    }

    fn empty() -> Self {
        None
    }
}

// Option A <-> Option B implementations
impl<Value, OtherValue> UnionAssign<Option<OtherValue>> for Option<Value>
where
    Value: Set<Empty = Value> + UnionAssign<OtherValue>,
    OtherValue: Set,
{
    fn union_assign(&mut self, rhs: Option<OtherValue>) {
        let Some(rhs) = rhs else {
            return;
        };
        if rhs.is_empty() {
            return;
        }

        let value = self.get_or_insert_with(Value::empty);

        value.union_assign(rhs);

        if value.is_empty() {
            *self = None;
        }
    }
}

impl<Value: Set<Empty = Value>, OtherValue: Set> DifferenceAssign<Option<OtherValue>>
    for Option<Value>
where
    Value: DifferenceAssign<OtherValue>,
{
    fn difference_assign(&mut self, rhs: Option<OtherValue>) {
        let Some(rhs) = rhs else {
            return;
        };

        if rhs.is_empty() {
            return;
        }

        let value = self.get_or_insert_with(Value::empty);

        value.difference_assign(rhs);

        if value.is_empty() {
            *self = None;
        }
    }
}

impl<Value, OtherValue: Set> IntersectionAssign<Option<OtherValue>> for Option<Value>
where
    Value: IntersectionAssign<OtherValue>,
{
    fn intersection_assign(&mut self, rhs: Option<OtherValue>) {
        let Some(rhs) = rhs else {
            *self = None;
            return;
        };

        let Some(value) = self.as_mut() else {
            return;
        };

        value.intersection_assign(rhs);

        if value.is_empty() {
            *self = None;
        }
    }
}

impl<Value, OtherValue> Intersection<Option<OtherValue>> for Option<Value>
where
    for<'a> Value: IntersectionAssign<OtherValue>,
{
    type Output = Self;
    fn intersection(mut self, rhs: Option<OtherValue>) -> Self::Output {
        let value = self.as_mut()?;

        let rhs = rhs?;

        value.intersection_assign(rhs);

        if value.is_empty() {
            return None;
        }

        self
    }
}

impl<Value: Clone, OtherValue: Into<Value> + Clone> DisjunctiveUnionAssign<&Option<OtherValue>>
    for Option<Value>
where
    for<'a> Value: DisjunctiveUnionAssign<&'a OtherValue>,
{
    fn disjunctive_union_assign(&mut self, other: &Option<OtherValue>) {
        let Some(other_value) = other.as_ref() else {
            return;
        };

        if let Some(value) = self.as_mut() {
            value.disjunctive_union_assign(other_value);

            if value.is_empty() {
                *self = None;
            }
        } else {
            *self = Some(other_value.clone().into());
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use rstest::*;

    #[allow(unused_imports)]
    use super::*;

    #[rstest]
    #[case::both_true(Some(true), Some(true), Some(true))]
    #[case::self_empty(Option::<bool>::empty(), Some(true), Some(true))]
    #[case::rhs_empty(Some(true), Option::<bool>::empty(), Some(true))]
    #[case::both_empty(Option::<bool>::empty(), Option::<bool>::empty(), Option::empty())]
    fn union_tests<Value, OtherValue>(
        #[case] mut value1: Option<Value>,
        #[case] value2: Option<OtherValue>,
        #[case] result: Option<Value>,
    ) where
        Value: Set<Empty = Value> + UnionAssign<OtherValue> + Debug + PartialEq,
        OtherValue: Set,
    {
        value1.union_assign(value2);
        assert_eq!(value1, result);
    }

    #[rstest]
    #[case::both_true(Some(true), Some(true), Option::empty())]
    #[case::self_empty(Option::<bool>::empty(), Some(true), Option::empty())]
    #[case::rhs_empty(Some(true), Option::<bool>::empty(), Some(true))]
    #[case::both_empty(Option::<bool>::empty(), Option::<bool>::empty(), Option::empty())]
    fn difference_tests<Value, OtherValue>(
        #[case] mut value1: Option<Value>,
        #[case] value2: Option<OtherValue>,
        #[case] result: Option<Value>,
    ) where
        Value: Debug + PartialEq + Set<Empty = Value> + DifferenceAssign<OtherValue>,
        OtherValue: Set<Empty = OtherValue>,
    {
        value1.difference_assign(value2);
        assert_eq!(value1, result);
    }

    #[rstest]
    #[case::both_true(Some(true), Some(true), Some(true))]
    #[case::self_empty(Option::<bool>::empty(), Some(true), Option::<bool>::empty())]
    #[case::rhs_empty(Some(true), Option::<bool>::empty(), Option::<bool>::empty())]
    #[case::both_empty(Option::<bool>::empty(), Option::<bool>::empty(), Option::<bool>::empty())]
    fn intersection_tests<Value, OtherValue>(
        #[case] mut value1: Option<Value>,
        #[case] value2: Option<OtherValue>,
        #[case] result: Option<Value>,
    ) where
        Value: Debug + PartialEq + Set<Empty = Value> + IntersectionAssign<OtherValue>,
        OtherValue: Set<Empty = OtherValue>,
    {
        value1.intersection_assign(value2);
        assert_eq!(value1, result);
    }

    #[rstest]
    #[case::both_true(Some(true), Some(true), Option::<bool>::empty())]
    #[case::self_empty(Option::<bool>::empty(), Some(true), Some(true))]
    #[case::rhs_empty(Some(true), Option::<bool>::empty(), Some(true))]
    #[case::both_empty(Option::<bool>::empty(), Option::<bool>::empty(), Option::<bool>::empty())]
    fn disjunctive_union_tests<Value, OtherValue>(
        #[case] mut value1: Option<Value>,
        #[case] value2: Option<OtherValue>,
        #[case] result: Option<Value>,
    ) where
        Value: Clone + Debug + PartialEq,
        for<'a> Value: DisjunctiveUnionAssign<&'a OtherValue>,
        OtherValue: Into<Value> + Clone,
    {
        value1.disjunctive_union_assign(&value2);
        assert_eq!(value1, result);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serializing_none_is_empty() {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Serialize, Deserialize)]
        struct PermissionNode {
            #[serde(skip_serializing_if = "Set::is_empty")]
            value1: Option<bool>,
            #[serde(skip_serializing_if = "Set::is_empty")]
            value2: Option<bool>,
        }

        let node1 = PermissionNode {
            value1: Some(true),
            value2: Some(false),
        };

        println!("{node1:?}");

        assert_eq!(
            "{\"value1\":true}",
            serde_json::to_string(&node1).unwrap().as_str()
        );
    }
}
