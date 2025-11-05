use cursive::{
  align::Align,
  theme::{Effect, Style},
  view::{Nameable, Resizable},
  views::{Button, Dialog, DummyView, LinearLayout, SelectView, TextView},
};

use crate::{
  structs::{Authentication, Config},
  ui::Ptr,
};

pub fn render(l: &mut LinearLayout, can_register: bool) {
  l.add_child(
    LinearLayout::horizontal()
      .child(TextView::new("⚒ Authentication Type").full_width())
      .child(Button::new_raw("Token (TokenBased)", |_| {})),
  );

  l.add_child(
    LinearLayout::horizontal()
      .child(TextView::new("⚒ Self Registration Allowed").full_width())
      .child(
        Button::new_raw(if can_register { "[Yes]" } else { "[No]" }, |x| {
          x.add_layer(
            Dialog::around(
              SelectView::new()
                .item("Yes", true)
                .item("No", false)
                .on_submit(|x, val| {
                  let state: &mut Ptr<Config> = x.user_data().unwrap();

                  if let Authentication::Account {
                    registration_allowed,
                    ..
                  } = &mut state.authentication
                  {
                    *registration_allowed = *val;
                  }

                  let val_f = *val;

                  x.call_on_name("user_reg_allowed", move |x: &mut Button| {
                    x.set_label_raw(if val_f { "[Yes]" } else { "[No]" });
                  });

                  x.pop_layer();
                }),
            )
            .title("Self Registration"),
          );
        })
        .with_name("user_reg_allowed"),
      ),
  );

  l.add_child(
    LinearLayout::horizontal()
      .child(TextView::new("⚒ Account Manager").full_width())
      .child(Button::new_raw("Use Admin API ↗", |_| {})),
  );

  l.add_child(DummyView::new().fixed_height(2));

  l.add_child(
    TextView::new("User Auth")
      .align(Align::center())
      .style(Style::merge(&[
        Effect::Dim.into(),
        Effect::Underline.into(),
      ])),
  );

  l.add_child(
    TextView::new("The Client application will be needed to provide a userid and password. This is the recommended authentication type for internet or LAN servers.")
  );
}
