use std::{ffi::CString, mem};

use crate::{
  raw::{SherpaOnnxOfflineModelConfig, SherpaOnnxOnlineModelConfig},
  transducer::{OfflineTransducerModelConfig, OnlineTransducerModelConfig},
};

macro_rules! generate_model_config {
  (
    $(
      {
        name = $name:ident,
        transducer = $transducer:ident
      }
    ),*
  ) => {
    pastey::paste! {
      $(
        pub struct $name {
          pub config: [<SherpaOnnx $name>],
          _provider: CString,
          _tokens: CString,
          _ref1: $transducer
        }

        unsafe impl Send for $name {}

        impl $name {
          pub fn new(
            transducer: $transducer,
            provider: Provider,
            model_tokens_path: &str,
          ) -> Option<Self> {
            let mut conf: [<SherpaOnnx $name>] = unsafe { mem::zeroed() };

            conf.debug = 0;
            conf.num_threads = 4;

            let provider = provider.into()?;

            let tokens = CString::new(model_tokens_path).ok()?;

            let mut out = Self {
              config: conf,
              _provider: provider,
              _tokens: tokens,
              _ref1: transducer,
            };

            out.config.provider = out._provider.as_ptr();
            out.config.tokens = out._tokens.as_ptr();
            out.config.transducer = out._ref1._ptr;

            Some(out)
          }

          pub fn set_debug(&mut self, debug: bool) {
            self.config.debug = if debug { 1 } else { 0 };
          }

          pub fn set_threads(&mut self, threads: i32) {
            self.config.num_threads = threads;
          }
        }
      )*
    }
  };
}

pub enum Provider<'a> {
  Cpu,
  CustomUnsafeIKnowWhatImDoing(&'a str),
}

impl<'a> Provider<'a> {
  pub(crate) fn into(self) -> Option<CString> {
    match self {
      Self::Cpu => CString::new("cpu").ok(),
      Provider::CustomUnsafeIKnowWhatImDoing(x) => CString::new(x).ok(),
    }
  }
}

generate_model_config! {
  {
    name = OfflineModelConfig,
    transducer = OfflineTransducerModelConfig
  },
  {
    name = OnlineModelConfig,
    transducer = OnlineTransducerModelConfig
  }
}
