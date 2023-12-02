pub use core::sync::atomic::{AtomicBool, AtomicPtr, AtomicU32, AtomicUsize, Ordering};
use super::{AtomicExt, AtomicArithExt};

impl AtomicExt for AtomicU32 {
    type Value = u32;

    fn swap_polyfill(&self, val: Self::Value, ordering: Ordering) -> Self::Value {
        self.swap(val, ordering)
    }
}

impl AtomicArithExt for AtomicU32 {
    fn fetch_add_polyfill(&self, val: Self::Value, ordering: Ordering) -> Self::Value {
        self.fetch_add(val, ordering)
    }
    fn fetch_or_polyfill(&self, val: Self::Value, ordering: Ordering) -> Self::Value {
        self.fetch_or(val, ordering)
    }
}

impl AtomicExt for AtomicUsize {
    type Value = usize;

    fn swap_polyfill(&self, val: Self::Value, ordering: Ordering) -> Self::Value {
        self.swap(val, ordering)
    }
}

impl AtomicArithExt for AtomicUsize {
    fn fetch_add_polyfill(&self, val: Self::Value, ordering: Ordering) -> Self::Value {
        self.fetch_add(val, ordering)
    }
    fn fetch_or_polyfill(&self, val: Self::Value, ordering: Ordering) -> Self::Value {
        self.fetch_or(val, ordering)
    }
}

impl<T> AtomicExt for AtomicPtr<T> {
    type Value = *mut T;

    fn swap_polyfill(&self, val: Self::Value, ordering: Ordering) -> Self::Value {
        self.swap(val, ordering)
    }
}

impl AtomicExt for AtomicBool {
    type Value = bool;

    fn swap_polyfill(&self, val: Self::Value, ordering: Ordering) -> Self::Value {
        self.swap(val, ordering)
    }
}
