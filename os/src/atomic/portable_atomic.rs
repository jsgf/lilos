pub use portable_atomic::{AtomicBool, AtomicPtr, AtomicU32, AtomicUsize, Ordering};
use super::{AtomicExt, AtomicArithExt};

impl<T> AtomicExt for AtomicPtr<T> {
    type Value = *mut T;

    #[inline(always)]
    fn swap_polyfill(&self, val: Self::Value, ordering: Ordering) -> Self::Value {
        self.swap(val, ordering)
    }
}

impl AtomicExt for AtomicU32 {
    type Value = u32;

    #[inline(always)]
    fn swap_polyfill(&self, val: Self::Value, ordering: Ordering) -> Self::Value {
        self.swap(val, ordering)
    }
}

impl AtomicArithExt for AtomicU32 {
    #[inline(always)]
    fn fetch_add_polyfill(&self, val: Self::Value, ordering: Ordering) -> Self::Value {
        self.fetch_add(val, ordering)
    }

    #[inline(always)]
    fn fetch_or_polyfill(&self, val: Self::Value, ordering: Ordering) -> Self::Value {
        self.fetch_or(val, ordering)
    }
}

impl AtomicExt for AtomicUsize {
    type Value = usize;

    #[inline(always)]
    fn swap_polyfill(&self, val: Self::Value, ordering: Ordering) -> Self::Value {
        self.swap(val, ordering)
    }
}

impl AtomicArithExt for AtomicUsize {
    #[inline(always)]
    fn fetch_add_polyfill(&self, val: Self::Value, ordering: Ordering) -> Self::Value {
        AtomicUsize::fetch_add(self, val, ordering)
    }

    #[inline(always)]
    fn fetch_or_polyfill(&self, val: Self::Value, ordering: Ordering) -> Self::Value {
        self.fetch_or(val, ordering)
    }
}

impl AtomicExt for AtomicBool {
    type Value = bool;

    #[inline(always)]
    fn swap_polyfill(&self, val: Self::Value, ordering: Ordering) -> Self::Value {
        self.swap(val, ordering)
    }
}
