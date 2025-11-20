use async_trait::async_trait;
use moka::future::Cache;

use crate::{
  auth::authserver::AuthServer,
  structs::error::{Returns, ServerError},
};

pub struct MokaTestingDB {
  cache: Cache<String, String>,
}

impl Default for MokaTestingDB {
  fn default() -> Self {
    Self::new()
  }
}

impl MokaTestingDB {
  pub fn new() -> Self {
    Self {
      cache: Cache::builder().build(),
    }
  }
}

#[async_trait]
impl AuthServer for MokaTestingDB {
  async fn get<'a>(&'a self, uid: &'a str) -> Returns<Option<String>> {
    Ok(self.cache.get(uid).await)
  }

  async fn search<'a>(&'a self, _: String) -> Returns<Vec<Vec<u8>>> {
    Err(ServerError::StringConvertErr)
  }

  async fn exists<'a>(&'a self, uid: &'a str) -> Returns<bool> {
    Ok(self.cache.contains_key(uid))
  }

  async fn update<'a>(&'a self, uid: String, data: String) -> Returns<()> {
    self.cache.insert(uid, data).await;

    Ok(())
  }

  async fn remove<'a>(&'a self, uid: String) -> Returns<()> {
    self.cache.remove(&uid).await;

    Ok(())
  }
}
