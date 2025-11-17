use zeroize::{Zeroize, ZeroizeOnDrop};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize, Zeroize, ZeroizeOnDrop)]
pub struct DatabaseConfig {
  pub authdb: AuthDbConfig,
  pub cache: CacheConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Zeroize, ZeroizeOnDrop)]
#[serde(tag = "db")]
pub enum AuthDbConfig {
  #[serde(rename = "moka")]
  Moka {},
  #[serde(rename = "mongodb")]
  Mongodb { url: Box<str> },
  #[serde(rename = "tikv")]
  Tikv {
    endpoints: Box<[Box<str>]>,
    tls_config: Option<TlsConfig>,
    #[serde(default = "def_timeout")]
    timeout_secs: u64,
  },
}

fn def_timeout() -> u64 {
  10
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Zeroize, ZeroizeOnDrop)]
#[serde(tag = "db")]
pub enum CacheConfig {
  #[default]
  #[serde(rename = "moka")]
  Moka,
  #[serde(rename = "redis")]
  Redis { url: Box<str> },
}

impl Default for AuthDbConfig {
  fn default() -> Self {
    Self::Mongodb {
      url: String::new().into_boxed_str(),
    }
  }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Zeroize, ZeroizeOnDrop)]
pub struct TlsConfig {
  pub ca_path: Box<str>,
  pub cert_path: Box<str>,
  pub key_path: Box<str>,
}
