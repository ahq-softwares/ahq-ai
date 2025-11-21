use async_trait::async_trait;
use futures::StreamExt;
use mongodb::{Client, Collection, bson::doc};
use serde::{Deserialize, Serialize};
use tokio::spawn;

use crate::{
  auth::authserver::AuthServer,
  server::DECRYPTED_CONFIG,
  structs::{db::AuthDbConfig, error::Returns},
};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserOrAuthToken {
  #[serde(rename = "_id")]
  pub id: String,
  pub hash: String,
}

pub struct MongodbClient {
  _client: Client,
  auth_hash: Collection<UserOrAuthToken>,
}

impl MongodbClient {
  pub async fn new() -> Self {
    let AuthDbConfig::Mongodb { url } = &DECRYPTED_CONFIG.read().await.database.authdb else {
      unreachable!()
    };

    let client = Client::with_uri_str(url)
      .await
      .expect("Failed to connect to MongoDB");

    let auth_hash = client
      .database("auth")
      .collection::<UserOrAuthToken>("auth");

    Self {
      _client: client,
      auth_hash,
    }
  }
}

#[async_trait]
impl AuthServer for MongodbClient {
  async fn get<'a>(&'a self, uid: &'a str) -> Returns<Option<String>> {
    let auth_hash = self.auth_hash.clone();
    let doc = doc! {
      "_id": uid
    };

    tokio::spawn(async move {
      let out = auth_hash.find_one(doc).await?;

      Ok(out.map(|u| u.hash))
    })
    .await?
  }

  async fn search<'a>(&'a self, prefix: String) -> Returns<Vec<Vec<u8>>> {
    let auth_hash = self.auth_hash.clone();

    tokio::spawn(async move {
      Ok(
        auth_hash
          .find(doc! {
            "_id": {
              "$regex": format!("^{}.*", prefix),
              "$options": "i"
            }
          })
          .limit(100)
          .await?
          .filter(|x| {
            let out = x.is_ok();
            async move { out }
          })
          // SAFETY
          // Guaranteed to be non null
          .map(|x| {
            x.expect("The documents that have data are coming over here, nothing else")
              .id
              .into_bytes()
          })
          .collect::<Vec<_>>()
          .await,
      )
    })
    .await?
  }

  async fn exists<'a>(&'a self, uid: &'a str) -> Returns<bool> {
    let auth_hash = self.auth_hash.clone();
    let doc = doc! {
      "_id": uid
    };

    spawn(async move { Ok(auth_hash.find_one(doc).await?.is_some()) }).await?
  }

  async fn update<'a>(&'a self, uid: String, data: String) -> Returns<()> {
    let auth_hash = self.auth_hash.clone();

    spawn(async move {
      auth_hash
        .find_one_and_update(
          doc! {
            "_id": &uid,
          },
          doc! {
            "$set": {
              "hash": data,
            }
          },
        )
        .upsert(true)
        .await?;

      Ok(())
    })
    .await?
  }

  async fn remove<'a>(&'a self, uid: String) -> Returns<()> {
    let auth_hash = self.auth_hash.clone();

    spawn(async move {
      auth_hash
        .delete_one(doc! {
          "_id": uid
        })
        .await?;

      Ok(())
    })
    .await?
  }
}
