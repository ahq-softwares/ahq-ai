use std::{sync::Arc, thread::spawn};

use cursive::{
  Cursive,
  align::HAlign,
  theme::{Effect, Style},
  view::{Nameable, Resizable},
  views::{Button, Dialog, DummyView, EditView, LinearLayout, ScrollView, TextView},
};
use zeroize::{Zeroize, ZeroizeOnDrop, Zeroizing};

use crate::{
  auth::argon::{decrypt_with_key, encrypt_with_key},
  structs::{Config, db::AuthDbConfig},
  ui::Ptr,
};

#[derive(Debug, Zeroize, ZeroizeOnDrop)]
struct Tls {
  ca_path: String,
  cert_path: String,
  key_path: String,
}

pub fn tls(pass: String, x: &mut Cursive) {
  x.add_layer(Dialog::around(TextView::new("Decrypting...")));

  let sink = x.cb_sink().clone();
  let conf = x.user_data::<Ptr<Config>>().unwrap().clone();
  let pass: Arc<str> = Arc::from(pass);

  spawn(move || {
    let AuthDbConfig::Tikv { tls_config, .. } = &conf.database.authdb else {
      unreachable!();
    };
    let tls = tls_config.as_ref().unwrap();

    let decrypted = Tls {
      ca_path: decrypt_with_key(&pass, &tls.ca_path),
      cert_path: decrypt_with_key(&pass, &tls.cert_path),
      key_path: decrypt_with_key(&pass, &tls.key_path),
    };
    let decrypted = Zeroizing::new(decrypted);

    _ = sink.send(Box::new(move |x| {
      x.pop_layer();
      render(x, pass.clone(), decrypted);
    }));
  });
}

fn render(x: &mut Cursive, pass: Arc<str>, decrypted: Zeroizing<Tls>) {
  let p1 = pass.clone();
  let p2 = pass.clone();
  let p3 = pass.clone();

  let layout = LinearLayout::vertical()
    .child(DummyView::new().fixed_height(1))
    .child(
      TextView::new(
        "Please refer to TiKV official docs at https://tikv.org/docs/4.0/tasks/configure/security/",
      )
      .h_align(HAlign::Center),
    )
    .child(DummyView::new().fixed_height(1))
    .child(TextView::new("CA Cert").style(Style::from(Effect::Underline)))
    .child(
      TextView::new(
        "The path to the file that contains the PEM encoding of the server’s CA certificates.",
      )
      .style(Style::from(Effect::Dim)),
    )
    .child(DummyView::new().fixed_height(1))
    .child(TextView::new(format!("Currently \"{}\"", decrypted.ca_path)).with_name("ca_path"))
    .child(
      LinearLayout::horizontal()
        .child(DummyView::new().full_width())
        .child(
          Button::new_raw("Change ↗", move |x| {
            set(x, p1.clone(), ToChange::CaPath);
          })
          .fixed_width(6),
        ),
    )
    .child(DummyView::new().fixed_height(1))
    .child(TextView::new("Cert Path").style(Style::from(Effect::Underline)))
    .child(
      TextView::new(
        "The path to the file that contains the PEM encoding of the server’s certificate chain.",
      )
      .style(Style::from(Effect::Dim)),
    )
    .child(DummyView::new().fixed_height(1))
    .child(TextView::new(format!("Currently \"{}\"", decrypted.cert_path)).with_name("cert_path"))
    .child(
      LinearLayout::horizontal()
        .child(DummyView::new().full_width())
        .child(
          Button::new_raw("Change ↗", move |x| {
            set(x, p2.clone(), ToChange::CertPath);
          })
          .fixed_width(6),
        ),
    )
    .child(DummyView::new().fixed_height(1))
    .child(TextView::new("Key Path").style(Style::from(Effect::Underline)))
    .child(
      TextView::new(
        "The path to the file that contains the PEM encoding of the server’s private key.",
      )
      .style(Style::from(Effect::Dim)),
    )
    .child(DummyView::new().fixed_height(1))
    .child(TextView::new(format!("Currently \"{}\"", decrypted.key_path)).with_name("key_path"))
    .child(
      LinearLayout::horizontal()
        .child(DummyView::new().full_width())
        .child(
          Button::new_raw("Change ↗", move |x| {
            set(x, p3.clone(), ToChange::KeyPath);
          })
          .fixed_width(6),
        ),
    );

  x.add_layer(
    Dialog::around(ScrollView::new(layout))
      .title("Configure TLS")
      .dismiss_button("Back")
      .full_screen(),
  )
}

enum ToChange {
  CaPath,
  CertPath,
  KeyPath,
}

fn set(x: &mut Cursive, pass: Arc<str>, ty: ToChange) {
  x.add_layer(
    Dialog::around(
      LinearLayout::vertical()
        .child(TextView::new("Enter new path"))
        .child(EditView::new().with_name("path_data")),
    )
    .title("Set Path")
    .button("Ok", move |x| {
      let pass = pass.clone();
      let path = x
        .call_on_name("path_data", |x: &mut EditView| x.get_content())
        .unwrap();

      let mut conf = x.user_data::<Ptr<Config>>().unwrap().clone();

      let AuthDbConfig::Tikv { tls_config, .. } = &mut conf.database.authdb else {
        unreachable!();
      };
      let tls = tls_config.as_mut().unwrap();

      let name = match ty {
        ToChange::CaPath => {
          tls.ca_path = encrypt_with_key(&pass, &path).into_boxed_str();
          "ca_path"
        }
        ToChange::CertPath => {
          tls.cert_path = encrypt_with_key(&pass, &path).into_boxed_str();
          "cert_path"
        }
        ToChange::KeyPath => {
          tls.key_path = encrypt_with_key(&pass, &path).into_boxed_str();
          "key_path"
        }
      };

      x.call_on_name(name, move |x: &mut TextView| {
        x.set_content(format!("Currently \"{}\"", &path as &str));
      });
    })
    .dismiss_button("Cancel")
    .min_width(32)
    .max_width(64),
  );
}
