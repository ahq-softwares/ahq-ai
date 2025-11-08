use std::ffi::c_void;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn aligned_malloc(size: usize, align: usize) -> *mut c_void {
  unsafe {
    libc::aligned_malloc(size, align)
  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn aligned_free(ptr: *mut c_void) {
  unsafe {
    libc::aligned_free(ptr)
  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn aligned_realloc(ptr: *mut c_void, size: usize, align: usize) -> *mut c_void {
  unsafe {
    libc::aligned_realloc(ptr, size, align)
  }
}