use std::mem::transmute;
use std::sync::LazyLock;

use ed25519_dalek::Signature;
use tauri::async_runtime::spawn_blocking;
use tauri::{command, AppHandle, Runtime};

use crate::models::*;
use crate::AhqaiExt;
use crate::Result;

use ed25519_dalek::VerifyingKey;

pub static SIGNING_KEY: [u8; 32] = [
  28, 180, 64, 146, 90, 210, 95, 50, 112, 100, 44, 58, 148, 10, 179, 33, 62, 233, 246, 42, 141,
  248, 176, 46, 13, 209, 245, 17, 160, 243, 26, 14,
];

pub static FILE_INTEGRITY_VERIFIER: LazyLock<VerifyingKey> = LazyLock::new(|| {
  VerifyingKey::from_bytes(&SIGNING_KEY).expect("Unable to get integrity key")
});

#[tauri::command]
pub async fn check_file_integrity(file: Vec<u8>, sig: Vec<u8>) -> Result<bool> {
  let sig = Signature::from_bytes((&sig as &[u8]).try_into()?);

  let out = spawn_blocking(move || {
    FILE_INTEGRITY_VERIFIER.verify_strict(&file, &sig).is_ok()
  }).await?;

  Ok(
    out
  )
}

#[tauri::command]
pub async fn check_resp_integrity(resp: Vec<u8>, sig: Vec<u8>, pubkey: Vec<u8>) -> Result<bool> {
  let verifier = VerifyingKey::from_bytes((&pubkey as &[u8]).try_into()?)?;
  let sig = Signature::from_bytes((&sig as &[u8]).try_into()?);

  let out = spawn_blocking(move || {
    verifier.verify_strict(&resp, &sig).is_ok()
  }).await?;

  Ok(
    out
  )
}