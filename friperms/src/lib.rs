#[cfg(test)]
#[doc = include_str!("../../README.md")]
mod doc_test {}

//Set Traits
#[macro_use]
mod set;
pub use set::{
    Difference, DifferenceAssign, DisjunctiveUnion, DisjunctiveUnionAssign, HasSubset,
    Intersection, IntersectionAssign, Set, SubsetOf, Union, UnionAssign,
};

//Bool trivial set
mod trivial;

//KV List
#[macro_use]
mod hashmap;

//KV List with Wildcard
mod wildcard_hashmap;
pub use wildcard_hashmap::WildcardHashMap;

//Set Separator
mod separator;
pub use separator::Separator;

#[cfg(feature = "derive")]
pub use friperms_derive::*;
