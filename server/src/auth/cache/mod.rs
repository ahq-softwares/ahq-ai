use async_trait::async_trait;

use crate::structs::error::Returns;

pub mod moka;
pub mod redis;

#[async_trait]
pub trait AsyncCaching {
  async fn get(&self, key: &str) -> Returns<Option<String>>;
  async fn insert(&self, key: &str, value: String) -> Returns<()>;
}
