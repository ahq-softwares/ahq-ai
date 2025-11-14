use crate::server::{
  AUTH, CONFIG, HISTORY_LENGTH,
  chat::ollama::{Message, OllamaMsgResp, OllamaRequest},
};
use actix_web::{HttpRequest, HttpResponse, Result, rt, web::Payload};
use actix_ws::{AggregatedMessage, Session};

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

  if let Some(auth) = AUTH.get()
    && !auth.verify_session(session).await
  {
    return Ok(HttpResponse::Unauthorized().body("{\"msg\": \"Invalid SessionToken\"}"));
  }

  // Checks if the Model is capable of handling images
  let img_capable;

  if CONFIG.ollama.cvmodels.contains(model) {
    img_capable = true;
  } else if CONFIG.ollama.txtmodels.contains(model) {
    img_capable = false;
  } else {
    return Ok(HttpResponse::NotFound().body("{\"msg\": \"Model not found!\"}"));
  }

  let model = model.to_owned();

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

    // Max HISTORY_LENGTH messages
    let mut history = Vec::with_capacity(*HISTORY_LENGTH);

    while let Some(msg) = stream.recv().await {
      match msg {
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
) -> String {
  match handle_msg_faillable(model, history, img_capable, init, msg, session).await {
    Some(model) => model,
    _ => {
      _ = session.text(r#"{ "msg": "Internal Server Error" }"#).await;

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
) -> Option<String> {
  match msg {
    OllamaRequest::Init { history: hist } => {
      if *init {
        _ = session.text(r#"{ "msg": "Already initialized" }"#).await;

        return Some(model);
      }

      if hist.len() > *HISTORY_LENGTH {
        _ = session
            .text(r#"{ "msg": "Max History length reached" }"#)
            .await;

        return Some(model);
      }

      *init = true;
      history.extend(hist.into_iter().map(|x| match x {
        Message::User { message, images } => {
          let mut msg = ChatMessage::new(MessageRole::User, message);

          if let Some(images) = images {
            msg = msg.with_images(
              images
                .into_iter()
                .map(Image::from_base64)
                .collect::<Vec<_>>(),
            )
          }

          msg
        }
        Message::System { prompt } => ChatMessage::new(MessageRole::System, prompt),
        Message::Assistant { message, thinking } => {
          let mut msg = ChatMessage::new(MessageRole::Assistant, message);

          msg.thinking = thinking;

          msg
        }
      }));

      Some(model)
    }
    OllamaRequest::ChatCompletion { prompt, images } => {
      if !*init {
        _ = session
            .text(r#"{ "msg": "Initialization Required" }"#)
            .await;

        return Some(model);
      }

      if history.len() > *HISTORY_LENGTH {
        _ = session
            .text(r#"{ "msg": "Maximum message length reached!" }"#)
            .await;
          
        return None;
      }

      let mut message = ChatMessage::user(prompt);

      if let Some(images) = images {
        if !img_capable {
          _ = session
              .text(r#"{ "msg": "The model is not image capable" }"#)
              .await;
          return None;
        }

        message = message.with_images(
          images
            .into_iter()
            .map(Image::from_base64)
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

      _ = session.text(serde_json::to_string(&out).ok()?).await;

      Some(resp.model)
    }
  }
}
