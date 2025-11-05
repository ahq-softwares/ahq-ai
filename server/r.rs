#![feature(prelude_import)]
#![feature(duration_constructors)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
use std::{env::args, panic};
mod server {
    use std::{
        env, fs as stdfs, sync::{LazyLock, OnceLock},
        thread::available_parallelism,
    };
    use crate::{
        auth::AuthSessionManager, structs::{Authentication, Config, db::DatabaseConfig},
    };
    use actix_web::{App, HttpServer, web};
    use bcrypt::verify;
    use chalk_rs::Chalk;
    use ollama_rs::Ollama;
    use secrecy::SecretString;
    use serde_json::from_str;
    pub mod admin {
        use actix_web::{
            HttpResponse, HttpResponseBuilder, Responder, Result, delete,
            http::StatusCode, post, web::Bytes,
        };
        use secrecy::ExposeSecret;
        use serde::Deserialize;
        use serde_json::from_slice;
        use tokio::task::yield_now;
        use async_stream::stream;
        use futures::Stream;
        use crate::{
            auth::{AccountCreateOutcome, AuthSessionManager},
            server::{AUTH, CONFIG, REAL_ADMIN_PASSWORD},
            structs::Authentication,
        };
        #[serde(deny_unknown_fields)]
        struct AdminAuthRequest<'a> {
            #[serde(borrow)]
            password: &'a str,
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
            impl<'de: 'a, 'a> _serde::Deserialize<'de> for AdminAuthRequest<'a> {
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
                                _ => {
                                    _serde::__private228::Err(
                                        _serde::de::Error::invalid_value(
                                            _serde::de::Unexpected::Unsigned(__value),
                                            &"field index 0 <= i < 1",
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
                                "password" => _serde::__private228::Ok(__Field::__field0),
                                _ => {
                                    _serde::__private228::Err(
                                        _serde::de::Error::unknown_field(__value, FIELDS),
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
                                b"password" => _serde::__private228::Ok(__Field::__field0),
                                _ => {
                                    let __value = &_serde::__private228::from_utf8_lossy(
                                        __value,
                                    );
                                    _serde::__private228::Err(
                                        _serde::de::Error::unknown_field(__value, FIELDS),
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
                    struct __Visitor<'de: 'a, 'a> {
                        marker: _serde::__private228::PhantomData<AdminAuthRequest<'a>>,
                        lifetime: _serde::__private228::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de: 'a, 'a> _serde::de::Visitor<'de> for __Visitor<'de, 'a> {
                        type Value = AdminAuthRequest<'a>;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private228::Formatter,
                        ) -> _serde::__private228::fmt::Result {
                            _serde::__private228::Formatter::write_str(
                                __formatter,
                                "struct AdminAuthRequest",
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
                                &'a str,
                            >(&mut __seq)? {
                                _serde::__private228::Some(__value) => __value,
                                _serde::__private228::None => {
                                    return _serde::__private228::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct AdminAuthRequest with 1 element",
                                        ),
                                    );
                                }
                            };
                            _serde::__private228::Ok(AdminAuthRequest {
                                password: __field0,
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
                            let mut __field0: _serde::__private228::Option<&'a str> = _serde::__private228::None;
                            while let _serde::__private228::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private228::Option::is_some(&__field0) {
                                            return _serde::__private228::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "password",
                                                ),
                                            );
                                        }
                                        __field0 = _serde::__private228::Some(
                                            _serde::de::MapAccess::next_value::<&'a str>(&mut __map)?,
                                        );
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private228::Some(__field0) => __field0,
                                _serde::__private228::None => {
                                    _serde::__private228::de::missing_field("password")?
                                }
                            };
                            _serde::__private228::Ok(AdminAuthRequest {
                                password: __field0,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &["password"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "AdminAuthRequest",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private228::PhantomData::<
                                AdminAuthRequest<'a>,
                            >,
                            lifetime: _serde::__private228::PhantomData,
                        },
                    )
                }
            }
        };
        #[serde(deny_unknown_fields)]
        struct AdminSearchRequest<'a> {
            #[serde(borrow)]
            password: &'a str,
            search: String,
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
            impl<'de: 'a, 'a> _serde::Deserialize<'de> for AdminSearchRequest<'a> {
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
                                _ => {
                                    _serde::__private228::Err(
                                        _serde::de::Error::invalid_value(
                                            _serde::de::Unexpected::Unsigned(__value),
                                            &"field index 0 <= i < 2",
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
                                "password" => _serde::__private228::Ok(__Field::__field0),
                                "search" => _serde::__private228::Ok(__Field::__field1),
                                _ => {
                                    _serde::__private228::Err(
                                        _serde::de::Error::unknown_field(__value, FIELDS),
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
                                b"password" => _serde::__private228::Ok(__Field::__field0),
                                b"search" => _serde::__private228::Ok(__Field::__field1),
                                _ => {
                                    let __value = &_serde::__private228::from_utf8_lossy(
                                        __value,
                                    );
                                    _serde::__private228::Err(
                                        _serde::de::Error::unknown_field(__value, FIELDS),
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
                    struct __Visitor<'de: 'a, 'a> {
                        marker: _serde::__private228::PhantomData<
                            AdminSearchRequest<'a>,
                        >,
                        lifetime: _serde::__private228::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de: 'a, 'a> _serde::de::Visitor<'de> for __Visitor<'de, 'a> {
                        type Value = AdminSearchRequest<'a>;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private228::Formatter,
                        ) -> _serde::__private228::fmt::Result {
                            _serde::__private228::Formatter::write_str(
                                __formatter,
                                "struct AdminSearchRequest",
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
                                &'a str,
                            >(&mut __seq)? {
                                _serde::__private228::Some(__value) => __value,
                                _serde::__private228::None => {
                                    return _serde::__private228::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct AdminSearchRequest with 2 elements",
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
                                            &"struct AdminSearchRequest with 2 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private228::Ok(AdminSearchRequest {
                                password: __field0,
                                search: __field1,
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
                            let mut __field0: _serde::__private228::Option<&'a str> = _serde::__private228::None;
                            let mut __field1: _serde::__private228::Option<String> = _serde::__private228::None;
                            while let _serde::__private228::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private228::Option::is_some(&__field0) {
                                            return _serde::__private228::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "password",
                                                ),
                                            );
                                        }
                                        __field0 = _serde::__private228::Some(
                                            _serde::de::MapAccess::next_value::<&'a str>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private228::Option::is_some(&__field1) {
                                            return _serde::__private228::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("search"),
                                            );
                                        }
                                        __field1 = _serde::__private228::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private228::Some(__field0) => __field0,
                                _serde::__private228::None => {
                                    _serde::__private228::de::missing_field("password")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private228::Some(__field1) => __field1,
                                _serde::__private228::None => {
                                    _serde::__private228::de::missing_field("search")?
                                }
                            };
                            _serde::__private228::Ok(AdminSearchRequest {
                                password: __field0,
                                search: __field1,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &["password", "search"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "AdminSearchRequest",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private228::PhantomData::<
                                AdminSearchRequest<'a>,
                            >,
                            lifetime: _serde::__private228::PhantomData,
                        },
                    )
                }
            }
        };
        #[serde(deny_unknown_fields)]
        struct AdminUserCreateRequest<'a> {
            #[serde(borrow)]
            password: &'a str,
            #[serde(borrow)]
            unique_id: &'a str,
            #[serde(borrow)]
            user_password: &'a str,
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
            impl<'de: 'a, 'a> _serde::Deserialize<'de> for AdminUserCreateRequest<'a> {
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
                                _ => {
                                    _serde::__private228::Err(
                                        _serde::de::Error::invalid_value(
                                            _serde::de::Unexpected::Unsigned(__value),
                                            &"field index 0 <= i < 3",
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
                                "password" => _serde::__private228::Ok(__Field::__field0),
                                "unique_id" => _serde::__private228::Ok(__Field::__field1),
                                "user_password" => {
                                    _serde::__private228::Ok(__Field::__field2)
                                }
                                _ => {
                                    _serde::__private228::Err(
                                        _serde::de::Error::unknown_field(__value, FIELDS),
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
                                b"password" => _serde::__private228::Ok(__Field::__field0),
                                b"unique_id" => _serde::__private228::Ok(__Field::__field1),
                                b"user_password" => {
                                    _serde::__private228::Ok(__Field::__field2)
                                }
                                _ => {
                                    let __value = &_serde::__private228::from_utf8_lossy(
                                        __value,
                                    );
                                    _serde::__private228::Err(
                                        _serde::de::Error::unknown_field(__value, FIELDS),
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
                    struct __Visitor<'de: 'a, 'a> {
                        marker: _serde::__private228::PhantomData<
                            AdminUserCreateRequest<'a>,
                        >,
                        lifetime: _serde::__private228::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de: 'a, 'a> _serde::de::Visitor<'de> for __Visitor<'de, 'a> {
                        type Value = AdminUserCreateRequest<'a>;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private228::Formatter,
                        ) -> _serde::__private228::fmt::Result {
                            _serde::__private228::Formatter::write_str(
                                __formatter,
                                "struct AdminUserCreateRequest",
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
                                &'a str,
                            >(&mut __seq)? {
                                _serde::__private228::Some(__value) => __value,
                                _serde::__private228::None => {
                                    return _serde::__private228::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct AdminUserCreateRequest with 3 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                &'a str,
                            >(&mut __seq)? {
                                _serde::__private228::Some(__value) => __value,
                                _serde::__private228::None => {
                                    return _serde::__private228::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct AdminUserCreateRequest with 3 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match _serde::de::SeqAccess::next_element::<
                                &'a str,
                            >(&mut __seq)? {
                                _serde::__private228::Some(__value) => __value,
                                _serde::__private228::None => {
                                    return _serde::__private228::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct AdminUserCreateRequest with 3 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private228::Ok(AdminUserCreateRequest {
                                password: __field0,
                                unique_id: __field1,
                                user_password: __field2,
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
                            let mut __field0: _serde::__private228::Option<&'a str> = _serde::__private228::None;
                            let mut __field1: _serde::__private228::Option<&'a str> = _serde::__private228::None;
                            let mut __field2: _serde::__private228::Option<&'a str> = _serde::__private228::None;
                            while let _serde::__private228::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private228::Option::is_some(&__field0) {
                                            return _serde::__private228::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "password",
                                                ),
                                            );
                                        }
                                        __field0 = _serde::__private228::Some(
                                            _serde::de::MapAccess::next_value::<&'a str>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private228::Option::is_some(&__field1) {
                                            return _serde::__private228::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "unique_id",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private228::Some(
                                            _serde::de::MapAccess::next_value::<&'a str>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private228::Option::is_some(&__field2) {
                                            return _serde::__private228::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "user_password",
                                                ),
                                            );
                                        }
                                        __field2 = _serde::__private228::Some(
                                            _serde::de::MapAccess::next_value::<&'a str>(&mut __map)?,
                                        );
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private228::Some(__field0) => __field0,
                                _serde::__private228::None => {
                                    _serde::__private228::de::missing_field("password")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private228::Some(__field1) => __field1,
                                _serde::__private228::None => {
                                    _serde::__private228::de::missing_field("unique_id")?
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private228::Some(__field2) => __field2,
                                _serde::__private228::None => {
                                    _serde::__private228::de::missing_field("user_password")?
                                }
                            };
                            _serde::__private228::Ok(AdminUserCreateRequest {
                                password: __field0,
                                unique_id: __field1,
                                user_password: __field2,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &[
                        "password",
                        "unique_id",
                        "user_password",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "AdminUserCreateRequest",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private228::PhantomData::<
                                AdminUserCreateRequest<'a>,
                            >,
                            lifetime: _serde::__private228::PhantomData,
                        },
                    )
                }
            }
        };
        #[serde(deny_unknown_fields)]
        struct AdminDeleteRequest<'a> {
            #[serde(borrow)]
            password: &'a str,
            unique_id: String,
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
            impl<'de: 'a, 'a> _serde::Deserialize<'de> for AdminDeleteRequest<'a> {
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
                                _ => {
                                    _serde::__private228::Err(
                                        _serde::de::Error::invalid_value(
                                            _serde::de::Unexpected::Unsigned(__value),
                                            &"field index 0 <= i < 2",
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
                                "password" => _serde::__private228::Ok(__Field::__field0),
                                "unique_id" => _serde::__private228::Ok(__Field::__field1),
                                _ => {
                                    _serde::__private228::Err(
                                        _serde::de::Error::unknown_field(__value, FIELDS),
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
                                b"password" => _serde::__private228::Ok(__Field::__field0),
                                b"unique_id" => _serde::__private228::Ok(__Field::__field1),
                                _ => {
                                    let __value = &_serde::__private228::from_utf8_lossy(
                                        __value,
                                    );
                                    _serde::__private228::Err(
                                        _serde::de::Error::unknown_field(__value, FIELDS),
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
                    struct __Visitor<'de: 'a, 'a> {
                        marker: _serde::__private228::PhantomData<
                            AdminDeleteRequest<'a>,
                        >,
                        lifetime: _serde::__private228::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de: 'a, 'a> _serde::de::Visitor<'de> for __Visitor<'de, 'a> {
                        type Value = AdminDeleteRequest<'a>;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private228::Formatter,
                        ) -> _serde::__private228::fmt::Result {
                            _serde::__private228::Formatter::write_str(
                                __formatter,
                                "struct AdminDeleteRequest",
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
                                &'a str,
                            >(&mut __seq)? {
                                _serde::__private228::Some(__value) => __value,
                                _serde::__private228::None => {
                                    return _serde::__private228::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct AdminDeleteRequest with 2 elements",
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
                                            &"struct AdminDeleteRequest with 2 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private228::Ok(AdminDeleteRequest {
                                password: __field0,
                                unique_id: __field1,
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
                            let mut __field0: _serde::__private228::Option<&'a str> = _serde::__private228::None;
                            let mut __field1: _serde::__private228::Option<String> = _serde::__private228::None;
                            while let _serde::__private228::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private228::Option::is_some(&__field0) {
                                            return _serde::__private228::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "password",
                                                ),
                                            );
                                        }
                                        __field0 = _serde::__private228::Some(
                                            _serde::de::MapAccess::next_value::<&'a str>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private228::Option::is_some(&__field1) {
                                            return _serde::__private228::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "unique_id",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private228::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private228::Some(__field0) => __field0,
                                _serde::__private228::None => {
                                    _serde::__private228::de::missing_field("password")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private228::Some(__field1) => __field1,
                                _serde::__private228::None => {
                                    _serde::__private228::de::missing_field("unique_id")?
                                }
                            };
                            _serde::__private228::Ok(AdminDeleteRequest {
                                password: __field0,
                                unique_id: __field1,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &["password", "unique_id"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "AdminDeleteRequest",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private228::PhantomData::<
                                AdminDeleteRequest<'a>,
                            >,
                            lifetime: _serde::__private228::PhantomData,
                        },
                    )
                }
            }
        };
        fn verify_auth<'a>(passwd: &'a str) -> Result<(), HttpResponse> {
            let value = REAL_ADMIN_PASSWORD
                .get()
                .map(|x| passwd == x.expose_secret())
                .unwrap_or(false);
            match value {
                true => Ok(()),
                _ => {
                    Err(
                        HttpResponse::Unauthorized().body(r#"{ "msg": "Unauthorized" }"#),
                    )
                }
            }
        }
        #[allow(non_camel_case_types, missing_docs)]
        pub struct verify;
        impl ::actix_web::dev::HttpServiceFactory for verify {
            fn register(self, __config: &mut actix_web::dev::AppService) {
                async fn verify(body: Bytes) -> Result<impl Responder> {
                    let auth: AdminAuthRequest = from_slice(&body)?;
                    if let Err(r) = verify_auth(auth.password) {
                        return Ok(r);
                    }
                    Ok(HttpResponse::NoContent().body::<&[u8]>(&[]))
                }
                let __resource = ::actix_web::Resource::new("/admin/verify")
                    .name("verify")
                    .guard(::actix_web::guard::Post())
                    .to(verify);
                ::actix_web::dev::HttpServiceFactory::register(__resource, __config);
            }
        }
        #[allow(non_camel_case_types, missing_docs)]
        pub struct list;
        impl ::actix_web::dev::HttpServiceFactory for list {
            fn register(self, __config: &mut actix_web::dev::AppService) {
                async fn list(body: Bytes) -> Result<impl Responder> {
                    let data: AdminSearchRequest = from_slice(&body)?;
                    if let Err(r) = verify_auth(data.password) {
                        return Ok(r);
                    }
                    if let Some(auth) = AUTH.get() {
                        return Ok(
                            HttpResponseBuilder::new(StatusCode::OK)
                                .streaming(user_list_stream(auth, data.search)),
                        );
                    }
                    Ok(
                        HttpResponse::ServiceUnavailable()
                            .body::<&[u8]>(br#"{ "msg": "Auth is disabled" }"#),
                    )
                }
                let __resource = ::actix_web::Resource::new("/admin/clients")
                    .name("list")
                    .guard(::actix_web::guard::Post())
                    .to(list);
                ::actix_web::dev::HttpServiceFactory::register(__resource, __config);
            }
        }
        fn user_list_stream<'a>(
            auth: &'static AuthSessionManager,
            prefix: String,
        ) -> impl Stream<Item = Result<Bytes>> {
            {
                let (mut __yield_tx, __yield_rx) = unsafe {
                    ::async_stream::__private::yielder::pair()
                };
                ::async_stream::__private::AsyncStream::new(
                    __yield_rx,
                    async move {
                        '__async_stream_private_check_scope: {
                            let mut index = 0usize;
                            for uid in match auth.accounts.search(prefix).await {
                                ::core::result::Result::Ok(v) => v,
                                ::core::result::Result::Err(e) => {
                                    __yield_tx
                                        .send(::core::result::Result::Err(e.into()))
                                        .await;
                                    return;
                                }
                            } {
                                if index != 0 {
                                    {
                                        #[allow(unreachable_code)]
                                        if false {
                                            break '__async_stream_private_check_scope (loop {});
                                        }
                                        __yield_tx.send(Ok(Bytes::from_static(b"\n"))).await
                                    };
                                }
                                {
                                    #[allow(unreachable_code)]
                                    if false {
                                        break '__async_stream_private_check_scope (loop {});
                                    }
                                    __yield_tx.send(Ok(Bytes::from_owner(uid))).await
                                };
                                if index % 30 == 0 {
                                    yield_now().await;
                                }
                                index += 1;
                            }
                        }
                    },
                )
            }
        }
        #[allow(non_camel_case_types, missing_docs)]
        pub struct create;
        impl ::actix_web::dev::HttpServiceFactory for create {
            fn register(self, __config: &mut actix_web::dev::AppService) {
                async fn create(body: Bytes) -> Result<impl Responder> {
                    let data: AdminUserCreateRequest = from_slice(&body)?;
                    if let Err(r) = verify_auth(data.password) {
                        return Ok(r);
                    }
                    let Authentication::Account { .. } = CONFIG.authentication else {
                        return Ok(
                            HttpResponse::ServiceUnavailable()
                                .body::<&[u8]>(br#"{ "msg": "Auth is not account based" }"#),
                        );
                    };
                    if let Some(auth) = AUTH.get() {
                        return match auth
                            .register(data.unique_id, data.user_password)
                            .await?
                        {
                            AccountCreateOutcome::InternalServerError => {
                                Ok(
                                    HttpResponse::InternalServerError()
                                        .body(r#"{ "msg": "Internal Server Error" }"#),
                                )
                            }
                            AccountCreateOutcome::Successful => {
                                Ok(HttpResponse::NoContent().body(::alloc::vec::Vec::new()))
                            }
                            AccountCreateOutcome::UsernameExists => {
                                Ok(
                                    HttpResponse::Conflict()
                                        .body(r#"{ "msg": "User already exists" }"#),
                                )
                            }
                            AccountCreateOutcome::WeakPassword => {
                                Ok(
                                    HttpResponse::BadRequest()
                                        .body(r#"{ "msg": "Insecure Password" }"#),
                                )
                            }
                            _ => {
                                Ok(
                                    HttpResponse::UnprocessableEntity()
                                        .body(r#"{ "msg": "Unreachable Output" }"#),
                                )
                            }
                        };
                    }
                    Ok(
                        HttpResponse::ServiceUnavailable()
                            .body::<&[u8]>(br#"{ "msg": "Auth is disabled" }"#),
                    )
                }
                let __resource = ::actix_web::Resource::new("/admin/user")
                    .name("create")
                    .guard(::actix_web::guard::Post())
                    .to(create);
                ::actix_web::dev::HttpServiceFactory::register(__resource, __config);
            }
        }
        #[allow(non_camel_case_types, missing_docs)]
        pub struct create_token;
        impl ::actix_web::dev::HttpServiceFactory for create_token {
            fn register(self, __config: &mut actix_web::dev::AppService) {
                async fn create_token(body: Bytes) -> Result<impl Responder> {
                    let auth: AdminAuthRequest = from_slice(&body)?;
                    if let Err(r) = verify_auth(auth.password) {
                        return Ok(r);
                    }
                    let Authentication::TokenBased = CONFIG.authentication else {
                        return Ok(
                            HttpResponse::ServiceUnavailable()
                                .body::<&[u8]>(br#"{ "msg": "Auth is not token based" }"#),
                        );
                    };
                    if let Some(auth) = AUTH.get() {
                        return match auth.add_token().await? {
                            AccountCreateOutcome::InternalServerError => {
                                Ok(
                                    HttpResponse::InternalServerError()
                                        .body(r#"{ "msg": "Internal Server Error" }"#),
                                )
                            }
                            AccountCreateOutcome::SuccessfulOut(out) => {
                                Ok(HttpResponse::NoContent().body(Bytes::from_owner(out)))
                            }
                            AccountCreateOutcome::UsernameExists => {
                                Ok(
                                    HttpResponse::Conflict()
                                        .body(r#"{ "msg": "User already exists" }"#),
                                )
                            }
                            AccountCreateOutcome::WeakPassword => {
                                Ok(
                                    HttpResponse::BadRequest()
                                        .body(r#"{ "msg": "Insecure Password" }"#),
                                )
                            }
                            _ => {
                                Ok(
                                    HttpResponse::UnprocessableEntity()
                                        .body(r#"{ "msg": "Unreachable Output" }"#),
                                )
                            }
                        };
                    }
                    Ok(
                        HttpResponse::ServiceUnavailable()
                            .body::<&[u8]>(br#"{ "msg": "Auth is disabled" }"#),
                    )
                }
                let __resource = ::actix_web::Resource::new("/admin/token")
                    .name("create_token")
                    .guard(::actix_web::guard::Post())
                    .to(create_token);
                ::actix_web::dev::HttpServiceFactory::register(__resource, __config);
            }
        }
        #[allow(non_camel_case_types, missing_docs)]
        pub struct delete;
        impl ::actix_web::dev::HttpServiceFactory for delete {
            fn register(self, __config: &mut actix_web::dev::AppService) {
                async fn delete(body: Bytes) -> Result<impl Responder> {
                    let data: AdminDeleteRequest = from_slice(&body)?;
                    if let Err(r) = verify_auth(data.password) {
                        return Ok(r);
                    }
                    let Authentication::Account { .. } = CONFIG.authentication else {
                        return Ok(
                            HttpResponse::ServiceUnavailable()
                                .body::<&[u8]>(br#"{ "msg": "Auth is not account based" }"#),
                        );
                    };
                    if let Some(auth) = AUTH.get() {
                        _ = auth.accounts.remove(data.unique_id).await;
                        return Ok(HttpResponse::NoContent().body::<&[u8]>(&[]));
                    }
                    Ok(
                        HttpResponse::ServiceUnavailable()
                            .body::<&[u8]>(br#"{ "msg": "Auth is disabled" }"#),
                    )
                }
                let __resource = ::actix_web::Resource::new("/admin/client")
                    .name("delete")
                    .guard(::actix_web::guard::Delete())
                    .to(delete);
                ::actix_web::dev::HttpServiceFactory::register(__resource, __config);
            }
        }
    }
    pub mod auth {
        use actix_web::{HttpResponse, Responder, Result, post, web::Bytes};
        use serde::Deserialize;
        use crate::{
            auth::{AccountCheckOutcome, AccountCreateOutcome},
            server::{AUTH, TOKEN},
        };
        #[serde(deny_unknown_fields)]
        struct Auth<'a> {
            #[serde(borrow)]
            username: Option<&'a str>,
            #[serde(borrow)]
            pass: &'a str,
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
            impl<'de: 'a, 'a> _serde::Deserialize<'de> for Auth<'a> {
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
                                _ => {
                                    _serde::__private228::Err(
                                        _serde::de::Error::invalid_value(
                                            _serde::de::Unexpected::Unsigned(__value),
                                            &"field index 0 <= i < 2",
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
                                "username" => _serde::__private228::Ok(__Field::__field0),
                                "pass" => _serde::__private228::Ok(__Field::__field1),
                                _ => {
                                    _serde::__private228::Err(
                                        _serde::de::Error::unknown_field(__value, FIELDS),
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
                                b"username" => _serde::__private228::Ok(__Field::__field0),
                                b"pass" => _serde::__private228::Ok(__Field::__field1),
                                _ => {
                                    let __value = &_serde::__private228::from_utf8_lossy(
                                        __value,
                                    );
                                    _serde::__private228::Err(
                                        _serde::de::Error::unknown_field(__value, FIELDS),
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
                    struct __Visitor<'de: 'a, 'a> {
                        marker: _serde::__private228::PhantomData<Auth<'a>>,
                        lifetime: _serde::__private228::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de: 'a, 'a> _serde::de::Visitor<'de> for __Visitor<'de, 'a> {
                        type Value = Auth<'a>;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private228::Formatter,
                        ) -> _serde::__private228::fmt::Result {
                            _serde::__private228::Formatter::write_str(
                                __formatter,
                                "struct Auth",
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
                                Option<&'a str>,
                            >(&mut __seq)? {
                                _serde::__private228::Some(__value) => __value,
                                _serde::__private228::None => {
                                    return _serde::__private228::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct Auth with 2 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                &'a str,
                            >(&mut __seq)? {
                                _serde::__private228::Some(__value) => __value,
                                _serde::__private228::None => {
                                    return _serde::__private228::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct Auth with 2 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private228::Ok(Auth {
                                username: __field0,
                                pass: __field1,
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
                                Option<&'a str>,
                            > = _serde::__private228::None;
                            let mut __field1: _serde::__private228::Option<&'a str> = _serde::__private228::None;
                            while let _serde::__private228::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private228::Option::is_some(&__field0) {
                                            return _serde::__private228::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "username",
                                                ),
                                            );
                                        }
                                        __field0 = _serde::__private228::Some(
                                            _serde::de::MapAccess::next_value::<
                                                Option<&'a str>,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private228::Option::is_some(&__field1) {
                                            return _serde::__private228::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("pass"),
                                            );
                                        }
                                        __field1 = _serde::__private228::Some(
                                            _serde::de::MapAccess::next_value::<&'a str>(&mut __map)?,
                                        );
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private228::Some(__field0) => __field0,
                                _serde::__private228::None => {
                                    _serde::__private228::de::missing_field("username")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private228::Some(__field1) => __field1,
                                _serde::__private228::None => {
                                    _serde::__private228::de::missing_field("pass")?
                                }
                            };
                            _serde::__private228::Ok(Auth {
                                username: __field0,
                                pass: __field1,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &["username", "pass"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "Auth",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private228::PhantomData::<Auth<'a>>,
                            lifetime: _serde::__private228::PhantomData,
                        },
                    )
                }
            }
        };
        #[serde(deny_unknown_fields)]
        struct AuthRegn<'a> {
            #[serde(borrow)]
            username: &'a str,
            #[serde(borrow)]
            pass: &'a str,
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
            impl<'de: 'a, 'a> _serde::Deserialize<'de> for AuthRegn<'a> {
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
                                _ => {
                                    _serde::__private228::Err(
                                        _serde::de::Error::invalid_value(
                                            _serde::de::Unexpected::Unsigned(__value),
                                            &"field index 0 <= i < 2",
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
                                "username" => _serde::__private228::Ok(__Field::__field0),
                                "pass" => _serde::__private228::Ok(__Field::__field1),
                                _ => {
                                    _serde::__private228::Err(
                                        _serde::de::Error::unknown_field(__value, FIELDS),
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
                                b"username" => _serde::__private228::Ok(__Field::__field0),
                                b"pass" => _serde::__private228::Ok(__Field::__field1),
                                _ => {
                                    let __value = &_serde::__private228::from_utf8_lossy(
                                        __value,
                                    );
                                    _serde::__private228::Err(
                                        _serde::de::Error::unknown_field(__value, FIELDS),
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
                    struct __Visitor<'de: 'a, 'a> {
                        marker: _serde::__private228::PhantomData<AuthRegn<'a>>,
                        lifetime: _serde::__private228::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de: 'a, 'a> _serde::de::Visitor<'de> for __Visitor<'de, 'a> {
                        type Value = AuthRegn<'a>;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private228::Formatter,
                        ) -> _serde::__private228::fmt::Result {
                            _serde::__private228::Formatter::write_str(
                                __formatter,
                                "struct AuthRegn",
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
                                &'a str,
                            >(&mut __seq)? {
                                _serde::__private228::Some(__value) => __value,
                                _serde::__private228::None => {
                                    return _serde::__private228::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct AuthRegn with 2 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                &'a str,
                            >(&mut __seq)? {
                                _serde::__private228::Some(__value) => __value,
                                _serde::__private228::None => {
                                    return _serde::__private228::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct AuthRegn with 2 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private228::Ok(AuthRegn {
                                username: __field0,
                                pass: __field1,
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
                            let mut __field0: _serde::__private228::Option<&'a str> = _serde::__private228::None;
                            let mut __field1: _serde::__private228::Option<&'a str> = _serde::__private228::None;
                            while let _serde::__private228::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private228::Option::is_some(&__field0) {
                                            return _serde::__private228::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "username",
                                                ),
                                            );
                                        }
                                        __field0 = _serde::__private228::Some(
                                            _serde::de::MapAccess::next_value::<&'a str>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private228::Option::is_some(&__field1) {
                                            return _serde::__private228::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("pass"),
                                            );
                                        }
                                        __field1 = _serde::__private228::Some(
                                            _serde::de::MapAccess::next_value::<&'a str>(&mut __map)?,
                                        );
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private228::Some(__field0) => __field0,
                                _serde::__private228::None => {
                                    _serde::__private228::de::missing_field("username")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private228::Some(__field1) => __field1,
                                _serde::__private228::None => {
                                    _serde::__private228::de::missing_field("pass")?
                                }
                            };
                            _serde::__private228::Ok(AuthRegn {
                                username: __field0,
                                pass: __field1,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &["username", "pass"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "AuthRegn",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private228::PhantomData::<AuthRegn<'a>>,
                            lifetime: _serde::__private228::PhantomData,
                        },
                    )
                }
            }
        };
        #[allow(non_camel_case_types, missing_docs)]
        pub struct auth;
        impl ::actix_web::dev::HttpServiceFactory for auth {
            fn register(self, __config: &mut actix_web::dev::AppService) {
                pub async fn auth(payload: Bytes) -> Result<impl Responder> {
                    let Ok(auth) = serde_json::from_slice::<Auth>(&payload) else {
                        return Ok(
                            HttpResponse::BadRequest()
                                .body(r#"{ "msg": "Invalid Data" }"#),
                        );
                    };
                    let auth_ref = AUTH
                        .get()
                        .expect(
                            "Auth must be defined or else this function cant be registered",
                        );
                    let resp = match *TOKEN {
                        true => auth_ref.is_valid_token(auth.pass).await?,
                        false => {
                            auth_ref
                                .is_valid_account(
                                    auth.username.unwrap_or_default(),
                                    auth.pass,
                                )
                                .await?
                        }
                    };
                    match resp {
                        AccountCheckOutcome::Some(x) => Ok(HttpResponse::Ok().body(x)),
                        AccountCheckOutcome::InvalidPassword
                        | AccountCheckOutcome::NotFound => {
                            Ok(
                                HttpResponse::Unauthorized()
                                    .body("{\"msg\": \"Invalid Credentials\"}"),
                            )
                        }
                        AccountCheckOutcome::TooManyRequests => {
                            Ok(
                                HttpResponse::TooManyRequests()
                                    .body("{\"msg\": \"Too Many Requests\"}"),
                            )
                        }
                    }
                }
                let __resource = ::actix_web::Resource::new("/login")
                    .name("auth")
                    .guard(::actix_web::guard::Post())
                    .to(auth);
                ::actix_web::dev::HttpServiceFactory::register(__resource, __config);
            }
        }
        #[allow(non_camel_case_types, missing_docs)]
        pub struct register;
        impl ::actix_web::dev::HttpServiceFactory for register {
            fn register(self, __config: &mut actix_web::dev::AppService) {
                pub async fn register(payload: Bytes) -> Result<impl Responder> {
                    let Ok(regn) = serde_json::from_slice::<AuthRegn>(&payload) else {
                        return Ok(
                            HttpResponse::BadRequest()
                                .body(r#"{ "msg": "Invalid Data" }"#),
                        );
                    };
                    let auth_ref = AUTH
                        .get()
                        .expect(
                            "Auth must be defined or else this function cant be registered",
                        );
                    if !auth_ref.can_register().await {
                        return Ok(
                            HttpResponse::UnprocessableEntity()
                                .body(
                                    r#"{ "msg": "Registration is disabled due to maximum user saturation" }"#,
                                ),
                        );
                    }
                    match auth_ref.register(regn.username, regn.pass).await? {
                        AccountCreateOutcome::InternalServerError => {
                            Ok(
                                HttpResponse::InternalServerError()
                                    .body(r#"{ "msg": "Internal Server Error" }"#),
                            )
                        }
                        AccountCreateOutcome::Successful => {
                            Ok(HttpResponse::NoContent().body(::alloc::vec::Vec::new()))
                        }
                        AccountCreateOutcome::UsernameExists => {
                            Ok(
                                HttpResponse::Conflict()
                                    .body(r#"{ "msg": "User already exists" }"#),
                            )
                        }
                        AccountCreateOutcome::WeakPassword => {
                            Ok(
                                HttpResponse::BadRequest()
                                    .body(r#"{ "msg": "Insecure Password" }"#),
                            )
                        }
                        _ => {
                            Ok(
                                HttpResponse::UnprocessableEntity()
                                    .body(r#"{ "msg": "Unreachable Output" }"#),
                            )
                        }
                    }
                }
                let __resource = ::actix_web::Resource::new("/register")
                    .name("register")
                    .guard(::actix_web::guard::Post())
                    .to(register);
                ::actix_web::dev::HttpServiceFactory::register(__resource, __config);
            }
        }
    }
    pub mod chat {
        use crate::server::{
            AUTH, CONFIG, HISTORY_LENGTH, OLLAMA,
            chat::ollama::{Message, OllamaMsgResp, OllamaRequest},
        };
        use actix_web::{HttpRequest, HttpResponse, Result, rt, web::Payload};
        use actix_ws::{AggregatedMessage, Session};
        use ollama_rs::generation::{
            chat::{ChatMessage, MessageRole, request::ChatMessageRequest},
            images::Image,
        };
        pub mod ollama {
            use serde::{Deserialize, Serialize};
            pub type History = Vec<Message>;
            #[serde(tag = "user")]
            pub enum Message {
                User { message: String, images: Option<Vec<String>> },
                System { prompt: String },
                Assistant { message: String, thinking: Option<String> },
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for Message {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match self {
                        Message::User { message: __self_0, images: __self_1 } => {
                            ::core::fmt::Formatter::debug_struct_field2_finish(
                                f,
                                "User",
                                "message",
                                __self_0,
                                "images",
                                &__self_1,
                            )
                        }
                        Message::System { prompt: __self_0 } => {
                            ::core::fmt::Formatter::debug_struct_field1_finish(
                                f,
                                "System",
                                "prompt",
                                &__self_0,
                            )
                        }
                        Message::Assistant { message: __self_0, thinking: __self_1 } => {
                            ::core::fmt::Formatter::debug_struct_field2_finish(
                                f,
                                "Assistant",
                                "message",
                                __self_0,
                                "thinking",
                                &__self_1,
                            )
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
                impl _serde::Serialize for Message {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> _serde::__private228::Result<__S::Ok, __S::Error>
                    where
                        __S: _serde::Serializer,
                    {
                        match *self {
                            Message::User { ref message, ref images } => {
                                let mut __serde_state = _serde::Serializer::serialize_struct(
                                    __serializer,
                                    "Message",
                                    0 + 1 + 1 + 1,
                                )?;
                                _serde::ser::SerializeStruct::serialize_field(
                                    &mut __serde_state,
                                    "user",
                                    "User",
                                )?;
                                _serde::ser::SerializeStruct::serialize_field(
                                    &mut __serde_state,
                                    "message",
                                    message,
                                )?;
                                _serde::ser::SerializeStruct::serialize_field(
                                    &mut __serde_state,
                                    "images",
                                    images,
                                )?;
                                _serde::ser::SerializeStruct::end(__serde_state)
                            }
                            Message::System { ref prompt } => {
                                let mut __serde_state = _serde::Serializer::serialize_struct(
                                    __serializer,
                                    "Message",
                                    0 + 1 + 1,
                                )?;
                                _serde::ser::SerializeStruct::serialize_field(
                                    &mut __serde_state,
                                    "user",
                                    "System",
                                )?;
                                _serde::ser::SerializeStruct::serialize_field(
                                    &mut __serde_state,
                                    "prompt",
                                    prompt,
                                )?;
                                _serde::ser::SerializeStruct::end(__serde_state)
                            }
                            Message::Assistant { ref message, ref thinking } => {
                                let mut __serde_state = _serde::Serializer::serialize_struct(
                                    __serializer,
                                    "Message",
                                    0 + 1 + 1 + 1,
                                )?;
                                _serde::ser::SerializeStruct::serialize_field(
                                    &mut __serde_state,
                                    "user",
                                    "Assistant",
                                )?;
                                _serde::ser::SerializeStruct::serialize_field(
                                    &mut __serde_state,
                                    "message",
                                    message,
                                )?;
                                _serde::ser::SerializeStruct::serialize_field(
                                    &mut __serde_state,
                                    "thinking",
                                    thinking,
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
                impl<'de> _serde::Deserialize<'de> for Message {
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
                                    "User" => _serde::__private228::Ok(__Field::__field0),
                                    "System" => _serde::__private228::Ok(__Field::__field1),
                                    "Assistant" => _serde::__private228::Ok(__Field::__field2),
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
                                    b"User" => _serde::__private228::Ok(__Field::__field0),
                                    b"System" => _serde::__private228::Ok(__Field::__field1),
                                    b"Assistant" => _serde::__private228::Ok(__Field::__field2),
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
                            "User",
                            "System",
                            "Assistant",
                        ];
                        let (__tag, __content) = _serde::Deserializer::deserialize_any(
                            __deserializer,
                            _serde::__private228::de::TaggedContentVisitor::<
                                __Field,
                            >::new("user", "internally tagged enum Message"),
                        )?;
                        let __deserializer = _serde::__private228::de::ContentDeserializer::<
                            __D::Error,
                        >::new(__content);
                        match __tag {
                            __Field::__field0 => {
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
                                            "message" => _serde::__private228::Ok(__Field::__field0),
                                            "images" => _serde::__private228::Ok(__Field::__field1),
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
                                            b"message" => _serde::__private228::Ok(__Field::__field0),
                                            b"images" => _serde::__private228::Ok(__Field::__field1),
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
                                    marker: _serde::__private228::PhantomData<Message>,
                                    lifetime: _serde::__private228::PhantomData<&'de ()>,
                                }
                                #[automatically_derived]
                                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                    type Value = Message;
                                    fn expecting(
                                        &self,
                                        __formatter: &mut _serde::__private228::Formatter,
                                    ) -> _serde::__private228::fmt::Result {
                                        _serde::__private228::Formatter::write_str(
                                            __formatter,
                                            "struct variant Message::User",
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
                                                        &"struct variant Message::User with 2 elements",
                                                    ),
                                                );
                                            }
                                        };
                                        let __field1 = match _serde::de::SeqAccess::next_element::<
                                            Option<Vec<String>>,
                                        >(&mut __seq)? {
                                            _serde::__private228::Some(__value) => __value,
                                            _serde::__private228::None => {
                                                return _serde::__private228::Err(
                                                    _serde::de::Error::invalid_length(
                                                        1usize,
                                                        &"struct variant Message::User with 2 elements",
                                                    ),
                                                );
                                            }
                                        };
                                        _serde::__private228::Ok(Message::User {
                                            message: __field0,
                                            images: __field1,
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
                                        let mut __field1: _serde::__private228::Option<
                                            Option<Vec<String>>,
                                        > = _serde::__private228::None;
                                        while let _serde::__private228::Some(__key) = _serde::de::MapAccess::next_key::<
                                            __Field,
                                        >(&mut __map)? {
                                            match __key {
                                                __Field::__field0 => {
                                                    if _serde::__private228::Option::is_some(&__field0) {
                                                        return _serde::__private228::Err(
                                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                                "message",
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
                                                            <__A::Error as _serde::de::Error>::duplicate_field("images"),
                                                        );
                                                    }
                                                    __field1 = _serde::__private228::Some(
                                                        _serde::de::MapAccess::next_value::<
                                                            Option<Vec<String>>,
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
                                                _serde::__private228::de::missing_field("message")?
                                            }
                                        };
                                        let __field1 = match __field1 {
                                            _serde::__private228::Some(__field1) => __field1,
                                            _serde::__private228::None => {
                                                _serde::__private228::de::missing_field("images")?
                                            }
                                        };
                                        _serde::__private228::Ok(Message::User {
                                            message: __field0,
                                            images: __field1,
                                        })
                                    }
                                }
                                #[doc(hidden)]
                                const FIELDS: &'static [&'static str] = &[
                                    "message",
                                    "images",
                                ];
                                _serde::Deserializer::deserialize_any(
                                    __deserializer,
                                    __Visitor {
                                        marker: _serde::__private228::PhantomData::<Message>,
                                        lifetime: _serde::__private228::PhantomData,
                                    },
                                )
                            }
                            __Field::__field1 => {
                                #[allow(non_camel_case_types)]
                                #[doc(hidden)]
                                enum __Field {
                                    __field0,
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
                                            "prompt" => _serde::__private228::Ok(__Field::__field0),
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
                                            b"prompt" => _serde::__private228::Ok(__Field::__field0),
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
                                    marker: _serde::__private228::PhantomData<Message>,
                                    lifetime: _serde::__private228::PhantomData<&'de ()>,
                                }
                                #[automatically_derived]
                                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                    type Value = Message;
                                    fn expecting(
                                        &self,
                                        __formatter: &mut _serde::__private228::Formatter,
                                    ) -> _serde::__private228::fmt::Result {
                                        _serde::__private228::Formatter::write_str(
                                            __formatter,
                                            "struct variant Message::System",
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
                                                        &"struct variant Message::System with 1 element",
                                                    ),
                                                );
                                            }
                                        };
                                        _serde::__private228::Ok(Message::System {
                                            prompt: __field0,
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
                                        while let _serde::__private228::Some(__key) = _serde::de::MapAccess::next_key::<
                                            __Field,
                                        >(&mut __map)? {
                                            match __key {
                                                __Field::__field0 => {
                                                    if _serde::__private228::Option::is_some(&__field0) {
                                                        return _serde::__private228::Err(
                                                            <__A::Error as _serde::de::Error>::duplicate_field("prompt"),
                                                        );
                                                    }
                                                    __field0 = _serde::__private228::Some(
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
                                                _serde::__private228::de::missing_field("prompt")?
                                            }
                                        };
                                        _serde::__private228::Ok(Message::System {
                                            prompt: __field0,
                                        })
                                    }
                                }
                                #[doc(hidden)]
                                const FIELDS: &'static [&'static str] = &["prompt"];
                                _serde::Deserializer::deserialize_any(
                                    __deserializer,
                                    __Visitor {
                                        marker: _serde::__private228::PhantomData::<Message>,
                                        lifetime: _serde::__private228::PhantomData,
                                    },
                                )
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
                                            "message" => _serde::__private228::Ok(__Field::__field0),
                                            "thinking" => _serde::__private228::Ok(__Field::__field1),
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
                                            b"message" => _serde::__private228::Ok(__Field::__field0),
                                            b"thinking" => _serde::__private228::Ok(__Field::__field1),
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
                                    marker: _serde::__private228::PhantomData<Message>,
                                    lifetime: _serde::__private228::PhantomData<&'de ()>,
                                }
                                #[automatically_derived]
                                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                    type Value = Message;
                                    fn expecting(
                                        &self,
                                        __formatter: &mut _serde::__private228::Formatter,
                                    ) -> _serde::__private228::fmt::Result {
                                        _serde::__private228::Formatter::write_str(
                                            __formatter,
                                            "struct variant Message::Assistant",
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
                                                        &"struct variant Message::Assistant with 2 elements",
                                                    ),
                                                );
                                            }
                                        };
                                        let __field1 = match _serde::de::SeqAccess::next_element::<
                                            Option<String>,
                                        >(&mut __seq)? {
                                            _serde::__private228::Some(__value) => __value,
                                            _serde::__private228::None => {
                                                return _serde::__private228::Err(
                                                    _serde::de::Error::invalid_length(
                                                        1usize,
                                                        &"struct variant Message::Assistant with 2 elements",
                                                    ),
                                                );
                                            }
                                        };
                                        _serde::__private228::Ok(Message::Assistant {
                                            message: __field0,
                                            thinking: __field1,
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
                                        let mut __field1: _serde::__private228::Option<
                                            Option<String>,
                                        > = _serde::__private228::None;
                                        while let _serde::__private228::Some(__key) = _serde::de::MapAccess::next_key::<
                                            __Field,
                                        >(&mut __map)? {
                                            match __key {
                                                __Field::__field0 => {
                                                    if _serde::__private228::Option::is_some(&__field0) {
                                                        return _serde::__private228::Err(
                                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                                "message",
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
                                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                                "thinking",
                                                            ),
                                                        );
                                                    }
                                                    __field1 = _serde::__private228::Some(
                                                        _serde::de::MapAccess::next_value::<
                                                            Option<String>,
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
                                                _serde::__private228::de::missing_field("message")?
                                            }
                                        };
                                        let __field1 = match __field1 {
                                            _serde::__private228::Some(__field1) => __field1,
                                            _serde::__private228::None => {
                                                _serde::__private228::de::missing_field("thinking")?
                                            }
                                        };
                                        _serde::__private228::Ok(Message::Assistant {
                                            message: __field0,
                                            thinking: __field1,
                                        })
                                    }
                                }
                                #[doc(hidden)]
                                const FIELDS: &'static [&'static str] = &[
                                    "message",
                                    "thinking",
                                ];
                                _serde::Deserializer::deserialize_any(
                                    __deserializer,
                                    __Visitor {
                                        marker: _serde::__private228::PhantomData::<Message>,
                                        lifetime: _serde::__private228::PhantomData,
                                    },
                                )
                            }
                        }
                    }
                }
            };
            #[serde(tag = "event")]
            pub enum OllamaRequest {
                Init { history: History },
                ChatCompletion { prompt: String, images: Option<Vec<String>> },
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for OllamaRequest {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match self {
                        OllamaRequest::Init { history: __self_0 } => {
                            ::core::fmt::Formatter::debug_struct_field1_finish(
                                f,
                                "Init",
                                "history",
                                &__self_0,
                            )
                        }
                        OllamaRequest::ChatCompletion {
                            prompt: __self_0,
                            images: __self_1,
                        } => {
                            ::core::fmt::Formatter::debug_struct_field2_finish(
                                f,
                                "ChatCompletion",
                                "prompt",
                                __self_0,
                                "images",
                                &__self_1,
                            )
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
                impl _serde::Serialize for OllamaRequest {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> _serde::__private228::Result<__S::Ok, __S::Error>
                    where
                        __S: _serde::Serializer,
                    {
                        match *self {
                            OllamaRequest::Init { ref history } => {
                                let mut __serde_state = _serde::Serializer::serialize_struct(
                                    __serializer,
                                    "OllamaRequest",
                                    0 + 1 + 1,
                                )?;
                                _serde::ser::SerializeStruct::serialize_field(
                                    &mut __serde_state,
                                    "event",
                                    "Init",
                                )?;
                                _serde::ser::SerializeStruct::serialize_field(
                                    &mut __serde_state,
                                    "history",
                                    history,
                                )?;
                                _serde::ser::SerializeStruct::end(__serde_state)
                            }
                            OllamaRequest::ChatCompletion { ref prompt, ref images } => {
                                let mut __serde_state = _serde::Serializer::serialize_struct(
                                    __serializer,
                                    "OllamaRequest",
                                    0 + 1 + 1 + 1,
                                )?;
                                _serde::ser::SerializeStruct::serialize_field(
                                    &mut __serde_state,
                                    "event",
                                    "ChatCompletion",
                                )?;
                                _serde::ser::SerializeStruct::serialize_field(
                                    &mut __serde_state,
                                    "prompt",
                                    prompt,
                                )?;
                                _serde::ser::SerializeStruct::serialize_field(
                                    &mut __serde_state,
                                    "images",
                                    images,
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
                impl<'de> _serde::Deserialize<'de> for OllamaRequest {
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
                                    _ => {
                                        _serde::__private228::Err(
                                            _serde::de::Error::invalid_value(
                                                _serde::de::Unexpected::Unsigned(__value),
                                                &"variant index 0 <= i < 2",
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
                                    "Init" => _serde::__private228::Ok(__Field::__field0),
                                    "ChatCompletion" => {
                                        _serde::__private228::Ok(__Field::__field1)
                                    }
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
                                    b"Init" => _serde::__private228::Ok(__Field::__field0),
                                    b"ChatCompletion" => {
                                        _serde::__private228::Ok(__Field::__field1)
                                    }
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
                            "Init",
                            "ChatCompletion",
                        ];
                        let (__tag, __content) = _serde::Deserializer::deserialize_any(
                            __deserializer,
                            _serde::__private228::de::TaggedContentVisitor::<
                                __Field,
                            >::new("event", "internally tagged enum OllamaRequest"),
                        )?;
                        let __deserializer = _serde::__private228::de::ContentDeserializer::<
                            __D::Error,
                        >::new(__content);
                        match __tag {
                            __Field::__field0 => {
                                #[allow(non_camel_case_types)]
                                #[doc(hidden)]
                                enum __Field {
                                    __field0,
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
                                            "history" => _serde::__private228::Ok(__Field::__field0),
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
                                            b"history" => _serde::__private228::Ok(__Field::__field0),
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
                                    marker: _serde::__private228::PhantomData<OllamaRequest>,
                                    lifetime: _serde::__private228::PhantomData<&'de ()>,
                                }
                                #[automatically_derived]
                                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                    type Value = OllamaRequest;
                                    fn expecting(
                                        &self,
                                        __formatter: &mut _serde::__private228::Formatter,
                                    ) -> _serde::__private228::fmt::Result {
                                        _serde::__private228::Formatter::write_str(
                                            __formatter,
                                            "struct variant OllamaRequest::Init",
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
                                            History,
                                        >(&mut __seq)? {
                                            _serde::__private228::Some(__value) => __value,
                                            _serde::__private228::None => {
                                                return _serde::__private228::Err(
                                                    _serde::de::Error::invalid_length(
                                                        0usize,
                                                        &"struct variant OllamaRequest::Init with 1 element",
                                                    ),
                                                );
                                            }
                                        };
                                        _serde::__private228::Ok(OllamaRequest::Init {
                                            history: __field0,
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
                                        let mut __field0: _serde::__private228::Option<History> = _serde::__private228::None;
                                        while let _serde::__private228::Some(__key) = _serde::de::MapAccess::next_key::<
                                            __Field,
                                        >(&mut __map)? {
                                            match __key {
                                                __Field::__field0 => {
                                                    if _serde::__private228::Option::is_some(&__field0) {
                                                        return _serde::__private228::Err(
                                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                                "history",
                                                            ),
                                                        );
                                                    }
                                                    __field0 = _serde::__private228::Some(
                                                        _serde::de::MapAccess::next_value::<History>(&mut __map)?,
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
                                                _serde::__private228::de::missing_field("history")?
                                            }
                                        };
                                        _serde::__private228::Ok(OllamaRequest::Init {
                                            history: __field0,
                                        })
                                    }
                                }
                                #[doc(hidden)]
                                const FIELDS: &'static [&'static str] = &["history"];
                                _serde::Deserializer::deserialize_any(
                                    __deserializer,
                                    __Visitor {
                                        marker: _serde::__private228::PhantomData::<OllamaRequest>,
                                        lifetime: _serde::__private228::PhantomData,
                                    },
                                )
                            }
                            __Field::__field1 => {
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
                                            "prompt" => _serde::__private228::Ok(__Field::__field0),
                                            "images" => _serde::__private228::Ok(__Field::__field1),
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
                                            b"prompt" => _serde::__private228::Ok(__Field::__field0),
                                            b"images" => _serde::__private228::Ok(__Field::__field1),
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
                                    marker: _serde::__private228::PhantomData<OllamaRequest>,
                                    lifetime: _serde::__private228::PhantomData<&'de ()>,
                                }
                                #[automatically_derived]
                                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                    type Value = OllamaRequest;
                                    fn expecting(
                                        &self,
                                        __formatter: &mut _serde::__private228::Formatter,
                                    ) -> _serde::__private228::fmt::Result {
                                        _serde::__private228::Formatter::write_str(
                                            __formatter,
                                            "struct variant OllamaRequest::ChatCompletion",
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
                                                        &"struct variant OllamaRequest::ChatCompletion with 2 elements",
                                                    ),
                                                );
                                            }
                                        };
                                        let __field1 = match _serde::de::SeqAccess::next_element::<
                                            Option<Vec<String>>,
                                        >(&mut __seq)? {
                                            _serde::__private228::Some(__value) => __value,
                                            _serde::__private228::None => {
                                                return _serde::__private228::Err(
                                                    _serde::de::Error::invalid_length(
                                                        1usize,
                                                        &"struct variant OllamaRequest::ChatCompletion with 2 elements",
                                                    ),
                                                );
                                            }
                                        };
                                        _serde::__private228::Ok(OllamaRequest::ChatCompletion {
                                            prompt: __field0,
                                            images: __field1,
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
                                        let mut __field1: _serde::__private228::Option<
                                            Option<Vec<String>>,
                                        > = _serde::__private228::None;
                                        while let _serde::__private228::Some(__key) = _serde::de::MapAccess::next_key::<
                                            __Field,
                                        >(&mut __map)? {
                                            match __key {
                                                __Field::__field0 => {
                                                    if _serde::__private228::Option::is_some(&__field0) {
                                                        return _serde::__private228::Err(
                                                            <__A::Error as _serde::de::Error>::duplicate_field("prompt"),
                                                        );
                                                    }
                                                    __field0 = _serde::__private228::Some(
                                                        _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                                    );
                                                }
                                                __Field::__field1 => {
                                                    if _serde::__private228::Option::is_some(&__field1) {
                                                        return _serde::__private228::Err(
                                                            <__A::Error as _serde::de::Error>::duplicate_field("images"),
                                                        );
                                                    }
                                                    __field1 = _serde::__private228::Some(
                                                        _serde::de::MapAccess::next_value::<
                                                            Option<Vec<String>>,
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
                                                _serde::__private228::de::missing_field("prompt")?
                                            }
                                        };
                                        let __field1 = match __field1 {
                                            _serde::__private228::Some(__field1) => __field1,
                                            _serde::__private228::None => {
                                                _serde::__private228::de::missing_field("images")?
                                            }
                                        };
                                        _serde::__private228::Ok(OllamaRequest::ChatCompletion {
                                            prompt: __field0,
                                            images: __field1,
                                        })
                                    }
                                }
                                #[doc(hidden)]
                                const FIELDS: &'static [&'static str] = &[
                                    "prompt",
                                    "images",
                                ];
                                _serde::Deserializer::deserialize_any(
                                    __deserializer,
                                    __Visitor {
                                        marker: _serde::__private228::PhantomData::<OllamaRequest>,
                                        lifetime: _serde::__private228::PhantomData,
                                    },
                                )
                            }
                        }
                    }
                }
            };
            pub struct OllamaMsgResp {
                pub content: String,
                pub thinking: Option<String>,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for OllamaMsgResp {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "OllamaMsgResp",
                        "content",
                        &self.content,
                        "thinking",
                        &&self.thinking,
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
                impl _serde::Serialize for OllamaMsgResp {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> _serde::__private228::Result<__S::Ok, __S::Error>
                    where
                        __S: _serde::Serializer,
                    {
                        let mut __serde_state = _serde::Serializer::serialize_struct(
                            __serializer,
                            "OllamaMsgResp",
                            false as usize + 1 + 1,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "content",
                            &self.content,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "thinking",
                            &self.thinking,
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
                impl<'de> _serde::Deserialize<'de> for OllamaMsgResp {
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
                                    "content" => _serde::__private228::Ok(__Field::__field0),
                                    "thinking" => _serde::__private228::Ok(__Field::__field1),
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
                                    b"content" => _serde::__private228::Ok(__Field::__field0),
                                    b"thinking" => _serde::__private228::Ok(__Field::__field1),
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
                            marker: _serde::__private228::PhantomData<OllamaMsgResp>,
                            lifetime: _serde::__private228::PhantomData<&'de ()>,
                        }
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = OllamaMsgResp;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private228::Formatter,
                            ) -> _serde::__private228::fmt::Result {
                                _serde::__private228::Formatter::write_str(
                                    __formatter,
                                    "struct OllamaMsgResp",
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
                                                &"struct OllamaMsgResp with 2 elements",
                                            ),
                                        );
                                    }
                                };
                                let __field1 = match _serde::de::SeqAccess::next_element::<
                                    Option<String>,
                                >(&mut __seq)? {
                                    _serde::__private228::Some(__value) => __value,
                                    _serde::__private228::None => {
                                        return _serde::__private228::Err(
                                            _serde::de::Error::invalid_length(
                                                1usize,
                                                &"struct OllamaMsgResp with 2 elements",
                                            ),
                                        );
                                    }
                                };
                                _serde::__private228::Ok(OllamaMsgResp {
                                    content: __field0,
                                    thinking: __field1,
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
                                let mut __field1: _serde::__private228::Option<
                                    Option<String>,
                                > = _serde::__private228::None;
                                while let _serde::__private228::Some(__key) = _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map)? {
                                    match __key {
                                        __Field::__field0 => {
                                            if _serde::__private228::Option::is_some(&__field0) {
                                                return _serde::__private228::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field(
                                                        "content",
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
                                                    <__A::Error as _serde::de::Error>::duplicate_field(
                                                        "thinking",
                                                    ),
                                                );
                                            }
                                            __field1 = _serde::__private228::Some(
                                                _serde::de::MapAccess::next_value::<
                                                    Option<String>,
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
                                        _serde::__private228::de::missing_field("content")?
                                    }
                                };
                                let __field1 = match __field1 {
                                    _serde::__private228::Some(__field1) => __field1,
                                    _serde::__private228::None => {
                                        _serde::__private228::de::missing_field("thinking")?
                                    }
                                };
                                _serde::__private228::Ok(OllamaMsgResp {
                                    content: __field0,
                                    thinking: __field1,
                                })
                            }
                        }
                        #[doc(hidden)]
                        const FIELDS: &'static [&'static str] = &["content", "thinking"];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            "OllamaMsgResp",
                            FIELDS,
                            __Visitor {
                                marker: _serde::__private228::PhantomData::<OllamaMsgResp>,
                                lifetime: _serde::__private228::PhantomData,
                            },
                        )
                    }
                }
            };
        }
        pub async fn chat(req: HttpRequest, stream: Payload) -> Result<HttpResponse> {
            let headers = req.headers();
            let (Some(session), Some(model)) = (
                headers.get("Authorization"),
                headers.get("model"),
            ) else {
                return Ok(
                    HttpResponse::Unauthorized()
                        .body(
                            "{\"msg\": \"Headers `Authorization`, `model` are necessary\"}",
                        ),
                );
            };
            let Ok(model) = model.to_str() else {
                return Ok(
                    HttpResponse::Unauthorized()
                        .body("{\"msg\": \"Invalid `model` header\"}"),
                );
            };
            let Ok(session) = session.to_str() else {
                return Ok(
                    HttpResponse::Unauthorized()
                        .body("{\"msg\": \"Invalid `session` header\"}"),
                );
            };
            if let Some(auth) = AUTH.get() && !auth.verify_session(session).await {
                return Ok(
                    HttpResponse::Unauthorized()
                        .body("{\"msg\": \"Invalid SessionToken\"}"),
                );
            }
            let img_capable;
            if CONFIG.ollama.cvmodels.contains(model) {
                img_capable = true;
            } else if CONFIG.ollama.txtmodels.contains(model) {
                img_capable = false;
            } else {
                return Ok(
                    HttpResponse::NotFound().body("{\"msg\": \"Model not found!\"}"),
                );
            }
            let model = model.to_owned();
            let (res, mut session, stream) = actix_ws::handle(&req, stream)?;
            let mut stream = stream
                .aggregate_continuations()
                .max_continuation_size(8 * 1024 * 1024);
            rt::spawn(async move {
                let mut model = model;
                let img_capable = img_capable;
                let mut init = false;
                let mut history = Vec::with_capacity(*HISTORY_LENGTH);
                while let Some(msg) = stream.recv().await {
                    match msg {
                        Ok(AggregatedMessage::Text(x)) => {
                            let Ok::<OllamaRequest, _>(x) = serde_json::from_reader(
                                &*x.into_bytes(),
                            ) else {
                                break;
                            };
                            model = handle_msg(
                                    model,
                                    &mut history,
                                    img_capable,
                                    &mut init,
                                    x,
                                    &mut session,
                                    true,
                                )
                                .await;
                        }
                        Ok(AggregatedMessage::Binary(x)) => {
                            let Ok::<OllamaRequest, _>(x) = ciborium::from_reader(&*x)
                            else {
                                break;
                            };
                            model = handle_msg(
                                    model,
                                    &mut history,
                                    img_capable,
                                    &mut init,
                                    x,
                                    &mut session,
                                    false,
                                )
                                .await;
                        }
                        Ok(AggregatedMessage::Close(_)) => break,
                        Ok(AggregatedMessage::Ping(_)) => break,
                        Ok(AggregatedMessage::Pong(_)) => break,
                        _ => break,
                    }
                    if model.is_empty() {
                        break;
                    }
                }
                _ = session.close(None).await;
            });
            Ok(res)
        }
        async fn handle_msg(
            model: String,
            history: &mut Vec<ChatMessage>,
            img_capable: bool,
            init: &mut bool,
            msg: OllamaRequest,
            session: &mut Session,
            using_json: bool,
        ) -> String {
            match handle_msg_faillable(
                    model,
                    history,
                    img_capable,
                    init,
                    msg,
                    session,
                    using_json,
                )
                .await
            {
                Some(model) => model,
                _ => {
                    if using_json {
                        _ = session.text(r#"{ "msg": "Internal Server Error" }"#).await;
                    } else {
                        _ = session
                            .text(r#"{ "msg": "Internal Server Error TODO: BSON" }"#)
                            .await;
                    }
                    String::with_capacity(0)
                }
            }
        }
        async fn handle_msg_faillable(
            model: String,
            history: &mut Vec<ChatMessage>,
            img_capable: bool,
            init: &mut bool,
            msg: OllamaRequest,
            session: &mut Session,
            using_json: bool,
        ) -> Option<String> {
            match msg {
                OllamaRequest::Init { history: hist } => {
                    if *init {
                        if using_json {
                            _ = session
                                .text(r#"{ "msg": "Already initialized" }"#)
                                .await;
                        } else {
                            _ = session
                                .text(r#"{ "msg": "Already initialized TODO: BSON" }"#)
                                .await;
                        }
                        return Some(model);
                    }
                    if hist.len() > *HISTORY_LENGTH {
                        if using_json {
                            _ = session
                                .text(r#"{ "msg": "Max History length reached" }"#)
                                .await;
                        } else {
                            _ = session
                                .text(
                                    r#"{ "msg": "Max History length reached TODO: BSON" }"#,
                                )
                                .await;
                        }
                        return Some(model);
                    }
                    *init = true;
                    history
                        .extend(
                            hist
                                .into_iter()
                                .map(|x| match x {
                                    Message::User { message, images } => {
                                        let mut msg = ChatMessage::new(MessageRole::User, message);
                                        if let Some(images) = images {
                                            msg = msg
                                                .with_images(
                                                    images
                                                        .into_iter()
                                                        .map(Image::from_base64)
                                                        .collect::<Vec<_>>(),
                                                );
                                        }
                                        msg
                                    }
                                    Message::System { prompt } => {
                                        ChatMessage::new(MessageRole::System, prompt)
                                    }
                                    Message::Assistant { message, thinking } => {
                                        let mut msg = ChatMessage::new(
                                            MessageRole::Assistant,
                                            message,
                                        );
                                        msg.thinking = thinking;
                                        msg
                                    }
                                }),
                        );
                    Some(model)
                }
                OllamaRequest::ChatCompletion { prompt, images } => {
                    if !*init {
                        if using_json {
                            _ = session
                                .text(r#"{ "msg": "Initialization Required" }"#)
                                .await;
                        } else {
                            _ = session
                                .text(r#"{ "msg": "Initialization Required TODO: BSON" }"#)
                                .await;
                        }
                        return Some(model);
                    }
                    if history.len() > *HISTORY_LENGTH {
                        if using_json {
                            _ = session
                                .text(r#"{ "msg": "Maximum message length reached!" }"#)
                                .await;
                        } else {
                            _ = session
                                .text(
                                    r#"{ "msg": "Maximum message length reached TODO: BSON!" }"#,
                                )
                                .await;
                        }
                        return None;
                    }
                    let mut message = ChatMessage::user(prompt);
                    if let Some(images) = images {
                        if !img_capable {
                            if using_json {
                                _ = session
                                    .text(r#"{ "msg": "The model is not image capable" }"#)
                                    .await;
                            } else {
                                _ = session
                                    .text(
                                        r#"{ "msg": "The model is not image capable (TODO: BSON)" }"#,
                                    )
                                    .await;
                            }
                            return None;
                        }
                        message = message
                            .with_images(
                                images
                                    .into_iter()
                                    .map(Image::from_base64)
                                    .collect::<Vec<_>>(),
                            );
                    }
                    let resp = OLLAMA
                        .send_chat_messages_with_history(
                            history,
                            ChatMessageRequest::new(
                                model,
                                <[_]>::into_vec(::alloc::boxed::box_new([message])),
                            ),
                        )
                        .await
                        .ok()?;
                    let out = OllamaMsgResp {
                        content: resp.message.content,
                        thinking: resp.message.thinking,
                    };
                    if using_json {
                        _ = session.text(serde_json::to_string(&out).ok()?).await;
                    } else {
                        let mut bytes = ::alloc::vec::Vec::new();
                        ciborium::into_writer(&out, &mut bytes).ok()?;
                        _ = session.binary(bytes).await;
                    }
                    Some(resp.model)
                }
            }
        }
    }
    pub mod http {
        use actix_web::{
            HttpResponse, Responder, Result, get, http::header::ContentType, post,
            web::Bytes,
        };
        use crate::{auth::AGENT, server::{AUTH, http::structs::ROOT_RESPONSE_DATA}};
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
                version: &'static str,
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
                            false as usize + 1 + 1 + 1 + 1 + 1,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "version",
                            &self.version,
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
                        version: "0.1.3",
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
        #[allow(non_camel_case_types, missing_docs)]
        pub struct me;
        impl ::actix_web::dev::HttpServiceFactory for me {
            fn register(self, __config: &mut actix_web::dev::AppService) {
                async fn me(payload: Bytes) -> Result<impl Responder> {
                    let session = str::from_utf8(&payload);
                    match session {
                        Ok(session) => {
                            let auth_ref = AUTH
                                .get()
                                .expect(
                                    "Auth must be defined or else this function cant be registered",
                                );
                            if auth_ref.verify_session(session).await {
                                Ok(HttpResponse::Ok().body::<&[u8]>(br#"{ "msg": "Ok" }"#))
                            } else {
                                Ok(
                                    HttpResponse::Unauthorized()
                                        .body::<&[u8]>(br#"{ "msg": "Unauthorized" }"#),
                                )
                            }
                        }
                        _ => {
                            Ok(
                                HttpResponse::BadRequest()
                                    .body::<&[u8]>(br#"{ "msg": "Bad Request" }"#),
                            )
                        }
                    }
                }
                let __resource = ::actix_web::Resource::new("/me")
                    .name("me")
                    .guard(::actix_web::guard::Get())
                    .to(me);
                ::actix_web::dev::HttpServiceFactory::register(__resource, __config);
            }
        }
        #[allow(non_camel_case_types, missing_docs)]
        pub struct challenge;
        impl ::actix_web::dev::HttpServiceFactory for challenge {
            fn register(self, __config: &mut actix_web::dev::AppService) {
                async fn challenge(payload: Bytes) -> Result<impl Responder> {
                    match AGENT.gen_signature(&payload).await {
                        Some(x) => Ok(HttpResponse::Ok().body(x.to_vec())),
                        _ => {
                            Ok(
                                HttpResponse::InternalServerError()
                                    .body::<&[u8]>(br#"{ "msg": "Unable to hash" }"#),
                            )
                        }
                    }
                }
                let __resource = ::actix_web::Resource::new("/challenge")
                    .name("challenge")
                    .guard(::actix_web::guard::Post())
                    .to(challenge);
                ::actix_web::dev::HttpServiceFactory::register(__resource, __config);
            }
        }
    }
    pub static CONFIG: LazyLock<Config> = LazyLock::new(|| {
        let data = stdfs::read_to_string("config.json").expect("Unable to load config");
        from_str(&data).expect("Invalid configuration file, unable to parse")
    });
    pub static DBCONF: LazyLock<DatabaseConfig> = LazyLock::new(|| DatabaseConfig::get());
    pub static HISTORY_LENGTH: LazyLock<usize> = LazyLock::new(|| {
        CONFIG.ollama.msgs.saturating_mul(2)
    });
    pub static TOKEN: LazyLock<bool> = LazyLock::new(|| {
        #[allow(non_exhaustive_omitted_patterns)]
        match CONFIG.authentication {
            Authentication::TokenBased => true,
            _ => false,
        }
    });
    pub static AUTH: OnceLock<AuthSessionManager> = OnceLock::new();
    pub static REAL_ADMIN_PASSWORD: OnceLock<SecretString> = OnceLock::new();
    pub static OLLAMA: LazyLock<Ollama> = LazyLock::new(|| Ollama::new(
        CONFIG.ollama.host.as_ref(),
        CONFIG.ollama.port,
    ));
    pub fn launch() -> Chalk {
        let mut chalk = Chalk::new();
        chalk
            .blue()
            .bold()
            .println(
                &::alloc::__export::must_use({
                    ::alloc::fmt::format(format_args!("AHQ-AI Server v{0}", "0.1.3"))
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
                    let auth = !#[allow(non_exhaustive_omitted_patterns)]
                    match CONFIG.authentication {
                        Authentication::OpenToAll => true,
                        _ => false,
                    };
                    let mut registration_api = false;
                    if auth {
                        if let Authentication::Account { registration_allowed, .. } = &CONFIG
                            .authentication
                        {
                            registration_api = *registration_allowed;
                        }
                        _ = AUTH.set(AuthSessionManager::create().await);
                    }
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
                    let admin_api = request_admin_passwd();
                    let mut server = HttpServer::new(move || {
                            let mut app = App::new()
                                .service(http::index)
                                .route("/chat", web::get().to(chat::chat))
                                .service(http::challenge)
                                .service(http::me);
                            let auth = !#[allow(non_exhaustive_omitted_patterns)]
                            match CONFIG.authentication {
                                Authentication::OpenToAll => true,
                                _ => false,
                            };
                            if auth {
                                app = app.service(auth::auth);
                            }
                            if admin_api {
                                app = app
                                    .service(admin::verify)
                                    .service(admin::list)
                                    .service(admin::create)
                                    .service(admin::create_token)
                                    .service(admin::delete);
                            }
                            if registration_api {
                                app = app.service(auth::register);
                            }
                            app
                        })
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
                    chalk.blue().println(&"Server is starting!");
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
                    chalk
                        .reset_style()
                        .blue()
                        .bold()
                        .println(
                            &"Server state has been successfully set! Closing server. Session tokens will be discarded.",
                        );
                    out
                }
            })
    }
    fn request_admin_passwd() -> bool {
        if let Some(x) = &CONFIG.admin_pass_hash {
            let hash = x as &str;
            let passwd;
            if let Ok(x) = env::var("AHQAI_ADMIN_PASSWORD") {
                passwd = x;
            } else {
                {
                    ::std::io::_print(format_args!("----------------\n"));
                };
                {
                    ::std::io::_print(
                        format_args!(
                            "THE GIVEN SERVER IS PROTECTED BY SERVER ADMIN PASSWORD\n",
                        ),
                    );
                };
                {
                    ::std::io::_print(
                        format_args!(
                            "BUT THE `AHQAI_ADMIN_PASSWORD` VARIABLE WAS NOT FOUND\n",
                        ),
                    );
                };
                {
                    ::std::io::_print(
                        format_args!(
                            "IN THE CURRENT SERVER ENVIRONMENT. REQUESTING MANUAL ENTRY\n",
                        ),
                    );
                };
                {
                    ::std::io::_print(format_args!("----------------\n"));
                };
                {
                    ::std::io::_print(format_args!("\n"));
                };
                passwd = rpassword::prompt_password(
                        "Enter your administrator password : ",
                    )
                    .expect("Unable to read your password");
            }
            if !verify(&passwd, hash).unwrap_or(false) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("Invalid Password was provided"),
                    );
                }
            }
            {
                ::std::io::_print(format_args!("\n"));
            };
            {
                ::std::io::_print(format_args!("----------------\n"));
            };
            {
                ::std::io::_print(
                    format_args!("SERVER ADMIN PASSWORD AUTH SUCCESSFUL\n"),
                );
            };
            {
                ::std::io::_print(format_args!("SERVER WILL START UP NOW\n"));
            };
            {
                ::std::io::_print(format_args!("----------------\n"));
            };
            {
                ::std::io::_print(format_args!("\n"));
            };
            REAL_ADMIN_PASSWORD
                .set(SecretString::from(passwd))
                .expect("Impossible Error");
            return true;
        }
        false
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
    use crate::structs::{Authentication, BCRYPT_COST, Config};
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
                        "This means that the application would be required to supply a token for the purposes of verification. The token will be verified and finally then the application can interact with the server. This is as fast as Account Verification.",
                    ),
                );
            }
        }
        mod user {
            use cursive::{
                align::Align, theme::{Effect, Style},
                view::{Nameable, Resizable},
                views::{Button, Dialog, DummyView, LinearLayout, SelectView, TextView},
            };
            use crate::{
                structs::{Authentication, Config},
                ui::Ptr,
            };
            pub fn render(l: &mut LinearLayout, can_register: bool) {
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
                                        Authentication::Account { registration_allowed } => {
                                            user::render(layout, registration_allowed)
                                        }
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
                            state_.insert(model.to_string().into_boxed_str());
                            let state = state_.clone();
                            x.call_on_name(
                                "models",
                                |l: &mut LinearLayout| {
                                    iterate_layout(l, state.iter(), cv);
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
                    if cv {
                        s.ollama.cvmodels.iter()
                    } else {
                        s.ollama.txtmodels.iter()
                    },
                    cv,
                );
                layout.with_name("models")
            }
            fn iterate_layout<'a, T>(l: &mut LinearLayout, binds: T, cv: bool)
            where
                T: Iterator<Item = &'a Box<str>>,
            {
                l.clear();
                let binds = binds.collect::<Vec<_>>();
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
                let index_str = model.to_owned();
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
                                            x.ollama.cvmodels.remove(&index_str as &str);
                                        } else {
                                            x.ollama.txtmodels.remove(&index_str as &str);
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
                                            iterate_layout(l, state.iter(), cv);
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
                    TextView::new("Chat")
                        .style(Style::merge(&[Effect::Underline.into()])),
                );
            layout.add_child(msgs(s.clone()));
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
        fn msgs(s: Ptr<Config>) -> LinearLayout {
            LinearLayout::horizontal()
                .child(TextView::new("🖥 Max message pair per chat").full_width())
                .child(
                    Button::new_raw(
                            ::alloc::__export::must_use({
                                ::alloc::fmt::format(format_args!("<{0}>", &s.ollama.msgs))
                            }),
                            |x| {
                                x.add_layer(
                                    Dialog::around(
                                            EditView::new()
                                                .on_edit(|x, txt, _| {
                                                    let data: &mut Ptr<Config> = x.user_data().unwrap();
                                                    if let Ok(num) = txt.parse::<usize>() {
                                                        data.ollama.msgs = num;
                                                        x.call_on_name(
                                                            "ollama_msgs",
                                                            |x: &mut Button| {
                                                                x.set_label_raw(
                                                                    ::alloc::__export::must_use({
                                                                        ::alloc::fmt::format(format_args!("<{0}>", num))
                                                                    }),
                                                                );
                                                            },
                                                        );
                                                    }
                                                })
                                                .on_submit(|x, _| _ = x.pop_layer()),
                                        )
                                        .dismiss_button("Close")
                                        .title("Enter Maximum"),
                                );
                            },
                        )
                        .with_name("ollama_msgs"),
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
    use bcrypt::hash;
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
                        ::alloc::fmt::format(format_args!("AHQ AI Server v{0}", "0.1.3"))
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
                                                            hash(txt, BCRYPT_COST).expect("Unknown error"),
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
    use crate::{
        auth::{
            authserver::{AuthServer, tikv::TikvClient},
            hash::HashingAgent,
        },
        server::CONFIG, structs::{Authentication, BCRYPT_COST, error::Returns},
    };
    use base64::{Engine as _, engine::general_purpose};
    use bcrypt::hash;
    use moka::future::Cache;
    use rand::{Rng, seq::IndexedRandom};
    use std::{
        sync::{Arc, LazyLock},
        time::{Duration, SystemTime, UNIX_EPOCH},
    };
    use tokio::task::spawn_blocking;
    pub mod hash {
        use bcrypt::{hash, verify};
        use crossbeam_channel::{Sender, bounded};
        use ed25519_dalek::{Signature, SigningKey, ed25519::signature::SignerMut};
        use std::thread;
        use std::thread::available_parallelism;
        use tokio::sync::oneshot::{Sender as OneshotSender, channel};
        use crate::{auth::INTEGRITY_KEY, structs::BCRYPT_COST};
        pub struct HashingAgent(Sender<HashResp>);
        pub enum HashResp {
            CheckHash { pass: String, hash: String, tx: OneshotSender<Option<bool>> },
            GenHash { pass: String, tx: OneshotSender<Option<String>> },
            Challenge { bytes: Vec<u8>, tx: OneshotSender<Option<Signature>> },
        }
        impl HashingAgent {
            pub fn new() -> Self {
                let threads = available_parallelism()
                    .expect("Unable to get parallelism")
                    .get();
                let (tx, rx) = bounded::<HashResp>(2 * threads);
                for _ in 0..threads {
                    let rxc = rx.clone();
                    thread::spawn(move || {
                        let mut signer = SigningKey::from_keypair_bytes(INTEGRITY_KEY)
                            .unwrap();
                        while let Ok(x) = rxc.recv() {
                            match x {
                                HashResp::GenHash { pass, tx } => {
                                    _ = tx.send(hash(&pass, BCRYPT_COST).ok());
                                }
                                HashResp::CheckHash { pass, hash, tx } => {
                                    _ = tx.send(verify(&pass, &hash).ok());
                                }
                                HashResp::Challenge { bytes, tx } => {
                                    let sign = signer.try_sign(&bytes).ok();
                                    _ = tx.send(sign);
                                }
                            }
                        }
                    });
                }
                Self(tx)
            }
            /// # Cloning:
            /// This
            ///
            /// # Returns
            /// This function returns None in case of the server's queue being maxed out
            pub async fn verify_pass(&self, pass: &str, hash: &str) -> Option<bool> {
                if self.0.is_full() {
                    return None;
                }
                let hash: String = hash.to_owned();
                let pass: String = pass.to_owned();
                let (tx, rx) = channel::<Option<bool>>();
                self.0
                    .try_send(HashResp::CheckHash {
                        pass,
                        hash,
                        tx,
                    })
                    .ok()?;
                rx.await.ok()?
            }
            /// # Returns
            /// This function returns None in case of the server's queue being maxed out
            pub async fn gen_hash(&self, pass: &str) -> Option<String> {
                if self.0.is_full() {
                    return None;
                }
                let pass: String = pass.to_owned();
                let (tx, rx) = channel::<Option<String>>();
                self.0.try_send(HashResp::GenHash { pass, tx }).ok()?;
                rx.await.ok()?
            }
            /// # Returns
            /// This function returns None in case of the server's queue being maxed out
            pub async fn gen_signature(&self, data: &[u8]) -> Option<Signature> {
                if self.0.is_full() {
                    return None;
                }
                let bytes = data.to_owned();
                let (tx, rx) = channel::<Option<Signature>>();
                self.0.try_send(HashResp::Challenge { bytes, tx }).ok()?;
                rx.await.ok()?
            }
        }
        impl Default for HashingAgent {
            fn default() -> Self {
                Self::new()
            }
        }
    }
    pub mod authserver {
        use crate::structs::error::Returns;
        use async_trait::async_trait;
        pub mod mongodb {
            use async_trait::async_trait;
            use mongodb::Client;
            use crate::{auth::authserver::AuthServer, structs::error::Returns};
            pub struct MongodbClient {
                client: Client,
            }
            impl AuthServer for MongodbClient {
                #[allow(
                    elided_named_lifetimes,
                    clippy::async_yields_async,
                    clippy::diverging_sub_expression,
                    clippy::let_unit_value,
                    clippy::needless_arbitrary_self_type,
                    clippy::no_effect_underscore_binding,
                    clippy::shadow_same,
                    clippy::type_complexity,
                    clippy::type_repetition_in_bounds,
                    clippy::used_underscore_binding
                )]
                fn get<'a, 'async_trait>(
                    &'a self,
                    uid: &'a str,
                ) -> ::core::pin::Pin<
                    Box<
                        dyn ::core::future::Future<
                            Output = Returns<Option<String>>,
                        > + ::core::marker::Send + 'async_trait,
                    >,
                >
                where
                    'a: 'async_trait,
                    Self: 'async_trait,
                {
                    Box::pin(async move {
                        if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                            Returns<Option<String>>,
                        > {
                            #[allow(unreachable_code)] return __ret;
                        }
                        let __self = self;
                        let __ret: Returns<Option<String>> = {
                            ::core::panicking::panic("not yet implemented")
                        };
                        #[allow(unreachable_code)] __ret
                    })
                }
                #[allow(
                    elided_named_lifetimes,
                    clippy::async_yields_async,
                    clippy::diverging_sub_expression,
                    clippy::let_unit_value,
                    clippy::needless_arbitrary_self_type,
                    clippy::no_effect_underscore_binding,
                    clippy::shadow_same,
                    clippy::type_complexity,
                    clippy::type_repetition_in_bounds,
                    clippy::used_underscore_binding
                )]
                fn search<'a, 'async_trait>(
                    &'a self,
                    prefix: String,
                ) -> ::core::pin::Pin<
                    Box<
                        dyn ::core::future::Future<
                            Output = Returns<Vec<Vec<u8>>>,
                        > + ::core::marker::Send + 'async_trait,
                    >,
                >
                where
                    'a: 'async_trait,
                    Self: 'async_trait,
                {
                    Box::pin(async move {
                        if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                            Returns<Vec<Vec<u8>>>,
                        > {
                            #[allow(unreachable_code)] return __ret;
                        }
                        let __self = self;
                        let prefix = prefix;
                        let __ret: Returns<Vec<Vec<u8>>> = {
                            ::core::panicking::panic("not yet implemented")
                        };
                        #[allow(unreachable_code)] __ret
                    })
                }
                #[allow(
                    elided_named_lifetimes,
                    clippy::async_yields_async,
                    clippy::diverging_sub_expression,
                    clippy::let_unit_value,
                    clippy::needless_arbitrary_self_type,
                    clippy::no_effect_underscore_binding,
                    clippy::shadow_same,
                    clippy::type_complexity,
                    clippy::type_repetition_in_bounds,
                    clippy::used_underscore_binding
                )]
                fn exists<'a, 'async_trait>(
                    &'a self,
                    uid: &'a str,
                ) -> ::core::pin::Pin<
                    Box<
                        dyn ::core::future::Future<
                            Output = Returns<bool>,
                        > + ::core::marker::Send + 'async_trait,
                    >,
                >
                where
                    'a: 'async_trait,
                    Self: 'async_trait,
                {
                    Box::pin(async move {
                        if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                            Returns<bool>,
                        > {
                            #[allow(unreachable_code)] return __ret;
                        }
                        let __self = self;
                        let __ret: Returns<bool> = {
                            ::core::panicking::panic("not yet implemented")
                        };
                        #[allow(unreachable_code)] __ret
                    })
                }
                #[allow(
                    elided_named_lifetimes,
                    clippy::async_yields_async,
                    clippy::diverging_sub_expression,
                    clippy::let_unit_value,
                    clippy::needless_arbitrary_self_type,
                    clippy::no_effect_underscore_binding,
                    clippy::shadow_same,
                    clippy::type_complexity,
                    clippy::type_repetition_in_bounds,
                    clippy::used_underscore_binding
                )]
                fn update<'a, 'async_trait>(
                    &'a self,
                    uid: String,
                    data: String,
                ) -> ::core::pin::Pin<
                    Box<
                        dyn ::core::future::Future<
                            Output = Returns<()>,
                        > + ::core::marker::Send + 'async_trait,
                    >,
                >
                where
                    'a: 'async_trait,
                    Self: 'async_trait,
                {
                    Box::pin(async move {
                        if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                            Returns<()>,
                        > {
                            #[allow(unreachable_code)] return __ret;
                        }
                        let __self = self;
                        let uid = uid;
                        let data = data;
                        let __ret: Returns<()> = {
                            ::core::panicking::panic("not yet implemented")
                        };
                        #[allow(unreachable_code)] __ret
                    })
                }
                #[allow(
                    elided_named_lifetimes,
                    clippy::async_yields_async,
                    clippy::diverging_sub_expression,
                    clippy::let_unit_value,
                    clippy::needless_arbitrary_self_type,
                    clippy::no_effect_underscore_binding,
                    clippy::shadow_same,
                    clippy::type_complexity,
                    clippy::type_repetition_in_bounds,
                    clippy::used_underscore_binding
                )]
                fn remove<'a, 'async_trait>(
                    &'a self,
                    uid: String,
                ) -> ::core::pin::Pin<
                    Box<
                        dyn ::core::future::Future<
                            Output = Returns<()>,
                        > + ::core::marker::Send + 'async_trait,
                    >,
                >
                where
                    'a: 'async_trait,
                    Self: 'async_trait,
                {
                    Box::pin(async move {
                        if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                            Returns<()>,
                        > {
                            #[allow(unreachable_code)] return __ret;
                        }
                        let __self = self;
                        let uid = uid;
                        let __ret: Returns<()> = {
                            ::core::panicking::panic("not yet implemented")
                        };
                        #[allow(unreachable_code)] __ret
                    })
                }
            }
        }
        pub mod tikv {
            use async_trait::async_trait;
            use std::{path::PathBuf, str::FromStr, time::Duration};
            use tikv_client::{
                BoundRange, Config, Error, Key, RawClient, TransactionClient,
            };
            use tokio::time::sleep;
            use crate::{
                auth::authserver::AuthServer, server::DBCONF,
                structs::{db::AuthDbConfig, error::{Returns, ServerError}},
            };
            pub struct TikvClient {
                raw: RawClient,
                transactional: TransactionClient,
            }
            impl TikvClient {
                pub async fn new() -> Self {
                    {
                        ::std::io::_print(format_args!("Connecting to database\n"));
                    };
                    let mut config = Config::default();
                    let AuthDbConfig::Tikv { endpoints, tls_config, timeout_secs } = &DBCONF
                        .authdb else {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!("This is not TiKV"),
                            );
                        };
                    };
                    if *timeout_secs > 0 {
                        config.timeout = Duration::from_secs(*timeout_secs as u64);
                    }
                    if let Some(tls) = tls_config {
                        config.ca_path = Some(
                            PathBuf::from_str(&tls.ca_path).expect("Invalid `ca_path`"),
                        );
                        config.cert_path = Some(
                            PathBuf::from_str(&tls.cert_path)
                                .expect("Invalid `cert_path`"),
                        );
                        config.key_path = Some(
                            PathBuf::from_str(&tls.key_path).expect("Invalid `key_path`"),
                        );
                    }
                    let endpoints = endpoints
                        .iter()
                        .map(|x| x as &str)
                        .collect::<Vec<_>>();
                    Self {
                        raw: RawClient::new_with_config(
                                endpoints.clone(),
                                config.clone(),
                            )
                            .await
                            .expect("Unable to initialize Database connection"),
                        transactional: TransactionClient::new_with_config(
                                endpoints,
                                config,
                            )
                            .await
                            .expect("Unable to initialize Database connection"),
                    }
                }
            }
            impl AuthServer for TikvClient {
                #[allow(
                    elided_named_lifetimes,
                    clippy::async_yields_async,
                    clippy::diverging_sub_expression,
                    clippy::let_unit_value,
                    clippy::needless_arbitrary_self_type,
                    clippy::no_effect_underscore_binding,
                    clippy::shadow_same,
                    clippy::type_complexity,
                    clippy::type_repetition_in_bounds,
                    clippy::used_underscore_binding
                )]
                fn get<'a, 'async_trait>(
                    &'a self,
                    uid: &'a str,
                ) -> ::core::pin::Pin<
                    Box<
                        dyn ::core::future::Future<
                            Output = Returns<Option<String>>,
                        > + ::core::marker::Send + 'async_trait,
                    >,
                >
                where
                    'a: 'async_trait,
                    Self: 'async_trait,
                {
                    Box::pin(async move {
                        if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                            Returns<Option<String>>,
                        > {
                            #[allow(unreachable_code)] return __ret;
                        }
                        let __self = self;
                        let __ret: Returns<Option<String>> = {
                            let out = __self.raw.get(uid.to_owned()).await?;
                            Ok(out.map(|x| String::from_utf8(x).ok()).flatten())
                        };
                        #[allow(unreachable_code)] __ret
                    })
                }
                #[allow(
                    elided_named_lifetimes,
                    clippy::async_yields_async,
                    clippy::diverging_sub_expression,
                    clippy::let_unit_value,
                    clippy::needless_arbitrary_self_type,
                    clippy::no_effect_underscore_binding,
                    clippy::shadow_same,
                    clippy::type_complexity,
                    clippy::type_repetition_in_bounds,
                    clippy::used_underscore_binding
                )]
                fn search<'a, 'async_trait>(
                    &'a self,
                    prefix: String,
                ) -> ::core::pin::Pin<
                    Box<
                        dyn ::core::future::Future<
                            Output = Returns<Vec<Vec<u8>>>,
                        > + ::core::marker::Send + 'async_trait,
                    >,
                >
                where
                    'a: 'async_trait,
                    Self: 'async_trait,
                {
                    Box::pin(async move {
                        if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                            Returns<Vec<Vec<u8>>>,
                        > {
                            #[allow(unreachable_code)] return __ret;
                        }
                        let __self = self;
                        let prefix = prefix;
                        let __ret: Returns<Vec<Vec<u8>>> = {
                            let start = prefix.into_bytes();
                            let mut end = start.clone();
                            end.push(255);
                            let range = BoundRange::from(
                                Key::from(start)..Key::from(end),
                            );
                            Ok(
                                __self
                                    .raw
                                    .scan_keys(range, 100)
                                    .await?
                                    .into_iter()
                                    .map(Vec::from)
                                    .collect::<Vec<_>>(),
                            )
                        };
                        #[allow(unreachable_code)] __ret
                    })
                }
                #[allow(
                    elided_named_lifetimes,
                    clippy::async_yields_async,
                    clippy::diverging_sub_expression,
                    clippy::let_unit_value,
                    clippy::needless_arbitrary_self_type,
                    clippy::no_effect_underscore_binding,
                    clippy::shadow_same,
                    clippy::type_complexity,
                    clippy::type_repetition_in_bounds,
                    clippy::used_underscore_binding
                )]
                fn exists<'a, 'async_trait>(
                    &'a self,
                    uid: &'a str,
                ) -> ::core::pin::Pin<
                    Box<
                        dyn ::core::future::Future<
                            Output = Returns<bool>,
                        > + ::core::marker::Send + 'async_trait,
                    >,
                >
                where
                    'a: 'async_trait,
                    Self: 'async_trait,
                {
                    Box::pin(async move {
                        if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                            Returns<bool>,
                        > {
                            #[allow(unreachable_code)] return __ret;
                        }
                        let __self = self;
                        let __ret: Returns<bool> = {
                            let out = __self.raw.get(uid.to_owned()).await?;
                            Ok(out.is_some())
                        };
                        #[allow(unreachable_code)] __ret
                    })
                }
                #[allow(
                    elided_named_lifetimes,
                    clippy::async_yields_async,
                    clippy::diverging_sub_expression,
                    clippy::let_unit_value,
                    clippy::needless_arbitrary_self_type,
                    clippy::no_effect_underscore_binding,
                    clippy::shadow_same,
                    clippy::type_complexity,
                    clippy::type_repetition_in_bounds,
                    clippy::used_underscore_binding
                )]
                fn update<'a, 'async_trait>(
                    &'a self,
                    uid: String,
                    data: String,
                ) -> ::core::pin::Pin<
                    Box<
                        dyn ::core::future::Future<
                            Output = Returns<()>,
                        > + ::core::marker::Send + 'async_trait,
                    >,
                >
                where
                    'a: 'async_trait,
                    Self: 'async_trait,
                {
                    Box::pin(async move {
                        if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                            Returns<()>,
                        > {
                            #[allow(unreachable_code)] return __ret;
                        }
                        let __self = self;
                        let uid = uid;
                        let data = data;
                        let __ret: Returns<()> = {
                            for attempt in 1..=5 {
                                let mut txn = __self
                                    .transactional
                                    .begin_optimistic()
                                    .await?;
                                txn.put(uid.clone(), data.clone()).await?;
                                match txn.commit().await {
                                    Ok(_) => return Ok(()),
                                    Err(e) => {
                                        if should_retry(&e) {
                                            sleep(
                                                    Duration::from_millis(
                                                        (30 * 2u64.pow(attempt - 1)).min(1000),
                                                    ),
                                                )
                                                .await;
                                            continue;
                                        } else {
                                            break;
                                        }
                                    }
                                }
                            }
                            Err(ServerError::RetryFailed)
                        };
                        #[allow(unreachable_code)] __ret
                    })
                }
                #[allow(
                    elided_named_lifetimes,
                    clippy::async_yields_async,
                    clippy::diverging_sub_expression,
                    clippy::let_unit_value,
                    clippy::needless_arbitrary_self_type,
                    clippy::no_effect_underscore_binding,
                    clippy::shadow_same,
                    clippy::type_complexity,
                    clippy::type_repetition_in_bounds,
                    clippy::used_underscore_binding
                )]
                fn remove<'a, 'async_trait>(
                    &'a self,
                    uid: String,
                ) -> ::core::pin::Pin<
                    Box<
                        dyn ::core::future::Future<
                            Output = Returns<()>,
                        > + ::core::marker::Send + 'async_trait,
                    >,
                >
                where
                    'a: 'async_trait,
                    Self: 'async_trait,
                {
                    Box::pin(async move {
                        if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                            Returns<()>,
                        > {
                            #[allow(unreachable_code)] return __ret;
                        }
                        let __self = self;
                        let uid = uid;
                        let __ret: Returns<()> = {
                            for attempt in 1..=5 {
                                let mut txn = __self
                                    .transactional
                                    .begin_optimistic()
                                    .await?;
                                txn.delete(uid.clone()).await?;
                                match txn.commit().await {
                                    Ok(_) => return Ok(()),
                                    Err(e) => {
                                        if should_retry(&e) {
                                            sleep(
                                                    Duration::from_millis(
                                                        (30 * 2u64.pow(attempt - 1)).min(1000),
                                                    ),
                                                )
                                                .await;
                                            continue;
                                        } else {
                                            break;
                                        }
                                    }
                                }
                            }
                            Err(ServerError::RetryFailed)
                        };
                        #[allow(unreachable_code)] __ret
                    })
                }
            }
            fn should_retry(e: &Error) -> bool {
                #[allow(non_exhaustive_omitted_patterns)]
                match e {
                    Error::KeyError(_)
                    | Error::PessimisticLockError { .. }
                    | Error::RegionError(_)
                    | Error::LeaderNotFound { .. }
                    | Error::UndeterminedError(_) => true,
                    _ => false,
                }
            }
        }
        pub(crate) trait AuthServer {
            #[must_use]
            #[allow(
                elided_named_lifetimes,
                clippy::type_complexity,
                clippy::type_repetition_in_bounds
            )]
            fn get<'a, 'async_trait>(
                &'a self,
                uid: &'a str,
            ) -> ::core::pin::Pin<
                Box<
                    dyn ::core::future::Future<
                        Output = Returns<Option<String>>,
                    > + ::core::marker::Send + 'async_trait,
                >,
            >
            where
                'a: 'async_trait,
                Self: 'async_trait;
            #[must_use]
            #[allow(
                elided_named_lifetimes,
                clippy::type_complexity,
                clippy::type_repetition_in_bounds
            )]
            fn search<'a, 'async_trait>(
                &'a self,
                prefix: String,
            ) -> ::core::pin::Pin<
                Box<
                    dyn ::core::future::Future<
                        Output = Returns<Vec<Vec<u8>>>,
                    > + ::core::marker::Send + 'async_trait,
                >,
            >
            where
                'a: 'async_trait,
                Self: 'async_trait;
            #[must_use]
            #[allow(
                elided_named_lifetimes,
                clippy::type_complexity,
                clippy::type_repetition_in_bounds
            )]
            fn exists<'a, 'async_trait>(
                &'a self,
                uid: &'a str,
            ) -> ::core::pin::Pin<
                Box<
                    dyn ::core::future::Future<
                        Output = Returns<bool>,
                    > + ::core::marker::Send + 'async_trait,
                >,
            >
            where
                'a: 'async_trait,
                Self: 'async_trait;
            #[must_use]
            #[allow(
                elided_named_lifetimes,
                clippy::type_complexity,
                clippy::type_repetition_in_bounds
            )]
            fn update<'a, 'async_trait>(
                &'a self,
                uid: String,
                data: String,
            ) -> ::core::pin::Pin<
                Box<
                    dyn ::core::future::Future<
                        Output = Returns<()>,
                    > + ::core::marker::Send + 'async_trait,
                >,
            >
            where
                'a: 'async_trait,
                Self: 'async_trait;
            #[must_use]
            #[allow(
                elided_named_lifetimes,
                clippy::type_complexity,
                clippy::type_repetition_in_bounds
            )]
            fn remove<'a, 'async_trait>(
                &'a self,
                uid: String,
            ) -> ::core::pin::Pin<
                Box<
                    dyn ::core::future::Future<
                        Output = Returns<()>,
                    > + ::core::marker::Send + 'async_trait,
                >,
            >
            where
                'a: 'async_trait,
                Self: 'async_trait;
        }
    }
    pub mod quickdata {}
    pub static INTEGRITY_KEY: &'static [u8; 64] = b"|\xa0lT\xe0\x8d\x08\xc1\x8f\xa8\xdb\xb2\x8b]\xd4x*$#\xd3\x8f\xae\xcdC\x88C\x8c\xee\x93\x11\xc3\x87Pg\x0b\x01\x8a^\xc1\xfa$\x7f\x1b\xbdt\x93\x0c\xf9s%\x81(\xee\xf6\xb2\xf7\xb4\xe2sZ-\xc40\xa7";
    pub static AGENT: LazyLock<HashingAgent> = LazyLock::new(|| HashingAgent::new());
    const TOKEN_ID_LENGTH: usize = 12;
    #[allow(dead_code)]
    pub(crate) struct AuthSessionManager {
        sessions: Cache<Box<str>, Arc<Box<str>>>,
        pub accounts: Box<dyn AuthServer + Send + Sync>,
        agent: &'static HashingAgent,
    }
    pub enum AccountCreateOutcome {
        UsernameExists,
        WeakPassword,
        InternalServerError,
        Successful,
        SuccessfulOut(String),
    }
    pub enum AccountCheckOutcome {
        NotFound,
        TooManyRequests,
        InvalidPassword,
        Some(String),
    }
    pub type AccountOrToken = (Box<str>, Box<str>);
    impl AuthSessionManager {
        pub async fn create() -> Self {
            let sessions = Cache::builder()
                .time_to_live(Duration::from_days(30))
                .build();
            Self {
                sessions,
                accounts: Box::new(TikvClient::new().await),
                agent: &*AGENT,
            }
        }
    }
    impl AuthSessionManager {
        pub async fn can_register(&self) -> bool {
            let Authentication::Account { registration_allowed, .. } = CONFIG
                .authentication else {
                return false;
            };
            registration_allowed
        }
        /// THIS ENDPOINT HAS ABSOLUTELY NO PROTECTION
        /// DEVS SHOULD USE `AuthSessionManager::can_register` first
        pub async fn register(
            &self,
            user: &str,
            pass: &str,
        ) -> Returns<AccountCreateOutcome> {
            if self.accounts.exists(user).await? {
                return Ok(AccountCreateOutcome::UsernameExists);
            }
            if !is_strong_password(pass).await {
                return Ok(AccountCreateOutcome::WeakPassword);
            }
            let Some(pwd_hash) = self.agent.gen_hash(pass).await else {
                return Ok(AccountCreateOutcome::InternalServerError);
            };
            self.accounts.update(user.to_owned(), pwd_hash).await?;
            Ok(AccountCreateOutcome::Successful)
        }
        pub async fn add_token(&self) -> Returns<AccountCreateOutcome> {
            let (key, (user, hash)) = gen_auth_token()?;
            self.accounts.update(user.into_string(), hash.into_string()).await?;
            Ok(AccountCreateOutcome::SuccessfulOut(key))
        }
        pub async fn is_valid_token(&self, token: &str) -> Returns<AccountCheckOutcome> {
            let Some((tok_id, token)) = token.split_once(".") else {
                return Ok(AccountCheckOutcome::NotFound);
            };
            self.is_valid_account(tok_id, token).await
        }
        pub async fn is_valid_account(
            &self,
            userid: &str,
            pass: &str,
        ) -> Returns<AccountCheckOutcome> {
            let Some(hash) = self.accounts.get(userid).await? else {
                return Ok(AccountCheckOutcome::NotFound);
            };
            let Some(x) = self.agent.verify_pass(pass, &hash).await else {
                return Ok(AccountCheckOutcome::TooManyRequests);
            };
            if !x {
                return Ok(AccountCheckOutcome::InvalidPassword);
            }
            if let Some(session) = self.sessions.get(userid).await {
                let sess_cloned = ::alloc::__export::must_use({
                    ::alloc::fmt::format(format_args!("{0}.{1}", userid, session))
                });
                return Ok(AccountCheckOutcome::Some(sess_cloned));
            }
            let sess = gen_session_token()?;
            let sess_cloned = ::alloc::__export::must_use({
                ::alloc::fmt::format(format_args!("{0}.{1}", userid, sess))
            });
            self.sessions
                .insert(
                    userid.to_owned().into_boxed_str(),
                    Arc::new(sess.into_boxed_str()),
                )
                .await;
            Ok(AccountCheckOutcome::Some(sess_cloned))
        }
        pub async fn verify_session(&self, token: &str) -> bool {
            let Some((userid, session)) = token.split_once(".") else {
                return false;
            };
            self.sessions
                .get(userid)
                .await
                .map(|x| session == (&x as &str))
                .is_some_and(|x| x)
        }
    }
    pub fn now() -> u64 {
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
    }
    pub async fn is_strong_password(password: &str) -> bool {
        if password.len() < 8 {
            return false;
        }
        let mut uppercase = false;
        let mut lowercase = false;
        let mut digit = false;
        let mut special = false;
        password
            .chars()
            .any(|x| {
                if x.is_ascii_digit() {
                    digit = true;
                }
                if x.is_ascii_uppercase() {
                    uppercase = true;
                }
                if x.is_ascii_lowercase() {
                    lowercase = true;
                }
                if !x.is_ascii_alphanumeric() {
                    special = true;
                }
                digit && uppercase && lowercase && special
            })
    }
    pub const VALUES: [char; 64] = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
        'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F',
        'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V',
        'W', 'X', 'Y', 'Z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/',
    ];
    pub type Hashed = Box<str>;
    pub fn gen_auth_token() -> Returns<(String, (Box<str>, Hashed))> {
        let mut rng = rand::rng();
        let token = VALUES.choose_multiple(&mut rng, 128).collect::<String>();
        let token_key = VALUES
            .choose_multiple(&mut rng, TOKEN_ID_LENGTH)
            .collect::<Box<str>>();
        let hashed = hash(&token, BCRYPT_COST)?.into_boxed_str();
        let token_to_output = ::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}.{1}", token_key, token))
        });
        Ok((token_to_output, (token_key, hashed)))
    }
    pub async fn gen_session_token_async() -> Returns<String> {
        spawn_blocking(gen_session_token).await?
    }
    pub fn gen_session_token() -> Returns<String> {
        let mut rng = rand::rng();
        let mut token = ::alloc::vec::from_elem(0u8, 96);
        rng.fill(&mut token as &mut [u8]);
        let token = general_purpose::URL_SAFE_NO_PAD.encode(&token);
        Ok(token)
    }
}
pub(crate) mod structs {
    use std::collections::HashSet;
    use serde::{Deserialize, Serialize};
    use serde_json::{from_str, to_string_pretty};
    use tokio::fs;
    use crate::structs::error::Returns;
    pub mod db {
        use std::fs;
        use serde::{Deserialize, Serialize};
        use serde_json::from_str;
        pub struct DatabaseConfig {
            pub authdb: AuthDbConfig,
            pub redis: RedisConfig,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for DatabaseConfig {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "DatabaseConfig",
                    "authdb",
                    &self.authdb,
                    "redis",
                    &&self.redis,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for DatabaseConfig {
            #[inline]
            fn clone(&self) -> DatabaseConfig {
                DatabaseConfig {
                    authdb: ::core::clone::Clone::clone(&self.authdb),
                    redis: ::core::clone::Clone::clone(&self.redis),
                }
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for DatabaseConfig {
            #[inline]
            fn default() -> DatabaseConfig {
                DatabaseConfig {
                    authdb: ::core::default::Default::default(),
                    redis: ::core::default::Default::default(),
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
            impl _serde::Serialize for DatabaseConfig {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private228::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "DatabaseConfig",
                        false as usize + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "authdb",
                        &self.authdb,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "redis",
                        &self.redis,
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
            impl<'de> _serde::Deserialize<'de> for DatabaseConfig {
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
                                "authdb" => _serde::__private228::Ok(__Field::__field0),
                                "redis" => _serde::__private228::Ok(__Field::__field1),
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
                                b"authdb" => _serde::__private228::Ok(__Field::__field0),
                                b"redis" => _serde::__private228::Ok(__Field::__field1),
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
                        marker: _serde::__private228::PhantomData<DatabaseConfig>,
                        lifetime: _serde::__private228::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = DatabaseConfig;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private228::Formatter,
                        ) -> _serde::__private228::fmt::Result {
                            _serde::__private228::Formatter::write_str(
                                __formatter,
                                "struct DatabaseConfig",
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
                                AuthDbConfig,
                            >(&mut __seq)? {
                                _serde::__private228::Some(__value) => __value,
                                _serde::__private228::None => {
                                    return _serde::__private228::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct DatabaseConfig with 2 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                RedisConfig,
                            >(&mut __seq)? {
                                _serde::__private228::Some(__value) => __value,
                                _serde::__private228::None => {
                                    return _serde::__private228::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct DatabaseConfig with 2 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private228::Ok(DatabaseConfig {
                                authdb: __field0,
                                redis: __field1,
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
                                AuthDbConfig,
                            > = _serde::__private228::None;
                            let mut __field1: _serde::__private228::Option<
                                RedisConfig,
                            > = _serde::__private228::None;
                            while let _serde::__private228::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private228::Option::is_some(&__field0) {
                                            return _serde::__private228::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("authdb"),
                                            );
                                        }
                                        __field0 = _serde::__private228::Some(
                                            _serde::de::MapAccess::next_value::<
                                                AuthDbConfig,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private228::Option::is_some(&__field1) {
                                            return _serde::__private228::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("redis"),
                                            );
                                        }
                                        __field1 = _serde::__private228::Some(
                                            _serde::de::MapAccess::next_value::<
                                                RedisConfig,
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
                                    _serde::__private228::de::missing_field("authdb")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private228::Some(__field1) => __field1,
                                _serde::__private228::None => {
                                    _serde::__private228::de::missing_field("redis")?
                                }
                            };
                            _serde::__private228::Ok(DatabaseConfig {
                                authdb: __field0,
                                redis: __field1,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &["authdb", "redis"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "DatabaseConfig",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private228::PhantomData::<DatabaseConfig>,
                            lifetime: _serde::__private228::PhantomData,
                        },
                    )
                }
            }
        };
        #[serde(tag = "db")]
        pub enum AuthDbConfig {
            Mongodb { url: Box<str> },
            Tikv {
                endpoints: Box<[Box<str>]>,
                tls_config: Option<TlsConfig>,
                timeout_secs: u8,
            },
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for AuthDbConfig {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    AuthDbConfig::Mongodb { url: __self_0 } => {
                        ::core::fmt::Formatter::debug_struct_field1_finish(
                            f,
                            "Mongodb",
                            "url",
                            &__self_0,
                        )
                    }
                    AuthDbConfig::Tikv {
                        endpoints: __self_0,
                        tls_config: __self_1,
                        timeout_secs: __self_2,
                    } => {
                        ::core::fmt::Formatter::debug_struct_field3_finish(
                            f,
                            "Tikv",
                            "endpoints",
                            __self_0,
                            "tls_config",
                            __self_1,
                            "timeout_secs",
                            &__self_2,
                        )
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for AuthDbConfig {
            #[inline]
            fn clone(&self) -> AuthDbConfig {
                match self {
                    AuthDbConfig::Mongodb { url: __self_0 } => {
                        AuthDbConfig::Mongodb {
                            url: ::core::clone::Clone::clone(__self_0),
                        }
                    }
                    AuthDbConfig::Tikv {
                        endpoints: __self_0,
                        tls_config: __self_1,
                        timeout_secs: __self_2,
                    } => {
                        AuthDbConfig::Tikv {
                            endpoints: ::core::clone::Clone::clone(__self_0),
                            tls_config: ::core::clone::Clone::clone(__self_1),
                            timeout_secs: ::core::clone::Clone::clone(__self_2),
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
            impl _serde::Serialize for AuthDbConfig {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private228::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    match *self {
                        AuthDbConfig::Mongodb { ref url } => {
                            let mut __serde_state = _serde::Serializer::serialize_struct(
                                __serializer,
                                "AuthDbConfig",
                                0 + 1 + 1,
                            )?;
                            _serde::ser::SerializeStruct::serialize_field(
                                &mut __serde_state,
                                "db",
                                "Mongodb",
                            )?;
                            _serde::ser::SerializeStruct::serialize_field(
                                &mut __serde_state,
                                "url",
                                url,
                            )?;
                            _serde::ser::SerializeStruct::end(__serde_state)
                        }
                        AuthDbConfig::Tikv {
                            ref endpoints,
                            ref tls_config,
                            ref timeout_secs,
                        } => {
                            let mut __serde_state = _serde::Serializer::serialize_struct(
                                __serializer,
                                "AuthDbConfig",
                                0 + 1 + 1 + 1 + 1,
                            )?;
                            _serde::ser::SerializeStruct::serialize_field(
                                &mut __serde_state,
                                "db",
                                "Tikv",
                            )?;
                            _serde::ser::SerializeStruct::serialize_field(
                                &mut __serde_state,
                                "endpoints",
                                endpoints,
                            )?;
                            _serde::ser::SerializeStruct::serialize_field(
                                &mut __serde_state,
                                "tls_config",
                                tls_config,
                            )?;
                            _serde::ser::SerializeStruct::serialize_field(
                                &mut __serde_state,
                                "timeout_secs",
                                timeout_secs,
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
            impl<'de> _serde::Deserialize<'de> for AuthDbConfig {
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
                                _ => {
                                    _serde::__private228::Err(
                                        _serde::de::Error::invalid_value(
                                            _serde::de::Unexpected::Unsigned(__value),
                                            &"variant index 0 <= i < 2",
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
                                "Mongodb" => _serde::__private228::Ok(__Field::__field0),
                                "Tikv" => _serde::__private228::Ok(__Field::__field1),
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
                                b"Mongodb" => _serde::__private228::Ok(__Field::__field0),
                                b"Tikv" => _serde::__private228::Ok(__Field::__field1),
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
                    const VARIANTS: &'static [&'static str] = &["Mongodb", "Tikv"];
                    let (__tag, __content) = _serde::Deserializer::deserialize_any(
                        __deserializer,
                        _serde::__private228::de::TaggedContentVisitor::<
                            __Field,
                        >::new("db", "internally tagged enum AuthDbConfig"),
                    )?;
                    let __deserializer = _serde::__private228::de::ContentDeserializer::<
                        __D::Error,
                    >::new(__content);
                    match __tag {
                        __Field::__field0 => {
                            #[allow(non_camel_case_types)]
                            #[doc(hidden)]
                            enum __Field {
                                __field0,
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
                                        "url" => _serde::__private228::Ok(__Field::__field0),
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
                                        b"url" => _serde::__private228::Ok(__Field::__field0),
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
                                marker: _serde::__private228::PhantomData<AuthDbConfig>,
                                lifetime: _serde::__private228::PhantomData<&'de ()>,
                            }
                            #[automatically_derived]
                            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                type Value = AuthDbConfig;
                                fn expecting(
                                    &self,
                                    __formatter: &mut _serde::__private228::Formatter,
                                ) -> _serde::__private228::fmt::Result {
                                    _serde::__private228::Formatter::write_str(
                                        __formatter,
                                        "struct variant AuthDbConfig::Mongodb",
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
                                                    &"struct variant AuthDbConfig::Mongodb with 1 element",
                                                ),
                                            );
                                        }
                                    };
                                    _serde::__private228::Ok(AuthDbConfig::Mongodb {
                                        url: __field0,
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
                                    while let _serde::__private228::Some(__key) = _serde::de::MapAccess::next_key::<
                                        __Field,
                                    >(&mut __map)? {
                                        match __key {
                                            __Field::__field0 => {
                                                if _serde::__private228::Option::is_some(&__field0) {
                                                    return _serde::__private228::Err(
                                                        <__A::Error as _serde::de::Error>::duplicate_field("url"),
                                                    );
                                                }
                                                __field0 = _serde::__private228::Some(
                                                    _serde::de::MapAccess::next_value::<Box<str>>(&mut __map)?,
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
                                            _serde::__private228::de::missing_field("url")?
                                        }
                                    };
                                    _serde::__private228::Ok(AuthDbConfig::Mongodb {
                                        url: __field0,
                                    })
                                }
                            }
                            #[doc(hidden)]
                            const FIELDS: &'static [&'static str] = &["url"];
                            _serde::Deserializer::deserialize_any(
                                __deserializer,
                                __Visitor {
                                    marker: _serde::__private228::PhantomData::<AuthDbConfig>,
                                    lifetime: _serde::__private228::PhantomData,
                                },
                            )
                        }
                        __Field::__field1 => {
                            #[allow(non_camel_case_types)]
                            #[doc(hidden)]
                            enum __Field {
                                __field0,
                                __field1,
                                __field2,
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
                                        "endpoints" => _serde::__private228::Ok(__Field::__field0),
                                        "tls_config" => _serde::__private228::Ok(__Field::__field1),
                                        "timeout_secs" => {
                                            _serde::__private228::Ok(__Field::__field2)
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
                                        b"endpoints" => _serde::__private228::Ok(__Field::__field0),
                                        b"tls_config" => _serde::__private228::Ok(__Field::__field1),
                                        b"timeout_secs" => {
                                            _serde::__private228::Ok(__Field::__field2)
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
                                marker: _serde::__private228::PhantomData<AuthDbConfig>,
                                lifetime: _serde::__private228::PhantomData<&'de ()>,
                            }
                            #[automatically_derived]
                            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                type Value = AuthDbConfig;
                                fn expecting(
                                    &self,
                                    __formatter: &mut _serde::__private228::Formatter,
                                ) -> _serde::__private228::fmt::Result {
                                    _serde::__private228::Formatter::write_str(
                                        __formatter,
                                        "struct variant AuthDbConfig::Tikv",
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
                                        Box<[Box<str>]>,
                                    >(&mut __seq)? {
                                        _serde::__private228::Some(__value) => __value,
                                        _serde::__private228::None => {
                                            return _serde::__private228::Err(
                                                _serde::de::Error::invalid_length(
                                                    0usize,
                                                    &"struct variant AuthDbConfig::Tikv with 3 elements",
                                                ),
                                            );
                                        }
                                    };
                                    let __field1 = match _serde::de::SeqAccess::next_element::<
                                        Option<TlsConfig>,
                                    >(&mut __seq)? {
                                        _serde::__private228::Some(__value) => __value,
                                        _serde::__private228::None => {
                                            return _serde::__private228::Err(
                                                _serde::de::Error::invalid_length(
                                                    1usize,
                                                    &"struct variant AuthDbConfig::Tikv with 3 elements",
                                                ),
                                            );
                                        }
                                    };
                                    let __field2 = match _serde::de::SeqAccess::next_element::<
                                        u8,
                                    >(&mut __seq)? {
                                        _serde::__private228::Some(__value) => __value,
                                        _serde::__private228::None => {
                                            return _serde::__private228::Err(
                                                _serde::de::Error::invalid_length(
                                                    2usize,
                                                    &"struct variant AuthDbConfig::Tikv with 3 elements",
                                                ),
                                            );
                                        }
                                    };
                                    _serde::__private228::Ok(AuthDbConfig::Tikv {
                                        endpoints: __field0,
                                        tls_config: __field1,
                                        timeout_secs: __field2,
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
                                        Box<[Box<str>]>,
                                    > = _serde::__private228::None;
                                    let mut __field1: _serde::__private228::Option<
                                        Option<TlsConfig>,
                                    > = _serde::__private228::None;
                                    let mut __field2: _serde::__private228::Option<u8> = _serde::__private228::None;
                                    while let _serde::__private228::Some(__key) = _serde::de::MapAccess::next_key::<
                                        __Field,
                                    >(&mut __map)? {
                                        match __key {
                                            __Field::__field0 => {
                                                if _serde::__private228::Option::is_some(&__field0) {
                                                    return _serde::__private228::Err(
                                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                                            "endpoints",
                                                        ),
                                                    );
                                                }
                                                __field0 = _serde::__private228::Some(
                                                    _serde::de::MapAccess::next_value::<
                                                        Box<[Box<str>]>,
                                                    >(&mut __map)?,
                                                );
                                            }
                                            __Field::__field1 => {
                                                if _serde::__private228::Option::is_some(&__field1) {
                                                    return _serde::__private228::Err(
                                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                                            "tls_config",
                                                        ),
                                                    );
                                                }
                                                __field1 = _serde::__private228::Some(
                                                    _serde::de::MapAccess::next_value::<
                                                        Option<TlsConfig>,
                                                    >(&mut __map)?,
                                                );
                                            }
                                            __Field::__field2 => {
                                                if _serde::__private228::Option::is_some(&__field2) {
                                                    return _serde::__private228::Err(
                                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                                            "timeout_secs",
                                                        ),
                                                    );
                                                }
                                                __field2 = _serde::__private228::Some(
                                                    _serde::de::MapAccess::next_value::<u8>(&mut __map)?,
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
                                            _serde::__private228::de::missing_field("endpoints")?
                                        }
                                    };
                                    let __field1 = match __field1 {
                                        _serde::__private228::Some(__field1) => __field1,
                                        _serde::__private228::None => {
                                            _serde::__private228::de::missing_field("tls_config")?
                                        }
                                    };
                                    let __field2 = match __field2 {
                                        _serde::__private228::Some(__field2) => __field2,
                                        _serde::__private228::None => {
                                            _serde::__private228::de::missing_field("timeout_secs")?
                                        }
                                    };
                                    _serde::__private228::Ok(AuthDbConfig::Tikv {
                                        endpoints: __field0,
                                        tls_config: __field1,
                                        timeout_secs: __field2,
                                    })
                                }
                            }
                            #[doc(hidden)]
                            const FIELDS: &'static [&'static str] = &[
                                "endpoints",
                                "tls_config",
                                "timeout_secs",
                            ];
                            _serde::Deserializer::deserialize_any(
                                __deserializer,
                                __Visitor {
                                    marker: _serde::__private228::PhantomData::<AuthDbConfig>,
                                    lifetime: _serde::__private228::PhantomData,
                                },
                            )
                        }
                    }
                }
            }
        };
        impl Default for AuthDbConfig {
            fn default() -> Self {
                Self::Mongodb {
                    url: String::new().into_boxed_str(),
                }
            }
        }
        pub struct TlsConfig {
            pub ca_path: Box<str>,
            pub cert_path: Box<str>,
            pub key_path: Box<str>,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for TlsConfig {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "TlsConfig",
                    "ca_path",
                    &self.ca_path,
                    "cert_path",
                    &self.cert_path,
                    "key_path",
                    &&self.key_path,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for TlsConfig {
            #[inline]
            fn clone(&self) -> TlsConfig {
                TlsConfig {
                    ca_path: ::core::clone::Clone::clone(&self.ca_path),
                    cert_path: ::core::clone::Clone::clone(&self.cert_path),
                    key_path: ::core::clone::Clone::clone(&self.key_path),
                }
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for TlsConfig {
            #[inline]
            fn default() -> TlsConfig {
                TlsConfig {
                    ca_path: ::core::default::Default::default(),
                    cert_path: ::core::default::Default::default(),
                    key_path: ::core::default::Default::default(),
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
            impl _serde::Serialize for TlsConfig {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private228::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "TlsConfig",
                        false as usize + 1 + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "ca_path",
                        &self.ca_path,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "cert_path",
                        &self.cert_path,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "key_path",
                        &self.key_path,
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
            impl<'de> _serde::Deserialize<'de> for TlsConfig {
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
                                "ca_path" => _serde::__private228::Ok(__Field::__field0),
                                "cert_path" => _serde::__private228::Ok(__Field::__field1),
                                "key_path" => _serde::__private228::Ok(__Field::__field2),
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
                                b"ca_path" => _serde::__private228::Ok(__Field::__field0),
                                b"cert_path" => _serde::__private228::Ok(__Field::__field1),
                                b"key_path" => _serde::__private228::Ok(__Field::__field2),
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
                        marker: _serde::__private228::PhantomData<TlsConfig>,
                        lifetime: _serde::__private228::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = TlsConfig;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private228::Formatter,
                        ) -> _serde::__private228::fmt::Result {
                            _serde::__private228::Formatter::write_str(
                                __formatter,
                                "struct TlsConfig",
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
                                            &"struct TlsConfig with 3 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                Box<str>,
                            >(&mut __seq)? {
                                _serde::__private228::Some(__value) => __value,
                                _serde::__private228::None => {
                                    return _serde::__private228::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct TlsConfig with 3 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match _serde::de::SeqAccess::next_element::<
                                Box<str>,
                            >(&mut __seq)? {
                                _serde::__private228::Some(__value) => __value,
                                _serde::__private228::None => {
                                    return _serde::__private228::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct TlsConfig with 3 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private228::Ok(TlsConfig {
                                ca_path: __field0,
                                cert_path: __field1,
                                key_path: __field2,
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
                            let mut __field1: _serde::__private228::Option<Box<str>> = _serde::__private228::None;
                            let mut __field2: _serde::__private228::Option<Box<str>> = _serde::__private228::None;
                            while let _serde::__private228::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private228::Option::is_some(&__field0) {
                                            return _serde::__private228::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "ca_path",
                                                ),
                                            );
                                        }
                                        __field0 = _serde::__private228::Some(
                                            _serde::de::MapAccess::next_value::<Box<str>>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private228::Option::is_some(&__field1) {
                                            return _serde::__private228::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "cert_path",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private228::Some(
                                            _serde::de::MapAccess::next_value::<Box<str>>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private228::Option::is_some(&__field2) {
                                            return _serde::__private228::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "key_path",
                                                ),
                                            );
                                        }
                                        __field2 = _serde::__private228::Some(
                                            _serde::de::MapAccess::next_value::<Box<str>>(&mut __map)?,
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
                                    _serde::__private228::de::missing_field("ca_path")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private228::Some(__field1) => __field1,
                                _serde::__private228::None => {
                                    _serde::__private228::de::missing_field("cert_path")?
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private228::Some(__field2) => __field2,
                                _serde::__private228::None => {
                                    _serde::__private228::de::missing_field("key_path")?
                                }
                            };
                            _serde::__private228::Ok(TlsConfig {
                                ca_path: __field0,
                                cert_path: __field1,
                                key_path: __field2,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &[
                        "ca_path",
                        "cert_path",
                        "key_path",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "TlsConfig",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private228::PhantomData::<TlsConfig>,
                            lifetime: _serde::__private228::PhantomData,
                        },
                    )
                }
            }
        };
        pub struct RedisConfig {}
        #[automatically_derived]
        impl ::core::fmt::Debug for RedisConfig {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(f, "RedisConfig")
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for RedisConfig {
            #[inline]
            fn clone(&self) -> RedisConfig {
                RedisConfig {}
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for RedisConfig {
            #[inline]
            fn default() -> RedisConfig {
                RedisConfig {}
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
            impl _serde::Serialize for RedisConfig {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private228::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "RedisConfig",
                        false as usize,
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
            impl<'de> _serde::Deserialize<'de> for RedisConfig {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private228::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
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
                        marker: _serde::__private228::PhantomData<RedisConfig>,
                        lifetime: _serde::__private228::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = RedisConfig;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private228::Formatter,
                        ) -> _serde::__private228::fmt::Result {
                            _serde::__private228::Formatter::write_str(
                                __formatter,
                                "struct RedisConfig",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            _: __A,
                        ) -> _serde::__private228::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            _serde::__private228::Ok(RedisConfig {})
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private228::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            while let _serde::__private228::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            _serde::__private228::Ok(RedisConfig {})
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &[];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "RedisConfig",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private228::PhantomData::<RedisConfig>,
                            lifetime: _serde::__private228::PhantomData,
                        },
                    )
                }
            }
        };
        impl DatabaseConfig {
            /// This is a panicking method as it should immediately crash the server
            pub fn get() -> Self {
                let data = fs::read_to_string("./database_conf.json")
                    .expect("Unable to get Database Config");
                from_str(&data)
                    .expect(
                        "Unable to parse your JSON Database Config. Make sure it is correct",
                    )
            }
        }
    }
    pub mod error {
        use actix_web::http::StatusCode;
        use base64::DecodeError;
        use thiserror::Error;
        use bcrypt::BcryptError;
        use serde_json::Error as SerdeError;
        use std::io::Error as StdError;
        use tikv_client::Error as TikvError;
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
            #[error(transparent)]
            Tikv(#[from] TikvError),
            #[error("Failed to convert OS String to String")]
            StringConvertErr,
            #[error("Tried to retry many times but failed")]
            RetryFailed,
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
                    ServerError::Tikv(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Tikv",
                            &__self_0,
                        )
                    }
                    ServerError::StringConvertErr => {
                        ::core::fmt::Formatter::write_str(f, "StringConvertErr")
                    }
                    ServerError::RetryFailed => {
                        ::core::fmt::Formatter::write_str(f, "RetryFailed")
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
                    ServerError::Tikv { 0: transparent } => {
                        ::thiserror::__private17::Error::source(
                            transparent.as_dyn_error(),
                        )
                    }
                    ServerError::StringConvertErr { .. } => ::core::option::Option::None,
                    ServerError::RetryFailed { .. } => ::core::option::Option::None,
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
                    ServerError::Tikv(_0) => ::core::fmt::Display::fmt(_0, __formatter),
                    ServerError::StringConvertErr {} => {
                        __formatter.write_str("Failed to convert OS String to String")
                    }
                    ServerError::RetryFailed {} => {
                        __formatter.write_str("Tried to retry many times but failed")
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
        impl ::core::convert::From<TikvError> for ServerError {
            fn from(source: TikvError) -> Self {
                ServerError::Tikv { 0: source }
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
    pub static BCRYPT_COST: u32 = 14;
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
        pub msgs: usize,
        pub cvmodels: HashSet<Box<str>>,
        pub txtmodels: HashSet<Box<str>>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for OllamaConfiguration {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field5_finish(
                f,
                "OllamaConfiguration",
                "host",
                &self.host,
                "port",
                &self.port,
                "msgs",
                &self.msgs,
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
                msgs: ::core::clone::Clone::clone(&self.msgs),
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
                    false as usize + 1 + 1 + 1 + 1 + 1,
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
                    "msgs",
                    &self.msgs,
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
                    __field4,
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
                            4u64 => _serde::__private228::Ok(__Field::__field4),
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
                            "msgs" => _serde::__private228::Ok(__Field::__field2),
                            "cvmodels" => _serde::__private228::Ok(__Field::__field3),
                            "txtmodels" => _serde::__private228::Ok(__Field::__field4),
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
                            b"msgs" => _serde::__private228::Ok(__Field::__field2),
                            b"cvmodels" => _serde::__private228::Ok(__Field::__field3),
                            b"txtmodels" => _serde::__private228::Ok(__Field::__field4),
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
                                        &"struct OllamaConfiguration with 5 elements",
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
                                        &"struct OllamaConfiguration with 5 elements",
                                    ),
                                );
                            }
                        };
                        let __field2 = match _serde::de::SeqAccess::next_element::<
                            usize,
                        >(&mut __seq)? {
                            _serde::__private228::Some(__value) => __value,
                            _serde::__private228::None => {
                                return _serde::__private228::Err(
                                    _serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct OllamaConfiguration with 5 elements",
                                    ),
                                );
                            }
                        };
                        let __field3 = match _serde::de::SeqAccess::next_element::<
                            HashSet<Box<str>>,
                        >(&mut __seq)? {
                            _serde::__private228::Some(__value) => __value,
                            _serde::__private228::None => {
                                return _serde::__private228::Err(
                                    _serde::de::Error::invalid_length(
                                        3usize,
                                        &"struct OllamaConfiguration with 5 elements",
                                    ),
                                );
                            }
                        };
                        let __field4 = match _serde::de::SeqAccess::next_element::<
                            HashSet<Box<str>>,
                        >(&mut __seq)? {
                            _serde::__private228::Some(__value) => __value,
                            _serde::__private228::None => {
                                return _serde::__private228::Err(
                                    _serde::de::Error::invalid_length(
                                        4usize,
                                        &"struct OllamaConfiguration with 5 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private228::Ok(OllamaConfiguration {
                            host: __field0,
                            port: __field1,
                            msgs: __field2,
                            cvmodels: __field3,
                            txtmodels: __field4,
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
                        let mut __field2: _serde::__private228::Option<usize> = _serde::__private228::None;
                        let mut __field3: _serde::__private228::Option<
                            HashSet<Box<str>>,
                        > = _serde::__private228::None;
                        let mut __field4: _serde::__private228::Option<
                            HashSet<Box<str>>,
                        > = _serde::__private228::None;
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
                                            <__A::Error as _serde::de::Error>::duplicate_field("msgs"),
                                        );
                                    }
                                    __field2 = _serde::__private228::Some(
                                        _serde::de::MapAccess::next_value::<usize>(&mut __map)?,
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::__private228::Option::is_some(&__field3) {
                                        return _serde::__private228::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "cvmodels",
                                            ),
                                        );
                                    }
                                    __field3 = _serde::__private228::Some(
                                        _serde::de::MapAccess::next_value::<
                                            HashSet<Box<str>>,
                                        >(&mut __map)?,
                                    );
                                }
                                __Field::__field4 => {
                                    if _serde::__private228::Option::is_some(&__field4) {
                                        return _serde::__private228::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "txtmodels",
                                            ),
                                        );
                                    }
                                    __field4 = _serde::__private228::Some(
                                        _serde::de::MapAccess::next_value::<
                                            HashSet<Box<str>>,
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
                                _serde::__private228::de::missing_field("msgs")?
                            }
                        };
                        let __field3 = match __field3 {
                            _serde::__private228::Some(__field3) => __field3,
                            _serde::__private228::None => {
                                _serde::__private228::de::missing_field("cvmodels")?
                            }
                        };
                        let __field4 = match __field4 {
                            _serde::__private228::Some(__field4) => __field4,
                            _serde::__private228::None => {
                                _serde::__private228::de::missing_field("txtmodels")?
                            }
                        };
                        _serde::__private228::Ok(OllamaConfiguration {
                            host: __field0,
                            port: __field1,
                            msgs: __field2,
                            cvmodels: __field3,
                            txtmodels: __field4,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &[
                    "host",
                    "port",
                    "msgs",
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
                msgs: ::core::default::Default::default(),
                cvmodels: ::core::default::Default::default(),
                txtmodels: ::core::default::Default::default(),
            }
        }
    }
    #[serde(tag = "kind")]
    pub enum Authentication {
        OpenToAll,
        TokenBased,
        Account { registration_allowed: bool },
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
                Authentication::Account { registration_allowed: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "Account",
                        "registration_allowed",
                        &__self_0,
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
                Authentication::Account { registration_allowed: __self_0 } => {
                    Authentication::Account {
                        registration_allowed: ::core::clone::Clone::clone(__self_0),
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
                    Authentication::Account { ref registration_allowed } => {
                        let mut __serde_state = _serde::Serializer::serialize_struct(
                            __serializer,
                            "Authentication",
                            0 + 1 + 1,
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
                                                &"struct variant Authentication::Account with 1 element",
                                            ),
                                        );
                                    }
                                };
                                _serde::__private228::Ok(Authentication::Account {
                                    registration_allowed: __field0,
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
                                _serde::__private228::Ok(Authentication::Account {
                                    registration_allowed: __field0,
                                })
                            }
                        }
                        #[doc(hidden)]
                        const FIELDS: &'static [&'static str] = &[
                            "registration_allowed",
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
                        Authentication::Account { registration_allowed: __self_0 },
                        Authentication::Account { registration_allowed: __arg1_0 },
                    ) => __self_0 == __arg1_0,
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
use chalk_rs::Chalk;
fn main() {
    panic::set_hook(
        Box::new(|x| {
            let mut chalk = Chalk::new();
            if let Some(x) = x.payload_as_str() {
                {
                    ::std::io::_print(format_args!("\n"));
                };
                chalk.red().println(&"----------------");
                chalk.red().underline().println(&"An Critical Error has occured");
                chalk.reset_style();
                chalk.yellow().println(&"The server was unable to achnowledge");
                chalk.yellow().println(&"and handle the error promptly without");
                chalk.yellow().println(&"resorting to server shutdown");
                {
                    ::std::io::_print(format_args!("\n"));
                };
                {
                    ::std::io::_print(format_args!("{0}\n", x));
                };
                {
                    ::std::io::_print(format_args!("\n"));
                };
                chalk.red().println(&"----------------");
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
