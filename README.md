iter_segment
===

Iterator extension for segmenting items.

*The author of this crate is not good at English.*  
*Forgive me if the document is hard to read.*

## What is this?

This crate provides an iterator extension that supports items grouping.
Note that “Grouping” here is dependent on orders, so it is slightly
different from SQL's `GROUP BY` caluse.

## Examples

```rust
use iter_segment::prelude::*;

let items = vec![("a", 0), ("a", 1), ("b", 0), ("a", 2)];
let segs = &mut items.iter().segment_by(|x| x.0);

let seg = &mut segs.next().unwrap();
assert_eq!(seg.next(), Some(&("a", 0)));
assert_eq!(seg.next(), Some(&("a", 1)));
assert_eq!(seg.next(), None);

let seg = &mut segs.next().unwrap();
assert_eq!(seg.next(), Some(&("b", 0)));
assert_eq!(seg.next(), None);

let seg = &mut segs.next().unwrap();
assert_eq!(seg.next(), Some(&("a", 2)));

let seg = &mut segs.next();
assert!(seg.is_none());
```

## Other options

There are many crates that can group items from iterators.

Followings are some of them.

📦 **[`itertools`][it_0] (De facto standard of iterator extension)**

* [`chunk_by`][it_1] - Creates an iterator for grouping.  
  It is unique in that if access to each group in any order is needed,  
  iterator itself provides storage to accomplish its operation.
* [`into_grouping_map_by`][it_2] - Creates grouping aggregation helper  
  It is designed for smart aggregation from grouping.

📦 **[`grouping_by`][gb_0] (Grouping hash map creator)**

* [`grouping_by`][gb_1] - Creates hash map grouped by key.

## Highlights

I feel this approach is natural. But as of 2025, I cannot find similar
approaches. This makes me little uneasy. Please check carefully before
using to make sure there are no problems.

😄 **Pros**

Low memory consumption. No heap memory required.  
So it can work in `core` environment.

🤔 **Cons**

Unlike popular iterator methods, iterators and closures are required
additional trait bounds.

* `Clone` trait is required for iterators.
* `Copy` trait is required for closures.

<!-- Links -->

[it_0]: https://crates.io/crates/itertools
[it_1]: https://docs.rs/itertools/0.14.0/itertools/trait.Itertools.html#method.chunk_by
[it_2]: https://docs.rs/itertools/0.14.0/itertools/trait.Itertools.html#method.into_grouping_map_by
[gb_0]: https://crates.io/crates/grouping_by
[gb_1]: https://docs.rs/grouping_by/0.2.2/grouping_by/trait.GroupingBy.html#tymethod.grouping_by