#[cfg(test)]
#[doc = include_str!("../../README.md")]
mod doc_test {}

//Set Traits
#[macro_use]
mod set;
pub use set::{
    Difference, DifferenceInPlace, DisjunctiveUnion, DisjunctiveUnionInPlace, HasSubset,
    Intersection, IntersectionInPlace, Set, SubsetOf, Union, UnionInPlace,
};

//Bool trivial set
mod trivial;

//KV List
#[macro_use]
mod list;
pub use list::KVListSet;

//KV List with Wildcard
mod list_with_wildcard;
pub use list_with_wildcard::KVListWithWildcardSet;

//Set Separator
mod separator;
pub use separator::Separator;

#[cfg(feature = "derive")]
pub use friperms_derive::*;
