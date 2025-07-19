iter_chunks_ext
===

Iterator extension for grouping items.

*The author of this crate is not good at English.*  
*Forgive me if the document is hard to read.*

## What is this?

This crate provides an iterator extension that supports items grouping.

Note that “Grouping” here is dependent on orders, so it is slightly
different from SQL's `GROUP BY` effect.

## Examples

```rust
use iter_chunks_ext::prelude::*;

let items = vec![("a", 0), ("a", 1), ("b", 0), ("a", 2)];
let chunks = &mut items.iter().chunks(|x| x.0);

let chunk = &mut chunks.next().unwrap();
assert_eq!(chunk.next(), Some(&("a", 0)));
assert_eq!(chunk.next(), Some(&("a", 1)));
assert_eq!(chunk.next(), None);

let chunk = &mut chunks.next().unwrap();
assert_eq!(chunk.next(), Some(&("b", 0)));
assert_eq!(chunk.next(), None);

let chunk = &mut chunks.next().unwrap();
assert_eq!(chunk.next(), Some(&("a", 2)));
assert_eq!(chunk.next(), None);

let chunk = &mut chunks.next();
assert!(chunk.is_none());
```

## Other options

There are many crates that can group items from iterators.

Followings are some of them.

📦 **[`itertools`][it_0] (Extra iterator tools)**

* [`chunk_by`][it_1] - Creates an iterator for grouping.  
  (Access to each group in any order is achieved by memory allocation.)

📦 **[`grouping_by`][gb_0] (Grouping hash map creator)**

* [`grouping_by`][gb_1] - Creates hash map grouped by key.  
  (Memory allocation is required for use of hash maps.)

## Highlights

This crate is characterized by following Pros/Cons.  
These Pros/Cons are two sides of the same coin.

😊 **Pros**

Low memory consumption. No heap memory required.  
So this crate can work in `core` environment.

🤔 **Cons**

Unlike popular iterator methods, iterators and closures are required
additional trait bounds.

* `Clone` trait is required for iterators.
* `Copy` trait is required for closures.

## Note

I feel my approach is natural. But as of 2025, I cannot find same approach.  
This makes me little uneasy. So, please check carefully before using.

## Versions

See [CHANGELOG](CHANGELOG.md).

<!-- Links -->

[it_0]: https://crates.io/crates/itertools
[it_1]: https://docs.rs/itertools/0.14.0/itertools/trait.Itertools.html#method.chunk_by
[gb_0]: https://crates.io/crates/grouping_by
[gb_1]: https://docs.rs/grouping_by/0.2.2/grouping_by/trait.GroupingBy.html#tymethod.grouping_by
