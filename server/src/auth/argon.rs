use std::{fs, sync::LazyLock};

use argon2::{
  Algorithm, Argon2, Params, PasswordHash, PasswordHasher, RECOMMENDED_SALT_LEN, Version,
  password_hash::SaltString,
};
use rand::{RngCore, rng, rngs::ThreadRng};

use crate::{server::CONFIG, structs::{Authentication, error::{Returns, ServerError}}};

const KEY_LEN: usize = 32;

static KEYARGON: LazyLock<Argon2> = LazyLock::new(|| {
  let params = Params::new(64 * 1024, 2, 1, Some(KEY_LEN)).unwrap();

  Argon2::new(Algorithm::Argon2id, Version::V0x13, params)
});

static HASHARGON: LazyLock<Argon2> = LazyLock::new(|| {
  let Authentication::Account { max_memory, time_cost, .. } = CONFIG.authentication.clone() else {
    unreachable!()
  };

  let params = Params::new(max_memory * 1024, time_cost, 1, None).unwrap();

  Argon2::new(Algorithm::Argon2id, Version::V0x13, params)
});

static KEYARGONSALT: LazyLock<[u8; 32]> = LazyLock::new(|| {
  if let Ok(x) = fs::read("./saltdata") {
    if let Ok(salt) = x.try_into() {
      return salt;
    }
  }

  let mut salt_bytes = [0u8; RECOMMENDED_SALT_LEN * 2];

  rng().fill_bytes(&mut salt_bytes);

  _ = fs::write("./saltdata", &salt_bytes);

  salt_bytes
});

pub fn hash_pass(pwd: &str, rng: &mut ThreadRng) -> Returns<String> {
  let mut salt_bytes = [0u8; RECOMMENDED_SALT_LEN * 2];

  rng.fill_bytes(&mut salt_bytes);

  let data: String = HASHARGON
    .hash_password(
      pwd.as_bytes(),
      &SaltString::encode_b64(&salt_bytes).map_err(|x| ServerError::ArgonErr(x))?,
    )
    .map_err(|x| ServerError::ArgonErr(x))?
    .to_string();

  Ok(data)
}

pub fn verify(pwd: &str, hash: &str) -> Returns<bool> {
  Ok(
    PasswordHash::new(hash)
      .map_err(|x| ServerError::ArgonErr(x))?
      .verify_password(&[&*HASHARGON], pwd)
      .ok()
      .is_some(),
  )
}

pub mod server {
  use crate::{auth::argon::KEYARGON, structs::error::Returns};
  use argon2::{PasswordHash, PasswordHasher, RECOMMENDED_SALT_LEN, password_hash::SaltString};
  use rand::{RngCore, rngs::ThreadRng};
  use crate::structs::error::ServerError;

  /// Use only in the Terminal User Interface
  pub fn hash_server_pass(pwd: &str, rng: &mut ThreadRng) -> Returns<String> {
    let mut salt_bytes = [0u8; RECOMMENDED_SALT_LEN * 2];

    rng.fill_bytes(&mut salt_bytes);

    let data: String = KEYARGON
      .hash_password(
        pwd.as_bytes(),
        &SaltString::encode_b64(&salt_bytes).map_err(|x| ServerError::ArgonErr(x))?,
      )
      .map_err(|x| ServerError::ArgonErr(x))?
      .to_string();

    Ok(data)
  }

  /// Use only in the Terminal User Interface
  pub fn verify_server_pass(pwd: &str, hash: &str) -> Returns<bool> {
    Ok(
      PasswordHash::new(hash)
        .map_err(|x| ServerError::ArgonErr(x))?
        .verify_password(&[&*KEYARGON], pwd)
        .ok()
        .is_some(),
    )
  }
}

pub fn get_privatekey(pwd: &str) -> [u8; KEY_LEN] {
  let mut key_bytes = [0u8; KEY_LEN];

  KEYARGON
    .hash_password_into(pwd.as_bytes(), KEYARGONSALT.as_slice(), &mut key_bytes)
    .unwrap();

  key_bytes
}
