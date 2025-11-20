use serde::{Deserialize, Serialize};

pub type History = Vec<Message>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "role")]
pub enum Message {
  #[serde(rename = "user")]
  User {
    content: Vec<MsgStruct>,
    _nonstandard_images: Option<Vec<String>>,
  },
  #[serde(rename = "system")]
  System { content: String },
  #[serde(rename = "tool")]
  Tool {},
  #[serde(rename = "assistant")]
  Assistant {
    content: String,
    thinking: Option<String>,
  },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MsgStruct {
  #[serde(rename = "text")]
  TextOrFile { text: String },
  #[serde(rename = "image_url")]
  Image { image_url: Url },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Url {
  url: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "event")]
pub enum LlamaRequest {
  #[serde(rename = "init")]
  Init { history: History },
  #[serde(rename = "completion")]
  ChatCompletion {
    prompt: String,
    attachments: Option<Vec<Attachment>>,
  },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Attachment {
  #[serde(rename = "audio")]
  Audio { url: String },
  #[serde(rename = "image")]
  Image { url: String },
  #[serde(rename = "file")]
  File { text: String },
  #[serde(rename = "pdf")]
  Pdf { text: String },
}

#[derive(Debug, Serialize)]
pub struct HTTPCompletion<'a> {
  #[serde(borrow)]
  pub model: &'a str,
  #[serde(borrow)]
  pub messages: &'a [Message],
  pub stream: bool
}

#[derive(Debug, Deserialize)]
pub struct HTTPAIResponse {
  pub choices: Vec<HTTPChoices>
}

#[derive(Debug, Deserialize)]
pub struct HTTPChoices {
  pub finish_reason: Box<str>,
  pub index: usize,
  pub message: Message
}