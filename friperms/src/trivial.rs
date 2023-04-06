use crate::{
    set::{DifferenceInPlace, IntersectionInPlace, UnionInPlace},
    Difference, DisjunctiveUnion, DisjunctiveUnionInPlace, Intersection, Set, Union,
};

//Set with a single value.
impl Set for bool {
    fn is_empty(&self) -> bool {
        !self
    }

    fn empty() -> Self {
        false
    }
}

impl<'v> Union<&'v bool> for bool {
    type Output = bool;

    fn union(self, rhs: &bool) -> Self::Output {
        self || *rhs
    }
}

impl<'v> UnionInPlace<&'v bool> for bool {
    fn union_in_place(&mut self, rhs: &bool) {
        *self = self.union(rhs);
    }
}

impl<'v> Difference<&'v bool> for bool {
    type Output = bool;

    fn difference(self, rhs: &bool) -> Self::Output {
        if *rhs {
            false
        } else {
            self
        }
    }
}

impl<'v> DifferenceInPlace<&'v bool> for bool {
    fn difference_in_place(&mut self, rhs: &bool) {
        *self = self.difference(rhs);
    }
}

impl<'v> Intersection<&'v bool> for bool {
    type Output = bool;

    fn intersection(self, rhs: &bool) -> Self::Output {
        self && *rhs
    }
}

impl<'v> IntersectionInPlace<&'v bool> for bool {
    fn intersection_in_place(&mut self, rhs: &bool) {
        *self = self.intersection(rhs);
    }
}

impl<'v> DisjunctiveUnion<&'v bool> for bool {
    type Output = bool;

    fn disjunctive_union(self, rhs: &'v bool) -> Self::Output {
        self ^ *rhs
    }
}

impl<'v> DisjunctiveUnionInPlace<&'v bool> for bool {
    fn disjunctive_union_in_place(&mut self, rhs: &'v bool) {
        *self = self.disjunctive_union(rhs);
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
        assert_eq!(val1.union(&val2), result);
    }

    #[rstest]
    #[case(true, true, false)]
    #[case(false, true, false)]
    #[case(true, false, true)]
    #[case(false, false, false)]
    fn difference_tests(#[case] val1: bool, #[case] val2: bool, #[case] result: bool) {
        assert_eq!(val1.difference(&val2), result);
    }

    #[rstest]
    #[case(true, true, true)]
    #[case(false, true, false)]
    #[case(true, false, false)]
    #[case(false, false, false)]
    fn intersection_tests(#[case] val1: bool, #[case] val2: bool, #[case] result: bool) {
        assert_eq!(val1.intersection(&val2), result);
    }

    #[rstest]
    #[case(true, true, false)]
    #[case(false, true, true)]
    #[case(true, false, true)]
    #[case(false, false, false)]
    fn disjunctive_union_tests(#[case] val1: bool, #[case] val2: bool, #[case] result: bool) {
        assert_eq!(val1.disjunctive_union(&val2), result);
    }
}
