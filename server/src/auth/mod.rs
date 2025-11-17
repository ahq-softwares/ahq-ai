use crate::{
  auth::{
    authserver::{AuthServer, moka::MokaTestingDB, mongodb::MongodbClient, tikv::TikvClient},
    cache::{AsyncCaching, moka::MokaSessions, redis::RedisSessions},
    hash::HashingAgent,
  },
  server::CONFIG,
  structs::{
    Authentication,
    db::{AuthDbConfig, CacheConfig},
    error::{Returns, ServerError},
  },
};
use base64::{Engine as _, engine::general_purpose};
use log::warn;
use rand::{Rng, seq::IndexedRandom};
use std::{
  sync::LazyLock,
  time::{SystemTime, UNIX_EPOCH},
};
use tokio::task::spawn_blocking;

pub mod argon;
pub mod hash;

pub mod authserver;
pub mod cache;

pub static INTEGRITY_KEY: &'static [u8; 64] = include_bytes!("./key.bin");

pub static AGENT: LazyLock<HashingAgent> = LazyLock::new(|| HashingAgent::new());

const TOKEN_ID_LENGTH: usize = 12;

#[allow(dead_code)]
pub(crate) struct AuthSessionManager {
  // UserID -> session token
  sessions: Box<dyn AsyncCaching + Send + Sync>,
  pub accounts: Box<dyn AuthServer + Send + Sync>,
  agent: &'static HashingAgent,
}

pub enum AccountCreateOutcome {
  UsernameExists,
  WeakPassword,
  InternalServerError,
  Successful,
  SuccessfulOut(String),
}

pub enum AccountCheckOutcome {
  NotFound,
  TooManyRequests,
  InvalidPassword,
  Some(String),
}

pub type AccountOrToken = (Box<str>, Box<str>);

impl AuthSessionManager {
  pub async fn create() -> Self {
    let accounts: Box<dyn AuthServer + Send + Sync>;
    let sessions: Box<dyn AsyncCaching + Send + Sync>;

    match &CONFIG.database.authdb {
      AuthDbConfig::Moka {} => {
        warn!(
          "CRITICAL WARNING! YOU ARE USING MOKA DB WHICH NEITHER HAS PERSISTENCE NOR IS RECOMMENDED FOR PRODUCTION IN ANY MEANS. PLEASE SHIFT TO A MORE ROBUST DB IMPLEMENTATION LIKE MONGODB OR TIKV FOR EVEN A HOBBY SERVER."
        );
        accounts = Box::new(MokaTestingDB::new());
      }
      AuthDbConfig::Mongodb { .. } => accounts = Box::new(MongodbClient::new().await),
      AuthDbConfig::Tikv { .. } => accounts = Box::new(TikvClient::new().await),
    };

    match &CONFIG.database.cache {
      CacheConfig::Moka => sessions = Box::new(MokaSessions::new()),
      CacheConfig::Redis { .. } => sessions = Box::new(RedisSessions::new().await),
    };

    Self {
      sessions,
      accounts,
      agent: &*AGENT,
    }
  }
}

impl AuthSessionManager {
  pub async fn can_register(&self) -> bool {
    let Authentication::Account {
      registration_allowed,
      ..
    } = CONFIG.authentication
    else {
      return false;
    };

    registration_allowed
  }

  /// THIS ENDPOINT HAS ABSOLUTELY NO PROTECTION
  /// DEVS SHOULD USE `AuthSessionManager::can_register` first
  pub async fn register(&self, user: &str, pass: &str) -> Returns<AccountCreateOutcome> {
    if self.accounts.exists(user).await? {
      return Ok(AccountCreateOutcome::UsernameExists);
    }

    if !is_strong_password(pass).await {
      return Ok(AccountCreateOutcome::WeakPassword);
    }

    let Some(pwd_hash) = self.agent.gen_hash(pass).await else {
      return Ok(AccountCreateOutcome::InternalServerError);
    };

    self.accounts.update(user.to_owned(), pwd_hash).await?;

    Ok(AccountCreateOutcome::Successful)
  }

  pub async fn add_token(&self) -> Returns<AccountCreateOutcome> {
    let (key, (user, hash)) = gen_auth_token(self.agent).await?;

    self.accounts.update(user, hash).await?;

    Ok(AccountCreateOutcome::SuccessfulOut(key))
  }

  pub async fn is_valid_token(&self, token: &str) -> Returns<AccountCheckOutcome> {
    let Some((tok_id, token)) = token.split_once(".") else {
      return Ok(AccountCheckOutcome::NotFound);
    };

    self.is_valid_account(tok_id, token).await
  }

  pub async fn is_valid_account(&self, userid: &str, pass: &str) -> Returns<AccountCheckOutcome> {
    let Some(hash) = self.accounts.get(userid).await? else {
      return Ok(AccountCheckOutcome::NotFound);
    };

    let Some(x) = self.agent.verify_pass(pass, &hash).await else {
      return Ok(AccountCheckOutcome::TooManyRequests);
    };

    if !x {
      return Ok(AccountCheckOutcome::InvalidPassword);
    }

    if let Some(session) = self.sessions.get(userid).await? {
      let sess_cloned = format!("{userid}.{session}");

      return Ok(AccountCheckOutcome::Some(sess_cloned));
    }

    let sess = gen_session_token()?;
    let sess_cloned = format!("{userid}.{sess}");

    self.sessions.insert(userid, sess).await?;

    Ok(AccountCheckOutcome::Some(sess_cloned))
  }

  pub async fn verify_session(&self, token: &str) -> bool {
    let Some((userid, session)) = token.split_once(".") else {
      return false;
    };

    self
      .sessions
      .get(userid)
      .await
      .ok()
      .flatten()
      .map(|x| session == (&x as &str))
      .is_some_and(|x| x)
  }
}

pub fn now() -> u64 {
  SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap()
    .as_secs()
}

pub async fn is_strong_password(password: &str) -> bool {
  if password.len() < 8 {
    return false;
  }

  let mut uppercase = false;
  let mut lowercase = false;
  let mut digit = false;
  let mut special = false;

  password.chars().any(|x| {
    if x.is_ascii_digit() {
      digit = true;
    }

    if x.is_ascii_uppercase() {
      uppercase = true;
    }

    if x.is_ascii_lowercase() {
      lowercase = true;
    }

    if !x.is_ascii_alphanumeric() {
      special = true;
    }

    digit && uppercase && lowercase && special
  })
}

pub const VALUES: [char; 64] = [
  'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
  't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
  'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '0', '1', '2', '3', '4',
  '5', '6', '7', '8', '9', '+', '/',
];

pub type Hashed = String;

pub fn gen_uid() -> Returns<String> {
  let mut rng = rand::rng();

  let token = VALUES.choose_multiple(&mut rng, 32).collect::<String>();

  Ok(token)
}

pub async fn gen_auth_token(cpufarm: &HashingAgent) -> Returns<(String, (String, Hashed))> {
  let mut rng = rand::rng();

  let token = VALUES.choose_multiple(&mut rng, 128).collect::<String>();

  let mut token_key = String::from("tok:");

  token_key.extend(VALUES.choose_multiple(&mut rng, TOKEN_ID_LENGTH));

  let hashed = cpufarm
    .gen_hash(&token)
    .await
    .map_or_else(|| Err(ServerError::StringConvertErr), |x| Ok(x))?;

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
