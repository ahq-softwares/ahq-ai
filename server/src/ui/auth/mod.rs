use cursive::{
  Cursive,
  view::Nameable,
  views::{LinearLayout, NamedView, ScrollView},
};

use crate::{
  structs::{Authentication, Config},
  ui::{Ptr, lazy::OnAuthStateUpdate},
};

mod open;
mod token;
mod user;

#[allow(clippy::type_complexity)]
pub fn auth_page(
  siv: &mut Cursive,
) -> NamedView<
  OnAuthStateUpdate<NamedView<ScrollView<NamedView<LinearLayout>>>, impl Fn(&mut Cursive) + 'static>,
> {
  let layout = LinearLayout::vertical().with_name("authpage");

  OnAuthStateUpdate::new(
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
          Authentication::Account {
            registration_allowed,
            max_users,
          } => user::render(layout, registration_allowed, max_users),
        }
      });
    },
  )
  .with_name("⚒ Authentication")
}
