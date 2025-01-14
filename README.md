[`SinglethreadedCell`] is a cell that only supports single-threaded platforms.

It's similar to [`core::cell::UnsafeCell`], except it implements `Deref` and
`Sync`, which it can do because it never exposes external mutation and only
supports single-threaded platforms.

[`SinglethreadedCell`]: https://docs.rs/singlethreaded-cell/*/singlethreaded_cell/struct.SinglethreadedCell.html
[`core::cell::UnsafeCell`]: https://doc.rust-lang.org/stable/core/cell/struct.UnsafeCell.html
