use serde::de::DeserializeOwned;
use tauri::{
  plugin::{PluginApi, PluginHandle},
  AppHandle, Runtime,
};

use crate::models::*;

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_ahqai);

// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned>(
  _app: &AppHandle<R>,
  api: PluginApi<R, C>,
) -> crate::Result<Ahqai<R>> {
  #[cfg(target_os = "android")]
  let handle = api.register_android_plugin("", "AhqAi")?;

  #[cfg(target_os = "ios")]
  let handle = api.register_ios_plugin(init_plugin_ahqai)?;

  Ok(Ahqai(handle))
}

pub struct Ahqai<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> Ahqai<R> {

}
