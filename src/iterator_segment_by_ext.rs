//! Provider of [`IteratorSegmentByExt`].

use crate::segment_by::SegmentBy;

/// Iterator extension for [`segment_by`](IteratorSegmentByExt::segment_by) method.
pub trait IteratorSegmentByExt: Clone + Iterator {
    /// Creates an iterator to segment items.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use iter_segment::prelude::*;
    /// let items = vec![("a", 0), ("a", 1), ("b", 0), ("a", 2)];
    /// let segs = &mut items.iter().segment_by(|x| x.0);
    /// 
    /// let seg = &mut segs.next().unwrap();
    /// assert_eq!(seg.next(), Some(&("a", 0)));
    /// assert_eq!(seg.next(), Some(&("a", 1)));
    /// assert_eq!(seg.next(), None);
    /// 
    /// let seg = &mut segs.next().unwrap();
    /// assert_eq!(seg.next(), Some(&("b", 0)));
    /// assert_eq!(seg.next(), None);
    /// 
    /// let seg = &mut segs.next().unwrap();
    /// assert_eq!(seg.next(), Some(&("a", 2)));
    /// 
    /// let seg = &mut segs.next();
    /// assert!(seg.is_none());
    /// ```
    fn segment_by<F, K>(self, f: F) -> SegmentBy<Self, F, K>
    where
        F: FnMut(&Self::Item) -> K,
        K: PartialEq,
    {
        SegmentBy::new(self, f)
    }
}

impl<T> IteratorSegmentByExt for T
where 
    T: Clone + Iterator
{
    // nop.
}
