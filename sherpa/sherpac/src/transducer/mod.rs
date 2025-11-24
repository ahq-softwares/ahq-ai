use std::{ffi::CString, mem};

use crate::raw::{SherpaOnnxOfflineTransducerModelConfig, SherpaOnnxOnlineTransducerModelConfig};

macro_rules! generate_config {
  (
    $(
      {
        name = $name:ident
      }
    ),*
  ) => {
    pastey::paste! {
      $(
        pub struct $name {
          pub _ptr: [<SherpaOnnx $name>],
          _encoder: CString,
          _decoder: CString,
          _joiner: CString
        }

        impl $name {
          pub fn new(
            encoder: &str,
            decoder: &str,
            joiner: &str
          ) -> Option<Self> {
            unsafe {
              let _encoder = CString::new(encoder).ok()?;
              let _decoder = CString::new(decoder).ok()?;
              let _joiner = CString::new(joiner).ok()?;

              let mut out = Self {
                _ptr: mem::zeroed(),
                _encoder,
                _decoder,
                _joiner
              };

              out._ptr.encoder = out._encoder.as_ptr();
              out._ptr.decoder = out._decoder.as_ptr();
              out._ptr.joiner = out._joiner.as_ptr();

              Some(out)
            }
          }
        }
      )*
    }
  };
}

generate_config! {
  {
    name = OfflineTransducerModelConfig
  },
  {
    name = OnlineTransducerModelConfig
  }
}
