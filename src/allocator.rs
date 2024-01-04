use core::alloc::{GlobalAlloc, Layout};

use crate::c_types;
use crate::bindings;

extern "C" {
    #[no_mangle]
    static RUST_GFP_KERNEL: bindings::gfp_t;
}

pub struct KernelAllocator;

unsafe impl GlobalAlloc for KernelAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        return bindings::krealloc(
            0 as *const c_types::c_void,
            layout.size(),
            RUST_GFP_KERNEL,
        ) as *mut u8;
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        bindings::kfree(ptr as *const c_types::c_void);
    }
}
