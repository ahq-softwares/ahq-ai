use base64::{Engine as _, engine::general_purpose};
use bcrypt::hash;
use moka::future::Cache;
use rand::{Rng, seq::IndexedRandom};
use serde_json::Deserializer;
use std::{
  io::{BufReader, Write},
  sync::Arc,
  time::{Duration, SystemTime, UNIX_EPOCH},
};
use tokio::{fs::File, task::spawn_blocking};
use crate::{auth::hash::HashingAgent, structs::{BCRYPT_COST, error::Returns}};

pub mod hash;

const TOKEN_ID_LENGTH: usize = 12;

#[allow(dead_code)]
pub struct AuthSessionManager {
  sessions: Cache<Box<str>, Arc<Box<str>>>,
  accounts: Cache<Box<str>, Arc<Box<str>>>,
  agent: HashingAgent
}

pub enum AccountCheckOutcome {
  NotFound,
  TooManyRequests,
  InvalidPassword,
  Some(String)
}

pub type AccountOrToken = (Box<str>, Box<str>);

impl AuthSessionManager {
  pub async fn create() -> Self {
    let sessions = Cache::builder()
      .time_to_live(Duration::from_days(30))
      .build();

    let accounts = Cache::builder().build();

    if let Ok(x) = File::open("./authdata.jsonl").await {
      let x = x.into_std().await;

      let x = BufReader::new(x);

      let list = Deserializer::from_reader(x)
        .into_iter::<AccountOrToken>()
        .map(|x| x.unwrap());

      for (id, pwd_hash) in list {
        accounts.insert(id, Arc::new(pwd_hash)).await;
      }
    }

    Self { sessions, accounts,agent: HashingAgent::new() }
  }

  pub async fn is_valid_token(&self, token: &str) -> Returns<AccountCheckOutcome> {
    let Some((tok_id, token)) = token.split_once(".") else {
      return Ok(AccountCheckOutcome::NotFound);
    };

    self.is_valid_account(tok_id, token).await
  }

  pub async fn is_valid_account(&self, userid: &str, pass: &str) -> Returns<AccountCheckOutcome> {
    let Some(hash) = self.accounts.get(userid).await else {
      return Ok(AccountCheckOutcome::NotFound);
    };

    let Some(x) = self.agent.verify_pass(pass, &hash).await else {
      return Ok(AccountCheckOutcome::TooManyRequests);
    };

    if !x {
      return Ok(AccountCheckOutcome::InvalidPassword);
    }

    let sess = gen_session_token()?;
    let sess_cloned = sess.clone();

    let userid_owned = Arc::new(userid.to_owned().into_boxed_str());

    self
      .sessions
      .insert(sess.into_boxed_str(), userid_owned)
      .await;

    Ok(AccountCheckOutcome::Some(sess_cloned))
  }

  pub async fn verify_session(&self, token: &str) -> bool {
    self
      .sessions
      .get(token)
      .await
      .map(|x| self.accounts.contains_key(&x as &str))
      .is_some_and(|x| x)
  }

  pub async fn before_exit(&self) -> Returns<()> {
    let file = File::create("authdata.jsonl").await?;
    let mut file = file.into_std().await;

    let data = self.accounts.iter().map(|(k, pass)| {
      let uid = &*k;
      let uid = uid.clone();

      let pass = &*pass;
      let pass = pass.clone();

      (uid, pass)
    });

    for data in data {
      serde_json::to_writer(&mut file, &data)?;

      file.write(b"\n")?;
      file.flush()?;
    }

    file.flush()?;

    Ok(())
  }
}

pub fn now() -> u64 {
  SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap()
    .as_secs()
}

pub const VALUES: [char; 64] = [
  'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
  't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
  'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '0', '1', '2', '3', '4',
  '5', '6', '7', '8', '9', '+', '/',
];

pub type Hashed = Box<str>;

pub fn gen_auth_token() -> Returns<(String, (Box<str>, Hashed))> {
  let mut rng = rand::rng();

  let token = VALUES.choose_multiple(&mut rng, 128).collect::<String>();

  let token_key = VALUES
    .choose_multiple(&mut rng, TOKEN_ID_LENGTH)
    .collect::<Box<str>>();

  let hashed = hash(&token, BCRYPT_COST)?.into_boxed_str();

  let token_to_output = format!("{token_key}.{token}");

  Ok((token_to_output, (token_key, hashed)))
}

pub async fn gen_session_token_async() -> Returns<String> {
  spawn_blocking(gen_session_token).await?
}

// Server Token
pub fn gen_session_token() -> Returns<String> {
  let mut rng = rand::rng();

  let mut token = vec![0u8; 96];

  rng.fill(&mut token as &mut [u8]);

  let token = general_purpose::URL_SAFE_NO_PAD.encode(&token);

  Ok(token)
}
