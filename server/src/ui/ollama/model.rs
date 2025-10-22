use cursive::{
  align::Align,
  theme::{Effect, Style},
  view::{Nameable, Resizable},
  views::{Button, Dialog, EditView, LinearLayout, NamedView, ResizedView, ScrollView, TextView},
};

use crate::{structs::Config, ui::Ptr};

pub fn bind(s: Ptr<Config>, vision: bool) -> ResizedView<Dialog> {
  Dialog::new()
    .title(if vision {
      "Vision AI Models"
    } else {
      "Text-Based AI Models"
    })
    .content(ScrollView::new(gen_cnt(s.clone(), vision)).show_scrollbars(true))
    .button("Add", move |x| {
      x.add_layer(add_model(vision));
    })
    .dismiss_button("Done")
    .full_screen()
}

fn add_model(cv: bool) -> Dialog {
  Dialog::new()
    .content(
      ScrollView::new(
        LinearLayout::vertical()
          .child(TextView::new("Enter model (eg. llava:7b)"))
          .child(EditView::new().with_name("model_name")),
      )
      .show_scrollbars(true),
    )
    .button("Add", move |x| {
      let model = x
        .call_on_name("model_name", |x: &mut EditView| x.get_content())
        .unwrap();

      let data: &mut Ptr<Config> = x.user_data().unwrap();

      let state_ = if cv {
        &mut data.ollama.cvmodels
      } else {
        &mut data.ollama.txtmodels
      };

      state_.push(model.to_string());

      let state = state_.clone();

      x.call_on_name("models", |l: &mut LinearLayout| {
        iterate_layout(l, &state, cv);
      });

      x.pop_layer();
      x.add_layer(
        Dialog::around(TextView::new("Successfully updated!"))
          .title("Successful")
          .dismiss_button("Ok"),
      );
    })
    .dismiss_button("Cancel")
}

fn gen_cnt(s: Ptr<Config>, cv: bool) -> NamedView<LinearLayout> {
  let mut layout = LinearLayout::vertical();

  iterate_layout(
    &mut layout,
    if cv {
      &s.ollama.cvmodels
    } else {
      &s.ollama.txtmodels
    },
    cv,
  );

  layout.with_name("models")
}

fn iterate_layout(l: &mut LinearLayout, binds: &[String], cv: bool) {
  l.clear();

  if binds.is_empty() {
    l.add_child(TextView::new("No models detected"));
  } else {
    l.add_child(
      LinearLayout::horizontal()
        .child(
          TextView::new("SNo")
            .style(Style::merge(&[Effect::Dim.into()]))
            .fixed_width(5),
        )
        .child(
          TextView::new("Model")
            .style(Style::merge(&[Effect::Dim.into()]))
            .full_width(),
        )
        .child(
          TextView::new("")
            .style(Style::merge(&[Effect::Dim.into()]))
            .fixed_width(12),
        ),
    );
  }

  binds.iter().enumerate().for_each(|(index, model)| {
    l.add_child(layout_child(index, model, cv));
  });
}

fn layout_child(index: usize, model: &str, cv: bool) -> LinearLayout {
  LinearLayout::horizontal()
    .child(
      TextView::new(format!("{}.", index + 1))
        .align(Align::center_left())
        .fixed_width(5),
    )
    .child(TextView::new(model).full_width())
    .child(
      Button::new_raw("✕ Remove", move |x| {
        x.with_user_data(|x: &mut Ptr<Config>| {
          if cv {
            x.ollama.cvmodels.remove(index);
          } else {
            x.ollama.txtmodels.remove(index);
          }
        });

        let state: &mut Ptr<Config> = x.user_data().unwrap();
        let state = if cv {
          &mut state.ollama.cvmodels
        } else {
          &mut state.ollama.txtmodels
        }
        .clone();

        x.call_on_name("models", |l: &mut LinearLayout| {
          iterate_layout(l, &state, cv);
        });
      })
      .fixed_width(12),
    )
}
