use std::fs;

use serde::{Deserialize, Serialize};
use serde_json::from_str;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DatabaseConfig {
  pub authdb: AuthDbConfig,
  pub cache: CacheConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TlsConfig {
  pub ca_path: Box<str>,
  pub cert_path: Box<str>,
  pub key_path: Box<str>,
}

impl DatabaseConfig {
  /// This is a panicking method as it should immediately crash the server
  pub fn get() -> Self {
    let data = fs::read_to_string("./database_conf.json").expect("Unable to get Database Config");

    let out: Self =
      from_str(&data).expect("Unable to parse your JSON Database Config. Make sure it is correct");

    out
  }
}
