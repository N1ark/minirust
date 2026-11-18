//@ compile-flags: --minirust-tree-borrows

// `&raw const X` goes through the shared reference to the static, so the resulting pointer is
// derived from a retagged shared reference. Writing to the non-`UnsafeCell` part through it is UB.

use std::sync::atomic::AtomicI32;

static X: (i32, AtomicI32) = (0, AtomicI32::new(1));

fn main() {
    let ptr = &raw const X;
    unsafe { ptr.cast_mut().write((1, AtomicI32::new(0))) }; // UB! Child write to Frozen
}
