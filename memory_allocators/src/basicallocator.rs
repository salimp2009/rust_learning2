#![feature(allocator_api)]

use core::slice;
use std::alloc::{AllocError, Allocator, Layout};
use std::ptr::NonNull;
use std::u8;

use libc::{free, malloc};

pub struct BasicAllocator;

unsafe impl Allocator for BasicAllocator {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        unsafe {
            let ptr = malloc(layout.size() as libc::size_t);
            let slice = std::slice::from_raw_parts_mut(ptr as *mut u8, layout.size());
            Ok(NonNull::new_unchecked(slice))
        }
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        free(ptr.as_ptr() as *mut libc::c_void);
    }
}
