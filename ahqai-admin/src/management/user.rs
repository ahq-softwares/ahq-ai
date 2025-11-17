use std::{
  sync::{
    Arc,
    mpsc::{Sender, channel},
  },
  thread,
  time::Duration,
};

use cursive::{
  Cursive, View,
  align::HAlign,
  theme::{Effect, Style},
  view::{Nameable, Resizable},
  views::{
    Button, Dialog, DummyView, EditView, LinearLayout, NamedView, PaddedView, ProgressBar,
    ScrollView, TextView,
  },
};

use crate::{
  ServerData,
  api::{self, copy},
};

pub fn tab() -> NamedView<impl View> {
  ScrollView::new(PaddedView::lrtb(
    1,
    1,
    0,
    1,
    LinearLayout::vertical()
      .child(
        TextView::new("± Users")
          .style(Style::from(Effect::Underline))
          .h_align(HAlign::Center),
      )
      .child(
        LinearLayout::horizontal()
          .child(TextView::new("+ Add new user").full_width())
          .child(
            Button::new_raw("[ Create ]", |s| {
              create_user(s);
            })
            .min_width(10),
          ),
      )
      .child(
        LinearLayout::horizontal()
          .child(TextView::new("- Remove user").full_width())
          .child(
            Button::new_raw("[ Remove ]", |s| {
              delete_user_or_tokens(s, false);
            })
            .min_width(10),
          ),
      )
      .child(DummyView::new().fixed_height(2))
      .child(
        TextView::new("± Tokens")
          .style(Style::from(Effect::Underline))
          .h_align(HAlign::Center),
      )
      .child(
        LinearLayout::horizontal()
          .child(TextView::new("+ Generate new token").full_width())
          .child(
            Button::new_raw("[ Generate ]", |s| {
              gen_token(s);
            })
            .min_width(12),
          ),
      )
      .child(
        LinearLayout::horizontal()
          .child(TextView::new("- Remove a token").full_width())
          .child(
            Button::new_raw("[ Revoke ]", |s| {
              delete_user_or_tokens(s, true);
            })
            .min_width(10),
          ),
      ),
  ))
  .with_name("☊ Users, Tokens")
}

fn gen_token(x: &mut Cursive) {
  let tx: Sender<()> = loading(x);
  let sink = x.cb_sink().clone();

  let state: Arc<ServerData> = x.user_data::<Arc<ServerData>>().unwrap().clone();
  thread::spawn(move || {
    let out = api::create_token(&state.url, &state.pwd);

    _ = tx.send(());

    _ = sink.send(Box::new(move |x| {
      // Remove loading bar
      x.pop_layer();

      match out {
        Ok(token) => {
          x.add_layer(
            Dialog::around(
              LinearLayout::vertical()
                .child(TextView::new(
                  "A new token has been generated! Copy the token as shown",
                ))
                .child(TextView::new(token.clone())),
            )
            .button("Copy to clipboard", move |_x| {
              #[cfg(target_os = "macos")]
              _x.add_layer(
                Dialog::around(
                  TextView::new("Please note that this is not supported in macOS environments and we do not fall back to any other methods on this os")
                ).dismiss_button("Ok")
              );

              copy::copy(&token);

              _x.add_layer(
                Dialog::around(
                  TextView::new("Successfully copied to clipboard")
                ).dismiss_button("Ok")
              );
            })
            .dismiss_button("Close"),
          );
        }
        Err(e) => {
          err(x, &e);
        }
      }
    }));
  });
}

fn delete_user_or_tokens(x: &mut Cursive, token: bool) {
  x.add_layer(
    Dialog::around(
      LinearLayout::vertical()
        .child(if token {
          TextView::new("Unique ID")
        } else {
          TextView::new("Enter the characters of the token upto the first `.`")
        })
        .child(EditView::new().with_name("uid").min_width(32)),
    )
    .title(if token {
      "Revoke a token"
    } else {
      "Remove an account"
    })
    .button("Proceed", move |x| {
      let tx = loading(x);

      let user: &mut Arc<ServerData> = x.user_data().unwrap();
      let user = user.clone();

      let unique_id = x
        .call_on_name("uid", |x: &mut EditView| x.get_content())
        .unwrap();

      let sink = x.cb_sink().clone();

      thread::spawn(move || {
        let mut uid = unique_id.as_str();

        if token {
          if let Some((uid_, _)) = unique_id.split_once(".") {
            uid = uid_;
          }
        }

        let out = api::remove_client(&user.url, &user.pwd, &uid);

        _ = tx.send(());

        _ = sink.send(Box::new(move |x| {
          // Remove loading bar
          x.pop_layer();

          match out {
            Ok(()) => {
              x.pop_layer();
              success(x);
            }
            Err(e) => {
              err(x, &e);
            }
          }
        }));
      });
    })
    .dismiss_button("Cancel"),
  );
}

fn create_user(x: &mut Cursive) {
  x.add_layer(
    Dialog::around(
      LinearLayout::vertical()
        .child(TextView::new("Unique ID"))
        .child(EditView::new().with_name("uid").min_width(32))
        .child(TextView::new("Password"))
        .child(EditView::new().secret().with_name("pwd").min_width(32)),
    )
    .title("Create Account")
    .button("Create", |x| {
      let tx = loading(x);

      let user: &mut Arc<ServerData> = x.user_data().unwrap();
      let user = user.clone();

      let unique_id = x
        .call_on_name("uid", |x: &mut EditView| x.get_content())
        .unwrap();
      let user_pass = x
        .call_on_name("pwd", |x: &mut EditView| x.get_content())
        .unwrap();

      let sink = x.cb_sink().clone();

      thread::spawn(move || {
        let out = api::create_user(&user.url, &user.pwd, &unique_id, &user_pass);

        _ = tx.send(());

        _ = sink.send(Box::new(move |x| {
          // Remove loading bar
          x.pop_layer();

          match out {
            Ok(()) => {
              x.pop_layer();
              success(x);
            }
            Err(e) => {
              err(x, &e);
            }
          }
        }));
      });
    })
    .dismiss_button("Cancel"),
  );
}

fn success(x: &mut Cursive) {
  x.add_layer(
    Dialog::around(TextView::new("Successful!"))
      .title("Successful")
      .dismiss_button("Ok"),
  );
}

fn err(x: &mut Cursive, e: &str) {
  x.add_layer(
    Dialog::around(TextView::new(e))
      .title("Something went wrong")
      .dismiss_button("Ok"),
  );
}

fn loading(x: &mut Cursive) -> Sender<()> {
  let sink = x.cb_sink().clone();
  let (tx, rx) = channel::<()>();

  x.add_layer(
    Dialog::around(
      LinearLayout::vertical()
        .child(DummyView::new().fixed_height(2))
        .child(
          ProgressBar::new()
            .with_task(move |_| {
              loop {
                for sts in (0..=100usize).chain((0..=100usize).rev()) {
                  _ = sink.send(Box::new(move |x| {
                    x.call_on_name("prog", move |x: &mut ProgressBar| {
                      x.set_value(sts);
                    });
                  }));
                  thread::sleep(Duration::from_millis(10));
                }

                match rx.try_recv() {
                  Ok(_) => break,
                  _ => {}
                }
              }
            })
            .with_label(|_, _| "Please wait...".to_string())
            .with_name("prog"),
        )
        .child(DummyView::new().fixed_height(2)),
    )
    .min_width(48),
  );

  tx
}
