use actix_web::{HttpResponse, Responder, Result, post, web::Bytes};
use serde::Deserialize;

use crate::{
  auth::{AccountCheckOutcome, AccountCreateOutcome},
  server::{AUTH, TOKEN},
};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Auth<'a> {
  #[serde(borrow)]
  username: Option<&'a str>,
  #[serde(borrow)]
  pass: &'a str,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct AuthRegn<'a> {
  #[serde(borrow)]
  username: &'a str,
  #[serde(borrow)]
  pass: &'a str,
}

#[post("/login")]
pub async fn auth(payload: Bytes) -> Result<impl Responder> {
  let Ok(auth) = serde_json::from_slice::<Auth>(&payload) else {
    return Ok(HttpResponse::BadRequest().body(r#"{ "msg": "Invalid Data" }"#));
  };

  let auth_ref = AUTH
    .get()
    .expect("Auth must be defined or else this function cant be registered");
  // If invalid close all
  // This is a cancel thread
  let resp = match *TOKEN {
    true => auth_ref.is_valid_token(auth.pass).await?,
    false => {
      auth_ref
        .is_valid_account(auth.username.unwrap_or_default(), auth.pass)
        .await?
    }
  };

  match resp {
    AccountCheckOutcome::Some(x) => Ok(HttpResponse::Ok().body(x)),
    AccountCheckOutcome::InvalidPassword | AccountCheckOutcome::NotFound => {
      Ok(HttpResponse::Unauthorized().body("{\"msg\": \"Invalid Credentials\"}"))
    }
    AccountCheckOutcome::TooManyRequests => {
      Ok(HttpResponse::TooManyRequests().body("{\"msg\": \"Too Many Requests\"}"))
    }
  }
}

#[post("/register")]
pub async fn register(payload: Bytes) -> Result<impl Responder> {
  let Ok(regn) = serde_json::from_slice::<AuthRegn>(&payload) else {
    return Ok(HttpResponse::BadRequest().body(r#"{ "msg": "Invalid Data" }"#));
  };

  let auth_ref = AUTH
    .get()
    .expect("Auth must be defined or else this function cant be registered");

  if !auth_ref.can_register().await {
    return Ok(
      HttpResponse::UnprocessableEntity()
        .body(r#"{ "msg": "Registration is disabled due to maximum user saturation" }"#),
    );
  }

  match auth_ref.register(regn.username, regn.pass).await? {
    AccountCreateOutcome::InternalServerError => {
      Ok(HttpResponse::InternalServerError().body(r#"{ "msg": "Internal Server Error" }"#))
    }
    AccountCreateOutcome::Successful => Ok(HttpResponse::NoContent().body(vec![])),
    AccountCreateOutcome::UsernameExists => {
      Ok(HttpResponse::Conflict().body(r#"{ "msg": "User already exists" }"#))
    }
    AccountCreateOutcome::WeakPassword => {
      Ok(HttpResponse::BadRequest().body(r#"{ "msg": "Insecure Password" }"#))
    }
  }
}
