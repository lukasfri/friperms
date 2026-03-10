//! A library for working with sets and their relationships, such as subsets, supersets, unions, intersections, etc. It provides traits for defining sets and their operations, as well as comparisons between sets.
//!
//! This library was originally designed to create a permission system, but it can be used for any kind of set operations and comparisons.

#[cfg(test)]
#[doc = include_str!("../../README.md")]
mod doc_test {}

/// The most basic trait that all sets often must implement to be able to use operations and comparisons, since most require them.
pub trait Set {
    type Empty;

    fn is_empty(&self) -> bool;
    fn empty() -> Self::Empty;
}

#[macro_use]
pub mod operations;

#[macro_use]
pub mod comparisons;

mod impls;

mod wildcards;
pub use wildcards::{WildcardBTreeMap, WildcardHashMap};

#[cfg(feature = "derive")]
pub use friperms_derive::Set;
