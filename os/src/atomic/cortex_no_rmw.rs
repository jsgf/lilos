pub use core::sync::atomic::{AtomicBool, AtomicPtr, AtomicU32, AtomicUsize, Ordering};
use super::{AtomicExt, AtomicArithExt};

#[inline(always)]
fn rmw_ordering(o: Ordering) -> (Ordering, Ordering) {
    match o {
        Ordering::AcqRel => (Ordering::Acquire, Ordering::Release),
        Ordering::Relaxed => (o, o),
        Ordering::SeqCst => (o, o),
        Ordering::Acquire => (Ordering::Acquire, Ordering::Relaxed),
        Ordering::Release => (Ordering::Relaxed, Ordering::Release),
        _ => panic!(),
    }
}

impl<T> AtomicExt for AtomicPtr<T> {
    type Value = *mut T;

    fn swap_polyfill(&self, val: Self::Value, ordering: Ordering) -> Self::Value {
        let (lo, so) = rmw_ordering(ordering);
        cortex_m::interrupt::free(|_| {
            let x = self.load(lo);
            self.store(val, so);
            x
        })
    }
}

impl AtomicExt for AtomicU32 {
    type Value = u32;

    #[inline(always)]
    fn swap_polyfill(&self, val: Self::Value, ordering: Ordering) -> Self::Value {
        let (lo, so) = rmw_ordering(ordering);
        cortex_m::interrupt::free(|_| {
            let x = self.load(lo);
            self.store(val, so);
            x
        })
    }
}

impl AtomicArithExt for AtomicU32 {
    #[inline(always)]
    fn fetch_add_polyfill(&self, val: Self::Value, ordering: Ordering) -> Self::Value {
        let (lo, so) = rmw_ordering(ordering);
        cortex_m::interrupt::free(|_| {
            let x = self.load(lo);
            self.store(x.wrapping_add(val), so);
            x
        })
    }

    #[inline(always)]
    fn fetch_or_polyfill(&self, val: Self::Value, ordering: Ordering) -> Self::Value {
        let (lo, so) = rmw_ordering(ordering);
        cortex_m::interrupt::free(|_| {
            let x = self.load(lo);
            self.store(x | val, so);
            x
        })
    }
}

impl AtomicExt for AtomicUsize {
    type Value = usize;

    #[inline(always)]
    fn swap_polyfill(&self, val: Self::Value, ordering: Ordering) -> Self::Value {
        let (lo, so) = rmw_ordering(ordering);
        cortex_m::interrupt::free(|_| {
            let x = self.load(lo);
            self.store(val, so);
            x
        })
    }
}

impl AtomicArithExt for AtomicUsize {
    #[inline(always)]
    fn fetch_add_polyfill(&self, val: Self::Value, ordering: Ordering) -> Self::Value {
        let (lo, so) = rmw_ordering(ordering);
        cortex_m::interrupt::free(|_| {
            let x = self.load(lo);
            self.store(x.wrapping_add(val), so);
            x
        })
    }

    #[inline(always)]
    fn fetch_or_polyfill(&self, val: Self::Value, ordering: Ordering) -> Self::Value {
        let (lo, so) = rmw_ordering(ordering);
        cortex_m::interrupt::free(|_| {
            let x = self.load(lo);
            self.store(x | val, so);
            x
        })
    }
}


impl AtomicExt for AtomicBool {
    type Value = bool;

    #[inline(always)]
    fn swap_polyfill(&self, val: Self::Value, ordering: Ordering) -> Self::Value {
        let (lo, so) = rmw_ordering(ordering);
        cortex_m::interrupt::free(|_| {
            let x = self.load(lo);
            self.store(val, so);
            x
        })
    }
}
