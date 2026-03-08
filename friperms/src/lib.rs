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

mod impls;

mod wildcards;
pub use wildcards::WildcardHashMap;

#[cfg(feature = "derive")]
pub use friperms_derive::*;
