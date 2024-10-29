#![deny(dead_code)]
#![deny(unused_imports)]

extern crate std;
extern crate libc;

use std::alloc::{Layout, GlobalAlloc, System};
use libc::{size_t, c_void};

#[repr(transparent)]
struct Heap {
    heap: System,
}

impl Heap {
    pub const fn new() -> Heap {
        Heap {
            heap: System,
        }
    }
}

unsafe impl GlobalAlloc for Heap {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.heap.alloc(layout)
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.heap.dealloc(ptr, layout);
    }
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        self.heap.realloc(ptr, layout, new_size)
    }
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        self.heap.alloc_zeroed(layout)
    }
}

#[global_allocator]
static HEAP: Heap = Heap::new();

unsafe fn __layout(size: usize, align: usize) -> Layout {
    Layout::from_size_align_unchecked(size, align)
}

unsafe fn layout(size: size_t, align: size_t) -> Layout {
    // SAFETY: we use as usize cause the type of size_t can be changed.
    __layout(size as usize, align as usize)
}

#[no_mangle]
pub unsafe extern "C" fn allocate(size: size_t, align: size_t) -> *mut c_void {
    let layout = layout(size, align);
    HEAP.alloc(layout) as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn allocate_zeroed(size: size_t, align: size_t) -> *mut c_void {
    let layout = layout(size, align);
    HEAP.alloc_zeroed(layout) as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn deallocate(ptr: *mut c_void, size: size_t, align: size_t) {
    let layout = layout(size, align);
    HEAP.dealloc(ptr as *mut u8, layout)
}

#[no_mangle]
pub unsafe extern "C" fn reallocate(ptr: *mut c_void, size: size_t, align: size_t, new_size: size_t) -> *mut c_void {
    let layout = layout(size, align);
    HEAP.realloc(ptr as *mut u8, layout, new_size as usize) as *mut c_void
}
