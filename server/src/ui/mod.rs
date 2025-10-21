use std::{ops::{Deref, DerefMut}, sync::LazyLock};

use cursive::{Cursive, CursiveExt, With, view::{Nameable, Resizable}, views::{Button, Dialog, EditView, LinearLayout, ScrollView, TextArea, TextView}};
use cursive_tabs::{TabPanel, TabView};
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

  let mut siv = Cursive::new();

  let c_ = Ptr(&mut config);

  siv.set_window_title("AHQ AI Server Configuration");
  siv.set_user_data(c_.clone());
  siv.set_global_callback('q', |x| x.quit());

  let mut tabs = TabPanel::new();

  tabs.add_tab(
    ScrollView::new(
        LinearLayout::vertical()
          .child(port(c_.clone()))
      )
      .show_scrollbars(true)
      .with_name("Hosting")
  );

  tabs.add_tab(
    ScrollView::new(
      Button::new("Save Changes and Exit", |x| {
        x.quit();
      })
    )
      .show_scrollbars(true)
      .with_name("Exit")
  );
  
  _ = tabs.set_active_tab("Hosting");
  
  siv.add_layer(Dialog::around(tabs).full_screen());

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
      s.port.to_string(),
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

                    c.call_on_name("btn", |c: &mut Button| {
                      c.set_label_raw(x.to_string());
                    });
                  }
                }) 
            )
            .button("Done", |x| { x.pop_layer(); })
        );
      }
    )
      .with_name("btn")
      .fixed_width(5)
    )
}


// THAT LOOKS CURSED
// this framework is like react
// i cannot edit
// use down arrow key to s
// the terminal is read only
// fixed
// check dsc