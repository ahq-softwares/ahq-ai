use std::{fs, ops::{Deref, DerefMut}, sync::LazyLock, time::{SystemTime, UNIX_EPOCH}};

use cursive::{Cursive, CursiveExt, view::{Nameable, Resizable}, views::{Button, Dialog, EditView, LinearLayout, ScrollView, TextView}};
use cursive_tabs::TabPanel;
use serde_json::to_string_pretty;
use tokio::runtime::{Builder, Runtime};

use crate::structs::Config;

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

  siv.set_window_title("AHQ AI Server Configuration");
  siv.set_user_data(c_.clone());
  siv.set_global_callback('q', |x| x.quit());

  let mut tabs = TabPanel::new();

  tabs.add_tab(
    ScrollView::new(
        LinearLayout::vertical()
          .child(hostname(c_.clone()))
          .child(port(c_.clone()))
      )
      .show_scrollbars(true)
      .with_name("Hosting")
  );

  tabs.add_tab(
    ScrollView::new(
      LinearLayout::vertical()
        .child(
          Button::new_raw("Save Changes and Exit", |x| {
            x.quit();
          })
        )
        .child(
          Button::new_raw("Backup Initial Config", move |x| {
            let file = format!("./config.bak.{}.json", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs());
            fs::write(&file, to_string_pretty(&initial_config).unwrap()).unwrap();

            x.add_layer(
              Dialog::new()
                .title("Successful")
                .content(
                  TextView::new(format!("Successfully backed up initial config at {file}"))
                )
                .dismiss_button("Ok")
            );
          })
        )
    )
      .show_scrollbars(true)
      .with_name("Exit")
  );
  
  _ = tabs.set_active_tab("Hosting");
  
  siv.add_layer(Dialog::around(tabs.with_name("tabs")).full_screen());

  siv.run();

  ASYNC.block_on(async move {
    config.save_config().await.expect("Unable to save edited config");
  });
}

#[derive(Debug, Clone, Copy)]
struct Ptr<T>(*mut T);

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

fn port(s: Ptr<Config>) -> LinearLayout {
  LinearLayout::horizontal()
    .child(TextView::new("Port (0-65535)").full_width())
    .child(Button::new_raw(
      format!("<{}>", s.port),
      |x| {
        x.add_layer(
          Dialog::new()
            .title("Enter port")
            .content(
              EditView::new()
                .max_content_width(5)
                .on_edit(move |c, txt, _| {
                  if let Ok(x) = txt.parse::<u16>() {
                    c.with_user_data(|s: &mut Ptr<Config>| {
                      s.port = x;
                    });

                    c.call_on_name("port", |c: &mut Button| {
                      c.set_label(x.to_string());
                    });
                  }
                }) 
            )
            .button("Done", |x| { x.pop_layer(); })
        );
      }
    )
      .with_name("port")
      .max_width(7)
    )
}

fn hostname(s: Ptr<Config>) -> LinearLayout {
  LinearLayout::horizontal()
    .child(TextView::new("Enter the hostname to use").full_width())
    .child(Button::new_raw(
      format!("[{}]", s.host),
      |x| {
        x.add_layer(
          Dialog::new()
            .title("Enter Hostname")
            .content(
              EditView::new()
                .on_edit(move |c, txt, _| {
                  c.with_user_data(|s: &mut Ptr<Config>| {
                    s.host = txt.to_string();
                  });

                  c.call_on_name("host", |c: &mut Button| {
                    c.set_label_raw(format!("[{}]", txt),);
                  });
                }) 
            )
            .button("Done", |x| { x.pop_layer(); })
        );
      }
    )
      .with_name("host")
    )
}
