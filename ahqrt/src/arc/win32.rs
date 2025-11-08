use std::ptr;

use windows::Win32::{Foundation::{CloseHandle, HANDLE}, System::Threading::{CreateMutexW, INFINITE, ReleaseMutex, WaitForSingleObject}};

#[repr(C)]
pub struct RawMutex {
  _inner: HANDLE
}

impl RawMutex {
  pub fn new() -> *mut Self {
    unsafe {
      let handle = CreateMutexW(None, false, None).expect("Unexpected Mutex Create Error");

      let data = Self {
        _inner: handle
      };

      let pt = libc::aligned_malloc(size_of::<Self>(), align_of::<Self>()) as *mut Self;

      ptr::write(pt, data);

      pt
    }
  }

  pub fn lock(&self) {
    unsafe {
      WaitForSingleObject(self._inner, INFINITE);
    }
  }

  pub fn unlock(&self) {
    unsafe {
      ReleaseMutex(self._inner).expect("Sudden Crash while trying to unlock");
    }
  }

  pub fn close(&self) {
    unsafe {
      _ = CloseHandle(self._inner);
    }
  }
}