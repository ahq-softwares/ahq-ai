use std::sync::LazyLock;

use reqwest::{StatusCode, blocking::Client};
use semver::{Version, VersionReq};
use serde::{Deserialize, Serialize};
use serde_json::to_string;

pub mod copy;

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

#[derive(Serialize)]
struct AdminDeleteRequest<'a> {
  #[serde(borrow)]
  password: &'a str,
  #[serde(borrow)]
  unique_id: &'a str,
}

static SCOPED_VER: LazyLock<VersionReq> =
  LazyLock::new(|| VersionReq::parse(">=0.3.2, <0.4.0").unwrap());

#[derive(Debug, Deserialize)]
struct ServerRoot {
  version: String,
}

pub fn root(server: &str) -> Result<(), &'static str> {
  let ver = CLIENT
    .get(server)
    .send()
    .map_err(|_| "Unable to send request")?
    .error_for_status()
    .map_err(|_| "Invalid response from server")?
    .json::<ServerRoot>()
    .map_err(|_| "Server returned invalid json output")?
    .version;

  let ver = Version::parse(&ver).map_err(|_| "Server returned invalid version")?;

  if !SCOPED_VER.matches(&ver) {
    return Err(
      "This administrator portal does not support this version of AHQ AI server executable.",
    );
  }

  Ok(())
}

pub fn verify(server: &str, passwd: &str) -> Result<(), &'static str> {
  CLIENT
    .post(format!("{server}/admin/verify"))
    .body(to_string(&VerifyRequest { password: passwd }).unwrap())
    .send()
    .map_err(|_| "Unable to send request")?
    .error_for_status()
    .map_err(|_| "Invalid credentials")?;

  Ok(())
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

pub fn remove_client(server: &str, passwd: &str, unique_id: &str) -> Result<(), String> {
  let resp = CLIENT
    .delete(format!("{server}/admin/client"))
    .body(
      to_string(&AdminDeleteRequest {
        password: passwd,
        unique_id,
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

pub fn create_token(server: &str, passwd: &str) -> Result<String, String> {
  let resp = CLIENT
    .post(format!("{server}/admin/token"))
    .body(to_string(&VerifyRequest { password: passwd }).unwrap())
    .send()
    .map_err(|_| String::from("Unable to send request"))?;

  if resp.status() == StatusCode::OK {
    let resp = resp
      .text()
      .map_err(|_| String::from("Unable to parse token"))?;
    return Ok(resp);
  }

  let Output { msg } = resp
    .json::<Output>()
    .map_err(|_| String::from("Unknown error"))?;

  Err(msg)
}
