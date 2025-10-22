use cursive::{
  align::Align,
  theme::{Effect, Style},
  view::Resizable,
  views::{Button, DummyView, LinearLayout, TextView},
};

pub fn render(l: &mut LinearLayout) {
  l.add_child(
    LinearLayout::horizontal()
      .child(TextView::new("⚒ Authentication Type").full_width())
      .child(Button::new_raw("No Auth (OpenToAll)", |_| {})),
  );

  l.add_child(DummyView::new().fixed_height(2));

  l.add_child(
    TextView::new("No Auth")
      .align(Align::center())
      .style(Style::merge(&[
        Effect::Dim.into(),
        Effect::Underline.into(),
      ])),
  );

  l.add_child(
    TextView::new("This means that the application requires ABSOLUTELY no authentication to talk to the api. This is only recommended for completely OFFLINE (DISCONNECTED FROM INTERNET) servers and must not be used for remote servers")
  );
}
