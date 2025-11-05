use async_trait::async_trait;
use redis::{AsyncTypedCommands, Client, aio::MultiplexedConnection};

use crate::{
  auth::cache::AsyncCaching,
  server::DBCONF,
  structs::{db::CacheConfig, error::Returns},
};

pub struct RedisSessions {
  _redis: Client,
  conn: MultiplexedConnection,
}

impl RedisSessions {
  pub async fn new() -> Self {
    let CacheConfig::Redis { url } = &DBCONF.cache else {
      unreachable!()
    };

    let redis = Client::open(url as &str).expect("Failed to initialize redis connection");

    let conn = redis
      .get_multiplexed_async_connection()
      .await
      .expect("Unable to get REDIS Connection");

    RedisSessions {
      _redis: redis,
      conn,
    }
  }
}

const THIRTY_DAYS_IN_SECS: u64 = 30 * 24 * 60 * 60;

// Multiplexed Conn is cheap to clone
#[async_trait]
impl AsyncCaching for RedisSessions {
  async fn get(&self, key: &str) -> Returns<Option<String>> {
    Ok(self.conn.clone().get(key).await?)
  }

  async fn insert(&self, key: &str, value: String) -> Returns<()> {
    self
      .conn
      .clone()
      .set_ex(key, value, THIRTY_DAYS_IN_SECS)
      .await?;

    Ok(())
  }
}
