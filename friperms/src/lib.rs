#[cfg(test)]
#[doc = include_str!("../../README.md")]
mod doc_test {}

/// The most basic trait that all sets must have to be able to have operations, since most require them.
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
