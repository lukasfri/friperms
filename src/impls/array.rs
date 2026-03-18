use crate::operations::{
    Difference, DifferenceAssign, DisjunctiveUnion, DisjunctiveUnionAssign, Intersection,
    IntersectionAssign, Union, UnionAssign,
};
use crate::{Complement, Set, UniversalSet};

impl<const N: usize, Value: Set> Set for [Value; N] {
    type Empty = [Value::Empty; N];

    fn is_empty(&self) -> bool {
        self.iter().all(|value| value.is_empty())
    }

    fn empty() -> Self::Empty {
        core::array::from_fn(|_| Value::empty())
    }
}

// List A <-> List B implementations
impl<const N: usize, Value: Set<Empty = Value>, OtherValue: Set> UnionAssign<&[OtherValue; N]>
    for [Value; N]
where
    for<'a> Value: UnionAssign<&'a OtherValue>,
{
    fn union_assign(&mut self, rhs: &[OtherValue; N]) {
        for (self_i, rhs_i) in self.iter_mut().zip(rhs.iter()) {
            self_i.union_assign(rhs_i);
        }
    }
}

impl<const N: usize, Value: Set<Empty = Value>, OtherValue: Set> Union<&[OtherValue; N]>
    for [Value; N]
where
    for<'a> Value: UnionAssign<&'a OtherValue>,
{
    type Output = Self;

    fn union(mut self, rhs: &[OtherValue; N]) -> Self::Output {
        self.union_assign(rhs);

        self
    }
}

impl<const N: usize, Value: Set<Empty = Value>, OtherValue: Set<Empty = OtherValue>>
    DifferenceAssign<&[OtherValue; N]> for [Value; N]
where
    for<'a> Value: DifferenceAssign<&'a OtherValue>,
{
    fn difference_assign(&mut self, rhs: &[OtherValue; N]) {
        for (self_i, rhs_i) in self.iter_mut().zip(rhs.iter()) {
            self_i.difference_assign(rhs_i);
        }
    }
}

impl<const N: usize, Value: Set<Empty = Value>, OtherValue: Set<Empty = OtherValue>>
    Difference<&[OtherValue; N]> for [Value; N]
where
    for<'a> Value: DifferenceAssign<&'a OtherValue>,
{
    type Output = Self;

    fn difference(mut self, rhs: &[OtherValue; N]) -> Self::Output {
        self.difference_assign(rhs);

        self
    }
}

impl<const N: usize, Value: Set<Empty = Value>, OtherValue: Set<Empty = OtherValue>>
    IntersectionAssign<&[OtherValue; N]> for [Value; N]
where
    for<'a> Value: IntersectionAssign<&'a OtherValue>,
{
    fn intersection_assign(&mut self, rhs: &[OtherValue; N]) {
        for (self_i, rhs_i) in self.iter_mut().zip(rhs.iter()) {
            self_i.intersection_assign(rhs_i);
        }
    }
}

impl<const N: usize, Value: Set<Empty = Value>, OtherValue: Set<Empty = OtherValue>>
    Intersection<&[OtherValue; N]> for [Value; N]
where
    for<'a> Value: IntersectionAssign<&'a OtherValue>,
{
    type Output = Self;
    fn intersection(mut self, other: &[OtherValue; N]) -> Self::Output {
        self.intersection_assign(other);

        self
    }
}

impl<const N: usize, Value: Set<Empty = Value>, OtherValue: Set>
    DisjunctiveUnionAssign<&[OtherValue; N]> for [Value; N]
where
    for<'a> Value: DisjunctiveUnionAssign<&'a OtherValue>,
{
    fn disjunctive_union_assign(&mut self, rhs: &[OtherValue; N]) {
        for (self_i, rhs_i) in self.iter_mut().zip(rhs.iter()) {
            self_i.disjunctive_union_assign(rhs_i);
        }
    }
}

impl<const N: usize, Value: Set<Empty = Value>, OtherValue: Set> DisjunctiveUnion<&[OtherValue; N]>
    for [Value; N]
where
    for<'a> Value: DisjunctiveUnionAssign<&'a OtherValue>,
{
    type Output = Self;

    fn disjunctive_union(mut self, rhs: &[OtherValue; N]) -> Self::Output {
        self.disjunctive_union_assign(rhs);

        self
    }
}

impl<const N: usize, Value: UniversalSet> UniversalSet for [Value; N] {
    type Universal = [Value::Universal; N];

    fn universal() -> Self::Universal {
        core::array::from_fn(|_| Value::universal())
    }

    fn is_universal(&self) -> bool {
        self.iter().all(|value| value.is_universal())
    }
}

impl<const N: usize, Value: Complement> Complement for [Value; N] {
    type Complement = [Value::Complement; N];

    fn complement(&self) -> Self::Complement {
        core::array::from_fn(|i| self[i].complement())
    }
}

#[cfg(test)]
mod tests {
    use core::fmt::Debug;

    use rstest::*;

    use crate::operations::Union;

    #[allow(unused_imports)]
    use super::*;

    #[rstest]
    #[case([true, false, false, true], [false, true, false, true], [true, true, false, true])]
    fn union_list_tests<T, U, V>(#[case] a: T, #[case] b: U, #[case] c: V)
    where
        T: for<'a> Union<&'a U, Output = V>,
        V: PartialEq + Debug,
    {
        assert_eq!(a.union(&b), c);
    }

    #[rstest]
    #[case([true, false, false, true], [false, true, false, true], [true, false, false, false])]
    fn difference_list_tests<T, U, V>(#[case] a: T, #[case] b: U, #[case] c: V)
    where
        T: for<'a> Difference<&'a U, Output = V>,
        V: PartialEq + Debug,
    {
        assert_eq!(a.difference(&b), c);
    }

    #[rstest]
    #[case([true, false, false, true], [false, true, false, true], [false, false, false, true])]
    fn intersection_list_tests<T, U, V>(#[case] a: T, #[case] b: U, #[case] c: V)
    where
        T: for<'a> Intersection<&'a U, Output = V>,
        V: PartialEq + Debug,
    {
        assert_eq!(a.intersection(&b), c);
    }
}
