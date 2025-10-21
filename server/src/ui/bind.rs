use cursive::{
  align::Align,
  view::{Nameable, Resizable},
  views::{Button, Dialog, EditView, LinearLayout, NamedView, ResizedView, ScrollView, TextView},
};

use crate::{structs::Config, ui::Ptr};

pub fn bind(s: Ptr<Config>) -> ResizedView<Dialog> {
  Dialog::new()
    .title("Hosts and Ports")
    .content(ScrollView::new(gen_cnt(s.clone())).show_scrollbars(true))
    .button("Add", |x| {
      x.add_layer(add_binding());
    })
    .dismiss_button("Done")
    .full_screen()
}

fn add_binding() -> Dialog {
  Dialog::new()
    .content(
      ScrollView::new(
        LinearLayout::vertical()
          .child(TextView::new("Enter hostname"))
          .child(EditView::new().with_name("host"))
          .child(TextView::new("Enter port"))
          .child(EditView::new().max_content_width(5).with_name("port")),
      )
      .show_scrollbars(true),
    )
    .button("Add", |x| {
      let host = x
        .call_on_name("host", |x: &mut EditView| x.get_content())
        .unwrap();

      let port = x
        .call_on_name("port", |x: &mut EditView| x.get_content())
        .unwrap();

      if let Ok(port) = port.parse::<u16>() {
        let data: &mut Ptr<Config> = x.user_data().unwrap();

        data.binds.push((host.to_string(), port));

        let state = data.binds.clone();

        x.call_on_name("bindings", |l: &mut LinearLayout| {
          iterate_layout(l, &state);
        });

        x.pop_layer();
        x.add_layer(
          Dialog::around(TextView::new("Successfully updated!"))
            .title("Successful")
            .dismiss_button("Ok"),
        );
      } else {
        x.add_layer(
          Dialog::around(TextView::new("Invalid Port Provided"))
            .title("Error")
            .dismiss_button("Ok"),
        );
      }
    })
    .dismiss_button("Cancel")
}

fn gen_cnt(s: Ptr<Config>) -> NamedView<LinearLayout> {
  if s.binds.is_empty() {
    LinearLayout::vertical()
      .child(TextView::new("No bindings detected"))
      .with_name("bindings")
  } else {
    let mut layout = LinearLayout::vertical();

    iterate_layout(&mut layout, &s.binds);

    layout.with_name("bindings")
  }
}

fn iterate_layout(l: &mut LinearLayout, binds: &[(String, u16)]) {
  l.clear();

  if binds.is_empty() {
    l.add_child(TextView::new("No bindings detected"));
  } else {
    l.add_child(
      LinearLayout::horizontal()
        .child(TextView::new("SNo").fixed_width(5))
        .child(TextView::new("Hostname").full_width())
        .child(TextView::new("Port ").fixed_width(5))
        .child(TextView::new("").fixed_width(12)),
    );
  }

  binds.iter().enumerate().for_each(|(index, (host, port))| {
    l.add_child(layout_child(index, host, port));
  });
}

fn layout_child(index: usize, host: &str, port: &u16) -> LinearLayout {
  LinearLayout::horizontal()
    .child(
      TextView::new(format!("{}.", index + 1))
        .align(Align::center_left())
        .fixed_width(5),
    )
    .child(TextView::new(host).full_width())
    .child(
      TextView::new(port.to_string())
        .align(Align::center())
        .fixed_width(5),
    )
    .child(Button::new_raw("✕ Remove", move |x| {
      x.with_user_data(|x: &mut Ptr<Config>| {
        x.binds.remove(index);
      });

      let state: &mut Ptr<Config> = x.user_data().unwrap();
      let state = state.binds.clone();

      x.call_on_name("bindings", |l: &mut LinearLayout| {
        iterate_layout(l, &state);
      });
    })
    .fixed_width(12))
}
