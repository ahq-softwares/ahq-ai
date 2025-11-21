use crossbeam_channel::{Sender, bounded};
use ed25519_dalek::{Signature, SigningKey, ed25519::signature::SignerMut};
use log::info;
use rand::rngs::OsRng;
use std::thread;
use std::thread::available_parallelism;
use tokio::sync::oneshot::{Sender as OneshotSender, channel};

use crate::{
  auth::{INTEGRITY_KEY, argon},
  server::CONFIG,
};

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
  },
}

impl HashingAgent {
  pub fn new() -> Self {
    let total_threads = available_parallelism()
      .expect("Unable to get parallelism")
      .get() as f64;

    let threads = ((total_threads * CONFIG.performance.scale_factor).round() as usize).max(1);

    info!(
      "Allocating {threads} out of {total_threads} for CPU-Bound Task (scale_factor: {})",
      CONFIG.performance.scale_factor
    );

    let queue_size = CONFIG.performance.queue_size * threads;

    let (tx, rx) = bounded::<HashResp>(queue_size);

    info!("Queue has a maximum size of {queue_size}");
    info!("Queue Size Factor is : {}", CONFIG.performance.queue_size);

    for _ in 0..threads {
      let rxc = rx.clone();
      thread::spawn(move || {
        let mut signer = SigningKey::from_keypair_bytes(INTEGRITY_KEY)
          .expect("Invalid integrity key, this error should not come after server has started");

        let mut rng = OsRng;

        while let Ok(x) = rxc.recv() {
          match x {
            HashResp::GenHash { pass, tx } => {
              _ = tx.send(argon::hash_pass(&pass, &mut rng).ok());
            }
            HashResp::CheckHash { pass, hash, tx } => {
              _ = tx.send(argon::verify(&pass, &hash).ok());
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
