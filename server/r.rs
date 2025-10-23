#![feature(prelude_import)]
#![feature(duration_constructors)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
use std::{env::args, panic};
mod server {
    use std::{
        fs as stdfs, sync::{LazyLock, OnceLock},
        thread::available_parallelism,
    };
    use actix_web::{App, HttpServer};
    use chalk_rs::Chalk;
    use ollama_rs::Ollama;
    use serde_json::from_str;
    use crate::{auth::AuthSessionManager, structs::Config};
    pub mod http {
        use actix_web::{HttpResponse, Responder, Result, get, http::header::ContentType};
        use crate::server::http::structs::ROOT_RESPONSE_DATA;
        pub mod structs {
            use std::sync::LazyLock;
            use serde::Serialize;
            use crate::{server::CONFIG, structs::Authentication};
            pub enum ShowedAuth {
                OpenToAll,
                TokenBased,
                Account,
            }
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths,
            )]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl _serde::Serialize for ShowedAuth {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> _serde::__private228::Result<__S::Ok, __S::Error>
                    where
                        __S: _serde::Serializer,
                    {
                        match *self {
                            ShowedAuth::OpenToAll => {
                                _serde::Serializer::serialize_unit_variant(
                                    __serializer,
                                    "ShowedAuth",
                                    0u32,
                                    "OpenToAll",
                                )
                            }
                            ShowedAuth::TokenBased => {
                                _serde::Serializer::serialize_unit_variant(
                                    __serializer,
                                    "ShowedAuth",
                                    1u32,
                                    "TokenBased",
                                )
                            }
                            ShowedAuth::Account => {
                                _serde::Serializer::serialize_unit_variant(
                                    __serializer,
                                    "ShowedAuth",
                                    2u32,
                                    "Account",
                                )
                            }
                        }
                    }
                }
            };
            pub static ROOT_RESPONSE_DATA: LazyLock<Vec<u8>> = LazyLock::new(|| {
                let root_response = RootResponse::new();
                serde_json::to_vec(&root_response)
                    .expect("Failed to serialize static RootResponse")
            });
            pub struct RootResponse {
                auth: ShowedAuth,
                can_register: bool,
                vision_models: Vec<&'static str>,
                text_models: Vec<&'static str>,
            }
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths,
            )]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl _serde::Serialize for RootResponse {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> _serde::__private228::Result<__S::Ok, __S::Error>
                    where
                        __S: _serde::Serializer,
                    {
                        let mut __serde_state = _serde::Serializer::serialize_struct(
                            __serializer,
                            "RootResponse",
                            false as usize + 1 + 1 + 1 + 1,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "auth",
                            &self.auth,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "can_register",
                            &self.can_register,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "vision_models",
                            &self.vision_models,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "text_models",
                            &self.text_models,
                        )?;
                        _serde::ser::SerializeStruct::end(__serde_state)
                    }
                }
            };
            impl RootResponse {
                pub fn new() -> Self {
                    let mut out = Self {
                        auth: ShowedAuth::OpenToAll,
                        can_register: false,
                        text_models: ::alloc::vec::Vec::new(),
                        vision_models: ::alloc::vec::Vec::new(),
                    };
                    match CONFIG.authentication {
                        Authentication::Account { registration_allowed, .. } => {
                            out.can_register = registration_allowed;
                            out.auth = ShowedAuth::Account;
                        }
                        Authentication::OpenToAll => {
                            out.auth = ShowedAuth::OpenToAll;
                        }
                        Authentication::TokenBased => {
                            out.auth = ShowedAuth::TokenBased;
                        }
                    }
                    out.text_models.reserve(CONFIG.ollama.txtmodels.len());
                    out.vision_models.reserve(CONFIG.ollama.cvmodels.len());
                    CONFIG
                        .ollama
                        .cvmodels
                        .iter()
                        .for_each(|x| {
                            out.vision_models.push(x as &str);
                        });
                    CONFIG
                        .ollama
                        .txtmodels
                        .iter()
                        .for_each(|x| {
                            out.text_models.push(x as &str);
                        });
                    out
                }
            }
        }
        #[allow(non_camel_case_types, missing_docs)]
        pub struct index;
        impl ::actix_web::dev::HttpServiceFactory for index {
            fn register(self, __config: &mut actix_web::dev::AppService) {
                async fn index() -> impl Responder {
                    HttpResponse::Ok()
                        .content_type(ContentType::json())
                        .body::<&[u8]>(ROOT_RESPONSE_DATA.as_ref())
                }
                let __resource = ::actix_web::Resource::new("/")
                    .name("index")
                    .guard(::actix_web::guard::Get())
                    .to(index);
                ::actix_web::dev::HttpServiceFactory::register(__resource, __config);
            }
        }
    }
    pub static CONFIG: LazyLock<Config> = LazyLock::new(|| {
        let data = stdfs::read_to_string("config.json").expect("Unable to load config");
        from_str(&data).expect("Invalid configuration file, unable to parse")
    });
    pub static AUTH: OnceLock<AuthSessionManager> = OnceLock::new();
    pub static OLLAMA: LazyLock<Ollama> = LazyLock::new(|| {
        Ollama::new(CONFIG.ollama.host.as_ref(), CONFIG.ollama.port)
    });
    pub fn launch() -> Chalk {
        let mut chalk = Chalk::new();
        chalk
            .blue()
            .bold()
            .println(
                &::alloc::__export::must_use({
                    ::alloc::fmt::format(format_args!("AHQ-AI Server v{0}", "0.1.0"))
                }),
            );
        chalk.reset_style();
        chalk
    }
    pub fn main() -> std::io::Result<()> {
        <::actix_web::rt::System>::new()
            .block_on(async move {
                {
                    let mut chalk = launch();
                    _ = AUTH.set(AuthSessionManager::create(&CONFIG).await);
                    if OLLAMA.list_local_models().await.is_err() {
                        {
                            ::std::io::_print(format_args!("----------------\n"));
                        };
                        chalk
                            .red()
                            .println(
                                &"Connection to ollama failed. Are you sure configuration is correct?",
                            );
                        {
                            ::std::io::_print(format_args!("----------------\n"));
                        };
                    }
                    let mut server = HttpServer::new(|| App::new().service(http::index))
                        .workers(available_parallelism()?.get());
                    for (host, port) in &CONFIG.binds {
                        chalk
                            .blue()
                            .println(
                                &::alloc::__export::must_use({
                                    ::alloc::fmt::format(
                                        format_args!("Binding to {0}:{1}", host, port),
                                    )
                                }),
                            );
                        server = server.bind((host as &str, *port))?;
                    }
                    {
                        ::std::io::_print(format_args!("----------------\n"));
                    };
                    chalk.blue().println(&"Server is ready!");
                    {
                        ::std::io::_print(format_args!("----------------\n"));
                    };
                    {
                        ::std::io::_print(format_args!("\n"));
                    };
                    let out = server.run().await;
                    if let Err(e) = &out {
                        {
                            ::std::io::_print(format_args!("----------------\n"));
                        };
                        chalk.red().bold().println(&"Server Exited in an error state");
                        {
                            ::std::io::_print(format_args!("{0}\n", e));
                        };
                    }
                    {
                        ::std::io::_print(format_args!("----------------\n"));
                    };
                    chalk
                        .reset_style()
                        .blue()
                        .bold()
                        .println(
                            &"Starting shutdown procedure. Saving server state to disk... This might take a while",
                        );
                    chalk
                        .red()
                        .bold()
                        .println(
                            &"Please DO NOT use Ctrl+C to terminate. It will lead to data corruption!",
                        );
                    {
                        ::std::io::_print(format_args!("Shutdown Action\n"));
                    };
                    chalk
                        .reset_style()
                        .blue()
                        .bold()
                        .println(
                            &"Server state has been successfully set! Closing server",
                        );
                    out
                }
            })
    }
}
mod ui {
    use std::{
        env::home_dir, fs, ops::{Deref, DerefMut},
        sync::LazyLock, time::{SystemTime, UNIX_EPOCH},
    };
    use cursive::{
        Cursive, CursiveExt, align::Align, theme::{Effect, PaletteColor, Style, Theme},
        view::{Nameable, Resizable},
        views::{
            Button, Dialog, DummyView, EditView, LinearLayout, ScrollView, SelectView,
            TextView,
        },
    };
    use cursive_tabs::TabPanel;
    use serde_json::to_string_pretty;
    use tokio::runtime::{Builder, Runtime};
    use crate::structs::{Authentication, Config};
    mod auth {
        use cursive::{
            Cursive, view::Nameable, views::{LinearLayout, NamedView, ScrollView},
        };
        use crate::{
            structs::{Authentication, Config},
            ui::{Ptr, lazy::OnAuthStateUpdate},
        };
        mod open {
            use cursive::{
                align::Align, theme::{Effect, Style},
                view::Resizable, views::{Button, DummyView, LinearLayout, TextView},
            };
            pub fn render(l: &mut LinearLayout) {
                l.add_child(
                    LinearLayout::horizontal()
                        .child(TextView::new("⚒ Authentication Type").full_width())
                        .child(Button::new_raw("No Auth (OpenToAll)", |_| {})),
                );
                l.add_child(DummyView::new().fixed_height(2));
                l.add_child(
                    TextView::new("No Auth")
                        .align(Align::center())
                        .style(
                            Style::merge(&[Effect::Dim.into(), Effect::Underline.into()]),
                        ),
                );
                l.add_child(
                    TextView::new(
                        "This means that the application requires ABSOLUTELY no authentication to talk to the api. This is only recommended for completely OFFLINE (DISCONNECTED FROM INTERNET) servers and must not be used for remote servers",
                    ),
                );
            }
        }
        mod token {
            use cursive::{
                align::Align, theme::{Effect, Style},
                view::Resizable, views::{Button, DummyView, LinearLayout, TextView},
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
                        .style(
                            Style::merge(&[Effect::Dim.into(), Effect::Underline.into()]),
                        ),
                );
                l.add_child(
                    TextView::new(
                        "This means that the application would be required to supply a token for the purposes of verification. The token will be verified and finally then the application can interact with the server. This is comparatively more secure but can also be a bit tedious",
                    ),
                );
            }
        }
        mod user {
            use cursive::{
                align::Align, theme::{Effect, Style},
                view::{Nameable, Resizable},
                views::{
                    Button, Dialog, DummyView, EditView, LinearLayout, SelectView,
                    TextView,
                },
            };
            use crate::{
                structs::{Authentication, Config},
                ui::Ptr,
            };
            pub fn render(
                l: &mut LinearLayout,
                can_register: bool,
                max_users: Option<u64>,
            ) {
                l.add_child(
                    LinearLayout::horizontal()
                        .child(TextView::new("⚒ Authentication Type").full_width())
                        .child(Button::new_raw("Token (TokenBased)", |_| {})),
                );
                l.add_child(
                    LinearLayout::horizontal()
                        .child(
                            TextView::new("⚒ Self Registration Allowed").full_width(),
                        )
                        .child(
                            Button::new_raw(
                                    if can_register { "[Yes]" } else { "[No]" },
                                    |x| {
                                        x.add_layer(
                                            Dialog::around(
                                                    SelectView::new()
                                                        .item("Yes", true)
                                                        .item("No", false)
                                                        .on_submit(|x, val| {
                                                            let state: &mut Ptr<Config> = x.user_data().unwrap();
                                                            if let Authentication::Account {
                                                                registration_allowed,
                                                                ..
                                                            } = &mut state.authentication
                                                            {
                                                                *registration_allowed = *val;
                                                            }
                                                            let val_f = *val;
                                                            x.call_on_name(
                                                                "user_reg_allowed",
                                                                move |x: &mut Button| {
                                                                    x.set_label_raw(if val_f { "[Yes]" } else { "[No]" });
                                                                },
                                                            );
                                                            x.pop_layer();
                                                        }),
                                                )
                                                .title("Self Registration"),
                                        );
                                    },
                                )
                                .with_name("user_reg_allowed"),
                        ),
                );
                l.add_child(
                    LinearLayout::horizontal()
                        .child(TextView::new("⚒ Maximum Users").full_width())
                        .child(
                            Button::new_raw(
                                    match max_users {
                                        Some(x) => {
                                            ::alloc::__export::must_use({
                                                ::alloc::fmt::format(format_args!("[{0}]", x))
                                            })
                                        }
                                        _ => "[Infinite]".to_string(),
                                    },
                                    |x| {
                                        x.add_layer(
                                            Dialog::around(
                                                    LinearLayout::vertical()
                                                        .child(
                                                            EditView::new()
                                                                .on_edit(|x, val, _| {
                                                                    let state: &mut Ptr<Config> = x.user_data().unwrap();
                                                                    let resp;
                                                                    if let Authentication::Account { max_users, .. } = &mut state
                                                                        .authentication
                                                                    {
                                                                        if val == "Infinite" {
                                                                            *max_users = None;
                                                                        } else if let Ok(val) = val.parse::<u64>() {
                                                                            *max_users = Some(val);
                                                                        }
                                                                        resp = *max_users;
                                                                    } else {
                                                                        resp = None;
                                                                    }
                                                                    x.call_on_name(
                                                                        "max_users_allowed",
                                                                        move |x: &mut Button| {
                                                                            x.set_label_raw(
                                                                                match resp {
                                                                                    Some(x) => {
                                                                                        ::alloc::__export::must_use({
                                                                                            ::alloc::fmt::format(format_args!("[{0}]", x))
                                                                                        })
                                                                                    }
                                                                                    _ => "[Infinite]".to_string(),
                                                                                },
                                                                            );
                                                                        },
                                                                    );
                                                                })
                                                                .on_submit(|x, _| {
                                                                    x.pop_layer();
                                                                }),
                                                        )
                                                        .child(
                                                            TextView::new(
                                                                "You can write a valid positive integer or `Infinite`",
                                                            ),
                                                        ),
                                                )
                                                .title("Set Max Users"),
                                        );
                                    },
                                )
                                .with_name("max_users_allowed"),
                        ),
                );
                l.add_child(
                    LinearLayout::horizontal()
                        .child(TextView::new("⚒ Account Manager").full_width())
                        .child(Button::new_raw("Use Admin API ↗", |_| {})),
                );
                l.add_child(DummyView::new().fixed_height(2));
                l.add_child(
                    TextView::new("User Auth")
                        .align(Align::center())
                        .style(
                            Style::merge(&[Effect::Dim.into(), Effect::Underline.into()]),
                        ),
                );
                l.add_child(
                    TextView::new(
                        "The Client application will be needed to provide a userid and password. This is the recommended authentication type for internet or LAN servers.",
                    ),
                );
            }
        }
        #[allow(clippy::type_complexity)]
        pub fn auth_page(
            siv: &mut Cursive,
        ) -> NamedView<
            OnAuthStateUpdate<
                NamedView<ScrollView<NamedView<LinearLayout>>>,
                impl Fn(&mut Cursive) + 'static,
            >,
        > {
            let layout = LinearLayout::vertical().with_name("authpage");
            OnAuthStateUpdate::new(
                    ScrollView::new(layout)
                        .show_scrollbars(true)
                        .with_name("⚒ Authentication"),
                    siv,
                    |x: &mut Cursive| {
                        let state: &mut Ptr<Config> = x.user_data().unwrap();
                        let auth = state.authentication.clone();
                        _ = x
                            .call_on_name(
                                "authpage",
                                |layout: &mut LinearLayout| {
                                    layout.clear();
                                    match auth {
                                        Authentication::OpenToAll => open::render(layout),
                                        Authentication::TokenBased => token::render(layout),
                                        Authentication::Account {
                                            registration_allowed,
                                            max_users,
                                        } => user::render(layout, registration_allowed, max_users),
                                    }
                                },
                            );
                    },
                )
                .with_name("⚒ Authentication")
        }
    }
    mod bind {
        use cursive::{
            align::Align, theme::{Effect, Style},
            view::{Nameable, Resizable},
            views::{
                Button, Dialog, EditView, LinearLayout, NamedView, ResizedView,
                ScrollView, TextView,
            },
        };
        use crate::{structs::Config, ui::Ptr};
        pub fn bind(s: Ptr<Config>) -> ResizedView<Dialog> {
            Dialog::new()
                .title("Hosts and Ports")
                .content(ScrollView::new(gen_cnt(s.clone())).show_scrollbars(true))
                .button(
                    "Add",
                    |x| {
                        x.add_layer(add_binding());
                    },
                )
                .dismiss_button("Done")
                .full_screen()
        }
        fn add_binding() -> Dialog {
            Dialog::new()
                .content(
                    ScrollView::new(
                            LinearLayout::vertical()
                                .child(TextView::new("Enter hostname"))
                                .child(EditView::new().with_name("host"))
                                .child(TextView::new("Enter port"))
                                .child(
                                    EditView::new().max_content_width(5).with_name("port"),
                                ),
                        )
                        .show_scrollbars(true),
                )
                .button(
                    "Add",
                    |x| {
                        let host = x
                            .call_on_name("host", |x: &mut EditView| x.get_content())
                            .unwrap();
                        let port = x
                            .call_on_name("port", |x: &mut EditView| x.get_content())
                            .unwrap();
                        if let Ok(port) = port.parse::<u16>() {
                            let data: &mut Ptr<Config> = x.user_data().unwrap();
                            data.binds.push((host.to_string(), port));
                            let state = data.binds.clone();
                            x.call_on_name(
                                "bindings",
                                |l: &mut LinearLayout| {
                                    iterate_layout(l, &state);
                                },
                            );
                            x.pop_layer();
                            x.add_layer(
                                Dialog::around(TextView::new("Successfully updated!"))
                                    .title("Successful")
                                    .dismiss_button("Ok"),
                            );
                        } else {
                            x.add_layer(
                                Dialog::around(TextView::new("Invalid Port Provided"))
                                    .title("Error")
                                    .dismiss_button("Ok"),
                            );
                        }
                    },
                )
                .dismiss_button("Cancel")
        }
        fn gen_cnt(s: Ptr<Config>) -> NamedView<LinearLayout> {
            if s.binds.is_empty() {
                LinearLayout::vertical()
                    .child(TextView::new("No bindings detected"))
                    .with_name("bindings")
            } else {
                let mut layout = LinearLayout::vertical();
                iterate_layout(&mut layout, &s.binds);
                layout.with_name("bindings")
            }
        }
        fn iterate_layout(l: &mut LinearLayout, binds: &[(String, u16)]) {
            l.clear();
            if binds.is_empty() {
                l.add_child(TextView::new("No bindings detected"));
            } else {
                l.add_child(
                    LinearLayout::horizontal()
                        .child(
                            TextView::new("SNo")
                                .style(Style::merge(&[Effect::Dim.into()]))
                                .fixed_width(5),
                        )
                        .child(
                            TextView::new("Hostname")
                                .style(Style::merge(&[Effect::Dim.into()]))
                                .full_width(),
                        )
                        .child(
                            TextView::new("Port ")
                                .style(Style::merge(&[Effect::Dim.into()]))
                                .fixed_width(5),
                        )
                        .child(
                            TextView::new("")
                                .style(Style::merge(&[Effect::Dim.into()]))
                                .fixed_width(12),
                        ),
                );
            }
            binds
                .iter()
                .enumerate()
                .for_each(|(index, (host, port))| {
                    l.add_child(layout_child(index, host, port));
                });
        }
        fn layout_child(index: usize, host: &str, port: &u16) -> LinearLayout {
            LinearLayout::horizontal()
                .child(
                    TextView::new(
                            ::alloc::__export::must_use({
                                ::alloc::fmt::format(format_args!("{0}.", index + 1))
                            }),
                        )
                        .align(Align::center_left())
                        .fixed_width(5),
                )
                .child(TextView::new(host).full_width())
                .child(
                    TextView::new(port.to_string()).align(Align::center()).fixed_width(5),
                )
                .child(
                    Button::new_raw(
                            "✕ Remove",
                            move |x| {
                                x.with_user_data(|x: &mut Ptr<Config>| {
                                    x.binds.remove(index);
                                });
                                let state: &mut Ptr<Config> = x.user_data().unwrap();
                                let state = state.binds.clone();
                                x.call_on_name(
                                    "bindings",
                                    |l: &mut LinearLayout| {
                                        iterate_layout(l, &state);
                                    },
                                );
                            },
                        )
                        .fixed_width(12),
                )
        }
    }
    mod ollama {
        use cursive::{
            theme::{Effect, Style},
            view::{Nameable, Resizable},
            views::{
                Button, Dialog, DummyView, EditView, LinearLayout, NamedView, ScrollView,
                TextView,
            },
        };
        use crate::{structs::Config, ui::Ptr};
        mod model {
            use cursive::{
                align::Align, theme::{Effect, Style},
                view::{Nameable, Resizable},
                views::{
                    Button, Dialog, EditView, LinearLayout, NamedView, ResizedView,
                    ScrollView, TextView,
                },
            };
            use crate::{structs::Config, ui::Ptr};
            pub fn bind(s: Ptr<Config>, vision: bool) -> ResizedView<Dialog> {
                Dialog::new()
                    .title(
                        if vision { "Vision AI Models" } else { "Text-Based AI Models" },
                    )
                    .content(
                        ScrollView::new(gen_cnt(s.clone(), vision)).show_scrollbars(true),
                    )
                    .button(
                        "Add",
                        move |x| {
                            x.add_layer(add_model(vision));
                        },
                    )
                    .dismiss_button("Done")
                    .full_screen()
            }
            fn add_model(cv: bool) -> Dialog {
                Dialog::new()
                    .content(
                        ScrollView::new(
                                LinearLayout::vertical()
                                    .child(TextView::new("Enter model (eg. llava:7b)"))
                                    .child(EditView::new().with_name("model_name")),
                            )
                            .show_scrollbars(true),
                    )
                    .button(
                        "Add",
                        move |x| {
                            let model = x
                                .call_on_name(
                                    "model_name",
                                    |x: &mut EditView| x.get_content(),
                                )
                                .unwrap();
                            let data: &mut Ptr<Config> = x.user_data().unwrap();
                            let state_ = if cv {
                                &mut data.ollama.cvmodels
                            } else {
                                &mut data.ollama.txtmodels
                            };
                            state_.push(model.to_string().into_boxed_str());
                            let state = state_.clone();
                            x.call_on_name(
                                "models",
                                |l: &mut LinearLayout| {
                                    iterate_layout(l, &state, cv);
                                },
                            );
                            x.pop_layer();
                            x.add_layer(
                                Dialog::around(TextView::new("Successfully updated!"))
                                    .title("Successful")
                                    .dismiss_button("Ok"),
                            );
                        },
                    )
                    .dismiss_button("Cancel")
            }
            fn gen_cnt(s: Ptr<Config>, cv: bool) -> NamedView<LinearLayout> {
                let mut layout = LinearLayout::vertical();
                iterate_layout(
                    &mut layout,
                    if cv { &s.ollama.cvmodels } else { &s.ollama.txtmodels },
                    cv,
                );
                layout.with_name("models")
            }
            fn iterate_layout(l: &mut LinearLayout, binds: &[Box<str>], cv: bool) {
                l.clear();
                if binds.is_empty() {
                    l.add_child(TextView::new("No models detected"));
                } else {
                    l.add_child(
                        LinearLayout::horizontal()
                            .child(
                                TextView::new("SNo")
                                    .style(Style::merge(&[Effect::Dim.into()]))
                                    .fixed_width(5),
                            )
                            .child(
                                TextView::new("Model")
                                    .style(Style::merge(&[Effect::Dim.into()]))
                                    .full_width(),
                            )
                            .child(
                                TextView::new("")
                                    .style(Style::merge(&[Effect::Dim.into()]))
                                    .fixed_width(12),
                            ),
                    );
                }
                binds
                    .iter()
                    .enumerate()
                    .for_each(|(index, model)| {
                        l.add_child(layout_child(index, model, cv));
                    });
            }
            fn layout_child(index: usize, model: &str, cv: bool) -> LinearLayout {
                LinearLayout::horizontal()
                    .child(
                        TextView::new(
                                ::alloc::__export::must_use({
                                    ::alloc::fmt::format(format_args!("{0}.", index + 1))
                                }),
                            )
                            .align(Align::center_left())
                            .fixed_width(5),
                    )
                    .child(TextView::new(model).full_width())
                    .child(
                        Button::new_raw(
                                "✕ Remove",
                                move |x| {
                                    x.with_user_data(|x: &mut Ptr<Config>| {
                                        if cv {
                                            x.ollama.cvmodels.remove(index);
                                        } else {
                                            x.ollama.txtmodels.remove(index);
                                        }
                                    });
                                    let state: &mut Ptr<Config> = x.user_data().unwrap();
                                    let state = if cv {
                                        &mut state.ollama.cvmodels
                                    } else {
                                        &mut state.ollama.txtmodels
                                    }
                                        .clone();
                                    x.call_on_name(
                                        "models",
                                        |l: &mut LinearLayout| {
                                            iterate_layout(l, &state, cv);
                                        },
                                    );
                                },
                            )
                            .fixed_width(12),
                    )
            }
        }
        pub fn ollama_page(s: Ptr<Config>) -> NamedView<ScrollView<LinearLayout>> {
            let mut layout = LinearLayout::vertical();
            layout.add_child(server(s.clone()));
            layout.add_child(port(s.clone()));
            layout.add_child(DummyView::new().fixed_height(1));
            layout
                .add_child(
                    TextView::new("Models")
                        .style(Style::merge(&[Effect::Underline.into()])),
                );
            let s1 = s.clone();
            let s2 = s.clone();
            layout
                .add_child(
                    LinearLayout::horizontal()
                        .child(TextView::new("⊠ Vision enabled Models").full_width())
                        .child(
                            Button::new_raw(
                                "Manage ↗",
                                move |x| {
                                    x.add_layer(model::bind(s1.clone(), true));
                                },
                            ),
                        ),
                );
            layout
                .add_child(
                    LinearLayout::horizontal()
                        .child(TextView::new("⊟ Text only models").full_width())
                        .child(
                            Button::new_raw(
                                "Manage ↗",
                                move |x| {
                                    x.add_layer(model::bind(s2.clone(), false));
                                },
                            ),
                        ),
                );
            ScrollView::new(layout).show_scrollbars(true).with_name("🖧 Ollama")
        }
        fn server(s: Ptr<Config>) -> LinearLayout {
            LinearLayout::horizontal()
                .child(TextView::new("🖥 Ollama Server Hostname").full_width())
                .child(
                    Button::new_raw(
                            ::alloc::__export::must_use({
                                ::alloc::fmt::format(format_args!("[{0}]", &s.ollama.host))
                            }),
                            |x| {
                                x.add_layer(
                                    Dialog::around(
                                            EditView::new()
                                                .on_edit(|x, txt, _| {
                                                    let data: &mut Ptr<Config> = x.user_data().unwrap();
                                                    data.ollama.host = txt.into();
                                                    x.call_on_name(
                                                        "ollama_hostname",
                                                        |x: &mut Button| {
                                                            x.set_label_raw(
                                                                ::alloc::__export::must_use({
                                                                    ::alloc::fmt::format(format_args!("[{0}]", txt))
                                                                }),
                                                            );
                                                        },
                                                    );
                                                })
                                                .on_submit(|x, _| _ = x.pop_layer()),
                                        )
                                        .dismiss_button("Close")
                                        .title("Enter Ollama Hostname"),
                                );
                            },
                        )
                        .with_name("ollama_hostname"),
                )
        }
        fn port(s: Ptr<Config>) -> LinearLayout {
            LinearLayout::horizontal()
                .child(TextView::new("🕸 Ollama Server Port").full_width())
                .child(
                    Button::new_raw(
                            ::alloc::__export::must_use({
                                ::alloc::fmt::format(format_args!("<{0}>", &s.ollama.port))
                            }),
                            |x| {
                                x.add_layer(
                                    Dialog::around(
                                            EditView::new()
                                                .on_edit(|x, txt, _| {
                                                    let data: &mut Ptr<Config> = x.user_data().unwrap();
                                                    if let Ok(port) = txt.parse::<u16>() {
                                                        data.ollama.port = port;
                                                        x.call_on_name(
                                                            "ollama_port",
                                                            |x: &mut Button| {
                                                                x.set_label_raw(
                                                                    ::alloc::__export::must_use({
                                                                        ::alloc::fmt::format(format_args!("<{0}>", port))
                                                                    }),
                                                                );
                                                            },
                                                        );
                                                    }
                                                })
                                                .on_submit(|x, _| _ = x.pop_layer()),
                                        )
                                        .dismiss_button("Close")
                                        .title("Enter Ollama Hostname"),
                                );
                            },
                        )
                        .with_name("ollama_port"),
                )
        }
    }
    pub(crate) mod lazy {
        use std::sync::{Arc, Mutex};
        use cursive::view::{View, ViewWrapper};
        use cursive::{CbSink, Cursive, Printer, wrap_impl};
        use crate::structs::{Authentication, Config};
        use crate::ui::Ptr;
        pub struct OnAuthStateUpdate<V: View, F: Fn(&mut Cursive) + 'static> {
            inner: V,
            sink: CbSink,
            last_state: Arc<Mutex<Authentication>>,
            callback: F,
        }
        impl<V: View, F: Fn(&mut Cursive) + 'static> OnAuthStateUpdate<V, F> {
            pub fn new(inner: V, siv: &mut Cursive, callback: F) -> Self {
                Self {
                    inner,
                    sink: siv.cb_sink().clone(),
                    last_state: Arc::new(
                        Mutex::new({
                            let data: &mut Ptr<Config> = siv.user_data().unwrap();
                            match data.authentication {
                                Authentication::Account { .. } => Authentication::TokenBased,
                                Authentication::OpenToAll => Authentication::TokenBased,
                                Authentication::TokenBased => Authentication::OpenToAll,
                            }
                        }),
                    ),
                    callback,
                }
            }
        }
        impl<V: View, T: Fn(&mut Cursive) + Send + Sync + 'static> ViewWrapper
        for OnAuthStateUpdate<V, T> {
            type V = V;
            fn with_view<F, R>(&self, f: F) -> ::std::option::Option<R>
            where
                F: ::std::ops::FnOnce(&Self::V) -> R,
            {
                ::std::option::Option::Some(f(&self.inner))
            }
            fn with_view_mut<F, R>(&mut self, f: F) -> ::std::option::Option<R>
            where
                F: ::std::ops::FnOnce(&mut Self::V) -> R,
            {
                ::std::option::Option::Some(f(&mut self.inner))
            }
            fn into_inner(self) -> ::std::result::Result<Self::V, Self>
            where
                Self::V: ::std::marker::Sized,
            {
                ::std::result::Result::Ok(self.inner)
            }
            fn wrap_draw(&self, printer: &Printer) {
                let cb_ref = &self.callback;
                let cb_ref: &'static T = unsafe { &*(cb_ref as *const T) };
                let state = self.last_state.clone();
                _ = self
                    .sink
                    .clone()
                    .send(
                        Box::new(move |x| {
                            let mut lock = state.lock().unwrap();
                            let data: &mut Ptr<Config> = x.user_data().unwrap();
                            let auth = &data.authentication;
                            if &*lock != auth {
                                *lock = auth.clone();
                                (cb_ref)(x);
                            }
                        }),
                    );
                self.inner.draw(printer);
            }
        }
    }
    pub static ASYNC: LazyLock<Runtime> = LazyLock::new(|| {
        Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("Unable to build async runtime")
    });
    use bcrypt::{DEFAULT_COST, hash};
    fn general(l: &mut LinearLayout, c_: Ptr<Config>) {
        l.add_child(
            TextView::new("Welcome to AHQ-AI Server Configuration")
                .align(Align::center())
                .style(Style::merge(&[PaletteColor::Highlight.into()]))
                .fixed_height(3),
        );
        l.add_child(
            TextView::new(
                    ::alloc::__export::must_use({
                        ::alloc::fmt::format(format_args!("AHQ AI Server v{0}", "0.1.0"))
                    }),
                )
                .align(Align::top_right())
                .style(Style::merge(&[Effect::Dim.into()]))
                .fixed_height(2),
        );
        l.add_child(
            TextView::new("Quick Guide").style(Style::merge(&[Effect::Underline.into()])),
        );
        l.add_child(TextView::new("» Use ← ↑ → ↓ to navigate"));
        l.add_child(TextView::new("» Press <Enter> key to interact with buttons"));
        l.add_child(
            TextView::new("» You can also use mouse to interact with buttons or tabs"),
        );
        l.add_child(TextView::new("» You can also scroll with the mouse scrollbar"));
        l.add_child(
            TextView::new(
                "» <q> key, <Ctrl+C> or going to <Save> tab updates the config file",
            ),
        );
        l.add_child(DummyView::new().fixed_height(1).full_width());
        l.add_child(
            TextView::new("General Settings")
                .style(Style::merge(&[Effect::Underline.into()])),
        );
        l.add_child(binds(c_.clone()));
        l.add_child(
            LinearLayout::horizontal()
                .child(TextView::new("⚒ Administrator Password").full_width())
                .child(
                    Button::new_raw(
                        "Set/Update ↗",
                        move |x| {
                            x.add_layer(
                                Dialog::around(
                                        LinearLayout::vertical()
                                            .child(
                                                EditView::new()
                                                    .secret()
                                                    .on_submit(|x, txt| {
                                                        let c_: &mut Ptr<Config> = x.user_data().unwrap();
                                                        c_.admin_pass_hash = Some(
                                                            hash(txt, DEFAULT_COST).expect("Unknown error"),
                                                        );
                                                        x.pop_layer();
                                                    }),
                                            )
                                            .child(TextView::new("Press Enter key to submit"))
                                            .child(
                                                TextView::new(
                                                    "The UI might hand for a moment due to hashing algorithm",
                                                ),
                                            ),
                                    )
                                    .title("New Administrator Password")
                                    .dismiss_button("Cancel"),
                            );
                        },
                    ),
                )
                .child(DummyView::new().fixed_width(2))
                .child(
                    Button::new_raw(
                        "Remove ↗",
                        move |x| {
                            let c_: &mut Ptr<Config> = x.user_data().unwrap();
                            c_.admin_pass_hash = None;
                        },
                    ),
                ),
        );
        l.add_child(
            LinearLayout::horizontal()
                .child(TextView::new("⚒ Authentication Type").full_width())
                .child(
                    Button::new_raw(
                            ::alloc::__export::must_use({
                                ::alloc::fmt::format(
                                    format_args!(
                                        "{0} ↗",
                                        match c_.authentication {
                                            Authentication::OpenToAll => "No Auth",
                                            Authentication::TokenBased => "Token",
                                            Authentication::Account { .. } => "Account",
                                        },
                                    ),
                                )
                            }),
                            move |x| {
                                x.add_layer(
                                    Dialog::around(
                                            SelectView::new()
                                                .item("No Auth (OpenToAll)", 0u8)
                                                .item("Token (TokenBased)", 1u8)
                                                .item("Account (Account)", 2u8)
                                                .on_submit(|x, bit| {
                                                    let c_: &mut Ptr<Config> = x.user_data().unwrap();
                                                    c_.authentication = match bit {
                                                        0 => Authentication::OpenToAll,
                                                        1 => Authentication::TokenBased,
                                                        2 => {
                                                            Authentication::Account {
                                                                registration_allowed: true,
                                                                max_users: None,
                                                            }
                                                        }
                                                        _ => {
                                                            ::core::panicking::panic(
                                                                "internal error: entered unreachable code",
                                                            )
                                                        }
                                                    };
                                                    let label = ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!(
                                                                "{0} ↗",
                                                                match c_.authentication {
                                                                    Authentication::OpenToAll => "No Auth",
                                                                    Authentication::TokenBased => "Token",
                                                                    Authentication::Account { .. } => "Account",
                                                                },
                                                            ),
                                                        )
                                                    });
                                                    x.call_on_name(
                                                        "auth_type",
                                                        move |x: &mut Button| {
                                                            x.set_label_raw(label);
                                                        },
                                                    );
                                                    x.pop_layer();
                                                })
                                                .with_name("themeselect"),
                                        )
                                        .title("Authentication Type")
                                        .dismiss_button("Cancel"),
                                );
                            },
                        )
                        .with_name("auth_type"),
                ),
        );
        l.add_child(
            LinearLayout::horizontal()
                .child(TextView::new("🖌 TUI Theme").full_width())
                .child(
                    Button::new_raw(
                        "Select ↗",
                        move |x| {
                            x.add_layer(
                                Dialog::around(
                                        SelectView::new()
                                            .item("Default Theme", 0u8)
                                            .item("Monochrome Theme", 1u8)
                                            .on_submit(|x, bit| {
                                                x.set_theme(
                                                    match bit {
                                                        0 => Theme::retro(),
                                                        1 => Theme::terminal_default(),
                                                        _ => {
                                                            ::core::panicking::panic(
                                                                "internal error: entered unreachable code",
                                                            )
                                                        }
                                                    },
                                                );
                                                x.call_on_name(
                                                    "themeselect",
                                                    |x: &mut SelectView| { x.set_selection(*bit as usize) },
                                                );
                                                x.pop_layer();
                                                if let Some(mut home) = home_dir() {
                                                    home.push(".ahqaiservertheme");
                                                    _ = fs::write(
                                                        &home,
                                                        <[_]>::into_vec(::alloc::boxed::box_new([*bit])),
                                                    );
                                                }
                                            })
                                            .with_name("themeselect"),
                                    )
                                    .title("Select Theme")
                                    .dismiss_button("Cancel"),
                            );
                        },
                    ),
                ),
        );
    }
    pub fn ui() {
        let mut config = ASYNC.block_on(async { Config::new_or_default().await });
        let initial_config = config.clone();
        let mut siv = Cursive::new();
        let c_ = Ptr(&mut config);
        let prompt = config.binds.is_empty();
        siv.set_theme(Theme::retro());
        if let Some(mut home) = home_dir() {
            home.push(".ahqaiservertheme");
            if let Ok(x) = fs::read(&home) {
                let first_bit = &x[0];
                match *first_bit {
                    0 => {}
                    1 => siv.set_theme(Theme::terminal_default()),
                    _ => {}
                }
            }
        }
        siv.set_user_data(c_.clone());
        siv.set_global_callback('q', |x| x.quit());
        let mut tabs = TabPanel::new();
        let mut gene = LinearLayout::vertical();
        general(&mut gene, c_.clone());
        tabs.add_tab(
            ScrollView::new(gene).show_scrollbars(true).with_name("䷸ General"),
        );
        tabs.add_tab(ollama::ollama_page(c_.clone()));
        tabs.add_tab(auth::auth_page(&mut siv));
        tabs.add_tab(
            ScrollView::new(
                    LinearLayout::vertical()
                        .child(
                            Button::new_raw(
                                "🖴 Save Changes and Exit",
                                |x| {
                                    x.quit();
                                },
                            ),
                        )
                        .child(
                            Button::new_raw(
                                "🖪 Backup current Config",
                                move |x| {
                                    let con: &mut Ptr<Config> = x.user_data().unwrap();
                                    let con = unsafe { &*con.0 };
                                    let file = ::alloc::__export::must_use({
                                        ::alloc::fmt::format(
                                            format_args!(
                                                "./config.bak.{0}.json",
                                                SystemTime::now()
                                                    .duration_since(UNIX_EPOCH)
                                                    .unwrap()
                                                    .as_secs(),
                                            ),
                                        )
                                    });
                                    fs::write(&file, to_string_pretty(con).unwrap()).unwrap();
                                    x.add_layer(
                                        Dialog::new()
                                            .title("Successful")
                                            .content(
                                                TextView::new(
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!(
                                                                "Successfully backed up initial config at {0}",
                                                                file,
                                                            ),
                                                        )
                                                    }),
                                                ),
                                            )
                                            .dismiss_button("Ok"),
                                    );
                                },
                            ),
                        )
                        .child(
                            Button::new_raw(
                                "🖪 Backup Initial Config",
                                move |x| {
                                    let file = ::alloc::__export::must_use({
                                        ::alloc::fmt::format(
                                            format_args!(
                                                "./config.bak.{0}.json",
                                                SystemTime::now()
                                                    .duration_since(UNIX_EPOCH)
                                                    .unwrap()
                                                    .as_secs(),
                                            ),
                                        )
                                    });
                                    fs::write(&file, to_string_pretty(&initial_config).unwrap())
                                        .unwrap();
                                    x.add_layer(
                                        Dialog::new()
                                            .title("Successful")
                                            .content(
                                                TextView::new(
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(
                                                            format_args!(
                                                                "Successfully backed up initial config at {0}",
                                                                file,
                                                            ),
                                                        )
                                                    }),
                                                ),
                                            )
                                            .dismiss_button("Ok"),
                                    );
                                },
                            ),
                        ),
                )
                .show_scrollbars(true)
                .with_name("🖫 Save"),
        );
        _ = tabs.set_active_tab("䷸ General");
        siv.add_layer(
            Dialog::around(tabs.with_name("tabs"))
                .title("AHQ-AI Server Configuration Utility")
                .full_screen(),
        );
        if prompt {
            siv.add_layer(
                Dialog::around(
                        TextView::new(
                            "Please set up hostnames and ports under `☸ General`!",
                        ),
                    )
                    .title("Important")
                    .dismiss_button("Ok"),
            );
        }
        siv.run();
        ASYNC
            .block_on(async move {
                config.save_config().await.expect("Unable to save edited config");
            });
    }
    pub struct Ptr<T>(*mut T);
    #[automatically_derived]
    impl<T: ::core::fmt::Debug> ::core::fmt::Debug for Ptr<T> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Ptr", &&self.0)
        }
    }
    #[automatically_derived]
    impl<T: ::core::clone::Clone> ::core::clone::Clone for Ptr<T> {
        #[inline]
        fn clone(&self) -> Ptr<T> {
            Ptr(::core::clone::Clone::clone(&self.0))
        }
    }
    #[automatically_derived]
    impl<T: ::core::marker::Copy> ::core::marker::Copy for Ptr<T> {}
    unsafe impl<T> Send for Ptr<T> {}
    unsafe impl<T> Sync for Ptr<T> {}
    impl<T> Deref for Ptr<T> {
        type Target = T;
        fn deref(&self) -> &Self::Target {
            unsafe { &*self.0 }
        }
    }
    impl<T> DerefMut for Ptr<T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { &mut *self.0 }
        }
    }
    fn binds(s: Ptr<Config>) -> LinearLayout {
        LinearLayout::horizontal()
            .child(TextView::new("🖳 Hostnames and Ports").full_width())
            .child(
                Button::new_raw(
                        "View ↗",
                        move |x| {
                            x.add_layer(bind::bind(s.clone()));
                        },
                    )
                    .with_name("host"),
            )
    }
}
pub mod auth {
    use moka::future::Cache;
    use rand::{Rng, seq::IndexedRandom};
    use serde_json::Deserializer;
    use std::{io::BufReader, sync::Arc, time::{Duration, SystemTime, UNIX_EPOCH}};
    use base64::{engine::general_purpose, Engine as _};
    use tokio::{fs::File, task::spawn_blocking};
    use bcrypt::{DEFAULT_COST, hash, verify};
    use crate::structs::{Authentication, Config, error::Returns};
    #[allow(dead_code)]
    pub struct AuthSessionManager {
        sessions: Cache<Box<str>, Arc<Box<str>>>,
        accounts: Cache<Box<str>, Option<Box<str>>>,
        token: bool,
    }
    pub type Account = (Box<str>, Box<str>);
    impl AuthSessionManager {
        pub async fn create(config: &Config) -> Self {
            let sessions = Cache::builder()
                .time_to_live(Duration::from_days(60))
                .build();
            let accounts = Cache::builder().build();
            let token = #[allow(non_exhaustive_omitted_patterns)]
            match config.authentication {
                Authentication::TokenBased => true,
                _ => false,
            };
            if token {
                if let Ok(x) = File::open("./tokens.json").await {
                    let x = x.into_std().await;
                    let x = BufReader::new(x);
                    let list = Deserializer::from_reader(x)
                        .into_iter::<Box<str>>()
                        .map(|x| x.unwrap())
                        .collect::<Vec<_>>();
                    for token_hash in list {
                        accounts.insert(token_hash, None).await;
                    }
                }
            } else if let Ok(x) = File::open("./accounts.json").await {
                let x = x.into_std().await;
                let x = BufReader::new(x);
                let list = Deserializer::from_reader(x)
                    .into_iter::<Account>()
                    .map(|x| x.unwrap())
                    .collect::<Vec<_>>();
                for (userid, pwd_hash) in list {
                    accounts.insert(userid, Some(pwd_hash)).await;
                }
            }
            Self { sessions, accounts, token }
        }
        pub async fn before_exit() {}
    }
    pub fn now() -> u64 {
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
    }
    pub async fn create_hash(pass: &str) -> Returns<String> {
        let pass: &'static str = unsafe { &*(pass as *const str) };
        Ok(spawn_blocking(move || hash(pass, DEFAULT_COST)).await??)
    }
    pub async fn verify_hash(pass: &str, hash: &str) -> Returns<bool> {
        let hash: &'static str = unsafe { &*(hash as *const str) };
        let pass: &'static str = unsafe { &*(pass as *const str) };
        Ok(spawn_blocking(move || verify(pass, hash)).await??)
    }
    pub const VALUES: [char; 64] = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
        'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F',
        'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V',
        'W', 'X', 'Y', 'Z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/',
    ];
    pub type Hashed = String;
    pub fn gen_random_token() -> Returns<(String, Hashed)> {
        let token = VALUES.choose_multiple(&mut rand::rng(), 128).collect::<String>();
        let hashed = hash(&token, DEFAULT_COST)?;
        Ok((token, hashed))
    }
    pub async fn parse_session_token_async(token: &str) -> Returns<Vec<u8>> {
        let token: &'static str = unsafe { &*(token as *const str) };
        spawn_blocking(move || parse_session_token(token)).await?
    }
    pub async fn gen_session_token_async() -> Returns<(String, Hashed)> {
        spawn_blocking(gen_session_token).await?
    }
    pub fn gen_session_token() -> Returns<(String, Hashed)> {
        let mut rng = rand::rng();
        let token = ::alloc::vec::from_elem(rng.random::<u8>(), 128);
        let token = general_purpose::URL_SAFE_NO_PAD.encode(&token);
        let hashed = hash(&token, DEFAULT_COST)?;
        Ok((token, hashed))
    }
    pub fn parse_session_token(token: &str) -> Returns<Vec<u8>> {
        Ok(general_purpose::URL_SAFE_NO_PAD.decode(token)?)
    }
}
pub(crate) mod structs {
    use serde::{Deserialize, Serialize};
    use serde_json::{from_str, to_string_pretty};
    use tokio::fs;
    use crate::structs::error::Returns;
    pub mod error {
        use actix_web::http::StatusCode;
        use base64::DecodeError;
        use thiserror::Error;
        use bcrypt::BcryptError;
        use serde_json::Error as SerdeError;
        use std::io::Error as StdError;
        use tokio::task::JoinError;
        pub enum ServerError {
            #[error(transparent)]
            Serde(#[from] SerdeError),
            #[error(transparent)]
            Base64(#[from] DecodeError),
            #[error(transparent)]
            TokioJoinError(#[from] JoinError),
            #[error(transparent)]
            Std(#[from] StdError),
            #[error("Failed to convert OS String to String")]
            StringConvertErr,
            #[error(transparent)]
            BcryptErr(#[from] BcryptError),
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ServerError {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    ServerError::Serde(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Serde",
                            &__self_0,
                        )
                    }
                    ServerError::Base64(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Base64",
                            &__self_0,
                        )
                    }
                    ServerError::TokioJoinError(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "TokioJoinError",
                            &__self_0,
                        )
                    }
                    ServerError::Std(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Std",
                            &__self_0,
                        )
                    }
                    ServerError::StringConvertErr => {
                        ::core::fmt::Formatter::write_str(f, "StringConvertErr")
                    }
                    ServerError::BcryptErr(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "BcryptErr",
                            &__self_0,
                        )
                    }
                }
            }
        }
        #[allow(unused_qualifications)]
        #[automatically_derived]
        impl ::thiserror::__private17::Error for ServerError {
            fn source(
                &self,
            ) -> ::core::option::Option<
                &(dyn ::thiserror::__private17::Error + 'static),
            > {
                use ::thiserror::__private17::AsDynError as _;
                #[allow(deprecated)]
                match self {
                    ServerError::Serde { 0: transparent } => {
                        ::thiserror::__private17::Error::source(
                            transparent.as_dyn_error(),
                        )
                    }
                    ServerError::Base64 { 0: transparent } => {
                        ::thiserror::__private17::Error::source(
                            transparent.as_dyn_error(),
                        )
                    }
                    ServerError::TokioJoinError { 0: transparent } => {
                        ::thiserror::__private17::Error::source(
                            transparent.as_dyn_error(),
                        )
                    }
                    ServerError::Std { 0: transparent } => {
                        ::thiserror::__private17::Error::source(
                            transparent.as_dyn_error(),
                        )
                    }
                    ServerError::StringConvertErr { .. } => ::core::option::Option::None,
                    ServerError::BcryptErr { 0: transparent } => {
                        ::thiserror::__private17::Error::source(
                            transparent.as_dyn_error(),
                        )
                    }
                }
            }
        }
        #[allow(unused_qualifications)]
        #[automatically_derived]
        impl ::core::fmt::Display for ServerError {
            fn fmt(
                &self,
                __formatter: &mut ::core::fmt::Formatter,
            ) -> ::core::fmt::Result {
                #[allow(unused_variables, deprecated, clippy::used_underscore_binding)]
                match self {
                    ServerError::Serde(_0) => ::core::fmt::Display::fmt(_0, __formatter),
                    ServerError::Base64(_0) => ::core::fmt::Display::fmt(_0, __formatter),
                    ServerError::TokioJoinError(_0) => {
                        ::core::fmt::Display::fmt(_0, __formatter)
                    }
                    ServerError::Std(_0) => ::core::fmt::Display::fmt(_0, __formatter),
                    ServerError::StringConvertErr {} => {
                        __formatter.write_str("Failed to convert OS String to String")
                    }
                    ServerError::BcryptErr(_0) => {
                        ::core::fmt::Display::fmt(_0, __formatter)
                    }
                }
            }
        }
        #[allow(
            deprecated,
            unused_qualifications,
            clippy::elidable_lifetime_names,
            clippy::needless_lifetimes,
        )]
        #[automatically_derived]
        impl ::core::convert::From<SerdeError> for ServerError {
            fn from(source: SerdeError) -> Self {
                ServerError::Serde { 0: source }
            }
        }
        #[allow(
            deprecated,
            unused_qualifications,
            clippy::elidable_lifetime_names,
            clippy::needless_lifetimes,
        )]
        #[automatically_derived]
        impl ::core::convert::From<DecodeError> for ServerError {
            fn from(source: DecodeError) -> Self {
                ServerError::Base64 { 0: source }
            }
        }
        #[allow(
            deprecated,
            unused_qualifications,
            clippy::elidable_lifetime_names,
            clippy::needless_lifetimes,
        )]
        #[automatically_derived]
        impl ::core::convert::From<JoinError> for ServerError {
            fn from(source: JoinError) -> Self {
                ServerError::TokioJoinError {
                    0: source,
                }
            }
        }
        #[allow(
            deprecated,
            unused_qualifications,
            clippy::elidable_lifetime_names,
            clippy::needless_lifetimes,
        )]
        #[automatically_derived]
        impl ::core::convert::From<StdError> for ServerError {
            fn from(source: StdError) -> Self {
                ServerError::Std { 0: source }
            }
        }
        #[allow(
            deprecated,
            unused_qualifications,
            clippy::elidable_lifetime_names,
            clippy::needless_lifetimes,
        )]
        #[automatically_derived]
        impl ::core::convert::From<BcryptError> for ServerError {
            fn from(source: BcryptError) -> Self {
                ServerError::BcryptErr {
                    0: source,
                }
            }
        }
        impl actix_web::error::ResponseError for ServerError {
            fn status_code(&self) -> actix_web::http::StatusCode {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
        pub type Returns<T> = Result<T, ServerError>;
    }
    pub struct Config {
        #[serde(default = "def_bind")]
        pub binds: Vec<(String, u16)>,
        pub admin_pass_hash: Option<String>,
        pub ollama: OllamaConfiguration,
        pub authentication: Authentication,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Config {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field4_finish(
                f,
                "Config",
                "binds",
                &self.binds,
                "admin_pass_hash",
                &self.admin_pass_hash,
                "ollama",
                &self.ollama,
                "authentication",
                &&self.authentication,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Config {
        #[inline]
        fn clone(&self) -> Config {
            Config {
                binds: ::core::clone::Clone::clone(&self.binds),
                admin_pass_hash: ::core::clone::Clone::clone(&self.admin_pass_hash),
                ollama: ::core::clone::Clone::clone(&self.ollama),
                authentication: ::core::clone::Clone::clone(&self.authentication),
            }
        }
    }
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Config {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private228::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "Config",
                    false as usize + 1 + 1 + 1 + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "binds",
                    &self.binds,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "admin_pass_hash",
                    &self.admin_pass_hash,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "ollama",
                    &self.ollama,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "authentication",
                    &self.authentication,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Config {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private228::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private228::Formatter,
                    ) -> _serde::__private228::fmt::Result {
                        _serde::__private228::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private228::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private228::Ok(__Field::__field0),
                            1u64 => _serde::__private228::Ok(__Field::__field1),
                            2u64 => _serde::__private228::Ok(__Field::__field2),
                            3u64 => _serde::__private228::Ok(__Field::__field3),
                            _ => _serde::__private228::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private228::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "binds" => _serde::__private228::Ok(__Field::__field0),
                            "admin_pass_hash" => {
                                _serde::__private228::Ok(__Field::__field1)
                            }
                            "ollama" => _serde::__private228::Ok(__Field::__field2),
                            "authentication" => {
                                _serde::__private228::Ok(__Field::__field3)
                            }
                            _ => _serde::__private228::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private228::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"binds" => _serde::__private228::Ok(__Field::__field0),
                            b"admin_pass_hash" => {
                                _serde::__private228::Ok(__Field::__field1)
                            }
                            b"ollama" => _serde::__private228::Ok(__Field::__field2),
                            b"authentication" => {
                                _serde::__private228::Ok(__Field::__field3)
                            }
                            _ => _serde::__private228::Ok(__Field::__ignore),
                        }
                    }
                }
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private228::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private228::PhantomData<Config>,
                    lifetime: _serde::__private228::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Config;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private228::Formatter,
                    ) -> _serde::__private228::fmt::Result {
                        _serde::__private228::Formatter::write_str(
                            __formatter,
                            "struct Config",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private228::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match _serde::de::SeqAccess::next_element::<
                            Vec<(String, u16)>,
                        >(&mut __seq)? {
                            _serde::__private228::Some(__value) => __value,
                            _serde::__private228::None => def_bind(),
                        };
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            Option<String>,
                        >(&mut __seq)? {
                            _serde::__private228::Some(__value) => __value,
                            _serde::__private228::None => {
                                return _serde::__private228::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct Config with 4 elements",
                                    ),
                                );
                            }
                        };
                        let __field2 = match _serde::de::SeqAccess::next_element::<
                            OllamaConfiguration,
                        >(&mut __seq)? {
                            _serde::__private228::Some(__value) => __value,
                            _serde::__private228::None => {
                                return _serde::__private228::Err(
                                    _serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct Config with 4 elements",
                                    ),
                                );
                            }
                        };
                        let __field3 = match _serde::de::SeqAccess::next_element::<
                            Authentication,
                        >(&mut __seq)? {
                            _serde::__private228::Some(__value) => __value,
                            _serde::__private228::None => {
                                return _serde::__private228::Err(
                                    _serde::de::Error::invalid_length(
                                        3usize,
                                        &"struct Config with 4 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private228::Ok(Config {
                            binds: __field0,
                            admin_pass_hash: __field1,
                            ollama: __field2,
                            authentication: __field3,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private228::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private228::Option<
                            Vec<(String, u16)>,
                        > = _serde::__private228::None;
                        let mut __field1: _serde::__private228::Option<Option<String>> = _serde::__private228::None;
                        let mut __field2: _serde::__private228::Option<
                            OllamaConfiguration,
                        > = _serde::__private228::None;
                        let mut __field3: _serde::__private228::Option<Authentication> = _serde::__private228::None;
                        while let _serde::__private228::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private228::Option::is_some(&__field0) {
                                        return _serde::__private228::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("binds"),
                                        );
                                    }
                                    __field0 = _serde::__private228::Some(
                                        _serde::de::MapAccess::next_value::<
                                            Vec<(String, u16)>,
                                        >(&mut __map)?,
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private228::Option::is_some(&__field1) {
                                        return _serde::__private228::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "admin_pass_hash",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private228::Some(
                                        _serde::de::MapAccess::next_value::<
                                            Option<String>,
                                        >(&mut __map)?,
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private228::Option::is_some(&__field2) {
                                        return _serde::__private228::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("ollama"),
                                        );
                                    }
                                    __field2 = _serde::__private228::Some(
                                        _serde::de::MapAccess::next_value::<
                                            OllamaConfiguration,
                                        >(&mut __map)?,
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::__private228::Option::is_some(&__field3) {
                                        return _serde::__private228::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "authentication",
                                            ),
                                        );
                                    }
                                    __field3 = _serde::__private228::Some(
                                        _serde::de::MapAccess::next_value::<
                                            Authentication,
                                        >(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private228::Some(__field0) => __field0,
                            _serde::__private228::None => def_bind(),
                        };
                        let __field1 = match __field1 {
                            _serde::__private228::Some(__field1) => __field1,
                            _serde::__private228::None => {
                                _serde::__private228::de::missing_field("admin_pass_hash")?
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private228::Some(__field2) => __field2,
                            _serde::__private228::None => {
                                _serde::__private228::de::missing_field("ollama")?
                            }
                        };
                        let __field3 = match __field3 {
                            _serde::__private228::Some(__field3) => __field3,
                            _serde::__private228::None => {
                                _serde::__private228::de::missing_field("authentication")?
                            }
                        };
                        _serde::__private228::Ok(Config {
                            binds: __field0,
                            admin_pass_hash: __field1,
                            ollama: __field2,
                            authentication: __field3,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &[
                    "binds",
                    "admin_pass_hash",
                    "ollama",
                    "authentication",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "Config",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private228::PhantomData::<Config>,
                        lifetime: _serde::__private228::PhantomData,
                    },
                )
            }
        }
    };
    fn def_bind() -> Vec<(String, u16)> {
        <[_]>::into_vec(
            ::alloc::boxed::box_new([
                ("0.0.0.0".to_string(), 3000),
                ("localhost".to_string(), 3000),
            ]),
        )
    }
    pub struct OllamaConfiguration {
        pub host: Box<str>,
        pub port: u16,
        pub cvmodels: Vec<Box<str>>,
        pub txtmodels: Vec<Box<str>>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for OllamaConfiguration {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field4_finish(
                f,
                "OllamaConfiguration",
                "host",
                &self.host,
                "port",
                &self.port,
                "cvmodels",
                &self.cvmodels,
                "txtmodels",
                &&self.txtmodels,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for OllamaConfiguration {
        #[inline]
        fn clone(&self) -> OllamaConfiguration {
            OllamaConfiguration {
                host: ::core::clone::Clone::clone(&self.host),
                port: ::core::clone::Clone::clone(&self.port),
                cvmodels: ::core::clone::Clone::clone(&self.cvmodels),
                txtmodels: ::core::clone::Clone::clone(&self.txtmodels),
            }
        }
    }
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for OllamaConfiguration {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private228::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "OllamaConfiguration",
                    false as usize + 1 + 1 + 1 + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "host",
                    &self.host,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "port",
                    &self.port,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "cvmodels",
                    &self.cvmodels,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "txtmodels",
                    &self.txtmodels,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for OllamaConfiguration {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private228::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private228::Formatter,
                    ) -> _serde::__private228::fmt::Result {
                        _serde::__private228::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private228::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private228::Ok(__Field::__field0),
                            1u64 => _serde::__private228::Ok(__Field::__field1),
                            2u64 => _serde::__private228::Ok(__Field::__field2),
                            3u64 => _serde::__private228::Ok(__Field::__field3),
                            _ => _serde::__private228::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private228::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "host" => _serde::__private228::Ok(__Field::__field0),
                            "port" => _serde::__private228::Ok(__Field::__field1),
                            "cvmodels" => _serde::__private228::Ok(__Field::__field2),
                            "txtmodels" => _serde::__private228::Ok(__Field::__field3),
                            _ => _serde::__private228::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private228::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"host" => _serde::__private228::Ok(__Field::__field0),
                            b"port" => _serde::__private228::Ok(__Field::__field1),
                            b"cvmodels" => _serde::__private228::Ok(__Field::__field2),
                            b"txtmodels" => _serde::__private228::Ok(__Field::__field3),
                            _ => _serde::__private228::Ok(__Field::__ignore),
                        }
                    }
                }
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private228::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private228::PhantomData<OllamaConfiguration>,
                    lifetime: _serde::__private228::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = OllamaConfiguration;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private228::Formatter,
                    ) -> _serde::__private228::fmt::Result {
                        _serde::__private228::Formatter::write_str(
                            __formatter,
                            "struct OllamaConfiguration",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private228::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match _serde::de::SeqAccess::next_element::<
                            Box<str>,
                        >(&mut __seq)? {
                            _serde::__private228::Some(__value) => __value,
                            _serde::__private228::None => {
                                return _serde::__private228::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct OllamaConfiguration with 4 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            u16,
                        >(&mut __seq)? {
                            _serde::__private228::Some(__value) => __value,
                            _serde::__private228::None => {
                                return _serde::__private228::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct OllamaConfiguration with 4 elements",
                                    ),
                                );
                            }
                        };
                        let __field2 = match _serde::de::SeqAccess::next_element::<
                            Vec<Box<str>>,
                        >(&mut __seq)? {
                            _serde::__private228::Some(__value) => __value,
                            _serde::__private228::None => {
                                return _serde::__private228::Err(
                                    _serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct OllamaConfiguration with 4 elements",
                                    ),
                                );
                            }
                        };
                        let __field3 = match _serde::de::SeqAccess::next_element::<
                            Vec<Box<str>>,
                        >(&mut __seq)? {
                            _serde::__private228::Some(__value) => __value,
                            _serde::__private228::None => {
                                return _serde::__private228::Err(
                                    _serde::de::Error::invalid_length(
                                        3usize,
                                        &"struct OllamaConfiguration with 4 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private228::Ok(OllamaConfiguration {
                            host: __field0,
                            port: __field1,
                            cvmodels: __field2,
                            txtmodels: __field3,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private228::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private228::Option<Box<str>> = _serde::__private228::None;
                        let mut __field1: _serde::__private228::Option<u16> = _serde::__private228::None;
                        let mut __field2: _serde::__private228::Option<Vec<Box<str>>> = _serde::__private228::None;
                        let mut __field3: _serde::__private228::Option<Vec<Box<str>>> = _serde::__private228::None;
                        while let _serde::__private228::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private228::Option::is_some(&__field0) {
                                        return _serde::__private228::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("host"),
                                        );
                                    }
                                    __field0 = _serde::__private228::Some(
                                        _serde::de::MapAccess::next_value::<Box<str>>(&mut __map)?,
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private228::Option::is_some(&__field1) {
                                        return _serde::__private228::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("port"),
                                        );
                                    }
                                    __field1 = _serde::__private228::Some(
                                        _serde::de::MapAccess::next_value::<u16>(&mut __map)?,
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private228::Option::is_some(&__field2) {
                                        return _serde::__private228::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "cvmodels",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::__private228::Some(
                                        _serde::de::MapAccess::next_value::<
                                            Vec<Box<str>>,
                                        >(&mut __map)?,
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::__private228::Option::is_some(&__field3) {
                                        return _serde::__private228::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "txtmodels",
                                            ),
                                        );
                                    }
                                    __field3 = _serde::__private228::Some(
                                        _serde::de::MapAccess::next_value::<
                                            Vec<Box<str>>,
                                        >(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private228::Some(__field0) => __field0,
                            _serde::__private228::None => {
                                _serde::__private228::de::missing_field("host")?
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private228::Some(__field1) => __field1,
                            _serde::__private228::None => {
                                _serde::__private228::de::missing_field("port")?
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private228::Some(__field2) => __field2,
                            _serde::__private228::None => {
                                _serde::__private228::de::missing_field("cvmodels")?
                            }
                        };
                        let __field3 = match __field3 {
                            _serde::__private228::Some(__field3) => __field3,
                            _serde::__private228::None => {
                                _serde::__private228::de::missing_field("txtmodels")?
                            }
                        };
                        _serde::__private228::Ok(OllamaConfiguration {
                            host: __field0,
                            port: __field1,
                            cvmodels: __field2,
                            txtmodels: __field3,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &[
                    "host",
                    "port",
                    "cvmodels",
                    "txtmodels",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "OllamaConfiguration",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private228::PhantomData::<OllamaConfiguration>,
                        lifetime: _serde::__private228::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    impl ::core::default::Default for OllamaConfiguration {
        #[inline]
        fn default() -> OllamaConfiguration {
            OllamaConfiguration {
                host: ::core::default::Default::default(),
                port: ::core::default::Default::default(),
                cvmodels: ::core::default::Default::default(),
                txtmodels: ::core::default::Default::default(),
            }
        }
    }
    #[serde(tag = "kind")]
    pub enum Authentication {
        OpenToAll,
        TokenBased,
        Account { registration_allowed: bool, max_users: Option<u64> },
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Authentication {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                Authentication::OpenToAll => {
                    ::core::fmt::Formatter::write_str(f, "OpenToAll")
                }
                Authentication::TokenBased => {
                    ::core::fmt::Formatter::write_str(f, "TokenBased")
                }
                Authentication::Account {
                    registration_allowed: __self_0,
                    max_users: __self_1,
                } => {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "Account",
                        "registration_allowed",
                        __self_0,
                        "max_users",
                        &__self_1,
                    )
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Authentication {
        #[inline]
        fn clone(&self) -> Authentication {
            match self {
                Authentication::OpenToAll => Authentication::OpenToAll,
                Authentication::TokenBased => Authentication::TokenBased,
                Authentication::Account {
                    registration_allowed: __self_0,
                    max_users: __self_1,
                } => {
                    Authentication::Account {
                        registration_allowed: ::core::clone::Clone::clone(__self_0),
                        max_users: ::core::clone::Clone::clone(__self_1),
                    }
                }
            }
        }
    }
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Authentication {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private228::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                match *self {
                    Authentication::OpenToAll => {
                        let mut __struct = _serde::Serializer::serialize_struct(
                            __serializer,
                            "Authentication",
                            1,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __struct,
                            "kind",
                            "OpenToAll",
                        )?;
                        _serde::ser::SerializeStruct::end(__struct)
                    }
                    Authentication::TokenBased => {
                        let mut __struct = _serde::Serializer::serialize_struct(
                            __serializer,
                            "Authentication",
                            1,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __struct,
                            "kind",
                            "TokenBased",
                        )?;
                        _serde::ser::SerializeStruct::end(__struct)
                    }
                    Authentication::Account {
                        ref registration_allowed,
                        ref max_users,
                    } => {
                        let mut __serde_state = _serde::Serializer::serialize_struct(
                            __serializer,
                            "Authentication",
                            0 + 1 + 1 + 1,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "kind",
                            "Account",
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "registration_allowed",
                            registration_allowed,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "max_users",
                            max_users,
                        )?;
                        _serde::ser::SerializeStruct::end(__serde_state)
                    }
                }
            }
        }
    };
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Authentication {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private228::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private228::Formatter,
                    ) -> _serde::__private228::fmt::Result {
                        _serde::__private228::Formatter::write_str(
                            __formatter,
                            "variant identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private228::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private228::Ok(__Field::__field0),
                            1u64 => _serde::__private228::Ok(__Field::__field1),
                            2u64 => _serde::__private228::Ok(__Field::__field2),
                            _ => {
                                _serde::__private228::Err(
                                    _serde::de::Error::invalid_value(
                                        _serde::de::Unexpected::Unsigned(__value),
                                        &"variant index 0 <= i < 3",
                                    ),
                                )
                            }
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private228::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "OpenToAll" => _serde::__private228::Ok(__Field::__field0),
                            "TokenBased" => _serde::__private228::Ok(__Field::__field1),
                            "Account" => _serde::__private228::Ok(__Field::__field2),
                            _ => {
                                _serde::__private228::Err(
                                    _serde::de::Error::unknown_variant(__value, VARIANTS),
                                )
                            }
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private228::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"OpenToAll" => _serde::__private228::Ok(__Field::__field0),
                            b"TokenBased" => _serde::__private228::Ok(__Field::__field1),
                            b"Account" => _serde::__private228::Ok(__Field::__field2),
                            _ => {
                                let __value = &_serde::__private228::from_utf8_lossy(
                                    __value,
                                );
                                _serde::__private228::Err(
                                    _serde::de::Error::unknown_variant(__value, VARIANTS),
                                )
                            }
                        }
                    }
                }
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private228::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                const VARIANTS: &'static [&'static str] = &[
                    "OpenToAll",
                    "TokenBased",
                    "Account",
                ];
                let (__tag, __content) = _serde::Deserializer::deserialize_any(
                    __deserializer,
                    _serde::__private228::de::TaggedContentVisitor::<
                        __Field,
                    >::new("kind", "internally tagged enum Authentication"),
                )?;
                let __deserializer = _serde::__private228::de::ContentDeserializer::<
                    __D::Error,
                >::new(__content);
                match __tag {
                    __Field::__field0 => {
                        _serde::Deserializer::deserialize_any(
                            __deserializer,
                            _serde::__private228::de::InternallyTaggedUnitVisitor::new(
                                "Authentication",
                                "OpenToAll",
                            ),
                        )?;
                        _serde::__private228::Ok(Authentication::OpenToAll)
                    }
                    __Field::__field1 => {
                        _serde::Deserializer::deserialize_any(
                            __deserializer,
                            _serde::__private228::de::InternallyTaggedUnitVisitor::new(
                                "Authentication",
                                "TokenBased",
                            ),
                        )?;
                        _serde::__private228::Ok(Authentication::TokenBased)
                    }
                    __Field::__field2 => {
                        #[allow(non_camel_case_types)]
                        #[doc(hidden)]
                        enum __Field {
                            __field0,
                            __field1,
                            __ignore,
                        }
                        #[doc(hidden)]
                        struct __FieldVisitor;
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private228::Formatter,
                            ) -> _serde::__private228::fmt::Result {
                                _serde::__private228::Formatter::write_str(
                                    __formatter,
                                    "field identifier",
                                )
                            }
                            fn visit_u64<__E>(
                                self,
                                __value: u64,
                            ) -> _serde::__private228::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    0u64 => _serde::__private228::Ok(__Field::__field0),
                                    1u64 => _serde::__private228::Ok(__Field::__field1),
                                    _ => _serde::__private228::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_str<__E>(
                                self,
                                __value: &str,
                            ) -> _serde::__private228::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    "registration_allowed" => {
                                        _serde::__private228::Ok(__Field::__field0)
                                    }
                                    "max_users" => _serde::__private228::Ok(__Field::__field1),
                                    _ => _serde::__private228::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_bytes<__E>(
                                self,
                                __value: &[u8],
                            ) -> _serde::__private228::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    b"registration_allowed" => {
                                        _serde::__private228::Ok(__Field::__field0)
                                    }
                                    b"max_users" => _serde::__private228::Ok(__Field::__field1),
                                    _ => _serde::__private228::Ok(__Field::__ignore),
                                }
                            }
                        }
                        #[automatically_derived]
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private228::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(
                                    __deserializer,
                                    __FieldVisitor,
                                )
                            }
                        }
                        #[doc(hidden)]
                        struct __Visitor<'de> {
                            marker: _serde::__private228::PhantomData<Authentication>,
                            lifetime: _serde::__private228::PhantomData<&'de ()>,
                        }
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = Authentication;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private228::Formatter,
                            ) -> _serde::__private228::fmt::Result {
                                _serde::__private228::Formatter::write_str(
                                    __formatter,
                                    "struct variant Authentication::Account",
                                )
                            }
                            #[inline]
                            fn visit_seq<__A>(
                                self,
                                mut __seq: __A,
                            ) -> _serde::__private228::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::SeqAccess<'de>,
                            {
                                let __field0 = match _serde::de::SeqAccess::next_element::<
                                    bool,
                                >(&mut __seq)? {
                                    _serde::__private228::Some(__value) => __value,
                                    _serde::__private228::None => {
                                        return _serde::__private228::Err(
                                            _serde::de::Error::invalid_length(
                                                0usize,
                                                &"struct variant Authentication::Account with 2 elements",
                                            ),
                                        );
                                    }
                                };
                                let __field1 = match _serde::de::SeqAccess::next_element::<
                                    Option<u64>,
                                >(&mut __seq)? {
                                    _serde::__private228::Some(__value) => __value,
                                    _serde::__private228::None => {
                                        return _serde::__private228::Err(
                                            _serde::de::Error::invalid_length(
                                                1usize,
                                                &"struct variant Authentication::Account with 2 elements",
                                            ),
                                        );
                                    }
                                };
                                _serde::__private228::Ok(Authentication::Account {
                                    registration_allowed: __field0,
                                    max_users: __field1,
                                })
                            }
                            #[inline]
                            fn visit_map<__A>(
                                self,
                                mut __map: __A,
                            ) -> _serde::__private228::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::MapAccess<'de>,
                            {
                                let mut __field0: _serde::__private228::Option<bool> = _serde::__private228::None;
                                let mut __field1: _serde::__private228::Option<
                                    Option<u64>,
                                > = _serde::__private228::None;
                                while let _serde::__private228::Some(__key) = _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map)? {
                                    match __key {
                                        __Field::__field0 => {
                                            if _serde::__private228::Option::is_some(&__field0) {
                                                return _serde::__private228::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field(
                                                        "registration_allowed",
                                                    ),
                                                );
                                            }
                                            __field0 = _serde::__private228::Some(
                                                _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                            );
                                        }
                                        __Field::__field1 => {
                                            if _serde::__private228::Option::is_some(&__field1) {
                                                return _serde::__private228::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field(
                                                        "max_users",
                                                    ),
                                                );
                                            }
                                            __field1 = _serde::__private228::Some(
                                                _serde::de::MapAccess::next_value::<
                                                    Option<u64>,
                                                >(&mut __map)?,
                                            );
                                        }
                                        _ => {
                                            let _ = _serde::de::MapAccess::next_value::<
                                                _serde::de::IgnoredAny,
                                            >(&mut __map)?;
                                        }
                                    }
                                }
                                let __field0 = match __field0 {
                                    _serde::__private228::Some(__field0) => __field0,
                                    _serde::__private228::None => {
                                        _serde::__private228::de::missing_field(
                                            "registration_allowed",
                                        )?
                                    }
                                };
                                let __field1 = match __field1 {
                                    _serde::__private228::Some(__field1) => __field1,
                                    _serde::__private228::None => {
                                        _serde::__private228::de::missing_field("max_users")?
                                    }
                                };
                                _serde::__private228::Ok(Authentication::Account {
                                    registration_allowed: __field0,
                                    max_users: __field1,
                                })
                            }
                        }
                        #[doc(hidden)]
                        const FIELDS: &'static [&'static str] = &[
                            "registration_allowed",
                            "max_users",
                        ];
                        _serde::Deserializer::deserialize_any(
                            __deserializer,
                            __Visitor {
                                marker: _serde::__private228::PhantomData::<Authentication>,
                                lifetime: _serde::__private228::PhantomData,
                            },
                        )
                    }
                }
            }
        }
    };
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Authentication {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Authentication {
        #[inline]
        fn eq(&self, other: &Authentication) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
                && match (self, other) {
                    (
                        Authentication::Account {
                            registration_allowed: __self_0,
                            max_users: __self_1,
                        },
                        Authentication::Account {
                            registration_allowed: __arg1_0,
                            max_users: __arg1_1,
                        },
                    ) => __self_0 == __arg1_0 && __self_1 == __arg1_1,
                    _ => true,
                }
        }
    }
    impl Config {
        pub async fn new() -> Returns<Self> {
            let val = fs::read_to_string("./config.json").await?;
            Ok(from_str::<Self>(&val)?)
        }
        pub async fn new_or_default() -> Self {
            Self::new().await.unwrap_or_default()
        }
        pub async fn save_config(&self) -> Returns<()> {
            fs::write("./config.json", to_string_pretty(&self)?).await?;
            Ok(())
        }
    }
    impl Default for Config {
        fn default() -> Self {
            Self {
                binds: def_bind(),
                admin_pass_hash: None,
                ollama: OllamaConfiguration::default(),
                authentication: Authentication::OpenToAll,
            }
        }
    }
}
fn main() {
    panic::set_hook(
        Box::new(|x| {
            if let Some(x) = x.payload_as_str() {
                {
                    ::std::io::_print(format_args!("ERR: An Error Occured\n"));
                };
                {
                    ::std::io::_print(format_args!("ERR: {0}\n", x));
                };
            } else {
                {
                    ::std::io::_print(format_args!("ERR: Unknown\n"));
                };
            }
        }),
    );
    let mut args = args();
    _ = args.next();
    let mut config_ui = false;
    args.for_each(|x| {
        if &x == "config" {
            config_ui = true;
        } else {
            {
                ::core::panicking::panic_fmt(format_args!("Unknown arg: {0:?}", x));
            };
        }
    });
    if config_ui {
        ui::ui();
    } else {
        server::main().unwrap();
    }
}
