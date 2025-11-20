use crate::{
  server::{
    AUTH, CONFIG, llama::{LlamaChatHandler, structs::LlamaRequest}
  },
  structs::Capabilities,
};
use log::error;

#[cfg(debug_assertions)]
use log::debug;

use actix_web::{
  HttpRequest, HttpResponse, Result,
  http::{StatusCode, header::ContentType},
  rt,
  web::Payload,
};
use actix_ws::{AggregatedMessage, Session};

const MISSING_HEADERS_BODY: &str = r#"{ "msg": "Headers `Authorization`, `model` are necessary" }"#;
const INVALID_SESSION_BODY: &str = r#"{ "msg": "Invalid SessionToken" }"#;
const MODEL_NOT_FOUND_BODY: &str = r#"{ "msg": "Model not found!" }"#;
const INVALID_MODEL_BODY: &str = r#"{ "msg": "Invalid `model` header" }"#;
const INVALID_SESSION_HEADER_BODY: &str = r#"{ "msg": "Invalid `session` header" }"#;

const INVALID_WS_RESP: &str = r#"{ "msg": "Unexpected WebSocket data" }"#;

fn json_response(status: StatusCode, body: &'static str) -> HttpResponse {
  HttpResponse::build(status)
    .content_type(ContentType::json())
    .body(body)
}

pub async fn chat(req: HttpRequest, stream: Payload) -> Result<HttpResponse> {
  let headers = req.headers();

  let (Some(session), Some(model)) = (headers.get("Authorization"), headers.get("model")) else {
    return Ok(json_response(
      StatusCode::UNAUTHORIZED,
      MISSING_HEADERS_BODY,
    ));
  };

  let Ok(model) = model.to_str() else {
    return Ok(json_response(StatusCode::UNAUTHORIZED, INVALID_MODEL_BODY));
  };

  let Ok(session) = session.to_str() else {
    return Ok(json_response(
      StatusCode::UNAUTHORIZED,
      INVALID_SESSION_HEADER_BODY,
    ));
  };

  if let Some(auth) = AUTH.get()
    && !auth.verify_session(session).await
  {
    return Ok(json_response(
      StatusCode::UNAUTHORIZED,
      INVALID_SESSION_BODY,
    ));
  }

  // Checks if the Model is capable of handling images
  let cap;
  let mut chat;

  if let Some(x) = CONFIG.llama.models.get(model) {
    chat = LlamaChatHandler::new(model)?;

    // Very cheap to clone
    cap = x.capabilities.clone();
  } else {
    return Ok(json_response(StatusCode::NOT_FOUND, MODEL_NOT_FOUND_BODY));
  }

  let (res, mut session, stream) = actix_ws::handle(&req, stream)?;

  let mut stream = stream
    .aggregate_continuations()
    // 30 MB data size max
    .max_continuation_size(30 * 1024 * 1024);

  // Launch a new async task
  rt::spawn(async move {
    let mut init = false;

    #[cfg(debug_assertions)]
    debug!("Starting up websocket connection");

    while let Some(msg) = stream.recv().await {
      match msg {
        Ok(AggregatedMessage::Text(x)) => {
          let Ok::<LlamaRequest, _>(x) = serde_json::from_reader(&*x.into_bytes()) else {
            break;
          };

          let should_close = handle_msg(cap.clone(), &mut init, x, &mut chat, &mut session).await;

          if should_close {
            break;
          }
        }
        Ok(AggregatedMessage::Close(_)) => break,
        Ok(AggregatedMessage::Ping(_)) => continue, // Ping/Pong don't need a break
        Ok(AggregatedMessage::Pong(_)) => continue, // Use 'continue' to keep the loop going
        Ok(_) => {
          _ = session.text(INVALID_WS_RESP).await;
          break;
        } // Catch any unexpected AggregatedMessage (e.g., Binary) and close cleanly
        Err(e) => {
          // Log the stream error here! Use a proper logging crate (e.g., tracing)
          error!("WebSocket stream error: {:?}", e);
          break;
        }
      }
    }

    #[cfg(debug_assertions)]
    debug!("Cleaning up websocket connection");

    _ = session.close(None).await;
  });

  Ok(res)
}

async fn handle_msg(
  cap: Capabilities,
  init: &mut bool,
  msg: LlamaRequest,
  hwnd: &mut LlamaChatHandler,
  session: &mut Session,
) -> bool {
  handle_msg_faillable(cap, init, msg, hwnd, session)
    .await
    .is_none()
}

async fn handle_msg_faillable(
  cap: Capabilities,
  init: &mut bool,
  msg: LlamaRequest,
  hwnd: &mut LlamaChatHandler,
  session: &mut Session,
) -> Option<()> {
  match msg {
    LlamaRequest::Init { history: hist } => {
      if *init {
        _ = session.text(r#"{ "msg": "Already initialized" }"#).await;

        return None;
      }

      *init = true;
      hwnd.msg.extend(hist.into_iter());

      Some(())
    }
    LlamaRequest::ChatCompletion {
      prompt,
      attachments,
    } => {
      if !*init {
        _ = session
          .text(r#"{ "msg": "Initialization Required" }"#)
          .await;
        return None;
      }

      Some(())

      // let mut message = ChatMessage::user(prompt);

      // if let Some(images) = images {
      //   if !img_capable {
      //     _ = session
      //         .text(r#"{ "msg": "The model is not image capable" }"#)
      //         .await;
      //     return None;
      //   }

      //   message = message.with_images(
      //     images
      //       .into_iter()
      //       .map(Image::from_base64)
      //       .collect::<Vec<_>>(),
      //   );
      // }

      // let resp = OLLAMA
      //   .send_chat_messages_with_history(history, ChatMessageRequest::new(model, vec![message]))
      //   .await
      //   .ok()?;

      // let out = OllamaMsgResp {
      //   content: resp.message.content,
      //   thinking: resp.message.thinking,
      // };

      // _ = session.text(serde_json::to_string(&out).ok()?).await;
    }
  }
}
