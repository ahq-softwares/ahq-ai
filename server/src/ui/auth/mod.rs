use cursive::{
  Cursive,
  view::Nameable,
  views::{LinearLayout, NamedView, ScrollView},
};

use crate::{
  structs::{Authentication, Config},
  ui::{Ptr, lazy::OnVisible},
};

mod open;
mod token;

pub fn auth_page(
  siv: &Cursive,
) -> NamedView<
  OnVisible<NamedView<ScrollView<NamedView<LinearLayout>>>, impl Fn(&mut Cursive) + 'static>,
> {
  let layout = LinearLayout::vertical().with_name("authpage");

  OnVisible::new(
    ScrollView::new(layout)
      .show_scrollbars(true)
      .with_name("⚒ Authentication"),
    siv,
    |x: &mut Cursive| {
      let state: &mut Ptr<Config> = x.user_data().unwrap();

      let auth = state.authentication.clone();

      _ = x.call_on_name("authpage", |layout: &mut LinearLayout| {
        layout.clear();

        match auth {
          Authentication::OpenToAll => open::render(layout),
          Authentication::TokenBased => token::render(layout),
          _ => {}
        }
      });
    },
  )
  .with_name("⚒ Authentication")
}
