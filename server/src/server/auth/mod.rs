use actix_web::{HttpResponse, Responder, Result, post, web::Bytes};
use serde::Deserialize;

use crate::server::TOKEN;

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

  if *TOKEN {
  } else {
  }

  Ok(HttpResponse::Ok().body("Nice"))
}
