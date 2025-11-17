use std::{
  sync::{Arc, mpsc::channel},
  thread,
  time::Duration,
};

use cursive::{
  Cursive, CursiveExt, With,
  align::HAlign,
  theme::StyleType,
  view::{Nameable, Resizable},
  views::{Dialog, DummyView, EditView, LinearLayout, ProgressBar, ScrollView, TextView},
};

mod api;
mod management;

const AHQ: &'static str = "@@@@@
@@@@@@@
@@@@@@@@@
@@@@@ @@@@@
@@@@@   @@@@@
@@@@@     @@@@@
@@@@@       @@@@@
@@@@@         @@@@@
@@@@@           @@@@@
@@@@@     @@@@@@@@@@@@@
@@@@@     @@@@@@@@@@@@@@@
@@@@@     @@@@@@@@@@@@@@@@@
";

fn main() {
  let mut siv = Cursive::new();

  siv.add_layer(
    Dialog::around(
      ScrollView::new(
        LinearLayout::vertical()
          .child(DummyView::new().fixed_height(1))
          .with(|x| {
            for line in AHQ.lines() {
              x.add_child(TextView::new(line).h_align(HAlign::Center));
            }
          })
          .child(DummyView::new().fixed_height(1))
          .child(TextView::new("AHQ AI").h_align(HAlign::Center))
          .child(TextView::new("Administrator Portal").h_align(HAlign::Center))
          .child(DummyView::new().fixed_height(2))
          .child(TextView::new("©️2025 AHQ AI").h_align(HAlign::Center))
          .child(TextView::new("Proudly licensed under GPL-3").h_align(HAlign::Center)),
      )
      .show_scrollbars(true),
    )
    .min_width(40),
  );

  let sink = siv.cb_sink().clone();

  thread::spawn(move || {
    thread::sleep(Duration::from_millis(750));

    _ = sink.send(Box::new(|x| {
      x.pop_layer();
      render(x);
    }));
  });

  siv.run();
}

struct ServerData {
  url: String,
  pwd: String,
}

fn render(siv: &mut Cursive) {
  siv.add_layer(
    Dialog::around(
      ScrollView::new(
      LinearLayout::vertical()
        .child(TextView::new("Server URL"))
        .child(EditView::new().with(|_x| {
          #[cfg(debug_assertions)]
          _x.set_content("http://localhost:3000");
        }).with_name("srvrurl1").min_width(20))

        .child(DummyView::new().fixed_height(1))
        .child(TextView::new("Enter the server url that you'd enter in AHQ AI Client application. If you are using a load balancer and the server administrator password is same for all the servers, you can enter the url to the load balancer. If the passwords for each node is not same, enter the url of a specific node.").style(StyleType::secondary()))
        .child(DummyView::new().fixed_height(2))
        .child(TextView::new("Server Administrator Password"))
        .child(EditView::new().secret().with(|_x| {
          #[cfg(debug_assertions)]
          _x.set_content("ahqsoftwares");
        }).with_name("srvrpwd1").min_width(20))
      ),
    )
    .title("Connect with Server")
    .button("Connect", |x| {
      let sink = x.cb_sink().clone();
      let (tx, rx) = channel::<()>();

      let server_url = x.call_on_name("srvrurl1", |x: &mut EditView| x.get_content());
      let server_pwd = x.call_on_name("srvrpwd1", |x: &mut EditView| x.get_content());

      let server_url = server_url.unwrap();
      let server_pwd = server_pwd.unwrap();

      x.add_layer(
        Dialog::around(
          LinearLayout::vertical()
            .child(DummyView::new().fixed_height(2))
            .child(ProgressBar::new().with_task(move |_| {
                loop {
                  for sts in (0..=100usize).chain((0..=100usize).rev()) {
                    _ = sink.send(Box::new(move |x| {
                      x.call_on_name("prog", move |x: &mut ProgressBar| {
                        x.set_value(sts);
                      });
                    }));
                    thread::sleep(Duration::from_millis(10));
                  }

                  match rx.try_recv() {
                    Ok(_) => break,
                    _ => {}
                  }
                }
            }).with_label(|_, _| "Connecting...".to_string()).with_name("prog"))
            .child(DummyView::new().fixed_height(2))
        )
          .min_width(48)
      );

      let state = Arc::new(ServerData {
        pwd: server_pwd.to_string(),
        url: server_url.to_string()
      });

      let state1= state.clone();
      x.set_user_data(state);

      let sink = x.cb_sink().clone();

      thread::spawn(move || {
        let state1 = state1;

        match api::root(&state1.url) {
          Ok(()) => {},
          Err(txt) => {
            _ = tx.send(());
            _ = sink.send(Box::new(move |x| {
              x.pop_layer();
              x.add_layer(
                Dialog::around(
                  TextView::new(txt)
                ).title("Something went wrong").dismiss_button("Ok")
              );
            }));

            return;
          }
        }

        match api::verify(&state1.url, &state1.pwd) {
          Ok(()) => {
            _ = tx.send(());
            _ = sink.send(Box::new(|x| {
              x.pop_layer();
              x.pop_layer();

              management::run_manager(x);
            }));
          },
          Err(txt) => {
            _ = tx.send(());
            _ = sink.send(Box::new(move |x| {
              x.pop_layer();
              x.add_layer(
                Dialog::around(
                  TextView::new(txt)
                ).title("Something went wrong").dismiss_button("Ok")
              );
            }));
          }
        }
      });
    })
    .button("Quit", |x| {
      x.quit();
    })
    .title("Connect to server")
    .min_width(32)
    .max_width(64),
  );
}
