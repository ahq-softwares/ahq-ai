use actix_web::http::StatusCode;
use base64::DecodeError;
use thiserror::Error;

use bcrypt::BcryptError;
use serde_json::Error as SerdeError;
use std::io::Error as StdError;
use tokio::task::JoinError;

#[derive(Debug, Error)]
pub enum ServerError {
  #[error(transparent)]
  Serde(#[from] SerdeError),
  #[error(transparent)]
  Base64(#[from] DecodeError),
  #[error(transparent)]
  TokioJoinError(#[from] JoinError),
  #[error(transparent)]
  Std(#[from] StdError),
  #[error("Failed to convert OS String to String")]
  StringConvertErr,
  #[error(transparent)]
  BcryptErr(#[from] BcryptError),
}

impl actix_web::error::ResponseError for ServerError {
  fn status_code(&self) -> actix_web::http::StatusCode {
    StatusCode::INTERNAL_SERVER_ERROR
  }
}

pub type Returns<T> = Result<T, ServerError>;
