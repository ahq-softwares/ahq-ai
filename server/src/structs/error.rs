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
  TokioJoinError(#[from] JoinError),
  #[error(transparent)]
  Std(#[from] StdError),
  #[error("Failed to convert OS String to String")]
  StringConvertErr,
  #[error(transparent)]
  BcryptErr(#[from] BcryptError),
}

pub type Returns<T> = Result<T, ServerError>;
