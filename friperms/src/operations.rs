//! This module contains traits for performing operations on sets, such as [`Union`], [`Intersection`], [`Difference`], [`DisjunctiveUnion`], etc.

use crate::Set;

#[cfg(feature = "derive")]
pub use friperms_derive::{
    DifferenceAssign, DisjunctiveUnionAssign, IntersectionAssign, UnionAssign,
};

/// [`Union`] (denoted by ∪) is the sum of two sets. Union is a symmetric relation, which means that A ∪ B must equal B ∪ A.
pub trait Union<Rhs>: Set {
    type Output;

    fn union(self, rhs: Rhs) -> Self::Output;
}

/// [`UnionAssign`] is the in-place version of the [`Union`] trait.
pub trait UnionAssign<Rhs>: Set {
    fn union_assign(&mut self, rhs: Rhs);
}

/// [`Difference`] (denoted by -) is the difference between two sets. A - B means all elements in A except the elements that are also present in B.
pub trait Difference<Rhs>: Set {
    type Output;

    fn difference(self, rhs: Rhs) -> Self::Output;
}

/// [`DifferenceAssign`] is the in-place version of the [`Difference`] trait.
pub trait DifferenceAssign<Rhs>: Set {
    fn difference_assign(&mut self, rhs: Rhs);
}

/// [`Intersection`] (denoted by ∩) is the common values of two sets. Intersection is a symmetric relation, which means that A ∩ B must equal B ∩ A.
pub trait Intersection<Rhs>: Set {
    type Output;

    fn intersection(self, rhs: Rhs) -> Self::Output;
}

/// [`IntersectionAssign`] is the in-place version of the [`Intersection`] trait.
pub trait IntersectionAssign<Rhs>: Set {
    fn intersection_assign(&mut self, rhs: Rhs);
}

/// [`DisjunctiveUnion`] (denoted by ⊖) is the disjunctive union of two sets.
pub trait DisjunctiveUnion<Rhs>: Set {
    type Output;

    fn disjunctive_union(self, rhs: Rhs) -> Self::Output;
}

/// [`DisjunctiveUnionAssign`] is the in-place version of the [`DisjunctiveUnion`] trait.
pub trait DisjunctiveUnionAssign<Rhs>: Set {
    fn disjunctive_union_assign(&mut self, rhs: Rhs);
}
