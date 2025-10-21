use std::{
  fs,
  ops::{Deref, DerefMut},
  sync::LazyLock,
  time::{SystemTime, UNIX_EPOCH},
};

use cursive::{
  Cursive, CursiveExt,
  align::Align,
  theme::{Effect, PaletteColor, Style},
  view::{Nameable, Resizable},
  views::{Button, Dialog, DummyView, LinearLayout, ScrollView, TextView},
};
use cursive_tabs::TabPanel;
use serde_json::to_string_pretty;
use tokio::runtime::{Builder, Runtime};

use crate::structs::Config;

mod bind;
mod ollama;

pub static ASYNC: LazyLock<Runtime> = LazyLock::new(|| {
  Builder::new_current_thread()
    .enable_all()
    .build()
    .expect("Unable to build async runtime")
});

fn general(l: &mut LinearLayout) {
  l.add_child(
    TextView::new("Welcome to Server Configuration")
      .align(Align::center())
      .style(Style::merge(&[PaletteColor::Highlight.into()]))
      .fixed_height(3),
  );

  l.add_child(
    TextView::new(format!("AHQ AI Server v{}", env!("CARGO_PKG_VERSION")))
      .align(Align::top_right())
      .style(Style::merge(&[Effect::Dim.into()]))
      .fixed_height(2),
  );

  l.add_child(TextView::new("Quick Guide").style(Style::merge(&[Effect::Underline.into()])));

  l.add_child(TextView::new("» Use ← ↑ → ↓ to navigate"));

  l.add_child(TextView::new(
    "» Press <Enter> key to interact with buttons",
  ));

  l.add_child(TextView::new(
    "» <q> key, <Ctrl+C> or going to <Save> tab updates the config file",
  ));

  l.add_child(DummyView::new().fixed_height(1).full_width());

  l.add_child(TextView::new("General Settings").style(Style::merge(&[Effect::Underline.into()])));
}

pub fn ui() {
  let mut config = ASYNC.block_on(async { Config::new_or_default().await });

  let initial_config = config.clone();

  let mut siv = Cursive::new();

  let c_ = Ptr(&mut config);

  siv.set_user_data(c_.clone());
  siv.set_global_callback('q', |x| x.quit());

  let mut tabs = TabPanel::new();

  let mut gene = LinearLayout::vertical();

  general(&mut gene);
  gene.add_child(binds(c_.clone()));

  tabs.add_tab(
    ScrollView::new(gene)
      .show_scrollbars(true)
      .with_name("☸ General"),
  );

  tabs.add_tab(ollama::ollama_page());

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
