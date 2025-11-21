use std::time::Duration;

use async_trait::async_trait;
use moka::future::Cache;

use crate::{
  auth::cache::AsyncCaching,
  server::CONFIG,
  structs::{Authentication, error::Returns},
};

pub struct MokaSessions {
  cache: Cache<String, String>,
}

impl Default for MokaSessions {
  fn default() -> Self {
    Self::new()
  }
}

impl MokaSessions {
  pub fn new() -> Self {
    let Authentication::Account {
      session_expiry_days,
      ..
    } = CONFIG.authentication
    else {
      unreachable!()
    };

    Self {
      cache: Cache::builder()
        .time_to_live(Duration::from_days(session_expiry_days))
        .build(),
    }
  }
}

#[async_trait]
impl AsyncCaching for MokaSessions {
  async fn get(&self, key: &str) -> Returns<Option<String>> {
    Ok(self.cache.get(key).await)
  }

  async fn insert(&self, key: &str, value: String) -> Returns<()> {
    self.cache.insert(key.to_owned(), value).await;
    Ok(())
  }
}
