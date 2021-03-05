This crate is useful for preventing expensive actions from taking place while adding elements to collections.
It exports a single trait, `TryPush` and has implementations for both `Vec<T>` and `VecDeque<T>` from the standard library.
