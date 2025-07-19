//! Provider of [`IteratorChunksExt`].

use crate::chunks::Chunks;

/// Iterator extension for [`chunks`](IteratorChunksExt::chunks) method.
pub trait IteratorChunksExt: Clone + Iterator {
    /// Creates an iterator for grouping items.
    ///
    /// # Examples
    ///
    /// ```
    /// use iter_chunks_ext::prelude::*;
    ///
    /// let items = vec![("a", 0), ("a", 1), ("b", 0), ("a", 2)];
    /// let chunks = &mut items.iter().chunks(|x| x.0);
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
    fn chunks<F, K>(self, f: F) -> Chunks<Self, F, K>
    where
        F: FnMut(&Self::Item) -> K,
        K: PartialEq,
    {
        Chunks::new(self, f)
    }
}

impl<T> IteratorChunksExt for T
where
    T: Clone + Iterator,
{
    // nop.
}
