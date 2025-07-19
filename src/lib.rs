//! Iterator extension for grouping items.
//!
//! *The author of this crate is not good at English.*
//! *Forgive me if the document is hard to read.*

#![warn(missing_docs)]
#![cfg_attr(not(test), no_std)]

pub mod prelude;

pub use chunk::*;
pub use chunks::*;
pub use iterator_chunks_ext::*;

mod chunk;
mod chunks;
mod iterator_chunks_ext;
mod msg;
