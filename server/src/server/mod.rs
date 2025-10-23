use std::{
  fs as stdfs,
  sync::{LazyLock, OnceLock},
  thread::available_parallelism,
};

use actix_web::{App, HttpServer, web};
use chalk_rs::Chalk;
use ollama_rs::Ollama;
use serde_json::from_str;

use crate::{
  auth::AuthSessionManager,
  structs::{Authentication, Config},
};

pub mod auth;
pub mod chat;
pub mod http;

pub static CONFIG: LazyLock<Config> = LazyLock::new(|| {
  let data = stdfs::read_to_string("config.json").expect("Unable to load config");

  from_str(&data).expect("Invalid configuration file, unable to parse")
});

pub static TOKEN: LazyLock<bool> =
  LazyLock::new(|| matches!(CONFIG.authentication, Authentication::TokenBased));

pub static AUTH: OnceLock<AuthSessionManager> = OnceLock::new();

pub static OLLAMA: LazyLock<Ollama> =
  LazyLock::new(|| Ollama::new(CONFIG.ollama.host.as_ref(), CONFIG.ollama.port));

pub fn launch() -> Chalk {
  let mut chalk = Chalk::new();

  chalk
    .blue()
    .bold()
    .println(&format!("AHQ-AI Server v{}", env!("CARGO_PKG_VERSION")));

  chalk.reset_style();

  chalk
}

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
  let mut chalk = launch();

  let auth = !matches!(CONFIG.authentication, Authentication::OpenToAll);

  if auth {
    _ = AUTH.set(AuthSessionManager::create().await);
  }

  if OLLAMA.list_local_models().await.is_err() {
    println!("----------------");
    chalk
      .red()
      .println(&"Connection to ollama failed. Are you sure configuration is correct?");
    println!("----------------");
  }

  let mut server = HttpServer::new(|| {
    let mut app = App::new()
      .service(http::index)
      .route("/chat", web::get().to(chat::chat));

    let auth = !matches!(CONFIG.authentication, Authentication::OpenToAll);

    if auth {
      app = app.service(auth::auth);
    }

    app
  })
  .workers(available_parallelism()?.get());

  for (host, port) in &CONFIG.binds {
    chalk.blue().println(&format!("Binding to {host}:{port}"));
    server = server.bind((host as &str, *port))?
  }

  println!("----------------");
  chalk.blue().println(&"Server is ready!");
  println!("----------------");
  println!();

  let out = server.run().await;

  if let Err(e) = &out {
    println!("----------------");
    chalk
      .red()
      .bold()
      .println(&"Server Exited in an error state");
    println!("{e}");
  }

  println!("----------------");
  chalk.reset_style().blue().bold().println(
    &"Starting shutdown procedure. Saving server state to disk... This might take a while",
  );
  chalk
    .red()
    .bold()
    .println(&"Please DO NOT use Ctrl+C to terminate. It will lead to data corruption!");

  if auth {
    AUTH
      .get()
      .expect("Impossible error")
      .before_exit()
      .await
      .unwrap();
  }

  chalk.reset_style().blue().bold().println(
    &"Server state has been successfully set! Closing server. Session tokens will be discarded.",
  );

  out
}
