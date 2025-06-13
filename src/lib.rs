//! Iterator extension for segmenting items.
//!
//! *The author of this crate is not good at English.*
//! *Forgive me if the document is hard to read.*

#![warn(missing_docs)]
#![cfg_attr(not(test), no_std)]

pub mod prelude;

pub use segment::*;
pub use segment_by::*;
pub use iterator_segment_by_ext::*;

mod segment;
mod segment_by;
mod iterator_segment_by_ext;
mod msg;
