use serde::{Deserialize, Serialize};

pub type History = Vec<Message>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "role")]
pub enum Message {
  #[serde(rename = "user")]
  User { content: Vec<MsgStruct> },
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
#[serde(tag = "type")]
pub enum MsgStruct {
  #[serde(rename = "text")]
  TextOrFile { text: String },
  #[serde(rename = "image_url")]
  Image { image_url: Url },
  /// TODO: Verify
  #[serde(rename = "audio_url")]
  Audio { audio_url: Url },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Url {
  url: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "event")]
pub enum LlamaRequest {
  #[serde(rename = "feed")]
  Feed { history: History },
  #[serde(rename = "init")]
  Init {},
  #[serde(rename = "completion")]
  ChatCompletion { msg: Vec<MsgStruct> },
}

// #[derive(Debug, Serialize, Deserialize)]
// pub enum Attachment {
//   #[serde(rename = "audio")]
//   Audio { url: String },
//   #[serde(rename = "image")]
//   Image { url: String },
//   #[serde(rename = "file")]
//   File { text: String },
//   #[serde(rename = "pdf")]
//   Pdf { text: String },
// }

#[derive(Debug, Serialize)]
pub struct HTTPCompletion<'a> {
  #[serde(borrow)]
  pub model: &'a str,
  #[serde(borrow)]
  pub messages: &'a [Message],
  pub stream: bool,
}

#[derive(Debug, Deserialize)]
pub struct HTTPAIResponse {
  pub choices: Vec<HTTPChoices>,
}

#[derive(Debug, Deserialize)]
pub struct HTTPChoices {
  pub finish_reason: Box<str>,
  #[serde(rename = "index")]
  pub _index: usize,
  pub message: Message,
}

// #[derive(Debug, Deserialize)]
// pub struct HTTPLLMProps {
//   default_generation_settings: HTTPLLMSettings
// }

// #[derive(Debug, Deserialize)]
// pub struct HTTPLLMSettings {
//   pub n_ctx: usize
// }