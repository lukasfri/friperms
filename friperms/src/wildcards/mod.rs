#[cfg(feature = "std")]
mod hashmap;
#[cfg(feature = "std")]
pub use hashmap::WildcardHashMap;

#[cfg(feature = "std")]
mod btreemap;
#[cfg(feature = "std")]
pub use btreemap::WildcardBTreeMap;
