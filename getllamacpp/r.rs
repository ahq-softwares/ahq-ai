#![feature(prelude_import)]
#![feature(exit_status_error)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
pub mod compile {
    use std::{fs, process::Command};
    use inquire::Confirm;
    fn exec(cmd: &str, cwd: Option<&str>) {
        let mut data = cmd.split(" ");
        let bin = data.next().unwrap();
        let args = data.collect::<Vec<_>>();
        let mut cmd = Command::new(bin);
        cmd.args(args);
        if let Some(cwd) = cwd {
            cmd.current_dir(cwd);
        }
        cmd.spawn().unwrap().wait().unwrap().exit_ok().unwrap();
    }
    pub fn compile() {
        {
            ::std::io::_print(
                format_args!(
                    "\n      ___    __  ______      ______            _____      \n   /   |  / / / / __ \\    / ____/___  ____  / __(_)___ _\n  / /| | / /_/ / / / /   / /   / __ \\/ __ \\/ /_/ / __ `/\n / ___ |/ __  / /_/ /   / /___/ /_/ / / / / __/ / /_/ / \n/_/  |_/_/ /_/\\___\\_\\   \\____/\\____/_/ /_/_/ /_/\\__, /  \n                                               /____/   \n  \n",
                ),
            );
        };
        let mut cmd = "cmake -B build -DCMAKE_BUILD_TYPE=release".to_string();
        if Confirm::new("Do you want curl support (required libcurl to be installed)?")
            .with_default(false)
            .prompt_skippable()
            .unwrap()
            .unwrap_or_default()
        {
            cmd.push_str(" -DLLAMA_CURL=ON");
        } else {
            cmd.push_str(" -DLLAMA_CURL=OFF");
        }
        if Confirm::new(
                "Do you want CUDA support (associated toolkit should be installed)?",
            )
            .with_default(false)
            .prompt_skippable()
            .unwrap()
            .unwrap_or_default()
        {
            cmd.push_str(" -DGGML_CUDA=ON");
        }
        if Confirm::new(
                "Do you want CANN support (associated toolkit should be installed)?",
            )
            .with_default(false)
            .prompt_skippable()
            .unwrap()
            .unwrap_or_default()
        {
            cmd.push_str(" -DGGML_CANN=ON");
        }
        let exists = fs::exists("./llama.cpp").unwrap();
        let clone;
        if exists {
            clone = Confirm::new("Do you want to overwrite llama.cpp directory?")
                .with_default(false)
                .prompt_skippable()
                .unwrap()
                .unwrap_or_default();
            if clone {
                fs::remove_dir_all("./llama.cpp").unwrap();
            }
        } else {
            clone = true;
        }
        if clone {
            exec("git clone https://github.com/ggml-org/llama.cpp.git", None);
        }
        exec(&cmd, Some("llama.cpp"));
        exec("cmake --build build --config Release -j 8", Some("llama.cpp"));
        _ = fs::remove_dir_all("./llama");
        copy();
    }
    fn copy() {
        fs::create_dir_all("./llama").unwrap();
        let mut path = "./llama.cpp/build/bin/Release";
        let release = fs::exists("./llama.cpp/build/bin/Release").unwrap();
        if !release {
            path = "./llama.cpp/build/bin";
        }
        for entry in fs::read_dir(path).unwrap().into_iter().map(|x| x.unwrap()) {
            let name = entry.file_name().into_string().unwrap();
            fs::copy(
                    entry.path(),
                    ::alloc::__export::must_use({
                        ::alloc::fmt::format(format_args!("./llama/{0}", name))
                    }),
                )
                .unwrap();
        }
    }
}
pub mod download {
    use std::sync::LazyLock;
    use reqwest::Client;
    use serde::{Deserialize, Serialize};
    const DWNL_URL: &'static str = "https://api.github.com/repos/ahq-softwares/llama.cpp/releases/latest";
    pub static CLIENT: LazyLock<Client> = LazyLock::new(|| {
        Client::builder().user_agent("AHQ AI Downloader").build().unwrap()
    });
    struct Release {
        tag_name: String,
        assets: Vec<Asset>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Release {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "Release",
                "tag_name",
                &self.tag_name,
                "assets",
                &&self.assets,
            )
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
        impl _serde::Serialize for Release {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private228::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "Release",
                    false as usize + 1 + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "tag_name",
                    &self.tag_name,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "assets",
                    &self.assets,
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
        impl<'de> _serde::Deserialize<'de> for Release {
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
                            "tag_name" => _serde::__private228::Ok(__Field::__field0),
                            "assets" => _serde::__private228::Ok(__Field::__field1),
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
                            b"tag_name" => _serde::__private228::Ok(__Field::__field0),
                            b"assets" => _serde::__private228::Ok(__Field::__field1),
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
                    marker: _serde::__private228::PhantomData<Release>,
                    lifetime: _serde::__private228::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Release;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private228::Formatter,
                    ) -> _serde::__private228::fmt::Result {
                        _serde::__private228::Formatter::write_str(
                            __formatter,
                            "struct Release",
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
                            String,
                        >(&mut __seq)? {
                            _serde::__private228::Some(__value) => __value,
                            _serde::__private228::None => {
                                return _serde::__private228::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct Release with 2 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            Vec<Asset>,
                        >(&mut __seq)? {
                            _serde::__private228::Some(__value) => __value,
                            _serde::__private228::None => {
                                return _serde::__private228::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct Release with 2 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private228::Ok(Release {
                            tag_name: __field0,
                            assets: __field1,
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
                        let mut __field0: _serde::__private228::Option<String> = _serde::__private228::None;
                        let mut __field1: _serde::__private228::Option<Vec<Asset>> = _serde::__private228::None;
                        while let _serde::__private228::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private228::Option::is_some(&__field0) {
                                        return _serde::__private228::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "tag_name",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private228::Some(
                                        _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private228::Option::is_some(&__field1) {
                                        return _serde::__private228::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("assets"),
                                        );
                                    }
                                    __field1 = _serde::__private228::Some(
                                        _serde::de::MapAccess::next_value::<Vec<Asset>>(&mut __map)?,
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
                                _serde::__private228::de::missing_field("tag_name")?
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private228::Some(__field1) => __field1,
                            _serde::__private228::None => {
                                _serde::__private228::de::missing_field("assets")?
                            }
                        };
                        _serde::__private228::Ok(Release {
                            tag_name: __field0,
                            assets: __field1,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &["tag_name", "assets"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "Release",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private228::PhantomData::<Release>,
                        lifetime: _serde::__private228::PhantomData,
                    },
                )
            }
        }
    };
    pub struct Asset {
        pub name: String,
        pub browser_download_url: String,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Asset {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "Asset",
                "name",
                &self.name,
                "browser_download_url",
                &&self.browser_download_url,
            )
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
        impl _serde::Serialize for Asset {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private228::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "Asset",
                    false as usize + 1 + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "name",
                    &self.name,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "browser_download_url",
                    &self.browser_download_url,
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
        impl<'de> _serde::Deserialize<'de> for Asset {
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
                            "name" => _serde::__private228::Ok(__Field::__field0),
                            "browser_download_url" => {
                                _serde::__private228::Ok(__Field::__field1)
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
                            b"name" => _serde::__private228::Ok(__Field::__field0),
                            b"browser_download_url" => {
                                _serde::__private228::Ok(__Field::__field1)
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
                    marker: _serde::__private228::PhantomData<Asset>,
                    lifetime: _serde::__private228::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Asset;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private228::Formatter,
                    ) -> _serde::__private228::fmt::Result {
                        _serde::__private228::Formatter::write_str(
                            __formatter,
                            "struct Asset",
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
                            String,
                        >(&mut __seq)? {
                            _serde::__private228::Some(__value) => __value,
                            _serde::__private228::None => {
                                return _serde::__private228::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct Asset with 2 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq)? {
                            _serde::__private228::Some(__value) => __value,
                            _serde::__private228::None => {
                                return _serde::__private228::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct Asset with 2 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private228::Ok(Asset {
                            name: __field0,
                            browser_download_url: __field1,
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
                        let mut __field0: _serde::__private228::Option<String> = _serde::__private228::None;
                        let mut __field1: _serde::__private228::Option<String> = _serde::__private228::None;
                        while let _serde::__private228::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private228::Option::is_some(&__field0) {
                                        return _serde::__private228::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("name"),
                                        );
                                    }
                                    __field0 = _serde::__private228::Some(
                                        _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private228::Option::is_some(&__field1) {
                                        return _serde::__private228::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "browser_download_url",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private228::Some(
                                        _serde::de::MapAccess::next_value::<String>(&mut __map)?,
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
                                _serde::__private228::de::missing_field("name")?
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private228::Some(__field1) => __field1,
                            _serde::__private228::None => {
                                _serde::__private228::de::missing_field(
                                    "browser_download_url",
                                )?
                            }
                        };
                        _serde::__private228::Ok(Asset {
                            name: __field0,
                            browser_download_url: __field1,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &[
                    "name",
                    "browser_download_url",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "Asset",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private228::PhantomData::<Asset>,
                        lifetime: _serde::__private228::PhantomData,
                    },
                )
            }
        }
    };
    pub async fn dwnl(url: &str) -> Vec<u8> {
        CLIENT.get(url).send().await.unwrap().bytes().await.unwrap().into()
    }
    pub async fn get_platform_assets() -> (String, Vec<Asset>) {
        let data: Release = CLIENT
            .get(DWNL_URL)
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();
        (
            data.tag_name,
            data
                .assets
                .into_iter()
                .filter(|x| {
                    let term = "windows";
                    x.name.contains(term)
                })
                .collect(),
        )
    }
}
use inquire::Confirm;
use std::process;
fn main() {
    let body = async {
        use inquire::Select;
        {
            ::std::io::_print(
                format_args!(
                    "\n      ___    __  ______      ___    ____\n   /   |  / / / / __ \\    /   |  /  _/\n  / /| | / /_/ / / / /   / /| |  / /  \n / ___ |/ __  / /_/ /   / ___ |_/ /   \n/_/  |_/_/ /_/\\___\\_\\  /_/  |_/___/   \n                                    \n  \n",
                ),
            );
        };
        let compile = "Compile from source";
        let download = "Download prebuilt";
        let select = Select::new(
                "How do you want to get llama.cpp",
                <[_]>::into_vec(::alloc::boxed::box_new([download, compile])),
            )
            .prompt()
            .expect("Must respond");
        if select == download {
            use std::io::Cursor;
            let (tag, assets) = download::get_platform_assets().await;
            {
                ::std::io::_print(format_args!("\n> Found Release {0}\n\n", tag));
            };
            let cpu = Confirm::new(
                    "Do you know about the cpu features of this computer (eg, AVX1, AVX2, AMX-Int8 etc)?",
                )
                .with_default(false)
                .prompt()
                .unwrap_or_default();
            let name = if cpu {
                Select::new(
                        "Select the build based on your features",
                        assets.iter().map(|x| &x.name as &str).collect(),
                    )
                    .prompt()
                    .expect("Must select the build")
                    .to_string()
            } else {
                let asset_name = "llama-cpp-windows-x64-noavx.zip";
                asset_name.to_string()
            };
            let asset = assets.into_iter().find(|x| &x.name == &name).unwrap();
            let file = Cursor::new(download::dwnl(&asset.browser_download_url).await);
            let mut writer = zip::read::ZipArchive::new(file).unwrap();
            writer.extract("./llama").unwrap();
            drop(writer);
        } else {
            {
                ::std::io::_print(
                    format_args!("> Please wait while we compile using cmake...\n"),
                );
            };
            let resp = Confirm::new(
                    "Do you have C/C++ tools like `cmake` and `git` installed?",
                )
                .with_default(true)
                .prompt()
                .unwrap_or_default();
            if !resp {
                {
                    ::std::io::_print(
                        format_args!("> Please install the tools and try again\n"),
                    );
                };
                process::exit(0);
            }
            compile::compile();
        }
        {
            ::std::io::_print(
                format_args!(
                    "> Done. llama.cpp has been instantiated in the /llama/ folder\n",
                ),
            );
        };
    };
    #[allow(
        clippy::expect_used,
        clippy::diverging_sub_expression,
        clippy::needless_return,
        clippy::unwrap_in_result
    )]
    {
        return tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed building the Runtime")
            .block_on(body);
    }
}
