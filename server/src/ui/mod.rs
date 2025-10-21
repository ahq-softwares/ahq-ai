use std::{
  fs,
  ops::{Deref, DerefMut},
  sync::LazyLock,
  time::{SystemTime, UNIX_EPOCH},
};

use cursive::{
  Cursive, CursiveExt,
  view::{Nameable, Resizable},
  views::{Button, Dialog, LinearLayout, ScrollView, TextView},
};
use cursive_tabs::TabPanel;
use serde_json::to_string_pretty;
use tokio::runtime::{Builder, Runtime};

use crate::structs::Config;

mod auth;
mod bind;

pub static ASYNC: LazyLock<Runtime> = LazyLock::new(|| {
  Builder::new_current_thread()
    .enable_all()
    .build()
    .expect("Unable to build async runtime")
});

pub fn ui() {
  let mut config = ASYNC.block_on(async { Config::new_or_default().await });

  let initial_config = config.clone();

  let mut siv = Cursive::new();

  let c_ = Ptr(&mut config);

  siv.set_user_data(c_.clone());
  siv.set_global_callback('q', |x| x.quit());

  let mut tabs = TabPanel::new();

  tabs.add_tab(
    ScrollView::new(LinearLayout::vertical().child(binds(c_.clone())))
      .show_scrollbars(true)
      .with_name("☸ General"),
  );

  tabs.add_tab(
    ScrollView::new(LinearLayout::vertical())
      .show_scrollbars(true)
      .with_name("🖧 Ollama"),
  );

  tabs.add_tab(
    ScrollView::new(LinearLayout::vertical())
      .show_scrollbars(true)
      .with_name("⚒ Authentication"),
  );

  tabs.add_tab(
    ScrollView::new(
      LinearLayout::vertical()
        .child(Button::new_raw("🖴 Save Changes and Exit", |x| {
          x.quit();
        }))
        .child(Button::new_raw("🖪 Backup current Config", move |x| {
          let con: &mut Ptr<Config> = x.user_data().unwrap();

          let con = unsafe { &*con.0 };

          let file = format!(
            "./config.bak.{}.json",
            SystemTime::now()
              .duration_since(UNIX_EPOCH)
              .unwrap()
              .as_secs()
          );
          fs::write(&file, to_string_pretty(con).unwrap()).unwrap();

          x.add_layer(
            Dialog::new()
              .title("Successful")
              .content(TextView::new(format!(
                "Successfully backed up initial config at {file}"
              )))
              .dismiss_button("Ok"),
          );
        }))
        .child(Button::new_raw("🖪 Backup Initial Config", move |x| {
          let file = format!(
            "./config.bak.{}.json",
            SystemTime::now()
              .duration_since(UNIX_EPOCH)
              .unwrap()
              .as_secs()
          );
          fs::write(&file, to_string_pretty(&initial_config).unwrap()).unwrap();

          x.add_layer(
            Dialog::new()
              .title("Successful")
              .content(TextView::new(format!(
                "Successfully backed up initial config at {file}"
              )))
              .dismiss_button("Ok"),
          );
        })),
    )
    .show_scrollbars(true)
    .with_name("🖫 Save"),
  );

  _ = tabs.set_active_tab("☸ General");

  siv.add_layer(
    Dialog::around(tabs.with_name("tabs"))
      .title("AHQ-AI Server Configuration Utility")
      .full_screen(),
  );
  siv.run();

  ASYNC.block_on(async move {
    config
      .save_config()
      .await
      .expect("Unable to save edited config");
  });
}

#[derive(Debug, Clone, Copy)]
pub struct Ptr<T>(*mut T);

unsafe impl<T> Send for Ptr<T> {}
unsafe impl<T> Sync for Ptr<T> {}

impl<T> Deref for Ptr<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    unsafe { &*self.0 }
  }
}

impl<T> DerefMut for Ptr<T> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    unsafe { &mut *self.0 }
  }
}

fn binds(s: Ptr<Config>) -> LinearLayout {
  LinearLayout::horizontal()
    .child(TextView::new("🖳 Hostnames and Ports").full_width())
    .child(
      Button::new_raw("View ↗", move |x| {
        x.add_layer(bind::bind(s.clone()));
      })
      .with_name("host"),
    )
}
