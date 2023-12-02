//! A collection of atomic "polyfill" routines, to use a term from JavaScript.
//!
//! ARMv6-M processors like the Cortex-M0 don't support the fancier atomic
//! operations available on most other ARM processors. In particular, they have
//! no atomic swap or read-modify-write instructions. This module provides
//! traits that use the native atomics on M3 and later, and fallback
//! implementations on M0.
//!
//! The M0 implementations rely on disabling interrupts. This means that:
//!
//! 1. They will hurt interrupt latency/jitter. However, the M0 already has
//!    pretty poor interrupt latency/jitter because of uninterruptible
//!    instructions and lack of BASEPRI. So, not a big loss.
//!
//! 2. They don't work in unprivileged mode. But, neither does most of `lilos`.
//!
//! This is exposed so that applications don't have to rewrite it for M0
//! support.

#[cfg(all(not(feature = "portable-atomic"), feature = "has-native-rmw"))]
#[path = "atomic/native_rmw.rs"]
mod impl_mod;

#[cfg(all(not(feature = "portable-atomic"), target_arch = "arm", not(feature = "has-native-rmw")))]
#[path = "atomic/cortex_no_rmw.rs"]
mod impl_mod;

#[cfg(feature = "portable-atomic")]
#[path = "atomic/portable_atomic.rs"]
mod impl_mod;

pub use impl_mod::{AtomicBool, AtomicPtr, AtomicU32, AtomicUsize, Ordering};

/// Basic atomic operations.
pub trait AtomicExt {
    /// Primitive type corresponding to this atomic type.
    type Value;

    /// Atomically exchange our current contents for `val`, returning the
    /// original contents.
    fn swap_polyfill(&self, val: Self::Value, ordering: Ordering) -> Self::Value;
}

/// Atomic operations that apply to arithmetic types.
pub trait AtomicArithExt: AtomicExt {
    /// Atomically add `val` to our contents, returning the original value.
    fn fetch_add_polyfill(&self, val: Self::Value, ordering: Ordering) -> Self::Value;
    /// Atomically OR `val` into our contents, returning the original value.
    fn fetch_or_polyfill(&self, val: Self::Value, ordering: Ordering) -> Self::Value;
}

