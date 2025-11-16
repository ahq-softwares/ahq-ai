use cursive::{Cursive, view::Resizable};
use cursive_tabs::{Align, Placement, TabPanel};

mod general;
mod user;

pub fn run_manager(x: &mut Cursive) {
  let tabs = TabPanel::new()
    .with_tab(general::tab())
    .with_tab(user::tab())
    .with_active_tab("∿ General")
    .map_err(|_| ())
    .expect("Unknown erorr")
    .with_bar_alignment(Align::Start)
    .with_bar_placement(Placement::HorizontalTop)
    .full_screen();

  x.add_fullscreen_layer(tabs);
}
