//! This module contains traits for comparing sets, such as checking if two sets are equal, if one set is a subset of another, etc.
use crate::operations::*;

#[cfg(feature = "derive")]
pub use finit_derive::{SetEq, SubsetOf};

/// [`SetEq`] (≡) will check if Self and Rhs are equal as sets, ignoring any non-set properties.
/// This is not the same as PartialEq, since two sets can be equal even if they are different types, as long as they contain the same elements.
pub trait SetEq<Rhs = Self> {
    fn set_eq(&self, rhs: &Rhs) -> bool;
}

impl<T: SetEq<Rhs>, Rhs> SetEq<Rhs> for &T {
    fn set_eq(&self, rhs: &Rhs) -> bool {
        T::set_eq(*self, rhs)
    }
}

/// A helper macro to implement [`SetEq`] for types that also implement [`PartialEq`], since in many cases they will be the same.
#[macro_export]
macro_rules! set_eq_partial_eq_impl {
    (($($wh:tt)+): $($t:tt)+) => {
        impl<$($wh)*> $crate::comparisons::SetEq for $($t)* {
            fn set_eq(&self, rhs: &Self) -> bool {
                PartialEq::eq(self, rhs)
            }
        }
    };
    ($t:ty) => {
        impl $crate::comparisons::SetEq for $t {
            fn set_eq(&self, rhs: &Self) -> bool {
                PartialEq::eq(self, rhs)
            }
        }
    };
    ($($t:ty),*) => {
        $(
            set_eq_partial_eq_impl!($t);
        )*
    };
}

/// [`SubsetOf`] (⊆) will check if Rhs contains Self. This is the opposite of [`SupersetOf`], so A ⊆ B if and only if B ⊇ A.
pub trait SubsetOf<Rhs = Self> {
    fn subset_of(&self, rhs: &Rhs) -> bool;
}

impl<T: SubsetOf<Rhs>, Rhs> SubsetOf<Rhs> for &T {
    fn subset_of(&self, rhs: &Rhs) -> bool {
        T::subset_of(*self, rhs)
    }
}

#[macro_export]
macro_rules! subset_of_intersection_identity_impl {
    ($t:ty) => {
        impl $crate::comparisons::SubsetOf for $t {
            fn subset_of(&self, rhs: &Self) -> bool {
                $crate::comparisons::identity::subset_using_intersection_eq(self, rhs)
            }
        }
    };
    ($($t:ty),*) => {
        $(
            subset_of_intersection_identity_impl!($t);
        )*
    };
}

/// [`StrictSubsetOf`] (⊂) will check if Rhs contains Self, but they cannot be equal. This is the opposite of [`StrictSupersetOf`], so A ⊂ B if and only if B ⊃ A.
pub trait StrictSubsetOf<Rhs = Self> {
    fn strict_subset_of(&self, rhs: &Rhs) -> bool;
}

impl<T: SubsetOf<Rhs> + SetEq<Rhs>, Rhs> StrictSubsetOf<Rhs> for T {
    fn strict_subset_of(&self, rhs: &Rhs) -> bool {
        self.subset_of(rhs) && !self.set_eq(rhs)
    }
}

/// [`SupersetOf`] (⊇) will check if Self contains Rhs. This is the opposite of [`SubsetOf`], so A ⊇ B if and only if B ⊆ A.
pub trait SupersetOf<Rhs = Self> {
    fn superset_of(&self, rhs: &Rhs) -> bool;
}

impl<T, Rhs: SubsetOf<T>> SupersetOf<Rhs> for T {
    fn superset_of(&self, rhs: &Rhs) -> bool {
        rhs.subset_of(self)
    }
}

/// [`StrictSupersetOf`] (⊃) will check if Self contains Rhs, but they cannot be equal. This is the opposite of [`StrictSubsetOf`], so A ⊃ B if and only if B ⊂ A.
pub trait StrictSupersetOf<Rhs = Self> {
    fn strict_superset_of(&self, rhs: &Rhs) -> bool;
}

impl<T, Rhs: StrictSubsetOf<T>> StrictSupersetOf<Rhs> for T {
    fn strict_superset_of(&self, rhs: &Rhs) -> bool {
        rhs.strict_subset_of(self)
    }
}

/// The [`identity`] submodule contains identities that can be used to implement some comparisons in terms of others.
pub mod identity {
    use crate::Set;

    use super::*;

    /// A ⊆ B if A ∩ B = A
    pub fn subset_using_intersection_eq<T, Rhs>(a: &T, b: &Rhs) -> bool
    where
        T: Clone + SetEq,
        for<'a> T: IntersectionAssign<&'a Rhs>,
    {
        let mut intersection = a.clone();

        intersection.intersection_assign(b);

        intersection.set_eq(a)
    }

    /// A ⊆ B if A ∪ B = B
    pub fn subset_using_union_eq<T, Rhs>(a: &T, b: &Rhs) -> bool
    where
        Rhs: Clone + SetEq,
        for<'a> Rhs: UnionAssign<&'a T>,
    {
        let mut union_set = b.clone();

        union_set.union_assign(a);

        union_set.set_eq(b)
    }

    /// A ⊆ B if A \ B = ∅
    pub fn subset_using_difference_empty<T, Rhs>(a: &T, b: &Rhs) -> bool
    where
        T: Clone + Set,
        for<'a> T: DifferenceAssign<&'a Rhs>,
    {
        let mut difference = a.clone();

        difference.difference_assign(b);

        difference.is_empty()
    }

    /// A = B if A ⊆ B and B ⊆ A
    pub fn eq_using_subset<T, Rhs>(a: &T, b: &Rhs) -> bool
    where
        T: SubsetOf<Rhs>,
        Rhs: SubsetOf<T>,
    {
        a.subset_of(b) && b.subset_of(a)
    }
}
