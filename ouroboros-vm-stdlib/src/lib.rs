use std::{ffi::c_void, mem::ManuallyDrop, ptr};

pub mod echo;

#[no_mangle]
pub fn alloc(size: usize) -> *mut c_void {
    let mut buf = ManuallyDrop::new(Vec::with_capacity(size));
    let ptr = buf.as_mut_ptr();
    ptr as *mut c_void
}

#[no_mangle]
pub fn free(ptr: *mut c_void, size: usize) {
    unsafe {
        let mut buf = ManuallyDrop::new(Vec::from_raw_parts(ptr, size, size));
        ManuallyDrop::drop(&mut buf);
    }
}
