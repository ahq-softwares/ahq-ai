use actix_web::{HttpResponse, Responder, Result, post, web::Bytes};
use serde::Deserialize;

use crate::server::{AUTH, TOKEN};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Auth<'a> {
  #[serde(borrow)]
  username: Option<&'a str>,
  #[serde(borrow)]
  pass: &'a str,
}

#[post("/login")]
pub async fn auth(payload: Bytes) -> Result<impl Responder> {
  let auth: Auth = serde_json::from_slice(&payload)?;

  let auth_ref = AUTH
    .get()
    .expect("Auth must be defined or else this function cant be registered");
  // If invalid close all
  // This is a cancel thread
  let resp = match *TOKEN {
    true => auth_ref.is_valid_token(&auth.pass).await?,
    false => {
      auth_ref
        .is_valid_account(&auth.username.unwrap_or_default(), &auth.pass)
        .await?
    }
  };

  let Some(resp) = resp else {
    return Ok(HttpResponse::Unauthorized().body("{\"msg\": \"Invalid credentials\"}"));
  };

  Ok(HttpResponse::Ok().body(resp))
}
