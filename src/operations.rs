//! This module contains traits for performing operations on sets, such as [`Union`], [`Intersection`], [`Difference`], [`DisjunctiveUnion`], etc.

use crate::Set;

#[cfg(feature = "derive")]
pub use finit_derive::{DifferenceAssign, DisjunctiveUnionAssign, IntersectionAssign, UnionAssign};

/// [`Union`] (denoted by ∪) is the sum of two sets. Union is a symmetric relation, which means that A ∪ B must equal B ∪ A.
pub trait Union<Rhs = Self>: Set {
    type Output;

    fn union(self, rhs: Rhs) -> Self::Output;
}

/// [`UnionAssign`] is the in-place version of the [`Union`] trait.
pub trait UnionAssign<Rhs = Self>: Set {
    fn union_assign(&mut self, rhs: Rhs);
}

/// [`Difference`] (denoted by -) is the difference between two sets. A - B means all elements in A except the elements that are also present in B.
pub trait Difference<Rhs = Self>: Set {
    type Output;

    fn difference(self, rhs: Rhs) -> Self::Output;
}

/// [`DifferenceAssign`] is the in-place version of the [`Difference`] trait.
pub trait DifferenceAssign<Rhs = Self>: Set {
    fn difference_assign(&mut self, rhs: Rhs);
}

/// [`Intersection`] (denoted by ∩) is the common values of two sets. Intersection is a symmetric relation, which means that A ∩ B must equal B ∩ A.
pub trait Intersection<Rhs = Self>: Set {
    type Output;

    fn intersection(self, rhs: Rhs) -> Self::Output;
}

/// [`IntersectionAssign`] is the in-place version of the [`Intersection`] trait.
pub trait IntersectionAssign<Rhs = Self>: Set {
    fn intersection_assign(&mut self, rhs: Rhs);
}

/// [`DisjunctiveUnion`] (denoted by ⊖) is the disjunctive union of two sets.
pub trait DisjunctiveUnion<Rhs = Self>: Set {
    type Output;

    fn disjunctive_union(self, rhs: Rhs) -> Self::Output;
}

/// [`DisjunctiveUnionAssign`] is the in-place version of the [`DisjunctiveUnion`] trait.
pub trait DisjunctiveUnionAssign<Rhs = Self>: Set {
    fn disjunctive_union_assign(&mut self, rhs: Rhs);
}

/// The [`identity`] submodule contains identities that can be used to implement some operations in terms of others.
pub mod identity {
    /// A ∩ B = A - (A - B)
    pub fn intersection_using_double_difference<A, B>(
        a: A,
        b: B,
    ) -> <A as super::Difference<<A as super::Difference<B>>::Output>>::Output
    where
        A: Clone + super::Difference<B> + super::Difference<<A as super::Difference<B>>::Output>,
    {
        a.clone().difference(a.difference(b))
    }

    /// A ⊖ B = (A − B) ∪ (B − A)
    pub fn disjunctive_union_using_difference_and_union<A, B>(
        a: A,
        b: B,
    ) -> <<A as super::Difference<B>>::Output as super::Union<<B as super::Difference<A>>::Output>>::Output
    where
        A: Clone + super::Difference<B>,
        B: Clone + super::Difference<A>,
        <A as super::Difference<B>>::Output: super::Union<<B as super::Difference<A>>::Output>,
    {
        super::Union::union(a.clone().difference(b.clone()), b.difference(a))
    }

    /// A - B = A - (A ∩ B)
    pub fn difference_using_intersection<A, B>(
        a: A,
        b: B,
    ) -> <A as super::Difference<<A as super::Intersection<B>>::Output>>::Output
    where
        A: Clone
            + super::Intersection<B>
            + super::Difference<<A as super::Intersection<B>>::Output>,
    {
        a.clone().difference(a.intersection(b))
    }

    /// A ⊖ B = (A ∪ B) - (A ∩ B)
    pub fn disjunctive_union_using_union_intersection_and_difference<A, B>(
        a: A,
        b: B,
    ) -> <<A as super::Union<B>>::Output as super::Difference<
        <A as super::Intersection<B>>::Output,
    >>::Output
    where
        A: Clone + super::Union<B> + super::Intersection<B>,
        B: Clone,
        <A as super::Union<B>>::Output: super::Difference<<A as super::Intersection<B>>::Output>,
    {
        super::Difference::difference(a.clone().union(b.clone()), a.intersection(b))
    }
}
