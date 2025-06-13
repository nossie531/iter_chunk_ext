//! Provider of [`SegmentBy`].

use crate::segment::Segment;
use crate::msg;

/// An iterator that segments iterator items.
/// 
/// This struct is created by [`IteratorSegmentByExt::segment_by`][1].
/// 
/// [1]: crate::IteratorSegmentByExt::segment_by
#[derive(Clone, Debug)]
#[must_use = msg::iter_must_use!()]
pub struct SegmentBy<I, F, K>
where 
    I: Iterator
{
    /// Source items iterator.
    iter: I,
    /// Closure for key generating.
    f: F,
    /// Current segment key.
    key: Option<K>,
}

impl <I, F, K> SegmentBy<I, F, K>
where 
    I: Iterator
{
    /// Creates a new value.
    pub(crate) fn new(iter: I, f: F) -> Self {
        Self { iter, f, key: None }
    }
}

impl<I, F, K> Iterator for SegmentBy<I, F, K>
where 
    I: Clone + Iterator,
    F: Copy + FnMut(&I::Item) -> K,
    K: PartialEq,
{
    type Item = Segment<I, F, K>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(val) = self.iter.next() {
            if Some(&(self.f)(&val)) != self.key.as_ref() {
                let iter = self.iter.clone();
                let key = (self.f)(&val);
                self.key = Some((self.f)(&val));
                return Some(Segment::new(val, iter, self.f, key));
            }
        }

        None
    }
}
