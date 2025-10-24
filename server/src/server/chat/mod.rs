use crate::server::{
  AUTH, CONFIG, OLLAMA,
  chat::ollama::{Message, OllamaMsgResp, OllamaRequest},
};
use actix_web::{HttpRequest, HttpResponse, Result, rt, web::Payload};
use actix_ws::{AggregatedMessage, Session};
use ollama_rs::generation::{
  chat::{ChatMessage, MessageRole, request::ChatMessageRequest},
  images::Image,
};

pub mod ollama;

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
    // 8 MB data size max
    .max_continuation_size(8 * 1024 * 1024);

  // Launch a new async task
  rt::spawn(async move {
    let mut model = model;
    let img_capable = img_capable;
    let mut init = false;

    // Max 40 messages
    let mut history = Vec::with_capacity(40);

    while let Some(msg) = stream.recv().await {
      match msg {
        #[cfg(debug_assertions)]
        Ok(AggregatedMessage::Text(x)) => {
          let Ok::<OllamaRequest, _>(x) = serde_json::from_reader(&*x.into_bytes()) else {
            break;
          };

          model = handle_msg(
            model,
            &mut history,
            img_capable,
            &mut init,
            x,
            &mut session,
            true,
          )
          .await;
        }
        #[cfg(not(debug_assertions))]
        Ok(AggregatedMessage::Text(_)) => break,
        Ok(AggregatedMessage::Binary(x)) => {
          let Ok::<OllamaRequest, _>(x) = ciborium::from_reader(&*x) else {
            break;
          };

          model = handle_msg(
            model,
            &mut history,
            img_capable,
            &mut init,
            x,
            &mut session,
            false,
          )
          .await;
        }
        Ok(AggregatedMessage::Close(_)) => break,
        Ok(AggregatedMessage::Ping(_)) => break,
        Ok(AggregatedMessage::Pong(_)) => break,
        _ => break,
      }

      if model.is_empty() {
        break;
      }
    }
    _ = session.close(None).await;
  });

  Ok(res)
}

async fn handle_msg(
  model: String,
  history: &mut Vec<ChatMessage>,
  img_capable: bool,
  init: &mut bool,
  msg: OllamaRequest,
  session: &mut Session,
  using_json: bool,
) -> String {
  match handle_msg_faillable(model, history, img_capable, init, msg, session, using_json).await {
    Some(model) => model,
    _ => {
      if using_json {
        _ = session.text(r#"{ "msg": "Internal Server Error" }"#).await;
      } else {
        _ = session.text(r#"{ "msg": "Internal Server Error" }"#).await;
      }

      String::with_capacity(0)
    }
  }
}

async fn handle_msg_faillable(
  model: String,
  history: &mut Vec<ChatMessage>,
  img_capable: bool,
  init: &mut bool,
  msg: OllamaRequest,
  session: &mut Session,
  using_json: bool,
) -> Option<String> {
  match msg {
    OllamaRequest::Init { history: hist } => {
      if *init {
        if using_json {
          _ = session.text(r#"{ "msg": "Already initialized" }"#).await;
        } else {
          _ = session.text(r#"{ "msg": "Already initialized" }"#).await;
        }

        return Some(model);
      }

      if hist.len() > 40 {
        if using_json {
          _ = session
            .text(r#"{ "msg": "History cannot be more than 40 messages" }"#)
            .await;
        } else {
          _ = session
            .text(r#"{ "msg": "History cannot be more than 40 messages" }"#)
            .await;
        }

        return Some(model);
      }

      *init = true;
      history.extend(
        hist
          .into_iter()
          .map(|x| {
            let (role, content) = match x {
              Message::User { message } => (MessageRole::User, message),
              Message::Assistant { message } => (MessageRole::Assistant, message)
            };

            ChatMessage::new(role, content)
          }),
      );

      return Some(model);
    }
    OllamaRequest::ChatCompletion { prompt, images } => {
      if history.len() > 40 {
        if using_json {
          _ = session
            .text(r#"{ "msg": "Maximum message length reached!" }"#)
            .await;
        } else {
          _ = session
            .text(r#"{ "msg": "Maximum message length reached!" }"#)
            .await;
        }
        return None;
      }

      let mut message = ChatMessage::user(prompt);

      if let Some(images) = images {
        if !img_capable {
          if using_json {
            _ = session
              .text(r#"{ "msg": "The model is not image capable" }"#)
              .await;
          } else {
            _ = session
              .text(r#"{ "msg": "The model is not image capable" }"#)
              .await;
          }
          return None;
        }

        message = message.with_images(
          images
            .into_iter()
            .map(|x| Image::from_base64(x))
            .collect::<Vec<_>>(),
        );
      }

      let resp = OLLAMA
        .send_chat_messages_with_history(history, ChatMessageRequest::new(model, vec![message]))
        .await
        .ok()?;

      let out = OllamaMsgResp {
        content: resp.message.content,
        thinking: resp.message.thinking,
      };

      if using_json {
        _ = session.text(serde_json::to_string(&out).ok()?).await;
      } else {
        let mut bytes = vec![];

        ciborium::into_writer(&out, &mut bytes).ok()?;

        _ = session.binary(bytes).await;
      }

      Some(resp.model)
    }
  }
}
