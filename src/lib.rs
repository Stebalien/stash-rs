//! Stash is a library for efficiently storing maps of keys to values when one
//! doesn't care what the keys are but wants blazing fast `O(1)` insertions,
//! deletions, and lookups.
//!
//! Common use cases include file descriptor tables, session tables, or MIO
//! context tables.

#[macro_use]
mod iter_macro;

pub mod stash;
pub mod unique_stash;

#[doc(inline)]
pub use stash::Stash;
#[doc(inline)]
pub use unique_stash::{UniqueStash, Tag};
