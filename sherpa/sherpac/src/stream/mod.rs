use std::{borrow::Cow, ffi::CStr};

use crate::{
  Wave,
  raw::{
    SherpaOnnxAcceptWaveformOffline, SherpaOnnxCreateOfflineStream, SherpaOnnxDecodeOfflineStream,
    SherpaOnnxDestroyOfflineRecognizerResult, SherpaOnnxDestroyOfflineStream,
    SherpaOnnxGetOfflineStreamResult, SherpaOnnxOfflineRecognizerResult, SherpaOnnxOfflineStream,
  },
  recognizer::OfflineRecognizer,
};

pub mod online;

pub struct OfflineStream<'a> {
  _ptr: *const SherpaOnnxOfflineStream,
  _wave: Wave,
  _recog: &'a OfflineRecognizer,
}

unsafe impl<'a> Send for OfflineStream<'a> {}

impl<'a> Drop for OfflineStream<'a> {
  fn drop(&mut self) {
    unsafe {
      SherpaOnnxDestroyOfflineStream(self._ptr);
    }
  }
}

impl<'a> OfflineStream<'a> {
  pub fn new(recognizer: &'a OfflineRecognizer, wave: Wave) -> Option<Self> {
    unsafe {
      let _ptr = SherpaOnnxCreateOfflineStream(recognizer._ptr);

      if _ptr.is_null() {
        return None;
      }

      let out = Self {
        _ptr,
        _wave: wave,
        _recog: recognizer,
      };

      let wave = &*out._wave._ptr;
      SherpaOnnxAcceptWaveformOffline(_ptr, wave.sample_rate, wave.samples, wave.num_samples);

      Some(out)
    }
  }

  pub fn decode(&self) -> Option<OfflineStreamResult> {
    unsafe {
      SherpaOnnxDecodeOfflineStream(self._recog._ptr, self._ptr);

      let ptr = SherpaOnnxGetOfflineStreamResult(self._ptr);

      if ptr.is_null() {
        return None;
      }

      Some(OfflineStreamResult {
        _ptr: ptr,
        drop: true,
      })
    }
  }
}

pub struct OfflineStreamResult {
  pub(crate) _ptr: *const SherpaOnnxOfflineRecognizerResult,
  drop: bool,
}

macro_rules! parse {
  (
    $(
      $class:ident {
        $($param:tt)*
      }
    ),*
  ) => {
    $(
      parse! {
        inner $class {
          $($param)*
        }
      }
    )*
  };

  (
    inner cstr {
      $(
        $param:ident
      ),*
    }
  ) => {
    $(
      pastey::paste! {
        pub fn $param<'a>(&'a self) -> Option<&'a CStr> {
          unsafe {
            let txt = (&*self._ptr).$param;

            if txt.is_null() {
              return None;
            }

            Some(CStr::from_ptr(txt))
          }
        }

        pub fn [<$param _lossy>]<'a>(&'a self) -> Option<Cow<'a, str>> {
          self.$param().map(|x| x.to_string_lossy())
        }
      }
    )*
  };

  (
    inner arr {
      $(
        $datatype:ty where $param:ident
      ),*
    }
  ) => {
    $(
      pastey::paste! {
        pub fn $param<'a>(&'a self) -> Option<&'a [$datatype]> {
          unsafe {
            let data = (&*self._ptr).$param;

            if data.is_null() {
              return None;
            }

            Some(
              ::std::slice::from_raw_parts(data, (&*self._ptr).count as usize)
            )
          }
        }
      }
    )*
  };
}

impl OfflineStreamResult {
  pub unsafe fn as_ptr(&self) -> *const SherpaOnnxOfflineRecognizerResult {
    self._ptr
  }

  pub unsafe fn as_ptr_disable_auto_free(&mut self) -> *const SherpaOnnxOfflineRecognizerResult {
    self.drop = false;
    unsafe { self.as_ptr() }
  }

  parse! {
    cstr {
      text, tokens, json, lang, emotion, event
    },
    arr {
      f32 where timestamps,
      f32 where durations,
      *const i8 where tokens_arr
    }
  }

  pub fn parsed_tokens_arr<'a>(&'a self) -> Option<Box<[&'a str]>> {
    let arr = self.tokens_arr()?;

    arr
      .iter()
      .map(|x| unsafe {
        if x.is_null() {
          return None;
        }

        CStr::from_ptr(*x).to_str().ok()
      })
      .collect()
  }

  pub fn tokens_arr_lossy<'a>(&'a self) -> Option<Box<[Cow<'a, str>]>> {
    let arr = self.tokens_arr()?;

    arr
      .iter()
      .map(|x| unsafe {
        if x.is_null() {
          return None;
        }

        Some(CStr::from_ptr(*x).to_string_lossy())
      })
      .collect()
  }
}

impl Drop for OfflineStreamResult {
  fn drop(&mut self) {
    unsafe {
      if self.drop {
        SherpaOnnxDestroyOfflineRecognizerResult(self._ptr);
      }
    }
  }
}
