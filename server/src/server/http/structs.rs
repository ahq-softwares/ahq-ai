use serde::Serialize;

use crate::{server::CONFIG, structs::Authentication};

#[derive(Serialize)]
pub enum ShowedAuth {
  OpenToAll,
  TokenBased,
  Account
}

#[derive(Serialize)]
pub struct RootResponse {
  auth: ShowedAuth,
  can_register: bool,
  vision_models: Vec<&'static str>,
  text_models: Vec<&'static str>
}

impl RootResponse {
  pub fn new() -> Self {
    let mut out = Self {
      auth: ShowedAuth::OpenToAll,
      can_register: false,
      text_models: vec![],
      vision_models: vec![]
    };

    match CONFIG.authentication {
      Authentication::Account { registration_allowed, .. } => {
        out.can_register = registration_allowed;
        out.auth = ShowedAuth::Account;
      }
      Authentication::OpenToAll => {
        out.auth = ShowedAuth::OpenToAll;
      }
      Authentication::TokenBased => {
        out.auth = ShowedAuth::TokenBased;
      }
    }

    out.text_models.reserve(CONFIG.ollama.txtmodels.len());
    out.vision_models.reserve(CONFIG.ollama.cvmodels.len());
    
    CONFIG.ollama.cvmodels.iter().for_each(|x| {
      out.vision_models.push(x as &str);
    });

    CONFIG.ollama.txtmodels.iter().for_each(|x| {
      out.text_models.push(x as &str);
    });
    
    out
  }
}
