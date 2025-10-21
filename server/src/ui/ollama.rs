use cursive::{
  view::Nameable,
  views::{LinearLayout, NamedView, ScrollView},
};

pub fn ollama_page() -> NamedView<ScrollView<LinearLayout>> {
  ScrollView::new(LinearLayout::vertical())
    .show_scrollbars(true)
    .with_name("🖧 Ollama")
}
