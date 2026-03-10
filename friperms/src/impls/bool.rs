use crate::Set;
use crate::operations::{
    Difference, DifferenceAssign, DisjunctiveUnion, DisjunctiveUnionAssign, Intersection,
    IntersectionAssign, Union, UnionAssign,
};

//Set with a single value.
impl Set for bool {
    type Empty = Self;

    fn is_empty(&self) -> bool {
        !self
    }

    fn empty() -> Self::Empty {
        false
    }
}

impl Set for &bool {
    type Empty = bool;

    fn is_empty(&self) -> bool {
        (**self).is_empty()
    }

    fn empty() -> Self::Empty {
        bool::empty()
    }
}

impl Set for &mut bool {
    type Empty = bool;

    fn is_empty(&self) -> bool {
        (**self).is_empty()
    }

    fn empty() -> Self::Empty {
        bool::empty()
    }
}

impl Union<bool> for bool {
    type Output = bool;

    fn union(self, rhs: bool) -> Self::Output {
        self || rhs
    }
}

impl Union<&bool> for bool {
    type Output = bool;

    fn union(self, rhs: &bool) -> Self::Output {
        self.union(*rhs)
    }
}

impl UnionAssign<bool> for bool {
    fn union_assign(&mut self, rhs: bool) {
        *self = self.union(rhs);
    }
}

impl UnionAssign<&bool> for bool {
    fn union_assign(&mut self, rhs: &bool) {
        *self = self.union(*rhs);
    }
}

impl Difference<bool> for bool {
    type Output = bool;

    fn difference(self, rhs: bool) -> Self::Output {
        if rhs { false } else { self }
    }
}

impl Difference<&bool> for bool {
    type Output = bool;

    fn difference(self, rhs: &bool) -> Self::Output {
        self.difference(*rhs)
    }
}

impl DifferenceAssign<bool> for bool {
    fn difference_assign(&mut self, rhs: bool) {
        *self = self.difference(rhs);
    }
}

impl DifferenceAssign<&bool> for bool {
    fn difference_assign(&mut self, rhs: &bool) {
        *self = self.difference(*rhs);
    }
}

impl Intersection<bool> for bool {
    type Output = bool;

    fn intersection(self, rhs: bool) -> Self::Output {
        self && rhs
    }
}

impl Intersection<&bool> for bool {
    type Output = bool;

    fn intersection(self, rhs: &bool) -> Self::Output {
        self.intersection(*rhs)
    }
}

impl IntersectionAssign<bool> for bool {
    fn intersection_assign(&mut self, rhs: bool) {
        *self = self.intersection(rhs);
    }
}

impl IntersectionAssign<&bool> for bool {
    fn intersection_assign(&mut self, rhs: &bool) {
        *self = self.intersection(*rhs);
    }
}

impl DisjunctiveUnion<bool> for bool {
    type Output = bool;

    fn disjunctive_union(self, rhs: bool) -> Self::Output {
        self ^ rhs
    }
}

impl DisjunctiveUnion<&bool> for bool {
    type Output = bool;

    fn disjunctive_union(self, rhs: &bool) -> Self::Output {
        self.disjunctive_union(*rhs)
    }
}

impl DisjunctiveUnionAssign<bool> for bool {
    fn disjunctive_union_assign(&mut self, rhs: bool) {
        *self = self.disjunctive_union(rhs);
    }
}

impl DisjunctiveUnionAssign<&bool> for bool {
    fn disjunctive_union_assign(&mut self, rhs: &bool) {
        *self = self.disjunctive_union(*rhs);
    }
}

#[cfg(test)]
mod tests {
    use rstest::*;

    #[allow(unused_imports)]
    use super::*;

    #[rstest]
    #[case(true, true, true)]
    #[case(false, true, true)]
    #[case(true, false, true)]
    #[case(false, false, false)]
    fn union_tests(#[case] val1: bool, #[case] val2: bool, #[case] result: bool) {
        assert_eq!(val1.union(val2), result);
    }

    #[rstest]
    #[case(true, true, false)]
    #[case(false, true, false)]
    #[case(true, false, true)]
    #[case(false, false, false)]
    fn difference_tests(#[case] val1: bool, #[case] val2: bool, #[case] result: bool) {
        assert_eq!(val1.difference(val2), result);
    }

    #[rstest]
    #[case(true, true, true)]
    #[case(false, true, false)]
    #[case(true, false, false)]
    #[case(false, false, false)]
    fn intersection_tests(#[case] val1: bool, #[case] val2: bool, #[case] result: bool) {
        assert_eq!(val1.intersection(val2), result);
    }

    #[rstest]
    #[case(true, true, false)]
    #[case(false, true, true)]
    #[case(true, false, true)]
    #[case(false, false, false)]
    fn disjunctive_union_tests(#[case] val1: bool, #[case] val2: bool, #[case] result: bool) {
        assert_eq!(val1.disjunctive_union(val2), result);
    }
}
