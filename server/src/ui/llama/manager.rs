use crate::{
  auth::gen_uid,
  structs::{Capabilities, Config, LlamaServer, ModelFlag},
  ui::Ptr,
};
use cursive::{
  With,
  view::{Margins, Nameable, Resizable},
  views::{Checkbox, Dialog, DummyView, EditView, LinearLayout, ResizedView, ScrollView, TextView},
};

pub fn launch(data: Ptr<Config>) -> ResizedView<Dialog> {
  let s1 = data.clone();
  Dialog::new()
    .button("New", move |s| {
      if let Some(_) = s1.admin_pass_hash {
        s.add_layer(new_server(s1.clone()));
      } else {
        s.add_layer(
          Dialog::around(TextView::new("Please set a server password first!")).dismiss_button("Ok"),
        );
      }
    })
    .dismiss_button("Back")
    .full_screen()
}

pub fn render_table() {}

pub fn new_server(conf: Ptr<Config>) -> ResizedView<ResizedView<Dialog>> {
  Dialog::new()
    .content(
      ScrollView::new(LinearLayout::vertical().with(move |x| {
        x.add_child(TextView::new("Model Name"));
        x.add_child(EditView::new().with_name("model_name"));

        x.add_child(DummyView::new().fixed_height(1));

        x.add_child(TextView::new("Server Url (scheme://url:port)"));
        x.add_child(EditView::new().with_name("server_url"));

        x.add_child(DummyView::new().fixed_height(1));

        x.add_child(TextView::new("Server Admin Password (for verification)"));
        x.add_child(EditView::new().with_name("server_admin_key"));

        x.add_child(DummyView::new().fixed_height(1));

        x.add_child(TextView::new("API Key (leave blank if none)"));
        x.add_child(EditView::new().with_name("api_key"));

        x.add_child(DummyView::new().fixed_height(1));

        x.add_child(TextView::new("Model Capabilities"));
        x.add_child(
          LinearLayout::horizontal()
            .child(TextView::new("Image Support").full_width())
            .child(Checkbox::new().with_name("img")),
        );
        x.add_child(
          LinearLayout::horizontal()
            .child(TextView::new("Audio Support").full_width())
            .child(Checkbox::new().with_name("aud")),
        );
        x.add_child(
          LinearLayout::horizontal()
            .child(TextView::new("Files Support").full_width())
            .child(Checkbox::new().with_name("file")),
        );
      }))
      .show_scrollbars(true),
    )
    .button("Add", move |x| {
      let model = x
        .call_on_name("model_name", |x: &mut EditView| x.get_content())
        .unwrap();

      let url = x
        .call_on_name("server_url", |x: &mut EditView| x.get_content())
        .unwrap();

      let api = x
        .call_on_name("api_key", |x: &mut EditView| x.get_content())
        .unwrap();

      let img = x
        .call_on_name("img", |x: &mut Checkbox| x.is_checked())
        .unwrap();

      let audio = x
        .call_on_name("aud", |x: &mut Checkbox| x.is_checked())
        .unwrap();

      let file = x
        .call_on_name("file", |x: &mut Checkbox| x.is_checked())
        .unwrap();

      conf.clone().llama.models.insert(
        gen_uid().unwrap().into_boxed_str(),
        LlamaServer {
          name: model.to_string().into_boxed_str(),
          url: url.to_string().into_boxed_str(),
          apikey: if api.as_str() == "" {
            None
          } else {
            Some(api.to_string().into_boxed_str())
          },
          capabilities: {
            let mut capab = Capabilities(0u16);

            if img {
              capab.add(ModelFlag::Image);
            }
            if audio {
              capab.add(ModelFlag::Audio);
            }
            if file {
              capab.add(ModelFlag::Files);
            }

            capab
          },
        },
      );
    })
    .dismiss_button("Cancel")
    .padding(Margins::lrtb(1, 1, 1, 1))
    .max_height(50)
    .max_width(40)
}
