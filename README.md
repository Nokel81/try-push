This crate is useful for preventing expensive actions from taking place while adding elements to collections.
It exports a single trait, `TryPush` and has implementations for both `Vec<T>` and `VecDeque<T>` from the standard library.

Examples:
---

```rust
use try_push::*;

let mut vec = Vec::with_capacity(4);
vec.push(1);
vec.push(2);
vec.push(3);
vec.push(4); // won't reallocate
// vec.push(5); // will reallocate

assert_eq!(vec.try_push(5), Element::NotAdded(5));
```
