//! This module contains traits for comparing sets, such as checking if two sets are equal, if one set is a subset of another, etc.

use crate::operations::*;

/// [`SetEq`] (≡) will check if Self and Rhs are equal as sets, ignoring any non-set properties.
/// This is not the same as PartialEq, since two sets can be equal even if they are different types, as long as they contain the same elements.
pub trait SetEq<Rhs = Self> {
    fn set_eq(&self, rhs: &Rhs) -> bool;
}

#[macro_export]
macro_rules! set_eq_partial_eq_impl {
    ($($t:ty),*) => {
        $(
            impl $crate::comparisons::SetEq for $t {
                fn set_eq(&self, rhs: &Self) -> bool {
                    PartialEq::eq(self, rhs)
                }
            }
        )*
    };
}

/// [`SubsetOf`] (⊆) will check if Rhs contains Self. This is the opposite of [`SupersetOf`], so A ⊆ B if and only if B ⊇ A.
pub trait SubsetOf<Rhs> {
    fn subset_of(&self, rhs: &Rhs) -> bool;
}

impl<T: Clone + SetEq, Rhs> SubsetOf<Rhs> for T
where
    for<'a> T: IntersectionAssign<&'a Rhs>,
{
    fn subset_of(&self, rhs: &Rhs) -> bool {
        // Formula: A ⊆ B if A ∩ B = A
        let mut intersection = self.clone();

        intersection.intersection_assign(rhs);

        intersection.set_eq(self)
    }
}

/// [`StrictSubsetOf`] (⊂) will check if Rhs contains Self, but they cannot be equal. This is the opposite of [`StrictSupersetOf`], so A ⊂ B if and only if B ⊃ A.
pub trait StrictSubsetOf<Rhs> {
    fn strict_subset_of(&self, rhs: &Rhs) -> bool;
}

impl<T: SubsetOf<Rhs> + SetEq<Rhs>, Rhs> StrictSubsetOf<Rhs> for T {
    fn strict_subset_of(&self, rhs: &Rhs) -> bool {
        self.subset_of(rhs) && !self.set_eq(rhs)
    }
}

/// [`SupersetOf`] (⊇) will check if Self contains Rhs. This is the opposite of [`SubsetOf`], so A ⊇ B if and only if B ⊆ A.
pub trait SupersetOf<Rhs> {
    fn superset_of(&self, rhs: &Rhs) -> bool;
}

impl<T, Rhs: SubsetOf<T>> SupersetOf<Rhs> for T {
    fn superset_of(&self, rhs: &Rhs) -> bool {
        rhs.subset_of(self)
    }
}

/// [`StrictSupersetOf`] (⊃) will check if Self contains Rhs, but they cannot be equal. This is the opposite of [`StrictSubsetOf`], so A ⊃ B if and only if B ⊂ A.
pub trait StrictSupersetOf<Rhs> {
    fn strict_superset_of(&self, rhs: &Rhs) -> bool;
}

impl<T, Rhs: StrictSubsetOf<T>> StrictSupersetOf<Rhs> for T {
    fn strict_superset_of(&self, rhs: &Rhs) -> bool {
        rhs.strict_subset_of(self)
    }
}
