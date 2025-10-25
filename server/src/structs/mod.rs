use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string_pretty};
use tokio::fs;

use crate::structs::error::Returns;

pub mod error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
  #[serde(default = "def_bind")]
  pub binds: Vec<(String, u16)>,
  pub admin_pass_hash: Option<String>,
  pub ollama: OllamaConfiguration,
  pub authentication: Authentication,
}

pub static BCRYPT_COST: u32 = 14;

fn def_bind() -> Vec<(String, u16)> {
  vec![
    ("0.0.0.0".to_string(), 3000),
    ("localhost".to_string(), 3000),
  ]
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OllamaConfiguration {
  pub host: Box<str>,
  pub port: u16,
  pub msgs: usize,
  pub cvmodels: HashSet<Box<str>>,
  pub txtmodels: HashSet<Box<str>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "kind")]
pub enum Authentication {
  OpenToAll,
  TokenBased,
  Account {
    registration_allowed: bool,
    max_users: Option<u64>,
  },
}

impl Config {
  pub async fn new() -> Returns<Self> {
    let val = fs::read_to_string("./config.json").await?;

    Ok(from_str::<Self>(&val)?)
  }

  pub async fn new_or_default() -> Self {
    Self::new().await.unwrap_or_default()
  }

  pub async fn save_config(&self) -> Returns<()> {
    fs::write("./config.json", to_string_pretty(&self)?).await?;

    Ok(())
  }
}

impl Default for Config {
  fn default() -> Self {
    Self {
      binds: def_bind(),
      admin_pass_hash: None,
      ollama: OllamaConfiguration::default(),
      authentication: Authentication::OpenToAll,
    }
  }
}
