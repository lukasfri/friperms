#![cfg_attr(not(feature = "std"), no_std)]
//! A library for working with sets, their relationships and operations.
//!
//! The main traits are:
//! - [`Set`] - the most basic trait that all sets must implement to be able to use operations and comparisons, since most require them.
//!
//! ## [Operations](operations)
//! - [`operations::Union`] (∪)
//! - [`operations::UnionAssign`]
//! - [`operations::Intersection`] (∩)
//! - [`operations::IntersectionAssign`]
//! - [`operations::Difference`] (-)
//! - [`operations::DifferenceAssign`]
//! - [`operations::DisjunctiveUnion`] (⊖)
//! - [`operations::DisjunctiveUnionAssign`]
//!
//!
//! ## [Comparisons](comparisons)
//! - [`comparisons::SetEq`] (≡)
//! - [`comparisons::SubsetOf`] (⊆)
//! - [`comparisons::StrictSubsetOf`] (⊂)
//! - [`comparisons::SupersetOf`] (⊇)
//! - [`comparisons::StrictSupersetOf`] (⊃)
//!
//! Feature | Description
//! --- | --- 
//! `std` (default) | Adds support for [`std::collections::HashMap`] and [`std::collections::BTreeMap`] as well as adds the types [`collections::WildcardBTreeMap`] and [`collections::WildcardHashMap`].
//! `derive` | Adds derive macros for operations and comparisons.
//! `serde`| Adds [`serde::Serialize`] and [`serde::Deserialize`] support for built-in types.
//! `phf` | Adds operations between [`phf::Map`], [`phf::OrderedMap`] and [`std::collections`] maps.
//! 
//! This library was originally designed to create a permission system, but it can be used for any kind of system that requires set-based data structures.

#[cfg(test)]
#[doc = include_str!("../README.md")]
mod doc_test {}

/// The most basic trait that all sets often must implement to be able to use operations and comparisons, since most require them.
pub trait Set {
    type Empty;

    fn is_empty(&self) -> bool;
    fn empty() -> Self::Empty;
}

#[cfg(feature = "derive")]
pub use finit_derive::Set;

#[macro_use]
pub mod operations;

#[macro_use]
pub mod comparisons;

mod impls;

pub mod collections;
