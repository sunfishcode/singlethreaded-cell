// WASIp1 and WASIp2 don't support threads.
#![cfg(any(
    all(target_os = "wasi", target_env = "p1"),
    all(target_os = "wasi", target_env = "p2"),
))]
#![no_std]
#![doc = include_str!("../README.md")]
#![deny(clippy::undocumented_unsafe_blocks)]

/// Similar to [`core::cell::UnsafeCell`], except it implements `Deref` and
/// `Sync`, and only supports single-threaded platforms.
#[derive(Debug, Default)]
pub struct SinglethreadCell<T>(core::cell::UnsafeCell<T>);

impl<T> core::ops::Deref for SinglethreadCell<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        // SAFETY: We never expose external mutation.
        unsafe { &*self.0.get() }
    }
}

impl<T> SinglethreadCell<T> {
    /// Like [`core::cell::UnsafeCell::new`].
    #[inline]
    pub const fn new(value: T) -> Self {
        Self(core::cell::UnsafeCell::new(value))
    }
}

// SAFETY: We only support single-threaded platforms.
unsafe impl<T> Sync for SinglethreadCell<T> {}
