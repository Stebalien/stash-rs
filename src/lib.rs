//! Stash is a library for efficiently storing maps of keys to values when one
//! doesn't care what the keys are but wants blazing<sup>†</sup> fast `O(1)`
//! insertions, deletions, and lookups.
//!
//! Common use cases include file descriptor tables, session tables, or MIO
//! context tables.
//!
//! <small><sup>†</sup>Blazing means an order of magnitude faster than hash maps and btree maps.</small>
//!
//! # Serialization
//!
//! A stash can be serialized and deserialized with serde, preserving its _existing_ key/value
//! mapping. This can be used to save/restore a stash to persistant storage.
//!
//! However, in general, stashes make no guarantees on how keys are assigned. If stash **A** is
//! serialized then deserialized into stash **B**, values inserted into stash **A** will likely be
//! assigned different keys than values inserted into stash **B**.
#![cfg_attr(not(any(feature = "std", test)), no_std)]

extern crate alloc;
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
