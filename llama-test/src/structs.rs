use std::sync::Arc;

pub struct LlamaServer {
  url: Arc<str>,
  port: u16,
}

impl LlamaServer {
  pub fn new(url: Arc<str>, port: u16) -> Self {
    Self { url, port }
  }
}
