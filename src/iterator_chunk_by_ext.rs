//! Provider of [`IteratorChunkByExt`].

use crate::chunk_by::ChunkBy;

/// Iterator extension for [`chunk_by`](IteratorChunkByExt::chunk_by) method.
pub trait IteratorChunkByExt: Clone + Iterator {
    /// Creates an iterator for grouping items.
    ///
    /// # Examples
    ///
    /// ```
    /// use iter_chunk_ext::prelude::*;
    ///
    /// let items = vec![("a", 0), ("a", 1), ("b", 0), ("a", 2)];
    /// let chunks = &mut items.iter().chunk_by(|x| x.0);
    ///
    /// let chunk = &mut chunks.next().unwrap();
    /// assert_eq!(chunk.next(), Some(&("a", 0)));
    /// assert_eq!(chunk.next(), Some(&("a", 1)));
    /// assert_eq!(chunk.next(), None);
    ///
    /// let chunk = &mut chunks.next().unwrap();
    /// assert_eq!(chunk.next(), Some(&("b", 0)));
    /// assert_eq!(chunk.next(), None);
    ///
    /// let chunk = &mut chunks.next().unwrap();
    /// assert_eq!(chunk.next(), Some(&("a", 2)));
    /// assert_eq!(chunk.next(), None);
    ///
    /// let chunk = &mut chunks.next();
    /// assert!(chunk.is_none());
    /// ```
    fn chunk_by<F, K>(self, f: F) -> ChunkBy<Self, F, K>
    where
        F: FnMut(&Self::Item) -> K,
        K: PartialEq,
    {
        ChunkBy::new(self, f)
    }
}

impl<T> IteratorChunkByExt for T
where
    T: Clone + Iterator,
{
    // nop.
}
