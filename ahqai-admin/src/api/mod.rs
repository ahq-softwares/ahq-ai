use std::sync::LazyLock;

use reqwest::{
  StatusCode,
  blocking::{Body, Client},
};
use serde::{Deserialize, Serialize};
use serde_json::to_string;

static CLIENT: LazyLock<Client> = LazyLock::new(|| {
  Client::builder()
    .user_agent("AHQ AI Admin")
    .build()
    .unwrap()
});

#[derive(Debug, Serialize, Deserialize)]
struct VerifyRequest<'a> {
  #[serde(borrow)]
  password: &'a str,
}

#[derive(Deserialize, Serialize, Debug)]
struct AdminUserCreateRequest<'a> {
  #[serde(borrow)]
  password: &'a str,
  #[serde(borrow)]
  unique_id: &'a str,
  #[serde(borrow)]
  user_password: &'a str,
}

#[derive(Deserialize, Serialize, Debug)]
struct Output {
  msg: String,
}

pub fn verify(server: &str, passwd: &str) -> Option<()> {
  CLIENT
    .post(format!("{server}/admin/verify"))
    .body(to_string(&VerifyRequest { password: passwd }).unwrap())
    .send()
    .ok()?
    .error_for_status()
    .ok()?;

  Some(())
}

pub fn create_user(
  server: &str,
  passwd: &str,
  unique_id: &str,
  user_pass: &str,
) -> Result<(), String> {
  let resp = CLIENT
    .post(format!("{server}/admin/user"))
    .body(
      to_string(&AdminUserCreateRequest {
        password: passwd,
        unique_id,
        user_password: user_pass,
      })
      .unwrap(),
    )
    .send()
    .map_err(|_| String::from("Unable to send request"))?;

  if resp.status() == StatusCode::NO_CONTENT {
    return Ok(());
  }

  let Output { msg } = resp
    .json::<Output>()
    .map_err(|_| String::from("Unknown error"))?;

  Err(msg)
}
