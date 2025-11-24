use std::{
  borrow::Cow,
  ffi::CStr,
  sync::{
    Arc,
    mpsc::{Receiver, TryRecvError, channel},
  },
  thread,
};

use crate::{
  raw::{
    SherpaOnnxCreateOnlineStream, SherpaOnnxDecodeOnlineStream,
    SherpaOnnxDestroyOnlineRecognizerResult, SherpaOnnxDestroyOnlineStream,
    SherpaOnnxGetOnlineStreamResult, SherpaOnnxIsOnlineStreamReady,
    SherpaOnnxOnlineRecognizerResult, SherpaOnnxOnlineStream, SherpaOnnxOnlineStreamAcceptWaveform,
    SherpaOnnxOnlineStreamInputFinished, SherpaOnnxOnlineStreamIsEndpoint,
    SherpaOnnxOnlineStreamReset,
  },
  recognizer::OnlineRecognizer,
};

pub struct OnlineStream {
  _ptr: *const SherpaOnnxOnlineStream,
  _recog: Arc<OnlineRecognizer>,
  _sample_rate: i32,
}

unsafe impl Send for OnlineStream {}

impl Drop for OnlineStream {
  fn drop(&mut self) {
    unsafe {
      SherpaOnnxDestroyOnlineStream(self._ptr);
    }
  }
}

impl OnlineStream {
  pub fn new(recognizer: Arc<OnlineRecognizer>, sample_rate: i32) -> Option<Self> {
    unsafe {
      let _ptr = SherpaOnnxCreateOnlineStream(recognizer._ptr);

      if _ptr.is_null() {
        return None;
      }

      let out = Self {
        _ptr,
        _recog: recognizer,
        _sample_rate: sample_rate,
      };

      Some(out)
    }
  }

  pub fn decode(self, stream: Receiver<Box<[f32]>>) -> Option<Receiver<OnlineStreamResult>> {
    unsafe {
      let (tx, rx) = channel::<OnlineStreamResult>();

      thread::spawn(move || {
        let strrdata = self;

        loop {
          match stream.try_recv() {
            Ok(data) => {
              let samples = (&data as &[f32]).as_ptr();

              SherpaOnnxOnlineStreamAcceptWaveform(
                strrdata._ptr,
                strrdata._sample_rate,
                samples,
                data.len() as i32,
              );
            }
            Err(e) => match e {
              TryRecvError::Empty => {}
              TryRecvError::Disconnected => break,
            },
          }

          while SherpaOnnxIsOnlineStreamReady(strrdata._recog._ptr, strrdata._ptr) != 0 {
            SherpaOnnxDecodeOnlineStream(strrdata._recog._ptr, strrdata._ptr);
          }

          let result = SherpaOnnxGetOnlineStreamResult(strrdata._recog._ptr, strrdata._ptr);

          let result = OnlineStreamResult {
            _ptr: result,
            drop: true,
          };

          if SherpaOnnxOnlineStreamIsEndpoint(strrdata._recog._ptr, strrdata._ptr) != 0 {
            SherpaOnnxOnlineStreamReset(strrdata._recog._ptr, strrdata._ptr);
          }

          _ = tx.send(result);
        }

        SherpaOnnxOnlineStreamInputFinished(strrdata._ptr);

        while SherpaOnnxIsOnlineStreamReady(strrdata._recog._ptr, strrdata._ptr) != 0 {
          SherpaOnnxDecodeOnlineStream(strrdata._recog._ptr, strrdata._ptr);
        }

        let result = SherpaOnnxGetOnlineStreamResult(strrdata._recog._ptr, strrdata._ptr);

        let result = OnlineStreamResult {
          _ptr: result,
          drop: true,
        };

        _ = tx.send(result);
      });

      Some(rx)
    }
  }
}

pub struct OnlineStreamResult {
  pub(crate) _ptr: *const SherpaOnnxOnlineRecognizerResult,
  drop: bool,
}

unsafe impl Send for OnlineStreamResult {}

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

impl OnlineStreamResult {
  pub unsafe fn as_ptr(&self) -> *const SherpaOnnxOnlineRecognizerResult {
    self._ptr
  }

  pub unsafe fn as_ptr_disable_auto_free(&mut self) -> *const SherpaOnnxOnlineRecognizerResult {
    self.drop = false;
    unsafe { self.as_ptr() }
  }

  parse! {
    cstr {
      text, tokens, json
    },
    arr {
      f32 where timestamps,
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

impl Drop for OnlineStreamResult {
  fn drop(&mut self) {
    unsafe {
      if self.drop {
        SherpaOnnxDestroyOnlineRecognizerResult(self._ptr);
      }
    }
  }
}
