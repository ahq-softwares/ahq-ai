use crate::{
  model::{OfflineModelConfig, OnlineModelConfig},
  raw::{
    SherpaOnnxCreateOfflineRecognizer, SherpaOnnxCreateOnlineRecognizer,
    SherpaOnnxDestroyOfflineRecognizer, SherpaOnnxDestroyOnlineRecognizer,
    SherpaOnnxOfflineRecognizer, SherpaOnnxOfflineRecognizerConfig, SherpaOnnxOnlineRecognizer,
    SherpaOnnxOnlineRecognizerConfig,
  },
};
use std::{ffi::CString, mem, sync::Arc};

pub enum DecodingMethod {
  GreedySearch,
}

impl DecodingMethod {
  fn into(self) -> Option<CString> {
    match self {
      Self::GreedySearch => CString::new("greedy_search").ok(),
    }
  }
}

macro_rules! generate_config {
  (
    $(
      {
        name = $name: ident,
        model = $model: ident
      }
    ),*
  ) => {
    pastey::paste! {
    $(
    pub struct [<$name Config>] {
      pub config: [<SherpaOnnx $name Config>],
      model: $model,
      _decoding_method: CString,
    }

    unsafe impl Send for [<$name Config>] {}

    impl [<$name Config>] {
      pub fn new(
        model_config: $model,
        decoding_method: DecodingMethod,
      ) -> Option<Self> {
        let conf: [<SherpaOnnx $name Config>] = unsafe { mem::zeroed() };

        let method_str = decoding_method.into()?;

        let mut out = Self {
          config: conf,
          model: model_config,
          _decoding_method: method_str,
        };

        out.config.decoding_method = out._decoding_method.as_ptr();

        out.config.model_config = out.model.config;

        Some(out)
      }

      pub fn build_recognizer(self) -> Option<Arc<$name>> {
        $name::new(self)
      }
    }

    pub struct $name {
      pub _ptr: *const [<SherpaOnnx $name>],
      _conf: [<$name Config>]
    }

    unsafe impl Send for $name {}

    impl Drop for $name {
      fn drop(&mut self) {
        unsafe {
          [<SherpaOnnxDestroy $name>](self._ptr);
        }
      }
    }

    impl $name {
      pub fn new(config: [<$name Config>]) -> Option<Arc<Self>> {
        unsafe {
          let recog = [<SherpaOnnxCreate $name>](&config.config);

          if recog.is_null() {
            return None;
          }

          Some(Arc::new(Self {
            _ptr: recog,
            _conf: config,
          }))
        }
      }
    }
    )*
    }
  };
}

generate_config! {
  {
    name = OfflineRecognizer,
    model = OfflineModelConfig
  },
  {
    name = OnlineRecognizer,
    model = OnlineModelConfig
  }
}
