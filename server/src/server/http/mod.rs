use actix_web::{Responder, Result, get, web};

use crate::{server::http::structs::RootResponse};

pub mod structs;

#[get("/")]
async fn index() -> Result<impl Responder> {
  let resp = RootResponse::new();

  Ok(web::Json(resp))
}