#![allow(dead_code, unused)]

use tauri::{
  plugin::{Builder, TauriPlugin},
  Manager, Runtime,
};

pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::Ahqai;
#[cfg(mobile)]
use mobile::Ahqai;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the ahqai APIs.
pub trait AhqaiExt<R: Runtime> {
  fn ahqai(&self) -> &Ahqai<R>;
}

impl<R: Runtime, T: Manager<R>> crate::AhqaiExt<R> for T {
  fn ahqai(&self) -> &Ahqai<R> {
    self.state::<Ahqai<R>>().inner()
  }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("ahqai")
    .invoke_handler(tauri::generate_handler![])
    .setup(|app, api| {
      #[cfg(mobile)]
      let ahqai = mobile::init(app, api)?;
      #[cfg(desktop)]
      let ahqai = desktop::init(app, api)?;
      
      app.manage(ahqai);
      Ok(())
    })
    .build()
}
