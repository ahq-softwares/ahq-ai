use async_trait::async_trait;
use std::{path::PathBuf, str::FromStr, time::Duration};

use tikv_client::{BoundRange, Config, Error, Key, RawClient, TransactionClient};
use tokio::time::sleep;

use crate::{
  auth::authserver::AuthServer,
  server::DBCONF,
  structs::{
    db::AuthDbConfig,
    error::{Returns, ServerError},
  },
};

pub struct TikvClient {
  raw: RawClient,
  transactional: TransactionClient,
}

impl TikvClient {
  // This must crash the server on error
  pub async fn new() -> Self {
    println!("Connecting to database");

    let mut config = Config::default();

    let AuthDbConfig::Tikv {
      endpoints,
      tls_config,
      timeout_secs,
    } = &DBCONF.authdb
    else {
      panic!("This is not TiKV");
    };

    if *timeout_secs > 0 {
      config.timeout = Duration::from_secs(*timeout_secs as u64);
    }

    if let Some(tls) = tls_config {
      config.ca_path = Some(PathBuf::from_str(&tls.ca_path).expect("Invalid `ca_path`"));
      config.cert_path = Some(PathBuf::from_str(&tls.cert_path).expect("Invalid `cert_path`"));
      config.key_path = Some(PathBuf::from_str(&tls.key_path).expect("Invalid `key_path`"));
    }

    let endpoints = endpoints.iter().map(|x| x as &str).collect::<Vec<_>>();

    Self {
      raw: RawClient::new_with_config(endpoints.clone(), config.clone())
        .await
        .expect("Unable to initialize Database connection"),
      transactional: TransactionClient::new_with_config(endpoints, config)
        .await
        .expect("Unable to initialize Database connection"),
    }
  }
}

#[async_trait]
impl AuthServer for TikvClient {
  async fn get<'a>(&'a self, uid: &'a str) -> Returns<Option<String>> {
    let out = self.raw.get(uid.to_owned()).await?;

    Ok(out.map(|x| String::from_utf8(x).ok()).flatten())
  }

  async fn search<'a>(&'a self, prefix: String) -> Returns<Vec<Vec<u8>>> {
    let start = prefix.into_bytes();
    let mut end = start.clone();
    end.push(255);

    let range = BoundRange::from(Key::from(start)..Key::from(end));

    Ok(
      self
        .raw
        .scan_keys(range, 100)
        .await?
        .into_iter()
        .map(Vec::from)
        .collect::<Vec<_>>(),
    )
  }

  async fn exists<'a>(&'a self, uid: &'a str) -> Returns<bool> {
    let out = self.raw.get(uid.to_owned()).await?;
    Ok(out.is_some())
  }

  async fn update<'a>(&'a self, uid: String, data: String) -> Returns<()> {
    for attempt in 1..=5 {
      let mut txn = self.transactional.begin_optimistic().await?;

      txn.put(uid.clone(), data.clone()).await?;
      match txn.commit().await {
        Ok(_) => return Ok(()),
        Err(e) => {
          if should_retry(&e) {
            sleep(Duration::from_millis(
              (30 * 2u64.pow(attempt - 1)).min(1000),
            ))
            .await;
            continue;
          } else {
            break;
          }
        }
      }
    }

    Err(ServerError::RetryFailed)
  }

  async fn remove<'a>(&'a self, uid: String) -> Returns<()> {
    for attempt in 1..=5 {
      let mut txn = self.transactional.begin_optimistic().await?;

      txn.delete(uid.clone()).await?;
      match txn.commit().await {
        Ok(_) => return Ok(()),
        Err(e) => {
          if should_retry(&e) {
            sleep(Duration::from_millis(
              (30 * 2u64.pow(attempt - 1)).min(1000),
            ))
            .await;
            continue;
          } else {
            break;
          }
        }
      }
    }

    Err(ServerError::RetryFailed)
  }
}

fn should_retry(e: &Error) -> bool {
  matches!(
    e,
    Error::KeyError(_)
      | Error::PessimisticLockError { .. }
      | Error::RegionError(_)
      | Error::LeaderNotFound { .. }
      | Error::UndeterminedError(_)
  )
}
