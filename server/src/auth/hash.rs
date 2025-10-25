use bcrypt::{hash, verify};
use crossbeam_channel::{Sender, bounded};
use std::thread;
use std::thread::available_parallelism;
use tokio::sync::oneshot::{Sender as OneshotSender, channel};

use crate::structs::BCRYPT_COST;

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
        while let Ok(x) = rxc.recv() {
          match x {
            HashResp::GenHash { pass, tx } => {
              _ = tx.send(hash(&pass, BCRYPT_COST).ok());
            }
            HashResp::CheckHash { pass, hash, tx } => {
              _ = tx.send(verify(&pass, &hash).ok());
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
  pub async fn verify_pass<'a>(&self, pass: &'a str, hash: &'a str) -> Option<bool> {
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
  pub async fn gen_hash<'a>(&self, pass: &'a str) -> Option<String> {
    if self.0.is_full() {
      return None;
    }

    let pass: String = pass.to_owned();

    let (tx, rx) = channel::<Option<String>>();

    self.0.try_send(HashResp::GenHash { pass, tx }).ok()?;

    rx.await.ok()?
  }
}
