use std::ffi::{CStr, CString, c_char};

#[repr(C)]
pub struct SharedSafeStr {
  _raw: *mut c_char,
  _drop: unsafe extern "C" fn(*mut c_char)
}

unsafe extern "C" fn mfree(data: *mut c_char) {
  unsafe {
    _ = CString::from_raw(data);
  }
}

impl SharedSafeStr {
  pub fn create(data: CString) -> Self {
    let _raw = data.into_raw();

    Self {
      _raw,
      _drop: mfree
    }
  }

  /// Please note that the lifetime <'a> refers to the lifetime of the
  /// const reference
  /// Please ensure that the const reference stays as long as <'a>
  pub unsafe fn as_str<'a>(data: *const SharedSafeStr) -> &'a CStr {
    unsafe { CStr::from_ptr((*data)._raw) }
  }
}

impl Drop for SharedSafeStr {
  fn drop(&mut self) {
    unsafe {
      (self._drop)(self._raw)
    }
  }
}