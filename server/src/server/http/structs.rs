use std::sync::LazyLock;

use serde::Serialize;

use crate::{server::CONFIG, structs::Authentication};

#[derive(Serialize)]
pub enum ShowedAuth {
  OpenToAll,
  Account,
}

pub static ROOT_RESPONSE_DATA: LazyLock<Vec<u8>> = LazyLock::new(RootResponse::compile);

#[derive(Serialize)]
pub struct Model<'a> {
  #[serde(borrow)]
  id: &'a str,
  #[serde(borrow)]
  name: &'a str,
  capabilities: u16,
}

#[derive(Serialize)]
pub struct RootResponse<'a> {
  version: &'static str,
  auth: ShowedAuth,
  can_register: bool,
  models: Vec<Model<'a>>,
}

impl RootResponse<'_> {
  pub fn compile() -> Vec<u8> {
    let mut out = Self {
      version: env!("CARGO_PKG_VERSION"),
      auth: ShowedAuth::OpenToAll,
      can_register: false,
      models: vec![],
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
      out.models.push(Model {
        id: key as &str,
        name: &value.name,
        capabilities: value.capabilities.0,
      });
    });

    #[allow(clippy::expect_used)]
    serde_json::to_vec(&out).expect("Failed to serialize static RootResponse")
  }
}
