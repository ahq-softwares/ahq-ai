use std::{ffi::CString, slice};

use crate::raw::{SherpaOnnxFreeWave, SherpaOnnxReadWave, SherpaOnnxWave};

#[allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]
pub mod raw;

pub mod model;
pub mod recognizer;
pub mod stream;
pub mod transducer;

pub struct Wave {
  pub(crate) _ptr: *const SherpaOnnxWave,
}

unsafe impl Send for Wave {}

impl Drop for Wave {
  fn drop(&mut self) {
    unsafe {
      SherpaOnnxFreeWave(self._ptr);
    }
  }
}

impl Wave {
  pub fn new_from_file(file: &str) -> Option<Self> {
    let file = CString::new(file).ok()?;

    unsafe {
      let out = SherpaOnnxReadWave(file.as_ptr());

      if out.is_null() {
        return None;
      }

      Some(Self { _ptr: out })
    }
  }

  pub unsafe fn as_raw_ptr(&self) -> *const SherpaOnnxWave {
    self._ptr
  }

  pub unsafe fn samples_as_ptr(&self) -> *const f32 {
    unsafe { (&*self._ptr).samples }
  }

  pub fn samples(&self) -> &[f32] {
    unsafe { slice::from_raw_parts((&*self._ptr).samples, self.num_samples() as usize) }
  }

  pub fn sample_rate(&self) -> i32 {
    unsafe { (&*self._ptr).sample_rate }
  }

  pub fn num_samples(&self) -> i32 {
    unsafe { (&*self._ptr).num_samples }
  }
}
