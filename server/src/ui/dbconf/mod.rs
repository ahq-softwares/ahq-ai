use cursive::{
  view::Nameable,
  views::{LinearLayout, NamedView, ScrollView},
};

use crate::{structs::Config, ui::Ptr};

pub fn db_page(s: Ptr<Config>) -> NamedView<ScrollView<LinearLayout>> {
  let layout = LinearLayout::vertical();

  ScrollView::new(layout)
    .show_scrollbars(true)
    .with_name("㏈ Database")
}
