//! Stash is a library for efficiently storing maps of keys to values when one
//! doesn't care what the keys are but wants blazing<sup>†</sup> fast `O(1)`
//! insertions, deletions, and lookups.
//!
//! Common use cases include file descriptor tables, session tables, or MIO
//! context tables.
//!
//! <sup>†</sup>Blazing means an order of magnitude faster than hash maps and btree maps.

extern crate unreachable;

#[cfg(test)]
extern crate bincode;

#[cfg(feature = "serialization")]
extern crate serde;

#[cfg(feature = "serialization")]
#[macro_use]
extern crate serde_derive;

#[macro_use]
mod iter_macro;

pub mod index;
pub mod stash;
pub mod unique_stash;

#[doc(inline)]
pub use crate::index::Index;
#[doc(inline)]
pub use crate::stash::Stash;
#[doc(inline)]
pub use crate::unique_stash::{Tag, UniqueStash};
