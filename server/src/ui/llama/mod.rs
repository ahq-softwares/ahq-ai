use cursive::{
  theme::{Effect, Style},
  view::{Nameable, Resizable},
  views::{Button, DummyView, LinearLayout, NamedView, ScrollView, TextView},
};

use crate::{structs::Config, ui::Ptr};

// mod model;
mod manager;

pub fn llama_page(s: Ptr<Config>) -> NamedView<ScrollView<LinearLayout>> {
  let mut layout = LinearLayout::vertical();

  layout.add_child(DummyView::new().fixed_height(1));

  layout.add_child(
    TextView::new("AHQ AI uses llama-server to provide inference. You need to host a llama server with a given model that you can configure here. LLAMA.CPP allows us to enable audio, image, file support!")
  );

  layout.add_child(DummyView::new().fixed_height(1));

  layout.add_child(TextView::new("Models").style(Style::merge(&[Effect::Underline.into()])));

  let s1 = s.clone();
  layout.add_child(
    LinearLayout::horizontal()
      .child(TextView::new("🖧 Models").full_width())
      .child(Button::new_raw("Launch Model Manager ↗", move |x| {
        x.add_layer(manager::launch(s1.clone()));
      })),
  );

  ScrollView::new(layout)
    .show_scrollbars(true)
    .with_name("🖧 Llama Server")
}

// fn server(s: Ptr<Config>) -> LinearLayout {
//   LinearLayout::horizontal()
//     .child(TextView::new("🖥 Ollama Server Hostname").full_width())
//     .child(
//       Button::new_raw(format!("[{}]", &s.ollama.host), |x| {
//         x.add_layer(
//           Dialog::around(
//             EditView::new()
//               .on_edit(|x, txt, _| {
//                 let data: &mut Ptr<Config> = x.user_data().unwrap();

//                 data.ollama.host = txt.into();

//                 x.call_on_name("ollama_hostname", |x: &mut Button| {
//                   x.set_label_raw(format!("[{txt}]"));
//                 });
//               })
//               .on_submit(|x, _| _ = x.pop_layer()),
//           )
//           .dismiss_button("Close")
//           .title("Enter Ollama Hostname"),
//         );
//       })
//       .with_name("ollama_hostname"),
//     )
// }

// fn msgs(s: Ptr<Config>) -> LinearLayout {
//   LinearLayout::horizontal()
//     .child(TextView::new("🖥 Max message pair per chat").full_width())
//     .child(
//       Button::new_raw(format!("<{}>", &s.ollama.msgs), |x| {
//         x.add_layer(
//           Dialog::around(
//             EditView::new()
//               .on_edit(|x, txt, _| {
//                 let data: &mut Ptr<Config> = x.user_data().unwrap();

//                 if let Ok(num) = txt.parse::<usize>() {
//                   data.ollama.msgs = num;

//                   x.call_on_name("ollama_msgs", |x: &mut Button| {
//                     x.set_label_raw(format!("<{num}>"));
//                   });
//                 }
//               })
//               .on_submit(|x, _| _ = x.pop_layer()),
//           )
//           .dismiss_button("Close")
//           .title("Enter Maximum"),
//         );
//       })
//       .with_name("ollama_msgs"),
//     )
// }

// fn port(s: Ptr<Config>) -> LinearLayout {
//   LinearLayout::horizontal()
//     .child(TextView::new("🕸 Ollama Server Port").full_width())
//     .child(
//       Button::new_raw(format!("<{}>", &s.ollama.port), |x| {
//         x.add_layer(
//           Dialog::around(
//             EditView::new()
//               .on_edit(|x, txt, _| {
//                 let data: &mut Ptr<Config> = x.user_data().unwrap();

//                 if let Ok(port) = txt.parse::<u16>() {
//                   data.ollama.port = port;

//                   x.call_on_name("ollama_port", |x: &mut Button| {
//                     x.set_label_raw(format!("<{port}>"));
//                   });
//                 }
//               })
//               .on_submit(|x, _| _ = x.pop_layer()),
//           )
//           .dismiss_button("Close")
//           .title("Enter Ollama Hostname"),
//         );
//       })
//       .with_name("ollama_port"),
//     )
// }
