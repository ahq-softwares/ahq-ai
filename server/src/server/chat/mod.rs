use std::sync::Arc;

use actix_web::{HttpRequest, HttpResponse, Result, rt, web::Payload};
use actix_ws::AggregatedMessage;
use ollama_rs::{generation::{chat::{ChatMessage, request::ChatMessageRequest}, completion::request::GenerationRequest}, models::create::CreateModelRequest};

use crate::server::{AUTH, CONFIG, OLLAMA};

pub async fn chat(req: HttpRequest, stream: Payload) -> Result<HttpResponse> {
  let headers = req.headers();

  let (Some(session), Some(model)) = (headers.get("Authorization"), headers.get("model")) else {
    return Ok(
      HttpResponse::Unauthorized()
        .body("{\"msg\": \"Headers `Authorization`, `model` are necessary\"}"),
    );
  };

  let Ok(model) = model.to_str() else {
    return Ok(HttpResponse::Unauthorized().body("{\"msg\": \"Invalid `model` header\"}"));
  };

  let Ok(session) = session.to_str() else {
    return Ok(HttpResponse::Unauthorized().body("{\"msg\": \"Invalid `session` header\"}"));
  };

  if let Some(auth) = AUTH.get() {
    if !auth.verify_session(session).await {
      return Ok(HttpResponse::Unauthorized().body("{\"msg\": \"Invalid SessionToken\"}"));
    }
  }

  // Checks if the Model is capable of handling images
  let img_capable;

  if CONFIG.ollama.cvmodels.contains(&model as &str) {
    img_capable = true;
  } else if CONFIG.ollama.txtmodels.contains(&model as &str) {
    img_capable = false;
  } else {
    return Ok(HttpResponse::NotFound().body("{\"msg\": \"Model not found!\"}"));
  }

  let model = model.to_owned();

  println!("Capable of handling images: {img_capable}");

  let (res, mut session, stream) = actix_ws::handle(&req, stream)?;

  let mut stream = stream
    .aggregate_continuations()
    // 20MB data size man
    .max_continuation_size(20 * 1024 * 1024);

  // Launch a new async task
  rt::spawn(async move {
    while let Some(msg) = stream.recv().await {
      match msg {
        Ok(AggregatedMessage::Text(x)) => {
          let out = OLLAMA.generate(GenerationRequest::new(model.clone(), x.to_string())).await.unwrap();

          session.text(out.response).await.unwrap();
        }
        Ok(AggregatedMessage::Binary(x)) => {
          println!("Found byes {x:?}");
          _ = session.text("Hi").await;
        }
        Ok(AggregatedMessage::Close(_)) => break,
        _ => {}
      }
    }
    _ = session.close(None).await;
  });

  Ok(res)
}
