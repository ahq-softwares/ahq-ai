use std::{
  env, fs as stdfs,
  sync::{LazyLock, OnceLock},
  thread::available_parallelism,
};

use crate::{
  auth::AuthSessionManager,
  structs::{Authentication, Config, db::DatabaseConfig},
};
use actix_web::{App, HttpServer, web};
use bcrypt::verify;
use chalk_rs::Chalk;
use secrecy::SecretString;
use serde_json::from_str;

pub mod admin;
pub mod auth;
pub mod chat;
pub mod http;

pub mod llama;

pub mod ffi;

pub static CONFIG: LazyLock<Config> = LazyLock::new(|| {
  let data = stdfs::read_to_string("config.json").expect("Unable to load config");

  from_str(&data).expect("Invalid configuration file, unable to parse")
});

pub static DBCONF: LazyLock<DatabaseConfig> = LazyLock::new(|| DatabaseConfig::get());

pub static HISTORY_LENGTH: LazyLock<usize> = LazyLock::new(|| CONFIG.ollama.msgs.saturating_mul(2));

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
      .route("/chat", web::get().to(chat::chat))
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

  println!("----------------");
  chalk.blue().println(&"Server is starting!");
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

  chalk.reset_style().blue().bold().println(
    &"Server state has been successfully set! Closing server. Session tokens will be discarded.",
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
      println!("----------------");
      println!("THE GIVEN SERVER IS PROTECTED BY SERVER ADMIN PASSWORD");
      println!("BUT THE `AHQAI_ADMIN_PASSWORD` VARIABLE WAS NOT FOUND");
      println!("IN THE CURRENT SERVER ENVIRONMENT. REQUESTING MANUAL ENTRY");
      println!("----------------");
      println!();

      passwd = rpassword::prompt_password("Enter your administrator password : ")
        .expect("Unable to read your password");
    }

    if !verify(&passwd, hash).unwrap_or(false) {
      panic!("Invalid Password was provided")
    }

    println!();
    println!("----------------");
    println!("SERVER ADMIN PASSWORD AUTH SUCCESSFUL");
    println!("SERVER WILL START UP NOW");
    println!("----------------");
    println!();

    REAL_ADMIN_PASSWORD
      .set(SecretString::from(passwd))
      .expect("Impossible Error");

    return true;
  }

  false
}
