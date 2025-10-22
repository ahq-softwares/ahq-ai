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
      .child(Button::new_raw("Token (TokenBased)", |_| {})),
  );

  l.add_child(
    LinearLayout::horizontal()
      .child(TextView::new("⚒ Token Manager").full_width())
      .child(Button::new_raw("Use Admin API ↗", |_| {})),
  );

  l.add_child(DummyView::new().fixed_height(2));

  l.add_child(
    TextView::new("Token Auth")
      .align(Align::center())
      .style(Style::merge(&[
        Effect::Dim.into(),
        Effect::Underline.into(),
      ])),
  );

  l.add_child(
    TextView::new("This means that the application would be required to supply a token for the purposes of verification. The token will be verified and finally then the application can interact with the server. This is comparatively more secure but can also be a bit tedious")
  );
}
