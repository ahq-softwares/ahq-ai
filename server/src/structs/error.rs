use actix_web::http::StatusCode;
use base64::DecodeError;
use mongodb::error::Error as MongoDBError;
use rand::rand_core::OsError;
use redis::RedisError;
use reqwest::header::InvalidHeaderValue;
use thiserror::Error;

use reqwest::Error as ReqwestError;

use argon2::password_hash::Error as ArgonErr;
use serde_json::Error as SerdeError;
use std::io::Error as StdError;
use tikv_client::Error as TikvError;
use tokio::task::JoinError;

#[derive(Debug, Error)]
pub enum ServerError {
  #[error(transparent)]
  Serde(#[from] SerdeError),
  #[error(transparent)]
  Base64(#[from] DecodeError),
  #[error(transparent)]
  ReqwestError(#[from] ReqwestError),
  #[error(transparent)]
  InvalidHeaderValue(#[from] InvalidHeaderValue),
  #[error(transparent)]
  TokioJoinError(#[from] JoinError),
  #[error(transparent)]
  Std(#[from] StdError),
  #[error(transparent)]
  Tikv(#[from] Box<TikvError>),
  #[error(transparent)]
  MongoDB(#[from] MongoDBError),
  #[error(transparent)]
  RedisDBError(#[from] RedisError),
  #[error("Failed to convert OS String to String")]
  StringConvertErr,
  #[error("Tried to retry many times but failed")]
  RetryFailed,
  #[error("Argon Hashing Error")]
  ArgonErr(ArgonErr),
  #[error(transparent)]
  RngErr(#[from] OsError),
}

impl From<TikvError> for ServerError {
  fn from(value: TikvError) -> Self {
    Self::Tikv(Box::new(value))
  }
}

impl actix_web::error::ResponseError for ServerError {
  fn status_code(&self) -> actix_web::http::StatusCode {
    StatusCode::INTERNAL_SERVER_ERROR
  }
}

pub type Returns<T> = Result<T, ServerError>;
