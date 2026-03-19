mod bool;

#[cfg(feature = "std")]
pub(crate) mod btreemap;
#[cfg(feature = "std")]
pub(crate) mod hashmap;

#[macro_use]
pub(crate) mod map;

mod option;

#[cfg(feature = "std")]
mod r#box;

mod array;

mod tuples;
