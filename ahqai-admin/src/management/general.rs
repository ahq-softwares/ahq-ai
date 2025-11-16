use cursive::{
  View,
  align::HAlign,
  style::Color,
  theme::{Effect, Style},
  view::{Nameable, Resizable},
  views::{DummyView, LinearLayout, NamedView, PaddedView, ScrollView, TextView},
};

pub fn tab() -> NamedView<impl View> {
  ScrollView::new(
    PaddedView::lrtb(
      1,
      1,
      1,
      1,
    LinearLayout::vertical()
      .child(DummyView::new().fixed_height(1))
      .child(
        TextView::new("Welcome to AHQ AI Administration Utility")
          .style(Style::from(Color::from_256colors(4)).combine(Effect::Underline))
          .h_align(HAlign::Center),
      )
      .child(
        TextView::new("This utility is the AHQ AI Server Remote Administrator utility tool that can be used to remotely configure the server.")
      )
      .child(
        DummyView::new().fixed_height(1)
      )
      .child(
        TextView::new("Quick Guide")
          .style(Style::from(Effect::Underline))
          .h_align(HAlign::Center),
      )
      .child(
        TextView::new("» Use ← ↑ → ↓ to navigate")
      )
      .child(TextView::new(
      "» Pres s <Enter> key to interact with buttons",
      ))
      .child(TextView::new(
      "» You can also use mouse to interact with buttons or tabs",
      ))
      .child(TextView::new(
      "» You can also scroll with the mouse scrollbar",
      ))
      .child(TextView::new(
      "» <q> key, <Ctrl+C> will quit the application",
      ))
      .child(
        DummyView::new().fixed_height(1)
      )
      .child(
        TextView::new("Credits")
          .style(Style::from(Effect::Underline))
          .h_align(HAlign::Center),
      )
      .child(
        TextView::new("We would like to thank the following team members for dedicating their time and efforts to make this possible: "),
      )
      .child(
        DummyView::new().fixed_height(1)
      )
      .child(
        TextView::new("1. A. Chakraborty (@ahqsoftwares)"),
      )
      .child(
        TextView::new("2. AHQ Softwares' Team (Founder: A. Chakraborty, Rohan Murudkar and others)"),
      )
      .child(
        TextView::new("3. AHQ AI Team (Founder: A. Chakraborty, Rohan Murudkar and others)"),
      )
      .child(
        TextView::new("4. Open Sourced Collaborators and Contributors"),
      )
      .child(
        DummyView::new().fixed_height(3)
      )
      .child(
        TextView::new("©️2025 AHQ AI")
          .style(Style::from(Color::from_256colors(4)))
          .h_align(HAlign::Center),
      )
      .child(
        TextView::new("Proudly Licensed under GPL-3.0")
          .style(Style::from(Color::from_256colors(4)))
          .h_align(HAlign::Center),
      ),
    )
  )
  .with_name("∿ General")
}
