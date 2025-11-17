use cursive::{
  Cursive, View,
  align::HAlign,
  theme::{Effect, Style},
  view::{Nameable, Resizable},
  views::{Button, Dialog, DummyView, EditView, LinearLayout, NamedView, ScrollView, TextView},
};

use crate::{
  auth::argon::{self, server::verify_server_pass},
  structs::{
    Config,
    db::{AuthDbConfig, CacheConfig, TlsConfig},
  },
  ui::Ptr,
};

pub fn db_page(s: Ptr<Config>) -> NamedView<impl View> {
  let mut layout = LinearLayout::vertical();

  render(&mut layout, s);

  ScrollView::new(layout.with_name("dbconf"))
    .show_scrollbars(true)
    .with_name("㏈ Database")
}

pub fn render(layout: &mut LinearLayout, s: Ptr<Config>) {
  layout.clear();

  layout.add_child(DummyView::new().fixed_height(1));

  layout.add_child(
    TextView::new("All the secrets are safely encrypted using your server administrator password")
      .style(Style::from(Effect::Dim))
      .h_align(HAlign::Center),
  );

  layout.add_child(DummyView::new().fixed_height(2));

  layout.add_child(
    TextView::new("㏈ Authentication Store Database").style(Style::from(Effect::Underline)),
  );

  match &s.database.authdb {
    AuthDbConfig::Moka { .. } => {
      layout.add_child(authdb_moka());
    }
    AuthDbConfig::Mongodb { .. } => {
      layout.add_child(authdb_mongodb());
    }
    AuthDbConfig::Tikv {
      timeout_secs,
      tls_config,
      ..
    } => {
      layout.add_child(authdb_tikv(*timeout_secs, tls_config.is_some()));
    }
  }

  layout.add_child(DummyView::new().fixed_height(1));

  layout.add_child(TextView::new("㏈ Cache Store Database").style(Style::from(Effect::Underline)));

  match &s.database.cache {
    CacheConfig::Redis { .. } => {
      layout.add_child(cache_redis());
    }
    CacheConfig::Moka => {
      layout.add_child(cache_moka());
    }
  }
}

fn select_db(x: &mut Cursive) {
  x.add_layer(
    Dialog::around(ScrollView::new(
      LinearLayout::vertical()
        .child(TextView::new("Standard DBs").style(Style::from(Effect::Underline)))
        .child(
          LinearLayout::horizontal()
            .child(
              Button::new_raw("TiKV (Recommended)", |x| {
                x.pop_layer();
                let mut state = x.user_data::<Ptr<Config>>().unwrap().clone();
                state.database.authdb = AuthDbConfig::Tikv {
                  endpoints: Default::default(),
                  tls_config: Default::default(),
                  timeout_secs: 30,
                };

                x.call_on_name("dbconf", move |x: &mut LinearLayout| {
                  render(x, state);
                });
              })
              .fixed_width(18),
            )
            .child(DummyView::new().full_width()),
        )
        .child(
          LinearLayout::horizontal()
            .child(
              Button::new_raw("MongoDB (Simple Setup)", |x| {
                x.pop_layer();
                let mut state = x.user_data::<Ptr<Config>>().unwrap().clone();
                state.database.authdb = AuthDbConfig::Mongodb {
                  url: Default::default(),
                };

                x.call_on_name("dbconf", move |x: &mut LinearLayout| {
                  render(x, state);
                });
              })
              .fixed_width(22),
            )
            .child(DummyView::new().full_width()),
        )
        .child(DummyView::new().fixed_height(1))
        .child(TextView::new("Fake DBs").style(Style::from(Effect::Underline)))
        .child(TextView::new("These databases have no persistence nor are suitable for production environment. The database are stored in RAM temporarily only for testing purposes. Never use it in production.").style(Style::from(Effect::Dim)))
        .child(DummyView::new().fixed_height(1))
        .child(
          LinearLayout::horizontal()
            .child(
              Button::new_raw("Moka", |x| {
                x.pop_layer();
                let mut state = x.user_data::<Ptr<Config>>().unwrap().clone();
                state.database.authdb = AuthDbConfig::Moka {};

                x.call_on_name("dbconf", move |x: &mut LinearLayout| {
                  render(x, state);
                });
              })
              .fixed_width(4),
            )
            .child(DummyView::new().full_width()),
        ),
    ))
    .dismiss_button("Cancel")
    .title("Choose your Auth Database")
    .min_width(32)
    .max_width(48),
  );
}

fn select_cache_db(x: &mut Cursive) {
  x.add_layer(
    Dialog::around(ScrollView::new(
      LinearLayout::vertical()
        .child(
          LinearLayout::horizontal()
            .child(
              Button::new_raw("Redis (Recommended)", |x| {
                x.pop_layer();
                let mut state = x.user_data::<Ptr<Config>>().unwrap().clone();
                state.database.cache = CacheConfig::Redis {
                  url: Default::default(),
                };

                x.call_on_name("dbconf", move |x: &mut LinearLayout| {
                  render(x, state);
                });
              })
              .fixed_width(19),
            )
            .child(DummyView::new().full_width()),
        )
        .child(
          LinearLayout::horizontal()
            .child(
              Button::new_raw("Moka (In-RAM; Best for single server setup)", |x| {
                x.pop_layer();
                let mut state = x.user_data::<Ptr<Config>>().unwrap().clone();
                state.database.cache = CacheConfig::Moka;

                x.call_on_name("dbconf", move |x: &mut LinearLayout| {
                  render(x, state);
                });
              })
              .fixed_width(43),
            )
            .child(DummyView::new().full_width()),
        ),
    ))
    .dismiss_button("Cancel")
    .title("Choose your Auth Database")
    .min_width(32)
    .max_width(48),
  );
}

pub fn authdb_moka() -> impl View {
  LinearLayout::horizontal()
    .child(TextView::new("㏈ Database").full_width())
    .child(Button::new_raw("Moka", |x| {
      select_db(x);
    }))
}

pub fn authdb_mongodb() -> impl View {
  let db = LinearLayout::horizontal()
    .child(TextView::new("㏈ Database").full_width())
    .child(Button::new_raw("Mongodb", |x| {
      select_db(x);
    }));

  let url = LinearLayout::horizontal()
    .child(TextView::new("🖳 Mongodb URL").full_width())
    .child(Button::new_raw("Set ↗", |x| {
      set_url(x, Db::Mongo);
    }));

  LinearLayout::vertical().child(db).child(url)
}

pub fn cache_redis() -> impl View {
  let db = LinearLayout::horizontal()
    .child(TextView::new("㏈ Database").full_width())
    .child(Button::new_raw("Redis", |x| {
      select_cache_db(x);
    }));

  let url = LinearLayout::horizontal()
    .child(TextView::new("🖳 Redis URL").full_width())
    .child(Button::new_raw("Set ↗", |x| {
      set_url(x, Db::Redis);
    }));

  LinearLayout::vertical().child(db).child(url)
}

pub fn cache_moka() -> impl View {
  let db = LinearLayout::horizontal()
    .child(TextView::new("㏈ Database").full_width())
    .child(Button::new_raw("Moka", |x| {
      select_cache_db(x);
    }));

  LinearLayout::vertical().child(db)
}

enum Db {
  Mongo,
  Redis,
}

fn set_url(x: &mut Cursive, db: Db) {
  x.add_layer(
    Dialog::around(
      LinearLayout::vertical()
        .child(TextView::new("Enter your server admin password"))
        .child(EditView::new().secret().with_name("serverpass"))
        .child(TextView::new("Enter the new url"))
        .child(EditView::new().with_name("url")),
    )
    .button("Set", move |x| {
      let pass = x
        .call_on_name("serverpass", |x: &mut EditView| x.get_content())
        .unwrap();
      let given_url = x
        .call_on_name("url", |x: &mut EditView| x.get_content())
        .unwrap();

      let mut user = x.user_data::<Ptr<Config>>().unwrap().clone();

      if !verify_server_pass(&pass, user.admin_pass_hash.as_ref().unwrap()).unwrap_or(false) {
        x.add_layer(Dialog::around(TextView::new("Invalid password")).dismiss_button("Ok"));
        return;
      }

      match db {
        Db::Mongo => {
          let AuthDbConfig::Mongodb { url } = &mut user.database.authdb else {
            unreachable!()
          };

          *url = argon::encrypt_with_key(&pass, &given_url).into_boxed_str();
        }
        Db::Redis => {
          let CacheConfig::Redis { url } = &mut user.database.cache else {
            unreachable!()
          };

          *url = argon::encrypt_with_key(&pass, &given_url).into_boxed_str();
        }
      }

      x.pop_layer();
    })
    .dismiss_button("Cancel"),
  );
}

enum CallNext {
  Endpoints,
  TLSConf,
}

fn get_admin_pass(x: &mut Cursive, tocallnext: CallNext) {
  x.add_layer(
    Dialog::around(
      LinearLayout::vertical()
        .child(TextView::new("Please enter your administrator password"))
        .child(EditView::new().secret().with_name("admin_pass")),
    )
    .title("Authentication Required")
    .button("Continue", |x| {
      let pass = x
        .call_on_name("admin_pass", |x: &mut EditView| x.get_content())
        .unwrap();
      let hash: &str = x
        .user_data::<Ptr<Config>>()
        .unwrap()
        .admin_pass_hash
        .as_ref()
        .unwrap();

      if !verify_server_pass(&pass, hash).unwrap_or(false) {
        x.add_layer(Dialog::around(TextView::new("Invalid Password")).dismiss_button("Okay"));
        return;
      }

      let password = pass.to_string();
    })
    .dismiss_button("Cancel"),
  );
}

fn update_tls(x: &mut Cursive) {
  x.add_layer(
    Dialog::around(
      LinearLayout::vertical()
        .child(TextView::new("Select the state"))
        .child(
          LinearLayout::horizontal()
            .child(Button::new_raw("Enabled", |x| {
              x.pop_layer();

              let mut state = x.user_data::<Ptr<Config>>().unwrap().clone();

              let AuthDbConfig::Tikv { tls_config, .. } = &mut state.database.authdb else {
                unreachable!()
              };

              *tls_config = Some(TlsConfig::default());

              x.call_on_name("dbconf", move |x: &mut LinearLayout| {
                render(x, state);
              });
            }))
            .child(DummyView::new().full_width()),
        )
        .child(
          LinearLayout::horizontal()
            .child(Button::new_raw("Disabled", |x| {
              x.pop_layer();

              let mut state = x.user_data::<Ptr<Config>>().unwrap().clone();

              let AuthDbConfig::Tikv { tls_config, .. } = &mut state.database.authdb else {
                unreachable!()
              };

              *tls_config = None;

              x.call_on_name("dbconf", move |x: &mut LinearLayout| {
                render(x, state);
              });
            }))
            .child(DummyView::new().full_width()),
        ),
    )
    .dismiss_button("Cancel")
    .min_width(32)
    .max_width(64),
  );
}

pub fn authdb_tikv(timeout: u64, tls_enabled: bool) -> impl View {
  let db = LinearLayout::horizontal()
    .child(TextView::new("㏈ Database").full_width())
    .child(Button::new_raw("TiKV", |x| {
      select_db(x);
    }));

  let url = LinearLayout::horizontal()
    .child(TextView::new("🖳 TiPD Endpoints").full_width())
    .child(Button::new_raw("Configure ↗", |x| {
      get_admin_pass(x, CallNext::Endpoints);
    }));

  let tls = LinearLayout::horizontal()
    .child(TextView::new("🖧 TLS").full_width())
    .child(Button::new_raw(
      if tls_enabled {
        "Enabled ↗"
      } else {
        "Disabled ↗"
      },
      |x| {
        update_tls(x);
      },
    ));

  let tls_conf = LinearLayout::horizontal()
    .child(TextView::new("🖥 TLS Settings").full_width())
    .child(Button::new_raw("Configure ↗", |x| {
      get_admin_pass(x, CallNext::TLSConf);
    }));

  let timeout = LinearLayout::horizontal()
    .child(TextView::new("⊗ Timeout (in seconds)").full_width())
    .child(
      Button::new_raw(format!("<{timeout}>"), |x| {
        x.add_layer(
          Dialog::around(EditView::new().on_edit(|x, i, _| {
            let mut state = x.user_data::<Ptr<Config>>().unwrap().clone();

            if let Ok(data) = i.parse::<u64>() {
              let AuthDbConfig::Tikv { timeout_secs, .. } = &mut state.database.authdb else {
                unreachable!()
              };

              *timeout_secs = data;

              x.call_on_name("tikv_timeout", move |btn: &mut Button| {
                btn.set_label_raw(format!("<{data}>"));
              });
            }
          }))
          .dismiss_button("Okay")
          .title("Enter new timeout")
          .min_width(32)
          .max_width(48),
        );
      })
      .with_name("tikv_timeout"),
    );

  let mut out = LinearLayout::vertical()
    .child(db)
    .child(url)
    .child(timeout)
    .child(DummyView::new().fixed_height(1))
    .child(TextView::new("🖧 TLS").style(Style::from(Effect::Underline)))
    .child(tls);

  if tls_enabled {
    out = out.child(tls_conf);
  }

  out
}
