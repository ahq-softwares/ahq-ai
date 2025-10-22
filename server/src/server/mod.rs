use std::{fs as stdfs, sync::LazyLock, thread::available_parallelism};

use actix_web::{App, HttpServer};
use chalk_rs::Chalk;
use serde_json::from_str;

use crate::structs::Config;

pub mod http;

pub static CONFIG: LazyLock<Config> = LazyLock::new(|| {
  let data = stdfs::read_to_string("config.json").expect("Unable to load config");

  from_str(&data).expect("Invalid configuration file, unable to parse")
});

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

  let mut server = HttpServer::new(|| 
      App::new()
        .service(http::index)
    )
    .workers(available_parallelism()?.get());
  
  for (host, port) in &CONFIG.binds {
    chalk.blue().println(&format!("Binding to {host}:{port}"));
    server = server.bind((host as &str, *port))?
  }

  println!("----------------");
  chalk.blue().println(&"Server is ready!");
  println!("----------------");
  println!("");

  let out = server.run().await;

  if let Err(e) = &out {
    println!("----------------");
    chalk.red().bold().println(&"Server Exited in an error state");
    println!("{e}");
  }

  println!("----------------");
  chalk.reset_style().blue().bold().println(&"Starting shutdown procedure. Saving server state to disk... This might take a while");
  chalk.red().bold().println(&"Please DO NOT use Ctrl+C to terminate. It will lead to data corruption!");

  println!("Shutdown Action");

  chalk.reset_style().blue().bold().println(&"Server state has been successfully set! Closing server");

  out
}
