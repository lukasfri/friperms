mod bool;

#[cfg(feature = "std")]
pub(crate) mod btreemap;
#[cfg(feature = "std")]
pub(crate) mod hashmap;

mod option;

#[cfg(feature = "std")]
mod r#box;

mod array;
