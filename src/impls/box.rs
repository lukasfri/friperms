use crate::Set;
use crate::operations::{
    Difference, DifferenceAssign, DisjunctiveUnion, DisjunctiveUnionAssign, Intersection,
    IntersectionAssign, Union, UnionAssign,
};
use crate::comparisons::{SetEq, SubsetOf};

impl<Value: Set> Set for Box<Value> {
    type Empty = Box<Value::Empty>;

    fn is_empty(&self) -> bool {
        self.as_ref().is_empty()
    }

    fn empty() -> Self::Empty {
        Box::new(Value::empty())
    }
}

// Option A <-> Option B implementations
impl<Value, OtherValue> UnionAssign<OtherValue> for Box<Value>
where
    Value: UnionAssign<OtherValue>,
{
    fn union_assign(&mut self, other: OtherValue) {
        self.as_mut().union_assign(other);
    }
}

impl<Value, OtherValue> DifferenceAssign<OtherValue> for Box<Value>
where
    Value: DifferenceAssign<OtherValue>,
{
    fn difference_assign(&mut self, other: OtherValue) {
        self.as_mut().difference_assign(other);
    }
}

impl<Value, OtherValue> IntersectionAssign<OtherValue> for Box<Value>
where
    Value: IntersectionAssign<OtherValue>,
{
    fn intersection_assign(&mut self, other: OtherValue) {
        self.as_mut().intersection_assign(other);
    }
}

impl<Value, OtherValue> DisjunctiveUnionAssign<OtherValue> for Box<Value>
where
    Value: DisjunctiveUnionAssign<OtherValue>,
{
    fn disjunctive_union_assign(&mut self, other: OtherValue) {
        self.as_mut().disjunctive_union_assign(other);
    }
}

impl<Value, OtherValue> Union<OtherValue> for Box<Value>
where
    Value: Union<OtherValue>,
{
    type Output = Box<Value::Output>;

    fn union(self, rhs: OtherValue) -> Self::Output {
        Box::new((*self).union(rhs))
    }
}

impl<Value, OtherValue> Difference<OtherValue> for Box<Value>
where
    Value: Difference<OtherValue>,
{
    type Output = Box<Value::Output>;

    fn difference(self, rhs: OtherValue) -> Self::Output {
        Box::new((*self).difference(rhs))
    }
}

impl<Value, OtherValue> Intersection<OtherValue> for Box<Value>
where
    Value: Intersection<OtherValue>,
{
    type Output = Box<Value::Output>;

    fn intersection(self, rhs: OtherValue) -> Self::Output {
        Box::new((*self).intersection(rhs))
    }
}

impl<Value, OtherValue> DisjunctiveUnion<OtherValue> for Box<Value>
where
    Value: DisjunctiveUnion<OtherValue>,
{
    type Output = Box<Value::Output>;

    fn disjunctive_union(self, rhs: OtherValue) -> Self::Output {
        Box::new((*self).disjunctive_union(rhs))
    }
}

impl<Value, OtherValue> SetEq<OtherValue> for Box<Value>
where
    Value: SetEq<OtherValue>,
{
    fn set_eq(&self, rhs: &OtherValue) -> bool {
        Value::set_eq(&**self, rhs)
    }
}

impl<Value, OtherValue> SubsetOf<OtherValue> for Box<Value>
where
    Value: SubsetOf<OtherValue>,
{
    fn subset_of(&self, rhs: &OtherValue) -> bool {
        Value::subset_of(&**self, rhs)
    }
}
