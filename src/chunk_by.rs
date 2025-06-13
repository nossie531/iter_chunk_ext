//! Provider of [`ChunkBy`].

use crate::chunk::Chunk;
use crate::msg;

/// An iterator that grouping iterator items.
/// 
/// This struct is created by [`IteratorChunkByExt::chunk_by`][1].
/// 
/// [1]: crate::IteratorChunkByExt::chunk_by
#[derive(Clone, Debug)]
#[must_use = msg::iter_must_use!()]
pub struct ChunkBy<I, F, K>
where 
    I: Iterator
{
    /// Source items iterator.
    iter: I,
    /// Closure for key generating.
    f: F,
    /// Current grouping key.
    key: Option<K>,
}

impl <I, F, K> ChunkBy<I, F, K>
where 
    I: Iterator
{
    /// Creates a new value.
    pub(crate) fn new(iter: I, f: F) -> Self {
        Self { iter, f, key: None }
    }
}

impl<I, F, K> Iterator for ChunkBy<I, F, K>
where 
    I: Clone + Iterator,
    F: Copy + FnMut(&I::Item) -> K,
    K: PartialEq,
{
    type Item = Chunk<I, F, K>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(val) = self.iter.next() {
            if Some(&(self.f)(&val)) != self.key.as_ref() {
                let iter = self.iter.clone();
                let key = (self.f)(&val);
                self.key = Some((self.f)(&val));
                return Some(Chunk::new(val, iter, self.f, key));
            }
        }

        None
    }
}
