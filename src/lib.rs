//! Iterator extension for grouping items.
//!
//! *The author of this crate is not good at English.*
//! *Forgive me if the document is hard to read.*

#![warn(missing_docs)]
#![cfg_attr(not(test), no_std)]

pub mod prelude;

pub use chunk::*;
pub use chunk_by::*;
pub use iterator_chunk_by_ext::*;

mod chunk;
mod chunk_by;
mod iterator_chunk_by_ext;
mod msg;
