//! This module contains various collection types that can be used with the traits defined in this crate, such as [`WildcardHashMap`] and [`WildcardBTreeMap`].

#[cfg(feature = "std")]
mod wildcard_hashmap;
#[cfg(feature = "std")]
pub use wildcard_hashmap::WildcardHashMap;

#[cfg(feature = "std")]
mod wildcard_btreemap;
#[cfg(feature = "std")]
pub use wildcard_btreemap::WildcardBTreeMap;
