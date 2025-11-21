use actix_web::{
  HttpResponse, HttpResponseBuilder, Responder, Result, delete, http::StatusCode, post, web::Bytes,
};
use secrecy::ExposeSecret;

use serde::Deserialize;
use serde_json::from_slice;
use tokio::task::yield_now;

use async_stream::stream;
use futures::Stream;

use crate::{
  auth::{AccountCreateOutcome, AuthSessionManager},
  server::{AUTH, CONFIG, REAL_ADMIN_PASSWORD},
  structs::Authentication,
};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct AdminAuthRequest<'a> {
  #[serde(borrow)]
  password: &'a str,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct AdminSearchRequest<'a> {
  #[serde(borrow)]
  password: &'a str,
  search: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct AdminUserCreateRequest<'a> {
  #[serde(borrow)]
  password: &'a str,
  #[serde(borrow)]
  unique_id: &'a str,
  #[serde(borrow)]
  user_password: &'a str,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct AdminDeleteRequest<'a> {
  #[serde(borrow)]
  password: &'a str,
  unique_id: String,
}

async fn verify_auth(passwd: &str) -> Result<(), HttpResponse> {
  let value = REAL_ADMIN_PASSWORD
    .get()
    .map(|x| async move { passwd == x.read().await.expose_secret() });

  let val = if let Some(v) = value { v.await } else { false };

  if val {
    Ok(())
  } else {
    Err(HttpResponse::Unauthorized().body(r#"{ "msg": "Unauthorized" }"#))
  }
}

#[post("/admin/verify")]
async fn verify(body: Bytes) -> Result<impl Responder> {
  let auth: AdminAuthRequest = from_slice(&body)?;

  if let Err(r) = verify_auth(auth.password).await {
    return Ok(r);
  }

  Ok(HttpResponse::NoContent().body::<&[u8]>(&[]))
}

#[post("/admin/clients")]
// This method is not at all recommended to be called for a large server
// This lists all the client IDS (let it be accounts or tokens)
async fn list(body: Bytes) -> Result<impl Responder> {
  let data: AdminSearchRequest = from_slice(&body)?;

  if let Err(r) = verify_auth(data.password).await {
    return Ok(r);
  }

  if let Some(auth) = AUTH.get() {
    return Ok(
      HttpResponseBuilder::new(StatusCode::OK).streaming(user_list_stream(auth, data.search)),
    );
  }

  Ok(HttpResponse::ServiceUnavailable().body::<&[u8]>(br#"{ "msg": "Auth is disabled" }"#))
}

fn user_list_stream(
  auth: &'static AuthSessionManager,
  prefix: String,
) -> impl Stream<Item = Result<Bytes>> {
  stream! {
    for (index, uid) in auth.accounts.search(prefix).await?.into_iter().enumerate() {
      if index != 0 {
        yield Ok(Bytes::from_static(b"\n"));
      }

      yield Ok(Bytes::from_owner(uid));

      if index.is_multiple_of(30) {
        yield_now().await;
      }
    }
  }
}

#[post("/admin/user")]
async fn create(body: Bytes) -> Result<impl Responder> {
  let data: AdminUserCreateRequest = from_slice(&body)?;

  if let Err(r) = verify_auth(data.password).await {
    return Ok(r);
  }

  let Authentication::Account { .. } = CONFIG.authentication else {
    return Ok(
      HttpResponse::ServiceUnavailable()
        .body::<&[u8]>(br#"{ "msg": "Auth is not account based" }"#),
    );
  };

  if let Some(auth) = AUTH.get() {
    return match auth.register(data.unique_id, data.user_password).await? {
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
      AccountCreateOutcome::SuccessfulOut(_) => {
        Ok(HttpResponse::UnprocessableEntity().body(r#"{ "msg": "Unreachable Output" }"#))
      }
    };
  }

  Ok(HttpResponse::ServiceUnavailable().body::<&[u8]>(br#"{ "msg": "Auth is disabled" }"#))
}

#[post("/admin/token")]
async fn create_token(body: Bytes) -> Result<impl Responder> {
  let auth: AdminAuthRequest = from_slice(&body)?;

  if let Err(r) = verify_auth(auth.password).await {
    return Ok(r);
  }

  let Authentication::Account { .. } = CONFIG.authentication else {
    return Ok(
      HttpResponse::ServiceUnavailable()
        .body::<&[u8]>(br#"{ "msg": "Auth is not account based" }"#),
    );
  };

  if let Some(auth) = AUTH.get() {
    return match auth.add_token().await? {
      AccountCreateOutcome::InternalServerError => {
        Ok(HttpResponse::InternalServerError().body(r#"{ "msg": "Internal Server Error" }"#))
      }
      AccountCreateOutcome::SuccessfulOut(out) => {
        Ok(HttpResponse::Ok().body(Bytes::from_owner(out)))
      }
      AccountCreateOutcome::UsernameExists => {
        Ok(HttpResponse::Conflict().body(r#"{ "msg": "User already exists" }"#))
      }
      AccountCreateOutcome::WeakPassword => {
        Ok(HttpResponse::BadRequest().body(r#"{ "msg": "Insecure Password" }"#))
      }
      AccountCreateOutcome::Successful => {
        Ok(HttpResponse::UnprocessableEntity().body(r#"{ "msg": "Unreachable Output" }"#))
      }
    };
  }

  Ok(HttpResponse::ServiceUnavailable().body::<&[u8]>(br#"{ "msg": "Auth is disabled" }"#))
}

#[delete("/admin/client")]
async fn delete(body: Bytes) -> Result<impl Responder> {
  let data: AdminDeleteRequest = from_slice(&body)?;

  if let Err(r) = verify_auth(data.password).await {
    return Ok(r);
  }

  let Authentication::Account { .. } = CONFIG.authentication else {
    return Ok(
      HttpResponse::ServiceUnavailable()
        .body::<&[u8]>(br#"{ "msg": "Auth is not account based" }"#),
    );
  };

  if let Some(auth) = AUTH.get() {
    _ = auth.accounts.remove(data.unique_id).await;

    return Ok(HttpResponse::NoContent().body::<&[u8]>(&[]));
  }

  Ok(HttpResponse::ServiceUnavailable().body::<&[u8]>(br#"{ "msg": "Auth is disabled" }"#))
}
