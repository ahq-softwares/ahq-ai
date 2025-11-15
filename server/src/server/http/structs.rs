use std::{collections::HashMap, sync::LazyLock};

use serde::Serialize;

use crate::{server::CONFIG, structs::Authentication};

#[derive(Serialize)]
pub enum ShowedAuth {
  OpenToAll,
  Account,
}

pub static ROOT_RESPONSE_DATA: LazyLock<Vec<u8>> = LazyLock::new(|| {
  let root_response = RootResponse::new();

  serde_json::to_vec(&root_response).expect("Failed to serialize static RootResponse")
});

#[derive(Serialize)]
pub struct RootResponse {
  version: &'static str,
  auth: ShowedAuth,
  can_register: bool,
  models: HashMap<Box<str>, u16>,
}

impl RootResponse {
  pub fn new() -> Self {
    let mut out = Self {
      version: env!("CARGO_PKG_VERSION"),
      auth: ShowedAuth::OpenToAll,
      can_register: false,
      models: HashMap::new(),
    };

    match CONFIG.authentication {
      Authentication::Account {
        registration_allowed,
        ..
      } => {
        out.can_register = registration_allowed;
        out.auth = ShowedAuth::Account;
      }
      Authentication::OpenToAll => {
        out.auth = ShowedAuth::OpenToAll;
      }
    }

    CONFIG.llama.models.iter().for_each(|(key, value)| {
      _ = out.models.insert(key.to_owned(), value.capabilities.0);
    });

    out
  }
}
