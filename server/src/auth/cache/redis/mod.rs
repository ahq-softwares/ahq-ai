use async_trait::async_trait;
use redis::{AsyncTypedCommands, Client, aio::MultiplexedConnection};

use crate::{
  auth::cache::AsyncCaching,
  server::{CONFIG, DECRYPTED_CONFIG},
  structs::{Authentication, db::CacheConfig, error::Returns},
};

pub struct RedisSessions {
  _redis: Client,
  conn: MultiplexedConnection,
  expiry: u64,
}

const ONE_DAYS_IN_SECS: u64 = 24 * 60 * 60;

impl RedisSessions {
  pub async fn new() -> Self {
    let CacheConfig::Redis { url } = &DECRYPTED_CONFIG.read().await.database.cache else {
      unreachable!()
    };
    let Authentication::Account {
      session_expiry_days,
      ..
    } = CONFIG.authentication
    else {
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
      expiry: session_expiry_days * ONE_DAYS_IN_SECS,
    }
  }
}

// Multiplexed Conn is cheap to clone
#[async_trait]
impl AsyncCaching for RedisSessions {
  async fn get(&self, key: &str) -> Returns<Option<String>> {
    Ok(self.conn.clone().get(key).await?)
  }

  async fn insert(&self, key: &str, value: String) -> Returns<()> {
    self.conn.clone().set_ex(key, value, self.expiry).await?;

    Ok(())
  }
}
