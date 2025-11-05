use crate::structs::error::Returns;
use async_trait::async_trait;

pub mod mongodb;
pub mod tikv;

#[async_trait]
pub(crate) trait AuthServer {
  async fn get<'a>(&'a self, uid: &'a str) -> Returns<Option<String>>;

  async fn search<'a>(&'a self, prefix: String) -> Returns<Vec<Vec<u8>>>;

  async fn exists<'a>(&'a self, uid: &'a str) -> Returns<bool>;

  async fn update<'a>(&'a self, uid: String, data: String) -> Returns<()>;

  async fn remove<'a>(&'a self, uid: String) -> Returns<()>;
}
