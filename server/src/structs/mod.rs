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

fn def_bind() -> Vec<(String, u16)> {
  vec![("0.0.0.0".to_string(), 3000)]
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OllamaConfiguration {
  pub host: String,
  pub port: u16,
  pub cvmodels: Vec<String>,
  pub txtmodels: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum Authentication {
  OpenToAll,
  TokenBased,
  AccountAuthentication {
    registration_allowed: bool,
    max_users: Option<u16>,
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
