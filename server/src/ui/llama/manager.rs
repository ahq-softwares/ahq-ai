use std::sync::Arc;

use crate::{
  auth::{
    argon::{encrypt_with_key, server::verify_server_pass},
    gen_uid,
  },
  structs::{Capabilities, Config, LlamaServer, ModelFlag},
  ui::Ptr,
};
use cursive::{
  With,
  theme::{Effect, Style},
  utils::markup::StyledString,
  view::{Margins, Nameable, Resizable},
  views::{
    Button, Checkbox, Dialog, DummyView, EditView, LinearLayout, ResizedView, ScrollView, TextView,
  },
};

pub fn launch(data: Ptr<Config>) -> ResizedView<Dialog> {
  let s1 = data.clone();

  let mut layout = LinearLayout::vertical();

  render_table(&mut layout, data.clone());

  Dialog::new()
    .content(ScrollView::new(layout.with_name("renderedtable")).show_scrollbars(true))
    .button("New", move |s| {
      if s1.admin_pass_hash.is_some() {
        s.add_layer(new_server(s1.clone(), None));
      } else {
        s.add_layer(
          Dialog::around(TextView::new("Please set a server password first!")).dismiss_button("Ok"),
        );
      }
    })
    .dismiss_button("Back")
    .full_screen()
}

pub fn render_table(layout: &mut LinearLayout, conf: Ptr<Config>) {
  layout.clear();
  layout.add_child(
    LinearLayout::horizontal()
      .child(
        TextView::new("Model ID")
          .style(Style::merge(&[Effect::Dim.into()]))
          .fixed_width(34),
      )
      .child(
        TextView::new("Model Name")
          .style(Style::merge(&[Effect::Dim.into()]))
          .full_width(),
      )
      .child(
        TextView::new("Model URL")
          .style(Style::merge(&[Effect::Dim.into()]))
          .full_width(),
      )
      .child(
        Button::new_raw(
          StyledString::styled("Feat (i)", Style::from(Effect::Dim)),
          |x| {
            x.add_layer(
              Dialog::around(ScrollView::new(
                LinearLayout::vertical()
                  .child(TextView::new("`Abbr` : Capability"))
                  .child(TextView::new("`A`    : Audio"))
                  .child(TextView::new("`I`    : Image"))
                  .child(TextView::new("`F`    : Files")),
              ))
              .title("Legend")
              .dismiss_button("Got it"),
            );
          },
        )
        .fixed_width(10),
      )
      .child(DummyView::new().fixed_width(3))
      .child(
        TextView::new("Actions")
          .style(Style::merge(&[Effect::Dim.into()]))
          .fixed_width(7),
      ),
  );

  for (k, v) in &conf.llama.models {
    let conf2 = conf.clone();
    let key = Some(k.to_string());

    let id = k as &str;
    let name = &v.name as &str;
    let url = &v.url as &str;

    let mut cap = vec![" "];

    if v.capabilities.has(ModelFlag::Audio) {
      cap.push("A");
    }

    if v.capabilities.has(ModelFlag::Image) {
      cap.push("I");
    }

    if v.capabilities.has(ModelFlag::Files) {
      cap.push("F");
    }

    let cap = cap.join("");

    layout.add_child(
      LinearLayout::horizontal()
        .child(TextView::new(id).fixed_width(34))
        .child(TextView::new(name).full_width())
        .child(TextView::new(url).full_width())
        .child(TextView::new(cap).fixed_width(10))
        .child(DummyView::new().fixed_width(6))
        .child(
          Button::new_raw("Show", move |x| {
            let conf3 = conf2.clone();
            let conf4 = conf2.clone();
            let key_to_pass = key.clone();
            let key_to_pass2 = key.clone();

            x.add_layer(
              Dialog::around(
                LinearLayout::vertical()
                  .child(Button::new_raw("Edit", move |x| {
                    x.pop_layer();
                    x.add_layer(new_server(conf3.clone(), key_to_pass.clone()));
                  }))
                  .child(Button::new_raw("Remove", move |x| {
                    _ = conf4
                      .clone()
                      .llama
                      .models
                      .remove(&key_to_pass2.clone().unwrap() as &str);
                    x.pop_layer();

                    let conf2 = conf4.clone();
                    x.call_on_name("renderedtable", move |layout: &mut LinearLayout| {
                      render_table(layout, conf2);
                    });
                  })),
              )
              .title("Select an action")
              .dismiss_button("Cancel"),
            );
          })
          .fixed_width(4),
        ),
    );
  }
}

pub fn new_server(conf: Ptr<Config>, key: Option<String>) -> ResizedView<ResizedView<Dialog>> {
  let mut llama = None;

  if let Some(key) = key.as_ref() {
    llama = conf
      .llama
      .models
      .get(key as &str)
      .map(|x| Arc::new(x.clone()));
  }

  let l1 = llama.clone();
  let l2 = llama.clone();
  let l3 = llama.clone();
  let l4 = llama.clone();
  let l5 = llama.clone();
  let l6 = llama.clone();
  let l7 = llama.clone();

  let orig_key = key;

  Dialog::new()
    .content(
      ScrollView::new(LinearLayout::vertical().with(move |x| {
        x.add_child(TextView::new("Model Name"));
        x.add_child(
          EditView::new()
            .with(move |x| {
              if let Some(d) = l1 {
                x.set_content(&d.name as &str);
              }
            })
            .with_name("model_name"),
        );

        x.add_child(DummyView::new().fixed_height(1));

        x.add_child(TextView::new("Server Url (scheme://url:port)"));
        x.add_child(
          EditView::new()
            .with(move |x| {
              if let Some(d) = l2 {
                x.set_content(&d.url as &str);
              }
            })
            .with_name("server_url"),
        );

        x.add_child(DummyView::new().fixed_height(1));

        x.add_child(TextView::new("Server Admin Password (for verification)"));
        x.add_child(EditView::new().secret().with_name("server_admin_key"));

        x.add_child(DummyView::new().fixed_height(1));

        x.add_child(TextView::new("API Key (leave blank if none)"));
        x.add_child(
          EditView::new()
            .with(move |x| {
              if let Some(data) = l3
                && data.apikey.is_some()
              {
                x.set_content("< unchanged >");
              }
            })
            .with_name("api_key"),
        );

        x.add_child(DummyView::new().fixed_height(1));

        x.add_child(TextView::new("Model Capabilities"));
        x.add_child(
          LinearLayout::horizontal()
            .child(TextView::new("Image Support").full_width())
            .child(
              Checkbox::new()
                .with(move |x| {
                  if let Some(data) = l4 {
                    x.set_checked(data.capabilities.has(ModelFlag::Image));
                  }
                })
                .with_name("img"),
            ),
        );
        x.add_child(
          LinearLayout::horizontal()
            .child(TextView::new("Audio Support").full_width())
            .child(
              Checkbox::new()
                .with(move |x| {
                  if let Some(data) = l5 {
                    x.set_checked(data.capabilities.has(ModelFlag::Audio));
                  }
                })
                .with_name("aud"),
            ),
        );
        x.add_child(
          LinearLayout::horizontal()
            .child(TextView::new("Files Support").full_width())
            .child(
              Checkbox::new()
                .with(move |x| {
                  if let Some(data) = l6 {
                    x.set_checked(data.capabilities.has(ModelFlag::Files));
                  }
                })
                .with_name("file"),
            ),
        );
      }))
      .show_scrollbars(true),
    )
    .button("Confirm", move |x| {
      let model = x
        .call_on_name("model_name", |x: &mut EditView| x.get_content())
        .unwrap();

      let admin_pass = x
        .call_on_name("server_admin_key", |x: &mut EditView| x.get_content())
        .unwrap();

      if !verify_server_pass(&admin_pass, conf.admin_pass_hash.as_ref().unwrap()).unwrap() {
        x.add_layer(
          Dialog::around(TextView::new("Invalid Sever Administrator Password"))
            .dismiss_button("Ok"),
        );
        return;
      }

      let url = x
        .call_on_name("server_url", |x: &mut EditView| x.get_content())
        .unwrap();

      let api = x
        .call_on_name("api_key", |x: &mut EditView| x.get_content())
        .unwrap();

      let mut key = None;
      if api.as_str() != "" {
        if api.as_str() == "< unchanged >" {
          if let Some(x) = l7.clone().and_then(|x| x.apikey.clone()) {
            // Use the encrypted api key
            key = Some(x);
          } else {
            // Interpret `< unchanged >` as the api key itself
            key = Some(encrypt_with_key(&admin_pass, api.as_str()).into_boxed_str());
          }
        } else {
          key = Some(encrypt_with_key(&admin_pass, api.as_str()).into_boxed_str());
        }
      }

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
        orig_key
          .clone()
          .map(|x| x.into_boxed_str())
          .unwrap_or(gen_uid().unwrap().into_boxed_str()),
        LlamaServer {
          name: model.to_string().into_boxed_str(),
          url: url.to_string().into_boxed_str(),
          apikey: key,
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

      x.pop_layer();

      let conf2 = conf.clone();
      x.call_on_name("renderedtable", move |layout: &mut LinearLayout| {
        render_table(layout, conf2);
      });
    })
    .dismiss_button("Cancel")
    .padding(Margins::lrtb(1, 1, 1, 1))
    .max_height(50)
    .max_width(40)
}
