use crate::{
    DifferenceInPlace, DisjunctiveUnionInPlace, Intersection, IntersectionInPlace, Set,
    UnionInPlace,
};

/// Set-variation that exists mainly for the purpose of optimization, for example when building a set that will often be serialzied, so that a complicated empty set structure does not have to be serialized instead of a simple "None".
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct Separator<Value>(Option<Box<Value>>);

impl<Value: Set> Separator<Value> {
    pub fn new(value: Value) -> Self {
        Separator(if value.is_empty() {
            None
        } else {
            Some(Box::new(value))
        })
    }
}

impl<Value: Set> Default for Separator<Value> {
    fn default() -> Self {
        Self::empty()
    }
}

impl<Value: Set> Set for Separator<Value> {
    fn is_empty(&self) -> bool {
        self.0.as_ref().map_or(true, |value| value.is_empty())
    }

    fn empty() -> Self {
        Separator(None)
    }
}

// Separator A <-> Separator B implementations
impl<Value: Clone, OtherValue: Clone + Set + Into<Value>> UnionInPlace<&Separator<OtherValue>>
    for Separator<Value>
where
    for<'a> Value: UnionInPlace<&'a OtherValue>,
{
    fn union_in_place(&mut self, other: &Separator<OtherValue>) {
        let Some(other_value) = other.0.as_deref() else {
          return
        };

        if let Some(value) = &mut self.0 {
            value.union_in_place(other_value);
        } else {
            self.0 = Some(Box::new(other_value.clone().into()));
        }
    }
}

impl<Value, OtherValue> DifferenceInPlace<&Separator<OtherValue>> for Separator<Value>
where
    for<'a> Value: DifferenceInPlace<&'a OtherValue>,
{
    fn difference_in_place(&mut self, other: &Separator<OtherValue>) {
        let (Some(value), Some(other_value)) = (self.0.as_deref_mut(), other.0.as_deref()) else {
          return
        };

        value.difference_in_place(other_value);

        if value.is_empty() {
            self.0 = None;
        }
    }
}

impl<Value, OtherValue> IntersectionInPlace<&Separator<OtherValue>> for Separator<Value>
where
    for<'a> Value: IntersectionInPlace<&'a OtherValue>,
{
    fn intersection_in_place(&mut self, other: &Separator<OtherValue>) {
        let Some(value) = self.0.as_deref_mut() else {
        return
      };

        if let Some(other_value) = other.0.as_deref() {
            value.intersection_in_place(other_value);

            if value.is_empty() {
                self.0 = None;
            }
        } else {
            self.0 = None;
        }
    }
}

impl<Value, OtherValue> Intersection<&Separator<OtherValue>> for Separator<Value>
where
    for<'a> Value: IntersectionInPlace<&'a OtherValue>,
{
    type Output = Self;
    fn intersection(mut self, other: &Separator<OtherValue>) -> Self::Output {
        self.intersection_in_place(other);

        self
    }
}

impl<Value: Clone, OtherValue: Into<Value> + Clone> DisjunctiveUnionInPlace<&Separator<OtherValue>>
    for Separator<Value>
where
    for<'a> Value: DisjunctiveUnionInPlace<&'a OtherValue>,
{
    fn disjunctive_union_in_place(&mut self, other: &Separator<OtherValue>) {
        let Some(other_value) = other.0.as_deref() else {
          return
        };

        if let Some(value) = self.0.as_deref_mut() {
            value.disjunctive_union_in_place(other_value);

            if value.is_empty() {
                self.0 = None;
            }
        } else {
            self.0 = Some(Box::new(other_value.clone().into()));
        }
    }
}

impl<Value: Set> From<Value> for Separator<Value> {
    fn from(value: Value) -> Self {
        Separator::new(value)
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use rstest::*;

    #[allow(unused_imports)]
    use super::*;

    #[rstest]
    #[case(Separator::new(true), Separator::new(true), Separator::new(true))]
    #[case(Separator::<bool>::empty(), Separator::new(true), Separator::new(true))]
    #[case(Separator::new(true), Separator::empty(), Separator::new(true))]
    #[case(Separator::<bool>::empty(), Separator::empty(), Separator::empty())]
    fn union_tests<Value: Clone + Debug + PartialEq, OtherValue: Clone + Set + Into<Value>>(
        #[case] mut value1: Separator<Value>,
        #[case] value2: Separator<OtherValue>,
        #[case] result: Separator<Value>,
    ) where
        for<'a> Value: UnionInPlace<&'a OtherValue>,
    {
        value1.union_in_place(&value2);
        assert_eq!(value1, result);
    }

    #[rstest]
    #[case(Separator::new(true), Separator::new(true), Separator::empty())]
    #[case(Separator::<bool>::empty(), Separator::new(true), Separator::empty())]
    #[case(Separator::new(true), Separator::empty(), Separator::new(true))]
    #[case(Separator::<bool>::empty(), Separator::empty(), Separator::empty())]
    fn difference_tests<Value: Debug + PartialEq, OtherValue>(
        #[case] mut value1: Separator<Value>,
        #[case] value2: Separator<OtherValue>,
        #[case] result: Separator<Value>,
    ) where
        for<'a> Value: DifferenceInPlace<&'a OtherValue>,
    {
        value1.difference_in_place(&value2);
        assert_eq!(value1, result);
    }

    #[rstest]
    #[case(Separator::new(true), Separator::new(true), Separator::new(true))]
    #[case(Separator::<bool>::empty(), Separator::new(true), Separator::empty())]
    #[case(Separator::new(true), Separator::empty(), Separator::empty())]
    #[case(Separator::<bool>::empty(), Separator::empty(), Separator::empty())]
    fn intersection_tests<Value: Debug + PartialEq, OtherValue>(
        #[case] mut value1: Separator<Value>,
        #[case] value2: Separator<OtherValue>,
        #[case] result: Separator<Value>,
    ) where
        for<'a> Value: IntersectionInPlace<&'a OtherValue>,
    {
        value1.intersection_in_place(&value2);
        assert_eq!(value1, result);
    }

    #[rstest]
    #[case(Separator::new(true), Separator::new(true), Separator::empty())]
    #[case(Separator::<bool>::empty(), Separator::new(true), Separator::new(true))]
    #[case(Separator::new(true), Separator::empty(), Separator::new(true))]
    #[case(Separator::<bool>::empty(), Separator::empty(), Separator::empty())]
    fn disjunctive_union_tests<Value: Clone + Debug + PartialEq, OtherValue: Into<Value> + Clone>(
        #[case] mut value1: Separator<Value>,
        #[case] value2: Separator<OtherValue>,
        #[case] result: Separator<Value>,
    ) where
        for<'a> Value: DisjunctiveUnionInPlace<&'a OtherValue>,
    {
        value1.disjunctive_union_in_place(&value2);
        assert_eq!(value1, result);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serializing_none_is_empty() {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Serialize, Deserialize)]
        struct PermissionNode {
            #[serde(skip_serializing_if = "Set::is_empty")]
            value1: Separator<bool>,
            #[serde(skip_serializing_if = "Set::is_empty")]
            value2: Separator<bool>,
        }

        let node1 = PermissionNode {
            value1: Separator::new(true),
            value2: Separator::new(false),
        };

        println!("{node1:?}");

        assert_eq!(
            "{\"value1\":true}",
            serde_json::to_string(&node1).unwrap().as_str()
        );
    }
}
