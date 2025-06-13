//! Provider of [`Chunk`].

use crate::msg;

/// Chunk items iterator.
#[derive(Clone, Debug)]
#[must_use = msg::iter_must_use!()]
pub struct Chunk<I, F, K>
where 
    I: Iterator,
{
    /// Head item.
    head: Option<I::Item>,
    /// Source items iterator (without `head`).
    iter: I,
    /// Closure for key generating.
    f: F,
    /// Grouping key.
    key: K,
}

impl<I, F, K> Chunk<I, F, K>
where 
    I: Iterator,
{
    /// Creates a new value.
    pub(crate) fn new(head: I::Item, iter: I, f: F, key: K) -> Self {
        let head = Some(head);
        Self {head, iter, f, key}
    }
}

impl<I, F, K> Iterator for Chunk<I, F, K>
where 
    I: Iterator,
    F: FnMut(&I::Item) -> K,
    K: PartialEq,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.head.is_some() {
            self.head.take()
        } else {
            let val = self.iter.next()?;
            if (self.f)(&val) != self.key {
                None
            } else {
                Some(val)
            }
        }
    }
}
