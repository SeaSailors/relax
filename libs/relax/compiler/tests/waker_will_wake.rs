// `Waker::will_wake` decides whether calling `wake` on one waker does the same as on the other, and
// std answers it by comparing the data pointer and the VTABLE'S ADDRESS (`ptr::eq`), never the
// vtable's contents. Deriving `PartialEq` for `RawWaker` compares the four function pointers by
// value, so two distinct vtables with identical bodies compare equal — and any executor that skips
// re-registering a waker when `will_wake` says so then wakes the wrong task. tokio's timer wheel is
// one: a spawned task awaiting `sleep` was never resumed while another task was woken spuriously.
use std::sync::atomic::AtomicUsize;
use std::task::{RawWaker, RawWakerVTable, Waker};

unsafe fn v_clone(p: *const ()) -> RawWaker { RawWaker::new(p, &A) }
unsafe fn v_wake(_p: *const ()) {}
unsafe fn v_wake_by_ref(_p: *const ()) {}
unsafe fn v_drop(_p: *const ()) {}

// Two vtables with identical contents but different addresses.
static A: RawWakerVTable = RawWakerVTable::new(v_clone, v_wake, v_wake_by_ref, v_drop);
static B: RawWakerVTable = RawWakerVTable::new(v_clone, v_wake, v_wake_by_ref, v_drop);

#[test]
fn distinct_vtables_with_equal_contents_are_not_the_same_waker() {
    let data = &AtomicUsize::new(0) as *const AtomicUsize as *const ();
    let wa = unsafe { Waker::from_raw(RawWaker::new(data, &A)) };
    let wb = unsafe { Waker::from_raw(RawWaker::new(data, &B)) };
    assert!(!wa.will_wake(&wb), "two different vtables with equal contents compared equal");
    std::mem::forget(wa);
    std::mem::forget(wb);
}

#[test]
fn the_same_vtable_and_data_is_the_same_waker() {
    let data = &AtomicUsize::new(0) as *const AtomicUsize as *const ();
    let wa = unsafe { Waker::from_raw(RawWaker::new(data, &A)) };
    let wb = unsafe { Waker::from_raw(RawWaker::new(data, &A)) };
    assert!(wa.will_wake(&wb), "identical wakers compared unequal");
    std::mem::forget(wa);
    std::mem::forget(wb);
}
