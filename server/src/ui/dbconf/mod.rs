use cursive::{
  Cursive, View,
  align::HAlign,
  theme::{Effect, Style},
  view::{Nameable, Resizable},
  views::{Button, Dialog, DummyView, LinearLayout, NamedView, ScrollView, TextView},
};

use crate::{
  structs::{
    Config,
    db::{AuthDbConfig, CacheConfig},
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

  layout.add_child(DummyView::new().fixed_height(1));

  match &s.database.authdb {
    AuthDbConfig::Moka { .. } => {
      layout.add_child(authdb_moka());
    }
    AuthDbConfig::Mongodb { .. } => {
      layout.add_child(authdb_mongodb());
    }
    AuthDbConfig::Tikv { .. } => {
      layout.add_child(authdb_tikv());
    }
  }

  layout.add_child(DummyView::new().fixed_height(1));

  layout.add_child(TextView::new("㏈ Cache Store Database").style(Style::from(Effect::Underline)));

  layout.add_child(DummyView::new().fixed_height(1));
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
    .child(Button::new_raw("Configure ↗", |x| {}));

  LinearLayout::vertical().child(db).child(url)
}

pub fn authdb_tikv() -> impl View {
  let db = LinearLayout::horizontal()
    .child(TextView::new("Database").full_width())
    .child(Button::new_raw("TiKV", |x| {
      select_db(x);
    }));

  let url = LinearLayout::horizontal()
    .child(TextView::new("🖳 TiPD Endpoints").full_width())
    .child(Button::new_raw("Configure ↗", |x| {}));

  let tls = LinearLayout::horizontal()
    .child(TextView::new("🖳 TLS Configuration").full_width())
    .child(Button::new_raw("Configure ↗", |x| {}));

  LinearLayout::vertical().child(db).child(url).child(tls)
}
