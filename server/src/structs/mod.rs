use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string_pretty};
use tokio::fs;

use crate::structs::{db::DatabaseConfig, error::Returns};

pub mod db;
pub mod error;

const VERSION: u16 = 1;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
  pub version: u16,
  #[serde(default = "def_bind")]
  pub binds: Vec<(String, u16)>,
  pub admin_pass_hash: Option<String>,
  pub llama: LlamaConfiguration,
  pub authentication: Authentication,
  pub database: DatabaseConfig
}

fn def_bind() -> Vec<(String, u16)> {
  vec![
    ("0.0.0.0".to_string(), 3000),
    ("localhost".to_string(), 3000),
  ]
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LlamaConfiguration {
  pub models: HashMap<Box<str>, LlamaServer>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LlamaServer {
  pub name: Box<str>,
  pub url: Box<str>,
  pub capabilities: Capabilities,
  pub apikey: Option<Box<str>>,
}

pub enum ModelFlag {
  Image,
  Audio,
  Files,
}

impl ModelFlag {
  pub fn into_int(self) -> u16 {
    match self {
      Self::Image => 1,
      Self::Audio => 2,
      Self::Files => 4,
    }
  }
}

#[repr(transparent)]
#[derive(Debug, Clone, Default)]
pub struct Capabilities(pub u16);

impl Serialize for Capabilities {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    self.0.serialize(serializer)
  }
}

impl<'de> Deserialize<'de> for Capabilities {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>,
  {
    Ok(Self(u16::deserialize(deserializer)?))
  }
}

impl Capabilities {
  pub fn add(&mut self, flag: ModelFlag) {
    self.0 |= flag.into_int();
  }

  // pub fn remove(&mut self, flag: ModelFlag) {
  //   self.0 &= !flag.into_int();
  // }

  pub fn has(&mut self, flag: ModelFlag) -> bool {
    (self.0 & flag.into_int()) > 0
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "kind")]
pub enum Authentication {
  OpenToAll,
  Account {
    registration_allowed: bool,
    max_memory: u32,
    time_cost: u32,
  },
}

impl Config {
  pub async fn new() -> Returns<Self> {
    let val = fs::read_to_string("./config.json").await?;

    let out = from_str::<Self>(&val)?;
    
    if out.version != VERSION {
      panic!(
        "❌ Database Config version mismatch:
         Expected version {VERSION}, found {}.
         Please migrate your configuration file to match the current schema.",
        out.version
      );
    }

    Ok(out)
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
      version: VERSION,
      database: DatabaseConfig::default(),
      binds: def_bind(),
      admin_pass_hash: None,
      llama: LlamaConfiguration::default(),
      authentication: Authentication::Account {
        registration_allowed: true,
        max_memory: 64,
        time_cost: 5,
      },
    }
  }
}
