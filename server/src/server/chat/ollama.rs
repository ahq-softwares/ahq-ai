use serde::{Deserialize, Serialize};

pub type History = Vec<Message>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "event")]
pub enum Message {
  User { message: String, images: Option<Vec<String>> },
  Assistant { message: String }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "event")]
pub enum OllamaRequest {
  Init {
    history: History,
  },
  ChatCompletion {
    prompt: String,
    images: Option<Vec<String>>,
  },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OllamaMsgResp {
  pub content: String,
  pub thinking: Option<String>,
}
