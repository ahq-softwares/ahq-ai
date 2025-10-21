use std::{
  env::home_dir,
  fs,
  ops::{Deref, DerefMut},
  sync::LazyLock,
  time::{SystemTime, UNIX_EPOCH},
};

use cursive::{
  Cursive, CursiveExt,
  align::Align,
  theme::{Effect, PaletteColor, Style, Theme},
  view::{Nameable, Resizable},
  views::{Button, Dialog, DummyView, LinearLayout, ScrollView, SelectView, TextView},
};
use cursive_tabs::TabPanel;
use serde_json::to_string_pretty;
use tokio::runtime::{Builder, Runtime};

use crate::structs::{Authentication, Config};

mod auth;
mod bind;
mod ollama;

pub(crate) mod lazy;

pub static ASYNC: LazyLock<Runtime> = LazyLock::new(|| {
  Builder::new_current_thread()
    .enable_all()
    .build()
    .expect("Unable to build async runtime")
});

fn general(l: &mut LinearLayout, c_: Ptr<Config>) {
  l.add_child(
    TextView::new("Welcome to AHQ-AI Server Configuration")
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
    "» You can also use mouse to interact with buttons or tabs",
  ));

  l.add_child(TextView::new(
    "» You can also scroll with the mouse scrollbar",
  ));

  l.add_child(TextView::new(
    "» <q> key, <Ctrl+C> or going to <Save> tab updates the config file",
  ));

  l.add_child(DummyView::new().fixed_height(1).full_width());

  l.add_child(TextView::new("General Settings").style(Style::merge(&[Effect::Underline.into()])));

  l.add_child(binds(c_.clone()));

  l.add_child(
    LinearLayout::horizontal()
      .child(TextView::new("⚒ Authentication Type").full_width())
      .child(
        Button::new_raw(
          format!(
            "{} ↗",
            match c_.authentication {
              Authentication::OpenToAll => "No Auth",
              Authentication::TokenBased => "Token",
              Authentication::AccountAuthentication { .. } => "Accounts",
            }
          ),
          move |x| {
            x.add_layer(
              Dialog::around(
                SelectView::new()
                  .item("No Auth (OpenToAll)", 0u8)
                  .item("Token (TokenBased)", 1u8)
                  .item("Accounts (AccountAuthentication)", 2u8)
                  .on_submit(|x, bit| {
                    let c_: &mut Ptr<Config> = x.user_data().unwrap();

                    c_.authentication = match bit {
                      0 => Authentication::OpenToAll,
                      1 => Authentication::TokenBased,
                      2 => Authentication::AccountAuthentication {
                        registration_allowed: true,
                        max_users: None,
                      },
                      _ => unreachable!(),
                    };

                    let label = format!(
                      "{} ↗",
                      match c_.authentication {
                        Authentication::OpenToAll => "No Auth",
                        Authentication::TokenBased => "Token",
                        Authentication::AccountAuthentication { .. } => "Accounts",
                      }
                    );

                    x.call_on_name("auth_type", move |x: &mut Button| {
                      x.set_label_raw(label);
                    });

                    x.pop_layer();
                  })
                  .with_name("themeselect"),
              )
              .title("Authentication Type")
              .dismiss_button("Cancel"),
            );
          },
        )
        .with_name("auth_type"),
      ),
  );

  l.add_child(
    LinearLayout::horizontal()
      .child(TextView::new("🖌 TUI Theme").full_width())
      .child(Button::new_raw("Select ↗", move |x| {
        x.add_layer(
          Dialog::around(
            SelectView::new()
              .item("Default Theme", 0u8)
              .item("Monochrome Theme", 1u8)
              .on_submit(|x, bit| {
                x.set_theme(match bit {
                  0 => Theme::retro(),
                  1 => Theme::terminal_default(),
                  _ => unreachable!(),
                });

                x.call_on_name("themeselect", |x: &mut SelectView| {
                  x.set_selection(*bit as usize)
                });

                x.pop_layer();

                if let Some(mut home) = home_dir() {
                  home.push(".ahqaiservertheme");

                  _ = fs::write(&home, vec![*bit])
                }
              })
              .with_name("themeselect"),
          )
          .title("Select Theme")
          .dismiss_button("Cancel"),
        );
      })),
  );
}

pub fn ui() {
  let mut config = ASYNC.block_on(async { Config::new_or_default().await });

  let initial_config = config.clone();

  let mut siv = Cursive::new();

  let c_ = Ptr(&mut config);

  let prompt = config.binds.is_empty();

  siv.set_theme(Theme::retro());

  if let Some(mut home) = home_dir() {
    home.push(".ahqaiservertheme");

    if let Ok(x) = fs::read(&home) {
      let first_bit = &x[0];

      match *first_bit {
        0 => {}
        1 => siv.set_theme(Theme::terminal_default()),
        _ => {}
      }
    }
  }

  siv.set_user_data(c_.clone());
  siv.set_global_callback('q', |x| x.quit());

  let mut tabs = TabPanel::new();

  let mut gene = LinearLayout::vertical();

  general(&mut gene, c_.clone());

  tabs.add_tab(
    ScrollView::new(gene)
      .show_scrollbars(true)
      .with_name("䷸ General"),
  );

  tabs.add_tab(ollama::ollama_page(c_.clone()));

  tabs.add_tab(auth::auth_page(&siv));

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

  _ = tabs.set_active_tab("䷸ General");

  siv.add_layer(
    Dialog::around(tabs.with_name("tabs"))
      .title("AHQ-AI Server Configuration Utility")
      .full_screen(),
  );

  if prompt {
    siv.add_layer(
      Dialog::around(TextView::new(
        "Please set up hostnames and ports under `☸ General`!",
      ))
      .title("Important")
      .dismiss_button("Ok"),
    );
  }

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
