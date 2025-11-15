use std::{
  env, fs as stdfs,
  sync::{LazyLock, OnceLock},
  thread::available_parallelism,
};

use crate::{
  auth::{AuthSessionManager, argon::server::verify_server_pass},
  structs::{Authentication, Config, db::DatabaseConfig},
};
use actix_web::{App, HttpServer, web};
use chalk_rs::Chalk;
use log::*;
use secrecy::SecretString;
use serde_json::from_str;

pub mod admin;
pub mod auth;
// pub mod chat;
pub mod http;

pub mod llama;

pub mod ffi;

pub static CONFIG: LazyLock<Config> = LazyLock::new(|| {
  let data = stdfs::read_to_string("config.json").expect("Unable to load config");

  from_str(&data).expect("Invalid configuration file, unable to parse")
});

pub static DBCONF: LazyLock<DatabaseConfig> = LazyLock::new(|| DatabaseConfig::get());

// pub static HISTORY_LENGTH: LazyLock<usize> = LazyLock::new(|| CONFIG.ollama.msgs.saturating_mul(2));

pub static AUTH: OnceLock<AuthSessionManager> = OnceLock::new();

pub static REAL_ADMIN_PASSWORD: OnceLock<SecretString> = OnceLock::new();

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
  let mut registration_api = false;

  if auth {
    if let Authentication::Account {
      registration_allowed,
      ..
    } = &CONFIG.authentication
    {
      registration_api = *registration_allowed;
    }

    _ = AUTH.set(AuthSessionManager::create().await);
  }

  let admin_api = request_admin_passwd();

  let mut server = HttpServer::new(move || {
    let mut app = App::new()
      .service(http::index)
      // .route("/chat", web::get().to(chat::chat))
      .service(http::challenge)
      .service(http::me);

    let auth = !matches!(CONFIG.authentication, Authentication::OpenToAll);

    if auth {
      app = app.service(auth::auth);
    }

    if admin_api {
      app = app
        .service(admin::verify)
        .service(admin::list)
        .service(admin::create)
        .service(admin::create_token)
        .service(admin::delete);
    }

    if registration_api {
      app = app.service(auth::register);
    }

    app
  })
  .workers(available_parallelism()?.get());

  for (host, port) in &CONFIG.binds {
    chalk.blue().println(&format!("Binding to {host}:{port}"));
    server = server.bind((host as &str, *port))?
  }

  info!("Server is starting");

  let out = server.run().await;

  if let Err(e) = &out {
    error!("Server exited in an error state.");
    error!("{e}");
  }

  warn!("Ctrl+C detected (most probably). Starting shutdown procedure. This might take a while.");
  info!(
    "{}",
    chalk
      .red()
      .bold()
      .string(&"Please DO NOT use Ctrl+C to terminate. It will lead to data corruption!")
  );

  out
}

// Rquests admin password if needed and outputs if
// you can enable admin urls
fn request_admin_passwd() -> bool {
  if let Some(x) = &CONFIG.admin_pass_hash {
    let hash = x as &str;

    let passwd;

    if let Ok(x) = env::var("AHQAI_ADMIN_PASSWORD") {
      passwd = x;
    } else {
      warn!("----------------");
      warn!("THE GIVEN SERVER IS PROTECTED BY SERVER ADMIN PASSWORD");
      warn!("BUT THE `AHQAI_ADMIN_PASSWORD` VARIABLE WAS NOT FOUND");
      warn!("IN THE CURRENT SERVER ENVIRONMENT. REQUESTING MANUAL ENTRY");
      warn!("----------------");
      warn!("");

      passwd = rpassword::prompt_password("Enter your administrator password : ")
        .expect("Unable to read your password");
    }

    if !verify_server_pass(&passwd, hash).unwrap_or(false) {
      panic!("Invalid Password was provided")
    }

    warn!("");
    warn!("----------------");
    warn!("SERVER ADMIN PASSWORD AUTH SUCCESSFUL");
    warn!("SERVER WILL START UP NOW");
    warn!("----------------");
    warn!("");

    REAL_ADMIN_PASSWORD
      .set(SecretString::from(passwd))
      .expect("Impossible Error");

    return true;
  }

  false
}
