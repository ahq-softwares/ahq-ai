#![feature(prelude_import)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
mod server {}
mod ui {}
pub(crate) mod structs {
    use serde::{Deserialize, Serialize};
    use serde_json::from_str;
    use tokio::fs;
    use crate::structs::error::Returns;
    pub mod error {
        use thiserror::Error;
        use std::io::Error as StdError;
        use serde_json::Error as SerdeError;
        pub enum ServerError {
            #[error(transparent)]
            Serde(#[from] SerdeError),
            #[error(transparent)]
            Std(#[from] StdError),
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
                    ServerError::Std(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Std",
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
                    ServerError::Std { 0: transparent } => {
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
                    ServerError::Std(_0) => ::core::fmt::Display::fmt(_0, __formatter),
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
        impl ::core::convert::From<StdError> for ServerError {
            fn from(source: StdError) -> Self {
                ServerError::Std { 0: source }
            }
        }
        pub type Returns<T> = Result<T, ServerError>;
    }
    pub struct Config {
        pub port: u16,
        pub authentication: Authentication,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Config {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "Config",
                "port",
                &self.port,
                "authentication",
                &&self.authentication,
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
                    false as usize + 1 + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "port",
                    &self.port,
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
                            "port" => _serde::__private228::Ok(__Field::__field0),
                            "authentication" => {
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
                            b"port" => _serde::__private228::Ok(__Field::__field0),
                            b"authentication" => {
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
                            u16,
                        >(&mut __seq)? {
                            _serde::__private228::Some(__value) => __value,
                            _serde::__private228::None => {
                                return _serde::__private228::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct Config with 2 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            Authentication,
                        >(&mut __seq)? {
                            _serde::__private228::Some(__value) => __value,
                            _serde::__private228::None => {
                                return _serde::__private228::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct Config with 2 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private228::Ok(Config {
                            port: __field0,
                            authentication: __field1,
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
                        let mut __field0: _serde::__private228::Option<u16> = _serde::__private228::None;
                        let mut __field1: _serde::__private228::Option<Authentication> = _serde::__private228::None;
                        while let _serde::__private228::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private228::Option::is_some(&__field0) {
                                        return _serde::__private228::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("port"),
                                        );
                                    }
                                    __field0 = _serde::__private228::Some(
                                        _serde::de::MapAccess::next_value::<u16>(&mut __map)?,
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private228::Option::is_some(&__field1) {
                                        return _serde::__private228::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "authentication",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private228::Some(
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
                            _serde::__private228::None => {
                                _serde::__private228::de::missing_field("port")?
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private228::Some(__field1) => __field1,
                            _serde::__private228::None => {
                                _serde::__private228::de::missing_field("authentication")?
                            }
                        };
                        _serde::__private228::Ok(Config {
                            port: __field0,
                            authentication: __field1,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &["port", "authentication"];
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
    #[serde(tag = "kind")]
    pub enum Authentication {
        Unauthenticated,
        Authentication { config: AuthConfig },
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Authentication {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                Authentication::Unauthenticated => {
                    ::core::fmt::Formatter::write_str(f, "Unauthenticated")
                }
                Authentication::Authentication { config: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "Authentication",
                        "config",
                        &__self_0,
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
        impl _serde::Serialize for Authentication {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private228::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                match *self {
                    Authentication::Unauthenticated => {
                        let mut __struct = _serde::Serializer::serialize_struct(
                            __serializer,
                            "Authentication",
                            1,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __struct,
                            "kind",
                            "Unauthenticated",
                        )?;
                        _serde::ser::SerializeStruct::end(__struct)
                    }
                    Authentication::Authentication { ref config } => {
                        let mut __serde_state = _serde::Serializer::serialize_struct(
                            __serializer,
                            "Authentication",
                            0 + 1 + 1,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "kind",
                            "Authentication",
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "config",
                            config,
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
                            "Unauthenticated" => {
                                _serde::__private228::Ok(__Field::__field0)
                            }
                            "Authentication" => {
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
                            b"Unauthenticated" => {
                                _serde::__private228::Ok(__Field::__field0)
                            }
                            b"Authentication" => {
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
                    "Unauthenticated",
                    "Authentication",
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
                                "Unauthenticated",
                            ),
                        )?;
                        _serde::__private228::Ok(Authentication::Unauthenticated)
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
                                    "config" => _serde::__private228::Ok(__Field::__field0),
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
                                    b"config" => _serde::__private228::Ok(__Field::__field0),
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
                                    "struct variant Authentication::Authentication",
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
                                    AuthConfig,
                                >(&mut __seq)? {
                                    _serde::__private228::Some(__value) => __value,
                                    _serde::__private228::None => {
                                        return _serde::__private228::Err(
                                            _serde::de::Error::invalid_length(
                                                0usize,
                                                &"struct variant Authentication::Authentication with 1 element",
                                            ),
                                        );
                                    }
                                };
                                _serde::__private228::Ok(Authentication::Authentication {
                                    config: __field0,
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
                                    AuthConfig,
                                > = _serde::__private228::None;
                                while let _serde::__private228::Some(__key) = _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map)? {
                                    match __key {
                                        __Field::__field0 => {
                                            if _serde::__private228::Option::is_some(&__field0) {
                                                return _serde::__private228::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field("config"),
                                                );
                                            }
                                            __field0 = _serde::__private228::Some(
                                                _serde::de::MapAccess::next_value::<AuthConfig>(&mut __map)?,
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
                                        _serde::__private228::de::missing_field("config")?
                                    }
                                };
                                _serde::__private228::Ok(Authentication::Authentication {
                                    config: __field0,
                                })
                            }
                        }
                        #[doc(hidden)]
                        const FIELDS: &'static [&'static str] = &["config"];
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
    pub struct AuthConfig {
        pub registration_allowed: bool,
        pub max_users: Option<u16>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for AuthConfig {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "AuthConfig",
                "registration_allowed",
                &self.registration_allowed,
                "max_users",
                &&self.max_users,
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
        impl _serde::Serialize for AuthConfig {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private228::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "AuthConfig",
                    false as usize + 1 + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "registration_allowed",
                    &self.registration_allowed,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "max_users",
                    &self.max_users,
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
        impl<'de> _serde::Deserialize<'de> for AuthConfig {
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
                    marker: _serde::__private228::PhantomData<AuthConfig>,
                    lifetime: _serde::__private228::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = AuthConfig;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private228::Formatter,
                    ) -> _serde::__private228::fmt::Result {
                        _serde::__private228::Formatter::write_str(
                            __formatter,
                            "struct AuthConfig",
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
                                        &"struct AuthConfig with 2 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            Option<u16>,
                        >(&mut __seq)? {
                            _serde::__private228::Some(__value) => __value,
                            _serde::__private228::None => {
                                return _serde::__private228::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct AuthConfig with 2 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private228::Ok(AuthConfig {
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
                        let mut __field1: _serde::__private228::Option<Option<u16>> = _serde::__private228::None;
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
                                            Option<u16>,
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
                        _serde::__private228::Ok(AuthConfig {
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
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "AuthConfig",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private228::PhantomData::<AuthConfig>,
                        lifetime: _serde::__private228::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    impl ::core::default::Default for AuthConfig {
        #[inline]
        fn default() -> AuthConfig {
            AuthConfig {
                registration_allowed: ::core::default::Default::default(),
                max_users: ::core::default::Default::default(),
            }
        }
    }
    impl Config {
        pub async fn new() -> Returns<Self> {
            let val = fs::read_to_string("").await?;
            Ok(from_str::<Self>(&val)?)
        }
        pub async fn new_or_default() -> Self {
            Self::new().await.unwrap_or_default()
        }
    }
    impl Default for Config {
        fn default() -> Self {
            Self {
                port: 3000,
                authentication: Authentication::Unauthenticated,
            }
        }
    }
}
fn main() {
    {
        ::std::io::_print(format_args!("Hello, world!\n"));
    };
}
