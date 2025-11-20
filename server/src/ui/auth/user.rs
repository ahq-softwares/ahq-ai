use cursive::{
  align::Align,
  theme::{Effect, Style},
  view::{Nameable, Resizable},
  views::{Button, Dialog, DummyView, EditView, LinearLayout, SelectView, TextView},
};

use crate::{
  structs::{Authentication, Config},
  ui::Ptr,
};

pub fn render(l: &mut LinearLayout, can_register: bool, memory: u32, time: u32) {
  l.add_child(
    LinearLayout::horizontal()
      .child(TextView::new("⚒ Authentication Type").full_width())
      .child(Button::new_raw("Account Authentication", |_| {})),
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

  l.add_child(DummyView::new().fixed_height(2));

  l.add_child(TextView::new("⚒ Argon2").style(Style::merge(&[Effect::Underline.into()])));

  l.add_child(
    LinearLayout::horizontal()
      .child(TextView::new("⚒ Argon2 Memory Cost").full_width())
      .child(
        Button::new_raw(format!("[{memory} MiB]"), |x| {
          x.add_layer(
            Dialog::around(
              EditView::new()
                .on_edit(|x, val, _| {
                  let state: &mut Ptr<Config> = x.user_data().unwrap();

                  if let Ok(num) = val.parse::<u32>()
                    && num > 0
                  {
                    let Authentication::Account { max_memory, .. } = &mut state.authentication
                    else {
                      unreachable!()
                    };
                    *max_memory = num;

                    x.call_on_name("ram_usage", move |x: &mut Button| {
                      x.set_label_raw(format!("[{num} MiB]"));
                    });
                  }
                })
                .on_submit(|x, _| {
                  x.pop_layer();
                }),
            )
            .dismiss_button("Done")
            .title("Memory Cost"),
          );
        })
        .with_name("ram_usage"),
      ),
  );

  l.add_child(
    LinearLayout::horizontal()
      .child(TextView::new("⚒ Argon2 Time Cost (Total Rounds)").full_width())
      .child(
        Button::new_raw(format!("<{time}>"), |x| {
          x.add_layer(
            Dialog::around(
              EditView::new()
                .on_edit(|x, val, _| {
                  let state: &mut Ptr<Config> = x.user_data().unwrap();

                  if let Ok(num) = val.parse::<u32>()
                    && num > 0
                  {
                    let Authentication::Account { time_cost, .. } = &mut state.authentication
                    else {
                      unreachable!()
                    };
                    *time_cost = num;

                    x.call_on_name("time", move |x: &mut Button| {
                      x.set_label_raw(format!("<{num}>"));
                    });
                  }
                })
                .on_submit(|x, _| {
                  x.pop_layer();
                }),
            )
            .dismiss_button("Done")
            .title("Time Cost"),
          );
        })
        .with_name("time"),
      ),
  );

  l.add_child(DummyView::new().fixed_height(2));

  l.add_child(TextView::new("Miscellaneous").style(Style::merge(&[Effect::Underline.into()])));

  l.add_child(
    LinearLayout::horizontal()
      .child(TextView::new("⚒ Account Manager").full_width())
      .child(Button::new_raw("Use the admin binary ↗", |x| {
        x.add_layer(
          Dialog::around(
            TextView::new("AHQ AI team provides a dedicated cli application to manage server users (accounts and tokens) for the whole AHQ AI server. You should look into that. Also, you may review the source code to obtain the api endpoints to manage these.")
          )
          .title("Server Administrator Portal")
          .dismiss_button("Ok")
          .min_width(32)
          .max_width(64)
        );
      })),
  );

  l.add_child(DummyView::new().fixed_height(2));

  l.add_child(
    TextView::new("About User Auth")
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
