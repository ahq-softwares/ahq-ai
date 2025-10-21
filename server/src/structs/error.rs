use thiserror::Error;

use serde_json::Error as SerdeError;
use std::io::Error as StdError;

#[derive(Debug, Error)]
pub enum ServerError {
  #[error(transparent)]
  Serde(#[from] SerdeError),
  #[error(transparent)]
  Std(#[from] StdError),
}

pub type Returns<T> = Result<T, ServerError>;
