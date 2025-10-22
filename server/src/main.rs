#![feature(duration_constructors)]
use std::{env::args, panic};

mod server;
mod ui;

pub mod auth;
pub(crate) mod structs;

fn main() {
  panic::set_hook(Box::new(|x| {
    if let Some(x) = x.payload_as_str() {
      println!("ERR: An Error Occured");
      println!("ERR: {x}");
    } else {
      println!("ERR: Unknown");
    }
  }));

  let mut args = args();
  _ = args.next();

  let mut config_ui = false;

  args.for_each(|x| {
    if &x == "config" {
      config_ui = true;
    } else {
      // Error out & Crash app
      panic!("Unknown arg: {x:?}");
    }
  });

  if config_ui {
    ui::ui();
  } else {
    server::launch();
  }
}
