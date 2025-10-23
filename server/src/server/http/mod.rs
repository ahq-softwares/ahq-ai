use actix_web::{HttpResponse, Responder, get, http::header::ContentType};

use crate::server::http::structs::ROOT_RESPONSE_DATA;

pub mod structs;

#[get("/")]
async fn index() -> impl Responder {
  HttpResponse::Ok()
    .content_type(ContentType::json())
    .body::<&[u8]>(ROOT_RESPONSE_DATA.as_ref())
}
