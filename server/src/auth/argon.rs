use std::sync::LazyLock;

use aes_gcm::{Aes256Gcm, KeyInit, Nonce, aead::Aead};
use argon2::{
  Algorithm, Argon2, Params, PasswordHash, PasswordHasher, RECOMMENDED_SALT_LEN, Version,
  password_hash::SaltString,
};
use base64::{Engine, prelude::BASE64_STANDARD};
use rand::{TryRngCore, rngs::OsRng};
use zeroize::Zeroize;

use crate::{
  server::CONFIG,
  structs::{
    Authentication, Config,
    db::{AuthDbConfig, CacheConfig},
    error::{Returns, ServerError},
  },
};

const KEY_LEN: usize = 32;

static KEYARGON: LazyLock<Argon2> = LazyLock::new(|| {
  #[cfg(debug_assertions)]
  let iterations = 2;

  #[cfg(debug_assertions)]
  let memory = 32;

  #[cfg(not(debug_assertions))]
  let iterations = 5;

  #[cfg(not(debug_assertions))]
  let memory = 128;

  let params = Params::new(memory * 1024, iterations, 1, Some(KEY_LEN))
    .expect("Invalid argon2 configuration found. Exiting server");

  Argon2::new(Algorithm::Argon2id, Version::V0x13, params)
});

static HASHARGON: LazyLock<Argon2> = LazyLock::new(|| {
  let Authentication::Account {
    max_memory,
    time_cost,
    hash_bytes,
    ..
  } = CONFIG.authentication.clone()
  else {
    unreachable!()
  };

  let params = Params::new(max_memory * 1024, time_cost, 1, Some(hash_bytes))
    .expect("Invalid argon2 configuration found. Exiting server");

  Argon2::new(Algorithm::Argon2id, Version::V0x13, params)
});

pub static SALT_LEN: usize = RECOMMENDED_SALT_LEN * 2;

pub fn hash_pass(pwd: &str, rng: &mut OsRng) -> Returns<String> {
  let mut salt_bytes = [0u8; SALT_LEN];

  rng.try_fill_bytes(&mut salt_bytes)?;

  let data: String = HASHARGON
    .hash_password(
      pwd.as_bytes(),
      &SaltString::encode_b64(&salt_bytes).map_err(ServerError::ArgonErr)?,
    )
    .map_err(ServerError::ArgonErr)?
    .to_string();

  Ok(data)
}

pub fn verify(pwd: &str, hash: &str) -> Returns<bool> {
  Ok(
    PasswordHash::new(hash)
      .map_err(ServerError::ArgonErr)?
      .verify_password(&[&*HASHARGON], pwd)
      .ok()
      .is_some(),
  )
}

pub mod server {
  use crate::auth::argon::SALT_LEN;
  use crate::structs::error::ServerError;
  use crate::{auth::argon::KEYARGON, structs::error::Returns};
  use argon2::{PasswordHash, PasswordHasher, password_hash::SaltString};
  use rand::{TryRngCore, rngs::OsRng};

  /// Use only in the Terminal User Interface
  pub fn hash_server_pass(pwd: &str) -> Returns<String> {
    let mut salt_bytes = [0u8; SALT_LEN];

    OsRng.try_fill_bytes(&mut salt_bytes)?;

    let data: String = KEYARGON
      .hash_password(
        pwd.as_bytes(),
        &SaltString::encode_b64(&salt_bytes).map_err(ServerError::ArgonErr)?,
      )
      .map_err(ServerError::ArgonErr)?
      .to_string();

    Ok(data)
  }

  /// Use only in the Terminal User Interface
  pub fn verify_server_pass(pwd: &str, hash: &str) -> Returns<bool> {
    Ok(
      PasswordHash::new(hash)
        .map_err(ServerError::ArgonErr)?
        .verify_password(&[&*KEYARGON], pwd)
        .ok()
        .is_some(),
    )
  }
}

const NONCE_LEN: usize = 12;

/// ## PANICKING
/// This is a panicking functions
///
/// This function panicks like crazy because it is not used in
/// the server side code
pub fn encrypt_with_key(pwd: &str, data: &str) -> String {
  let salt = {
    let mut salt_bytes = [0u8; SALT_LEN];

    OsRng
      .try_fill_bytes(&mut salt_bytes)
      .expect("OS Error, the os does not have the capability to generate random data");

    salt_bytes
  };

  let mut key = {
    let mut key_bytes = [0u8; KEY_LEN];

    KEYARGON
      .hash_password_into(pwd.as_bytes(), salt.as_slice(), &mut key_bytes)
      .unwrap();

    key_bytes
  };

  let aes = { Aes256Gcm::new_from_slice(&key).unwrap() };

  let nonce_slice = {
    let mut nonce_slice = [0u8; NONCE_LEN];

    OsRng.try_fill_bytes(&mut nonce_slice).unwrap();

    nonce_slice
  };

  let nonce = Nonce::from_slice(&nonce_slice);

  let ciphertext_with_tag = aes.encrypt(nonce, data.as_bytes()).unwrap();

  key.zeroize();

  let mut out = Vec::from(salt);
  out.extend(nonce_slice);
  out.extend(ciphertext_with_tag);

  BASE64_STANDARD.encode(out)
}

/// # WARNING
/// This functions returns an empty string if the data provided
/// is empty. Please be informed
///
/// ## PANICKING
/// This is a panicking functions
///
/// This function panicks like crazy because it is not used in
/// the server side code
pub fn decrypt_with_key(pwd: &str, data: &str) -> String {
  // WARNING
  // A very powerful failsafe
  if data.is_empty() {
    return String::new();
  }

  let raw = BASE64_STANDARD.decode(data).unwrap();

  let salt_slice = &raw[0..SALT_LEN];

  let nonce_slice = &raw[SALT_LEN..(SALT_LEN + NONCE_LEN)];
  let nonce = Nonce::from_slice(nonce_slice);

  let cipher = &raw[(SALT_LEN + NONCE_LEN)..];

  let mut key = {
    let mut key_bytes = [0u8; KEY_LEN];

    KEYARGON
      .hash_password_into(pwd.as_bytes(), salt_slice, &mut key_bytes)
      .unwrap();

    key_bytes
  };

  let aes = { Aes256Gcm::new_from_slice(&key).unwrap() };

  key.zeroize();

  let ciphertext_with_tag = aes.decrypt(nonce, cipher).unwrap();

  String::from_utf8(ciphertext_with_tag).unwrap()
}

/// ## PANICKING
/// This is a panicking functions
///
/// This function panicks like crazy because it is not used in
/// the server side code
pub fn migrate_key(old_pass: &str, new_pass: &str, encrypted: &str) -> String {
  let data = decrypt_with_key(old_pass, encrypted);

  encrypt_with_key(new_pass, &data)
}

/// Since all the important / potentially dangerous can be leaked, credentials
/// are encrypted in config itself
///
/// We must decrypt and re-encrypt with my new password when the admin password
/// changes
/// ## PANICKING
/// This is a panicking functions
///
/// This function panicks like crazy because it is not used in
/// the server side code
pub fn migrate_config(old_pass: &str, new_pass: &str, config: &mut Config) {
  {
    config.llama.models.iter_mut().for_each(|(_, v)| {
      if let Some(api) = &mut v.apikey {
        *api = migrate_key(old_pass, new_pass, api).into_boxed_str();
      }
    });
  }

  {
    match &mut config.database.authdb {
      AuthDbConfig::Mongodb { url } => {
        *url = migrate_key(old_pass, new_pass, url).into_boxed_str();
      }
      AuthDbConfig::Tikv {
        endpoints,
        tls_config,
        ..
      } => {
        endpoints.iter_mut().for_each(|data| {
          *data = migrate_key(old_pass, new_pass, data).into_boxed_str();
        });

        if let Some(conf) = tls_config {
          conf.ca_path = migrate_key(old_pass, new_pass, &conf.ca_path).into_boxed_str();
          conf.cert_path = migrate_key(old_pass, new_pass, &conf.cert_path).into_boxed_str();
          conf.key_path = migrate_key(old_pass, new_pass, &conf.key_path).into_boxed_str();
        }
      }
      AuthDbConfig::Moka { .. } => {}
    }
  }

  {
    match &mut config.database.cache {
      CacheConfig::Moka => {}
      CacheConfig::Redis { url } => {
        *url = migrate_key(old_pass, new_pass, url).into_boxed_str();
      }
    }
  }
}

/// ## PANICKING
/// This is a panicking functions
///
/// This function panicks like crazy because it is not used in
/// the server side code
pub fn decrypt_config(pass: &str, config: &mut Config) {
  {
    config.llama.models.iter_mut().for_each(|(_, v)| {
      if let Some(api) = &mut v.apikey {
        let mut api_ = decrypt_with_key(pass, api);
        let api2 = format!("Bearer {api_}");

        *api = api2.into_boxed_str();

        api_.zeroize();
      }
    });
  }

  {
    match &mut config.database.authdb {
      AuthDbConfig::Mongodb { url } => {
        *url = decrypt_with_key(pass, url).into_boxed_str();
      }
      AuthDbConfig::Tikv {
        endpoints,
        tls_config,
        ..
      } => {
        endpoints.iter_mut().for_each(|data| {
          *data = decrypt_with_key(pass, data).into_boxed_str();
        });

        if let Some(conf) = tls_config {
          conf.ca_path = decrypt_with_key(pass, &conf.ca_path).into_boxed_str();
          conf.cert_path = decrypt_with_key(pass, &conf.cert_path).into_boxed_str();
          conf.key_path = decrypt_with_key(pass, &conf.key_path).into_boxed_str();
        }
      }
      AuthDbConfig::Moka { .. } => {}
    }
  }

  {
    match &mut config.database.cache {
      CacheConfig::Moka => {}
      CacheConfig::Redis { url } => {
        *url = decrypt_with_key(pass, url).into_boxed_str();
      }
    }
  }
}
