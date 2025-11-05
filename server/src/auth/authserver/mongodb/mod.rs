use async_trait::async_trait;
use futures::StreamExt;
use mongodb::{Client, Collection, bson::doc};
use serde::{Deserialize, Serialize};

use crate::{
  auth::authserver::AuthServer,
  server::DBCONF,
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
    let AuthDbConfig::Mongodb { url } = &DBCONF.authdb else {
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
    let out = self
      .auth_hash
      .find_one(doc! {
        "_id": uid
      })
      .await?;

    Ok(out.map(|u| u.hash))
  }

  async fn search<'a>(&'a self, prefix: String) -> Returns<Vec<Vec<u8>>> {
    Ok(
      self
        .auth_hash
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
        .map(|x| x.unwrap().id.into_bytes())
        .collect::<Vec<_>>()
        .await,
    )
  }

  async fn exists<'a>(&'a self, uid: &'a str) -> Returns<bool> {
    Ok(
      self
        .auth_hash
        .find_one(doc! {
          "_id": uid
        })
        .await?
        .is_some(),
    )
  }

  async fn update<'a>(&'a self, uid: String, data: String) -> Returns<()> {
    self
      .auth_hash
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
  }

  async fn remove<'a>(&'a self, uid: String) -> Returns<()> {
    self
      .auth_hash
      .delete_one(doc! {
        "_id": uid
      })
      .await?;

    Ok(())
  }
}
