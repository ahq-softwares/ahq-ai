use cursive::{
  view::{Nameable, Resizable, Scrollable},
  views::{Button, Dialog, EditView, LinearLayout, TextView},
};

use crate::{structs::Config, ui::Ptr};

pub fn perf(l: &mut LinearLayout, conf: Ptr<Config>) {
  l.add_child(
    LinearLayout::horizontal()
      .child(TextView::new("⚒ Scale factor").full_width())
      .child(
        Button::new_raw(format!("<{}>", conf.performance.scale_factor), |x| {
          x.add_layer(
            Dialog::around(
              LinearLayout::vertical()
              .child(TextView::new("This factor sets the maximum ratio of CPU cores used for heavy hashing and other CPU-Bound tasks. 1.0 uses every core, but for memory-hard tasks like Argon2id, the optimal setting is often <1.0 (default, 0.75) to prevent severe performance loss from context switching and cache thrashing."))
              .child(
              EditView::new()
                .on_edit(|x, val, _| {
                  let state: &mut Ptr<Config> = x.user_data().unwrap();

                  if let Ok(num) = val.parse::<f64>()
                    && num > 0.0
                  {
                    state.performance.scale_factor = num;

                    x.call_on_name("scaling", move |x: &mut Button| {
                      x.set_label_raw(format!("<{num}>"));
                    });
                  }
                })
                .on_submit(|x, _| {
                  x.pop_layer();
                })
              )
              .scrollable(),
            )
            .dismiss_button("Done")
            .title("Set the scale factor")
            .min_width(48)
            .max_width(96),
          );
        })
        .with_name("scaling"),
      ),
  );

  l.add_child(
    LinearLayout::horizontal()
      .child(TextView::new("⚒ Queue Size Multiplicity").full_width())
      .child(
        Button::new_raw(format!("<{}>", conf.performance.queue_size), |x| {
          x.add_layer(
            Dialog::around(
              LinearLayout::vertical()
              .child(TextView::new("Queue size is a multiplier for the thread capacity. (approximately, Capacity = Threads * Scale Factor * Queue Size Factor). A larger factor creates a greater buffer for load spikes, but risks higher latency if the queue fills up."))
              .child(
              EditView::new()
                .on_edit(|x, val, _| {
                  let state: &mut Ptr<Config> = x.user_data().unwrap();

                  if let Ok(num) = val.parse::<usize>()
                    && num > 0
                  {
                    state.performance.queue_size = num;

                    x.call_on_name("queue_size", move |x: &mut Button| {
                      x.set_label_raw(format!("<{num}>"));
                    });
                  }
                })
                .on_submit(|x, _| {
                  x.pop_layer();
                })
              )
              .scrollable(),
            )
            .dismiss_button("Done")
            .title("Set the queue size")
            .min_width(48)
            .max_width(96),
          );
        })
        .with_name("queue_size"),
      ),
  );
}
