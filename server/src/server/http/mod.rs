use actix_web::{
  HttpResponse, Responder, Result, get, http::header::ContentType, post, web::Bytes,
};

use crate::{
  auth::AGENT,
  server::{AUTH, http::structs::ROOT_RESPONSE_DATA},
};

pub mod structs;

#[get("/")]
async fn index() -> impl Responder {
  HttpResponse::Ok()
    .content_type(ContentType::json())
    .body::<&[u8]>(ROOT_RESPONSE_DATA.as_ref())
}

#[post("/me")]
async fn me(payload: Bytes) -> Result<impl Responder> {
  let session = str::from_utf8(&payload);

  match session {
    Ok(session) => {
      #[allow(clippy::expect_used)]
      let auth_ref = AUTH
        .get()
        .expect("Auth must be defined or else this function cant be registered");

      if auth_ref.verify_session(session).await {
        Ok(HttpResponse::Ok().body::<&[u8]>(br#"{ "msg": "Ok" }"#))
      } else {
        Ok(HttpResponse::Unauthorized().body::<&[u8]>(br#"{ "msg": "Unauthorized" }"#))
      }
    }
    _ => Ok(HttpResponse::BadRequest().body::<&[u8]>(br#"{ "msg": "Bad Request" }"#)),
  }
}

#[post("/challenge")]
async fn challenge(payload: Bytes) -> Result<impl Responder> {
  (AGENT.gen_signature(&payload).await).map_or_else(
    || Ok(HttpResponse::InternalServerError().body::<&[u8]>(br#"{ "msg": "Unable to hash" }"#)),
    |x| Ok(HttpResponse::Ok().body(x.to_vec())),
  )
}
