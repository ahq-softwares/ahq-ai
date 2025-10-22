use moka::future::Cache;
use rand::seq::IndexedRandom;
use serde_json::Deserializer;
use std::{
  io::BufReader,
  sync::Arc,
  time::{Duration, SystemTime, UNIX_EPOCH},
};
use tokio::{fs::File, task::spawn_blocking};

// Hashing Algorithm
use bcrypt::{DEFAULT_COST, hash, verify};

use crate::structs::{Authentication, Config, error::Returns};

// Assumption
// - The Server is safe
// - The User ain't hacked

// TODO: Use these types
#[allow(dead_code)]
pub struct AuthSessionManager {
  sessions: Cache<Box<str>, Arc<Box<str>>>,
  accounts: Cache<Box<str>, Option<Box<str>>>,
  token: bool,
}

pub type Account = (Box<str>, Box<str>);

impl AuthSessionManager {
  pub async fn create(config: &Config) -> Self {
    let sessions = Cache::builder()
      .time_to_live(Duration::from_days(60))
      .build();

    let accounts = Cache::builder().build();

    let token = matches!(config.authentication, Authentication::TokenBased);

    if token {
      if let Ok(x) = File::open("./tokens.json").await {
        let x = x.into_std().await;

        let x = BufReader::new(x);

        let list = Deserializer::from_reader(x)
          .into_iter::<Box<str>>()
          .map(|x| x.unwrap())
          .collect::<Vec<_>>();

        for token_hash in list {
          accounts.insert(token_hash, None).await;
        }
      }
    } else if let Ok(x) = File::open("./accounts.json").await {
      let x = x.into_std().await;

      let x = BufReader::new(x);

      let list = Deserializer::from_reader(x)
        .into_iter::<Account>()
        .map(|x| x.unwrap())
        .collect::<Vec<_>>();

      for (userid, pwd_hash) in list {
        accounts.insert(userid, Some(pwd_hash)).await;
      }
    }

    Self {
      sessions,
      accounts,
      token,
    }
  }

  pub async fn before_exit() {}
}

pub fn now() -> u64 {
  SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap()
    .as_secs()
}

pub async fn create_hash(pass: &str) -> Returns<String> {
  // I, the developer
  // Certify that this is safe
  let pass: &'static str = unsafe { &*(pass as *const str) };

  Ok(spawn_blocking(move || hash(pass, DEFAULT_COST)).await??)
}

pub async fn verify_hash(pass: &str, hash: &str) -> Returns<bool> {
  // I, the developer
  // Certify that this is safe
  let hash: &'static str = unsafe { &*(hash as *const str) };
  let pass: &'static str = unsafe { &*(pass as *const str) };

  Ok(spawn_blocking(move || verify(pass, hash)).await??)
}

pub const VALUES: [char; 64] = [
  'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
  't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
  'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '0', '1', '2', '3', '4',
  '5', '6', '7', '8', '9', '+', '/',
];

pub type Hashed = String;

pub fn gen_random_token() -> Returns<(String, Hashed)> {
  let token = VALUES
    .choose_multiple(&mut rand::rng(), 128)
    .collect::<String>();

  let hashed = hash(&token, DEFAULT_COST)?;

  Ok((token, hashed))
}

pub async fn gen_session_token_async() -> Returns<(String, Hashed)> {
  spawn_blocking(gen_session_token).await?
}

pub fn gen_session_token() -> Returns<(String, Hashed)> {
  let token = VALUES
    .choose_multiple(&mut rand::rng(), 64)
    .collect::<String>();

  let hashed = hash(&token, DEFAULT_COST)?;

  Ok((token, hashed))
}
