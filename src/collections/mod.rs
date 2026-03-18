//! This module contains various collection types that can be used with the traits defined in this crate, such as [`WildcardHashMap`] and [`WildcardBTreeMap`].

#[cfg(feature = "std")]
mod hashmap;
#[cfg(feature = "std")]
pub use hashmap::WildcardHashMap;

#[cfg(feature = "std")]
mod btreemap;
#[cfg(feature = "std")]
pub use btreemap::WildcardBTreeMap;
