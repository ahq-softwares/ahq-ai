use std::{
  sync::{Arc, Mutex},
  thread::spawn,
};

use cursive::{
  Cursive,
  theme::{Effect, Style},
  view::{Nameable, Resizable},
  views::{Button, Dialog, DummyView, EditView, LinearLayout, ScrollView, TextView},
};
use zeroize::Zeroizing;

use crate::{
  auth::argon::{decrypt_with_key, encrypt_with_key},
  structs::{Config, db::AuthDbConfig},
  ui::Ptr,
};

type DecryptedUrls = Arc<Mutex<Zeroizing<Vec<Box<str>>>>>;

pub fn url(pass: String, x: &mut Cursive) {
  x.add_layer(Dialog::around(TextView::new("Decrypting...")));

  let sink = x.cb_sink().clone();
  let conf = x.user_data::<Ptr<Config>>().unwrap().clone();
  let pass: Arc<str> = Arc::from(pass);

  spawn(move || {
    let AuthDbConfig::Tikv { endpoints, .. } = &conf.database.authdb else {
      unreachable!();
    };

    let decrypted = endpoints
      .iter()
      .map(|x| decrypt_with_key(&pass, &x as &str).into_boxed_str())
      .collect::<Vec<_>>();
    let decrypted = Arc::new(Mutex::new(Zeroizing::new(decrypted)));

    _ = sink.send(Box::new(move |x| {
      x.pop_layer();
      render(x, pass.clone(), decrypted);
    }));
  });
}

pub fn render(x: &mut Cursive, pass: Arc<str>, decrypted: DecryptedUrls) {
  let mut layout = LinearLayout::vertical();

  render_ui(&mut layout, decrypted.clone());

  let decr = decrypted.clone();

  x.add_layer(
    Dialog::around(ScrollView::new(layout.with_name("tikv_hostnames")))
      .title("Configure hostnames")
      .button("New", move |x| {
        add_new(x, pass.clone(), decr.clone());
      })
      .dismiss_button("Back")
      .full_screen(),
  )
}

fn add_new(x: &mut Cursive, pass: Arc<str>, decr: DecryptedUrls) {
  x.add_layer(
    Dialog::around(
      LinearLayout::vertical()
        .child(TextView::new("Enter the hostname"))
        .child(EditView::new().with_name("hostname")),
    )
    .button("Add", move |x| {
      let hostname = x
        .call_on_name("hostname", |x: &mut EditView| x.get_content())
        .unwrap();

      let encrypted = encrypt_with_key(&pass, &hostname);

      let conf = x.user_data::<Ptr<Config>>().unwrap();

      let AuthDbConfig::Tikv { endpoints, .. } = &mut conf.database.authdb else {
        unreachable!();
      };

      let mut vect = endpoints.to_vec();
      vect.push(encrypted.clone().into_boxed_str());

      *endpoints = vect.into_boxed_slice();

      decr
        .lock()
        .map_or_else(|x| x.into_inner(), |x| x)
        .push(hostname.to_string().into_boxed_str());

      let decr = decr.clone();

      x.pop_layer();

      x.call_on_name("tikv_hostnames", move |layout: &mut LinearLayout| {
        render_ui(layout, decr.clone());
      });
    })
    .dismiss_button("Cancel")
    .title("Add Hostname")
    .min_width(32)
    .max_width(64),
  );
}

pub fn render_ui(layout: &mut LinearLayout, decrypted: DecryptedUrls) {
  layout.clear();

  layout.add_child(
    LinearLayout::horizontal()
      .child(
        TextView::new("Model ID")
          .style(Style::merge(&[Effect::Dim.into()]))
          .full_width(),
      )
      .child(
        TextView::new("Actions")
          .style(Style::merge(&[Effect::Dim.into()]))
          .fixed_width(10),
      ),
  );

  let decr2 = decrypted.clone();

  decrypted
    .lock()
    .map_or_else(|e| e.into_inner(), |v| v)
    .iter()
    .enumerate()
    .for_each(move |(index, x)| {
      let decr2 = decr2.clone();

      layout.add_child(
        LinearLayout::horizontal()
          .child(TextView::new(x as &str).full_width())
          .child(
            Button::new_raw("Remove", move |x| {
              let decr = decr2.clone();
              let conf = x.user_data::<Ptr<Config>>().unwrap();

              let AuthDbConfig::Tikv { endpoints, .. } = &mut conf.database.authdb else {
                unreachable!();
              };

              let mut vect = endpoints.to_vec();

              vect.remove(index);

              decr
                .lock()
                .map_or_else(|x| x.into_inner(), |x| x)
                .remove(index);

              *endpoints = vect.into_boxed_slice();

              x.call_on_name("tikv_hostnames", move |layout: &mut LinearLayout| {
                render_ui(layout, decr.clone());
              });
            })
            .fixed_width(6),
          )
          .child(DummyView::new().fixed_width(4)),
      );
    });
}
