use cursive::{
  theme::{Effect, Style},
  view::{Nameable, Resizable},
  views::{Button, Dialog, DummyView, EditView, LinearLayout, NamedView, ScrollView, TextView},
};

use crate::{structs::Config, ui::Ptr};

mod model;

pub fn ollama_page(s: Ptr<Config>) -> NamedView<ScrollView<LinearLayout>> {
  let mut layout = LinearLayout::vertical();

  layout.add_child(server(s.clone()));
  layout.add_child(port(s.clone()));

  layout.add_child(DummyView::new().fixed_height(1));

  layout.add_child(TextView::new("Models").style(Style::merge(&[Effect::Underline.into()])));

  let s1 = s.clone();
  let s2 = s.clone();
  layout.add_child(
    LinearLayout::horizontal()
      .child(TextView::new("⊠ Vision enabled Models").full_width())
      .child(Button::new_raw("Manage ↗", move |x| {
        x.add_layer(
          model::bind(s1.clone(), true)
        );
      })),
  );

  layout.add_child(
    LinearLayout::horizontal()
      .child(TextView::new("⊟ Text only models").full_width())
      .child(Button::new_raw("Manage ↗", move |x| {
        x.add_layer(
          model::bind(s2.clone(), false)
        );
      })),
  );

  ScrollView::new(layout)
    .show_scrollbars(true)
    .with_name("🖧 Ollama")
}

fn server(s: Ptr<Config>) -> LinearLayout {
  LinearLayout::horizontal()
    .child(TextView::new("🖥 Ollama Server Hostname").full_width())
    .child(
      Button::new_raw(format!("[{}]", &s.ollama.host), |x| {
        x.add_layer(
          Dialog::around(
            EditView::new()
              .on_edit(|x, txt, _| {
                let data: &mut Ptr<Config> = x.user_data().unwrap();

                data.ollama.host = txt.into();

                x.call_on_name("ollama_hostname", |x: &mut Button| {
                  x.set_label_raw(format!("[{txt}]"));
                });
              })
              .on_submit(|x, _| _ = x.pop_layer()),
          )
          .dismiss_button("Close")
          .title("Enter Ollama Hostname"),
        );
      })
      .with_name("ollama_hostname"),
    )
}

fn port(s: Ptr<Config>) -> LinearLayout {
  LinearLayout::horizontal()
    .child(TextView::new("🕸 Ollama Server Port").full_width())
    .child(
      Button::new_raw(format!("<{}>", &s.ollama.port), |x| {
        x.add_layer(
          Dialog::around(
            EditView::new()
              .on_edit(|x, txt, _| {
                let data: &mut Ptr<Config> = x.user_data().unwrap();

                if let Ok(port) = txt.parse::<u16>() {
                  data.ollama.port = port;

                  x.call_on_name("ollama_port", |x: &mut Button| {
                    x.set_label_raw(format!("<{port}>"));
                  });
                }
              })
              .on_submit(|x, _| _ = x.pop_layer()),
          )
          .dismiss_button("Close")
          .title("Enter Ollama Hostname"),
        );
      })
      .with_name("ollama_port"),
    )
}
