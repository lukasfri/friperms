/// The most basic trait that all sets must have to be able to have operations, since most require them.
pub trait Set {
    fn is_empty(&self) -> bool;
    fn empty() -> Self;
}

/// Union (denoted by ∪) is the sum of two sets. Union is a symmetric relation, which means that A ∪ B must equal B ∪ A.
pub trait Union<Rhs>: Set {
    type Output;

    fn union(self, rhs: Rhs) -> Self::Output;
}

/// UnionInPlace is the in-place version of the Union trait.
pub trait UnionInPlace<Rhs>: Set {
    fn union_in_place(&mut self, rhs: Rhs);
}

/// Difference (denoted by -) is the difference between two sets. A - B means all elements in A except the elements that are also present in B.
pub trait Difference<Rhs>: Set {
    type Output;

    fn difference(self, rhs: Rhs) -> Self::Output;
}

/// DifferenceInPlace is the in-place version of the Difference trait.
pub trait DifferenceInPlace<Rhs>: Set {
    fn difference_in_place(&mut self, rhs: Rhs);
}

/// Intersection (denoted by ∩) is the common values of two sets. Intersection is a symmetric relation, which means that A ∩ B must equal B ∩ A.
pub trait Intersection<Rhs>: Set {
    type Output;

    fn intersection(self, rhs: Rhs) -> Self::Output;
}

/// IntersectionInPlace is the in-place version of the Intersection trait.
pub trait IntersectionInPlace<Rhs>: Set {
    fn intersection_in_place(&mut self, rhs: Rhs);
}

/// SubsetCmp will check if Rhs contains Self.
pub trait SubsetOf<Rhs>: Set {
    fn subset_of(&self, rhs: &Rhs) -> bool;
}

impl<T: Clone + PartialEq, Rhs> SubsetOf<Rhs> for T
where
    for<'a> T: IntersectionInPlace<&'a Rhs>,
{
    fn subset_of(&self, rhs: &Rhs) -> bool {
        let mut intersection = self.clone();

        intersection.intersection_in_place(rhs);

        intersection == *self
    }
}

/// SubsetCmp will check if Rhs contains Self.
pub trait HasSubset<Rhs>: Set {
    fn has_subset(&self, rhs: &Rhs) -> bool;
}

impl<T: Set, Rhs: SubsetOf<T>> HasSubset<Rhs> for T {
    fn has_subset(&self, rhs: &Rhs) -> bool {
        Rhs::subset_of(rhs, self)
    }
}

// DisjunctiveUnion (denoted by⊖)
pub trait DisjunctiveUnion<Rhs>: Set {
    type Output;

    fn disjunctive_union(self, rhs: Rhs) -> Self::Output;
}

/// DisjunctiveUnionInPlace is the in-place version of the DisjunctiveUnion trait.
pub trait DisjunctiveUnionInPlace<Rhs>: Set {
    fn disjunctive_union_in_place(&mut self, rhs: Rhs);
}
