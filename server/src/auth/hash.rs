use bcrypt::{hash, verify};
use crossbeam_channel::{Sender, bounded};
use ed25519_dalek::{Signature, SigningKey, ed25519::signature::SignerMut};
use std::thread;
use std::thread::available_parallelism;
use tokio::sync::oneshot::{Sender as OneshotSender, channel};

use crate::{auth::INTEGRITY_KEY, structs::BCRYPT_COST};

pub struct HashingAgent(Sender<HashResp>);

pub enum HashResp {
  CheckHash {
    pass: String,
    hash: String,
    tx: OneshotSender<Option<bool>>,
  },
  GenHash {
    pass: String,
    tx: OneshotSender<Option<String>>,
  },
  Challenge {
    bytes: Vec<u8>,
    tx: OneshotSender<Option<Signature>>,
  }
}

impl HashingAgent {
  pub fn new() -> Self {
    let threads = available_parallelism()
      .expect("Unable to get parallelism")
      .get();

    let (tx, rx) = bounded::<HashResp>(2 * threads);

    for _ in 0..threads {
      let rxc = rx.clone();
      thread::spawn(move || {
        let mut signer = SigningKey::from_keypair_bytes(INTEGRITY_KEY).unwrap();
        
        while let Ok(x) = rxc.recv() {
          match x {
            HashResp::GenHash { pass, tx } => {
              _ = tx.send(hash(&pass, BCRYPT_COST).ok());
            }
            HashResp::CheckHash { pass, hash, tx } => {
              _ = tx.send(verify(&pass, &hash).ok());
            }
            HashResp::Challenge { bytes, tx } => {
              let sign = signer.try_sign(&bytes).ok();

              _ = tx.send(sign)
            }
          }
        }
      });
    }

    Self(tx)
  }

  /// # Cloning:
  /// This
  ///
  /// # Returns
  /// This function returns None in case of the server's queue being maxed out
  pub async fn verify_pass(&self, pass: &str, hash: &str) -> Option<bool> {
    if self.0.is_full() {
      return None;
    }

    let hash: String = hash.to_owned();
    let pass: String = pass.to_owned();

    let (tx, rx) = channel::<Option<bool>>();

    self
      .0
      .try_send(HashResp::CheckHash { pass, hash, tx })
      .ok()?;

    rx.await.ok()?
  }

  /// # Returns
  /// This function returns None in case of the server's queue being maxed out
  pub async fn gen_hash(&self, pass: &str) -> Option<String> {
    if self.0.is_full() {
      return None;
    }

    let pass: String = pass.to_owned();

    let (tx, rx) = channel::<Option<String>>();

    self.0.try_send(HashResp::GenHash { pass, tx }).ok()?;

    rx.await.ok()?
  }

    /// # Returns
  /// This function returns None in case of the server's queue being maxed out
  pub async fn gen_signature(&self, data: &[u8]) -> Option<Signature> {
    if self.0.is_full() {
      return None;
    }

    let bytes = data.to_owned();

    let (tx, rx) = channel::<Option<Signature>>();

    self.0.try_send(HashResp::Challenge { bytes, tx }).ok()?;

    rx.await.ok()?
  }
}

impl Default for HashingAgent {
  fn default() -> Self {
    Self::new()
  }
}
