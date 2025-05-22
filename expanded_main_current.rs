#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use anyhow::Result;
use rmcp::{
    model::{
        AnnotateAble, CallToolResult, Content, GetPromptRequestParam, GetPromptResult,
        Implementation, ListPromptsResult, ListResourcesResult, PaginatedRequestParam,
        ProtocolVersion, RawResource, ReadResourceRequestParam, ReadResourceResult,
        Resource, ServerCapabilities, ServerInfo, ToolsCapability, ResourcesCapability,
    },
    Error as McpError, RoleServer, ServerHandler, ServiceExt, service::RequestContext,
    tool, transport::stdio,
};
use nannou_osc as osc;
use schemars::JsonSchema;
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::fs::OpenOptions;
use std::path::Path;
use tracing_subscriber::fmt::writer::MakeWriterExt;
use ardour_mcp::TrackInfo;
const ARDOUR_OSC_TARGET_ADDR: &str = "127.0.0.1:3819";
#[allow(dead_code)]
const MCP_SERVER_OSC_LISTEN_ADDR: &str = "127.0.0.1:9099";
enum PlaybackStatus {
    Playing,
    Stopped,
    Unknown,
}
#[automatically_derived]
impl ::core::fmt::Debug for PlaybackStatus {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(
            f,
            match self {
                PlaybackStatus::Playing => "Playing",
                PlaybackStatus::Stopped => "Stopped",
                PlaybackStatus::Unknown => "Unknown",
            },
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for PlaybackStatus {
    #[inline]
    fn clone(&self) -> PlaybackStatus {
        match self {
            PlaybackStatus::Playing => PlaybackStatus::Playing,
            PlaybackStatus::Stopped => PlaybackStatus::Stopped,
            PlaybackStatus::Unknown => PlaybackStatus::Unknown,
        }
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for PlaybackStatus {}
#[automatically_derived]
impl ::core::cmp::PartialEq for PlaybackStatus {
    #[inline]
    fn eq(&self, other: &PlaybackStatus) -> bool {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
        __self_discr == __arg1_discr
    }
}
struct ArdourState {
    playback_status: PlaybackStatus,
    strip_list: Vec<TrackInfo>,
    transport_frame: Option<i64>,
}
#[automatically_derived]
impl ::core::fmt::Debug for ArdourState {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(
            f,
            "ArdourState",
            "playback_status",
            &self.playback_status,
            "strip_list",
            &self.strip_list,
            "transport_frame",
            &&self.transport_frame,
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for ArdourState {
    #[inline]
    fn clone(&self) -> ArdourState {
        ArdourState {
            playback_status: ::core::clone::Clone::clone(&self.playback_status),
            strip_list: ::core::clone::Clone::clone(&self.strip_list),
            transport_frame: ::core::clone::Clone::clone(&self.transport_frame),
        }
    }
}
impl ArdourState {
    fn new() -> Self {
        Self {
            playback_status: PlaybackStatus::Unknown,
            strip_list: Vec::new(),
            transport_frame: None,
        }
    }
}
struct ArdourService {
    osc_sender: Arc<Mutex<osc::Sender<osc::Connected>>>,
    ardour_state: Arc<Mutex<ArdourState>>,
}
#[automatically_derived]
impl ::core::clone::Clone for ArdourService {
    #[inline]
    fn clone(&self) -> ArdourService {
        ArdourService {
            osc_sender: ::core::clone::Clone::clone(&self.osc_sender),
            ardour_state: ::core::clone::Clone::clone(&self.ardour_state),
        }
    }
}
struct SetTrackMuteArgs {
    #[schemars(description = "The Router ID (rid) of the track/bus.")]
    rid: i32,
    #[schemars(
        description = "The desired mute state (true for mute, false for unmute)."
    )]
    mute_state: bool,
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
    impl<'de> _serde::Deserialize<'de> for SetTrackMuteArgs {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
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
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "field identifier",
                    )
                }
                fn visit_u64<__E>(
                    self,
                    __value: u64,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "rid" => _serde::__private::Ok(__Field::__field0),
                        "mute_state" => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"rid" => _serde::__private::Ok(__Field::__field0),
                        b"mute_state" => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
            }
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
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
                marker: _serde::__private::PhantomData<SetTrackMuteArgs>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = SetTrackMuteArgs;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "struct SetTrackMuteArgs",
                    )
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<
                        i32,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct SetTrackMuteArgs with 2 elements",
                                ),
                            );
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<
                        bool,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct SetTrackMuteArgs with 2 elements",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(SetTrackMuteArgs {
                        rid: __field0,
                        mute_state: __field1,
                    })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<i32> = _serde::__private::None;
                    let mut __field1: _serde::__private::Option<bool> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                        __Field,
                    >(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("rid"),
                                    );
                                }
                                __field0 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<i32>(&mut __map)?,
                                );
                            }
                            __Field::__field1 => {
                                if _serde::__private::Option::is_some(&__field1) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "mute_state",
                                        ),
                                    );
                                }
                                __field1 = _serde::__private::Some(
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
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("rid")?
                        }
                    };
                    let __field1 = match __field1 {
                        _serde::__private::Some(__field1) => __field1,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("mute_state")?
                        }
                    };
                    _serde::__private::Ok(SetTrackMuteArgs {
                        rid: __field0,
                        mute_state: __field1,
                    })
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["rid", "mute_state"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "SetTrackMuteArgs",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<SetTrackMuteArgs>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
const _: () = {
    #[automatically_derived]
    #[allow(unused_braces)]
    impl schemars::JsonSchema for SetTrackMuteArgs {
        fn schema_name() -> std::string::String {
            "SetTrackMuteArgs".to_owned()
        }
        fn schema_id() -> std::borrow::Cow<'static, str> {
            std::borrow::Cow::Borrowed("ardour_mcp_server::SetTrackMuteArgs")
        }
        fn json_schema(
            generator: &mut schemars::gen::SchemaGenerator,
        ) -> schemars::schema::Schema {
            {
                let mut schema_object = schemars::schema::SchemaObject {
                    instance_type: Some(schemars::schema::InstanceType::Object.into()),
                    ..Default::default()
                };
                let object_validation = schema_object.object();
                {
                    schemars::_private::insert_object_property::<
                        i32,
                    >(
                        object_validation,
                        "rid",
                        false,
                        false,
                        schemars::_private::metadata::add_description(
                            generator.subschema_for::<i32>(),
                            "The Router ID (rid) of the track/bus.",
                        ),
                    );
                }
                {
                    schemars::_private::insert_object_property::<
                        bool,
                    >(
                        object_validation,
                        "mute_state",
                        false,
                        false,
                        schemars::_private::metadata::add_description(
                            generator.subschema_for::<bool>(),
                            "The desired mute state (true for mute, false for unmute).",
                        ),
                    );
                }
                schemars::schema::Schema::Object(schema_object)
            }
        }
    }
};
#[automatically_derived]
impl ::core::fmt::Debug for SetTrackMuteArgs {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "SetTrackMuteArgs",
            "rid",
            &self.rid,
            "mute_state",
            &&self.mute_state,
        )
    }
}
struct SetTransportSpeedArgs {
    #[schemars(description = "The desired transport speed. Valid range: -8.0 to 8.0.")]
    speed: f32,
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
    impl<'de> _serde::Deserialize<'de> for SetTransportSpeedArgs {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
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
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "field identifier",
                    )
                }
                fn visit_u64<__E>(
                    self,
                    __value: u64,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "speed" => _serde::__private::Ok(__Field::__field0),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"speed" => _serde::__private::Ok(__Field::__field0),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
            }
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
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
                marker: _serde::__private::PhantomData<SetTransportSpeedArgs>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = SetTransportSpeedArgs;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "struct SetTransportSpeedArgs",
                    )
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<
                        f32,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct SetTransportSpeedArgs with 1 element",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(SetTransportSpeedArgs {
                        speed: __field0,
                    })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<f32> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                        __Field,
                    >(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("speed"),
                                    );
                                }
                                __field0 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<f32>(&mut __map)?,
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
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("speed")?
                        }
                    };
                    _serde::__private::Ok(SetTransportSpeedArgs {
                        speed: __field0,
                    })
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["speed"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "SetTransportSpeedArgs",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<SetTransportSpeedArgs>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
const _: () = {
    #[automatically_derived]
    #[allow(unused_braces)]
    impl schemars::JsonSchema for SetTransportSpeedArgs {
        fn schema_name() -> std::string::String {
            "SetTransportSpeedArgs".to_owned()
        }
        fn schema_id() -> std::borrow::Cow<'static, str> {
            std::borrow::Cow::Borrowed("ardour_mcp_server::SetTransportSpeedArgs")
        }
        fn json_schema(
            generator: &mut schemars::gen::SchemaGenerator,
        ) -> schemars::schema::Schema {
            {
                let mut schema_object = schemars::schema::SchemaObject {
                    instance_type: Some(schemars::schema::InstanceType::Object.into()),
                    ..Default::default()
                };
                let object_validation = schema_object.object();
                {
                    schemars::_private::insert_object_property::<
                        f32,
                    >(
                        object_validation,
                        "speed",
                        false,
                        false,
                        schemars::_private::metadata::add_description(
                            generator.subschema_for::<f32>(),
                            "The desired transport speed. Valid range: -8.0 to 8.0.",
                        ),
                    );
                }
                schemars::schema::Schema::Object(schema_object)
            }
        }
    }
};
#[automatically_derived]
impl ::core::fmt::Debug for SetTransportSpeedArgs {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(
            f,
            "SetTransportSpeedArgs",
            "speed",
            &&self.speed,
        )
    }
}
struct LocateToolArgs {
    #[schemars(description = "The position in samples to locate to.")]
    spos: i64,
    #[schemars(
        description = "Whether to start playing after locating. 0 for stop, 1 for play."
    )]
    roll: i32,
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
    impl<'de> _serde::Deserialize<'de> for LocateToolArgs {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
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
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "field identifier",
                    )
                }
                fn visit_u64<__E>(
                    self,
                    __value: u64,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "spos" => _serde::__private::Ok(__Field::__field0),
                        "roll" => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"spos" => _serde::__private::Ok(__Field::__field0),
                        b"roll" => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
            }
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
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
                marker: _serde::__private::PhantomData<LocateToolArgs>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = LocateToolArgs;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "struct LocateToolArgs",
                    )
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<
                        i64,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct LocateToolArgs with 2 elements",
                                ),
                            );
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<
                        i32,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct LocateToolArgs with 2 elements",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(LocateToolArgs {
                        spos: __field0,
                        roll: __field1,
                    })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<i64> = _serde::__private::None;
                    let mut __field1: _serde::__private::Option<i32> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                        __Field,
                    >(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("spos"),
                                    );
                                }
                                __field0 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<i64>(&mut __map)?,
                                );
                            }
                            __Field::__field1 => {
                                if _serde::__private::Option::is_some(&__field1) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("roll"),
                                    );
                                }
                                __field1 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<i32>(&mut __map)?,
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
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("spos")?
                        }
                    };
                    let __field1 = match __field1 {
                        _serde::__private::Some(__field1) => __field1,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("roll")?
                        }
                    };
                    _serde::__private::Ok(LocateToolArgs {
                        spos: __field0,
                        roll: __field1,
                    })
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["spos", "roll"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "LocateToolArgs",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<LocateToolArgs>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
const _: () = {
    #[automatically_derived]
    #[allow(unused_braces)]
    impl schemars::JsonSchema for LocateToolArgs {
        fn schema_name() -> std::string::String {
            "LocateToolArgs".to_owned()
        }
        fn schema_id() -> std::borrow::Cow<'static, str> {
            std::borrow::Cow::Borrowed("ardour_mcp_server::LocateToolArgs")
        }
        fn json_schema(
            generator: &mut schemars::gen::SchemaGenerator,
        ) -> schemars::schema::Schema {
            {
                let mut schema_object = schemars::schema::SchemaObject {
                    instance_type: Some(schemars::schema::InstanceType::Object.into()),
                    ..Default::default()
                };
                let object_validation = schema_object.object();
                {
                    schemars::_private::insert_object_property::<
                        i64,
                    >(
                        object_validation,
                        "spos",
                        false,
                        false,
                        schemars::_private::metadata::add_description(
                            generator.subschema_for::<i64>(),
                            "The position in samples to locate to.",
                        ),
                    );
                }
                {
                    schemars::_private::insert_object_property::<
                        i32,
                    >(
                        object_validation,
                        "roll",
                        false,
                        false,
                        schemars::_private::metadata::add_description(
                            generator.subschema_for::<i32>(),
                            "Whether to start playing after locating. 0 for stop, 1 for play.",
                        ),
                    );
                }
                schemars::schema::Schema::Object(schema_object)
            }
        }
    }
};
#[automatically_derived]
impl ::core::fmt::Debug for LocateToolArgs {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "LocateToolArgs",
            "spos",
            &self.spos,
            "roll",
            &&self.roll,
        )
    }
}
struct SetTrackSoloArgs {
    #[schemars(description = "The Router ID (rid) of the track/bus.")]
    rid: i32,
    #[schemars(description = "The desired solo state. 0 for solo off, 1 for solo on.")]
    solo_st: i32,
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
    impl<'de> _serde::Deserialize<'de> for SetTrackSoloArgs {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
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
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "field identifier",
                    )
                }
                fn visit_u64<__E>(
                    self,
                    __value: u64,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "rid" => _serde::__private::Ok(__Field::__field0),
                        "solo_st" => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"rid" => _serde::__private::Ok(__Field::__field0),
                        b"solo_st" => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
            }
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
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
                marker: _serde::__private::PhantomData<SetTrackSoloArgs>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = SetTrackSoloArgs;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "struct SetTrackSoloArgs",
                    )
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<
                        i32,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct SetTrackSoloArgs with 2 elements",
                                ),
                            );
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<
                        i32,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct SetTrackSoloArgs with 2 elements",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(SetTrackSoloArgs {
                        rid: __field0,
                        solo_st: __field1,
                    })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<i32> = _serde::__private::None;
                    let mut __field1: _serde::__private::Option<i32> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                        __Field,
                    >(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("rid"),
                                    );
                                }
                                __field0 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<i32>(&mut __map)?,
                                );
                            }
                            __Field::__field1 => {
                                if _serde::__private::Option::is_some(&__field1) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "solo_st",
                                        ),
                                    );
                                }
                                __field1 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<i32>(&mut __map)?,
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
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("rid")?
                        }
                    };
                    let __field1 = match __field1 {
                        _serde::__private::Some(__field1) => __field1,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("solo_st")?
                        }
                    };
                    _serde::__private::Ok(SetTrackSoloArgs {
                        rid: __field0,
                        solo_st: __field1,
                    })
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["rid", "solo_st"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "SetTrackSoloArgs",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<SetTrackSoloArgs>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
const _: () = {
    #[automatically_derived]
    #[allow(unused_braces)]
    impl schemars::JsonSchema for SetTrackSoloArgs {
        fn schema_name() -> std::string::String {
            "SetTrackSoloArgs".to_owned()
        }
        fn schema_id() -> std::borrow::Cow<'static, str> {
            std::borrow::Cow::Borrowed("ardour_mcp_server::SetTrackSoloArgs")
        }
        fn json_schema(
            generator: &mut schemars::gen::SchemaGenerator,
        ) -> schemars::schema::Schema {
            {
                let mut schema_object = schemars::schema::SchemaObject {
                    instance_type: Some(schemars::schema::InstanceType::Object.into()),
                    ..Default::default()
                };
                let object_validation = schema_object.object();
                {
                    schemars::_private::insert_object_property::<
                        i32,
                    >(
                        object_validation,
                        "rid",
                        false,
                        false,
                        schemars::_private::metadata::add_description(
                            generator.subschema_for::<i32>(),
                            "The Router ID (rid) of the track/bus.",
                        ),
                    );
                }
                {
                    schemars::_private::insert_object_property::<
                        i32,
                    >(
                        object_validation,
                        "solo_st",
                        false,
                        false,
                        schemars::_private::metadata::add_description(
                            generator.subschema_for::<i32>(),
                            "The desired solo state. 0 for solo off, 1 for solo on.",
                        ),
                    );
                }
                schemars::schema::Schema::Object(schema_object)
            }
        }
    }
};
#[automatically_derived]
impl ::core::fmt::Debug for SetTrackSoloArgs {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "SetTrackSoloArgs",
            "rid",
            &self.rid,
            "solo_st",
            &&self.solo_st,
        )
    }
}
struct SetTrackRecEnableArgs {
    #[schemars(description = "The Router ID (rid) of the track/bus.")]
    rid: i32,
    #[schemars(description = "The desired record enable state. 0 for off, 1 for on.")]
    rec_st: i32,
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
    impl<'de> _serde::Deserialize<'de> for SetTrackRecEnableArgs {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
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
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "field identifier",
                    )
                }
                fn visit_u64<__E>(
                    self,
                    __value: u64,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "rid" => _serde::__private::Ok(__Field::__field0),
                        "rec_st" => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"rid" => _serde::__private::Ok(__Field::__field0),
                        b"rec_st" => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
            }
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
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
                marker: _serde::__private::PhantomData<SetTrackRecEnableArgs>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = SetTrackRecEnableArgs;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "struct SetTrackRecEnableArgs",
                    )
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<
                        i32,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct SetTrackRecEnableArgs with 2 elements",
                                ),
                            );
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<
                        i32,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct SetTrackRecEnableArgs with 2 elements",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(SetTrackRecEnableArgs {
                        rid: __field0,
                        rec_st: __field1,
                    })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<i32> = _serde::__private::None;
                    let mut __field1: _serde::__private::Option<i32> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                        __Field,
                    >(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("rid"),
                                    );
                                }
                                __field0 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<i32>(&mut __map)?,
                                );
                            }
                            __Field::__field1 => {
                                if _serde::__private::Option::is_some(&__field1) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("rec_st"),
                                    );
                                }
                                __field1 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<i32>(&mut __map)?,
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
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("rid")?
                        }
                    };
                    let __field1 = match __field1 {
                        _serde::__private::Some(__field1) => __field1,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("rec_st")?
                        }
                    };
                    _serde::__private::Ok(SetTrackRecEnableArgs {
                        rid: __field0,
                        rec_st: __field1,
                    })
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["rid", "rec_st"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "SetTrackRecEnableArgs",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<SetTrackRecEnableArgs>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
const _: () = {
    #[automatically_derived]
    #[allow(unused_braces)]
    impl schemars::JsonSchema for SetTrackRecEnableArgs {
        fn schema_name() -> std::string::String {
            "SetTrackRecEnableArgs".to_owned()
        }
        fn schema_id() -> std::borrow::Cow<'static, str> {
            std::borrow::Cow::Borrowed("ardour_mcp_server::SetTrackRecEnableArgs")
        }
        fn json_schema(
            generator: &mut schemars::gen::SchemaGenerator,
        ) -> schemars::schema::Schema {
            {
                let mut schema_object = schemars::schema::SchemaObject {
                    instance_type: Some(schemars::schema::InstanceType::Object.into()),
                    ..Default::default()
                };
                let object_validation = schema_object.object();
                {
                    schemars::_private::insert_object_property::<
                        i32,
                    >(
                        object_validation,
                        "rid",
                        false,
                        false,
                        schemars::_private::metadata::add_description(
                            generator.subschema_for::<i32>(),
                            "The Router ID (rid) of the track/bus.",
                        ),
                    );
                }
                {
                    schemars::_private::insert_object_property::<
                        i32,
                    >(
                        object_validation,
                        "rec_st",
                        false,
                        false,
                        schemars::_private::metadata::add_description(
                            generator.subschema_for::<i32>(),
                            "The desired record enable state. 0 for off, 1 for on.",
                        ),
                    );
                }
                schemars::schema::Schema::Object(schema_object)
            }
        }
    }
};
#[automatically_derived]
impl ::core::fmt::Debug for SetTrackRecEnableArgs {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "SetTrackRecEnableArgs",
            "rid",
            &self.rid,
            "rec_st",
            &&self.rec_st,
        )
    }
}
struct SetTrackGainAbsArgs {
    #[schemars(description = "The Router ID (rid) of the track/bus.")]
    rid: i32,
    #[schemars(description = "The desired absolute gain. Valid range: 0.0 to 2.0.")]
    gain_abs: f32,
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
    impl<'de> _serde::Deserialize<'de> for SetTrackGainAbsArgs {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
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
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "field identifier",
                    )
                }
                fn visit_u64<__E>(
                    self,
                    __value: u64,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "rid" => _serde::__private::Ok(__Field::__field0),
                        "gain_abs" => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"rid" => _serde::__private::Ok(__Field::__field0),
                        b"gain_abs" => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
            }
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
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
                marker: _serde::__private::PhantomData<SetTrackGainAbsArgs>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = SetTrackGainAbsArgs;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "struct SetTrackGainAbsArgs",
                    )
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<
                        i32,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct SetTrackGainAbsArgs with 2 elements",
                                ),
                            );
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<
                        f32,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct SetTrackGainAbsArgs with 2 elements",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(SetTrackGainAbsArgs {
                        rid: __field0,
                        gain_abs: __field1,
                    })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<i32> = _serde::__private::None;
                    let mut __field1: _serde::__private::Option<f32> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                        __Field,
                    >(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("rid"),
                                    );
                                }
                                __field0 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<i32>(&mut __map)?,
                                );
                            }
                            __Field::__field1 => {
                                if _serde::__private::Option::is_some(&__field1) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "gain_abs",
                                        ),
                                    );
                                }
                                __field1 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<f32>(&mut __map)?,
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
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("rid")?
                        }
                    };
                    let __field1 = match __field1 {
                        _serde::__private::Some(__field1) => __field1,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("gain_abs")?
                        }
                    };
                    _serde::__private::Ok(SetTrackGainAbsArgs {
                        rid: __field0,
                        gain_abs: __field1,
                    })
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["rid", "gain_abs"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "SetTrackGainAbsArgs",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<SetTrackGainAbsArgs>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
const _: () = {
    #[automatically_derived]
    #[allow(unused_braces)]
    impl schemars::JsonSchema for SetTrackGainAbsArgs {
        fn schema_name() -> std::string::String {
            "SetTrackGainAbsArgs".to_owned()
        }
        fn schema_id() -> std::borrow::Cow<'static, str> {
            std::borrow::Cow::Borrowed("ardour_mcp_server::SetTrackGainAbsArgs")
        }
        fn json_schema(
            generator: &mut schemars::gen::SchemaGenerator,
        ) -> schemars::schema::Schema {
            {
                let mut schema_object = schemars::schema::SchemaObject {
                    instance_type: Some(schemars::schema::InstanceType::Object.into()),
                    ..Default::default()
                };
                let object_validation = schema_object.object();
                {
                    schemars::_private::insert_object_property::<
                        i32,
                    >(
                        object_validation,
                        "rid",
                        false,
                        false,
                        schemars::_private::metadata::add_description(
                            generator.subschema_for::<i32>(),
                            "The Router ID (rid) of the track/bus.",
                        ),
                    );
                }
                {
                    schemars::_private::insert_object_property::<
                        f32,
                    >(
                        object_validation,
                        "gain_abs",
                        false,
                        false,
                        schemars::_private::metadata::add_description(
                            generator.subschema_for::<f32>(),
                            "The desired absolute gain. Valid range: 0.0 to 2.0.",
                        ),
                    );
                }
                schemars::schema::Schema::Object(schema_object)
            }
        }
    }
};
#[automatically_derived]
impl ::core::fmt::Debug for SetTrackGainAbsArgs {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "SetTrackGainAbsArgs",
            "rid",
            &self.rid,
            "gain_abs",
            &&self.gain_abs,
        )
    }
}
struct SetTrackGainDBArgs {
    #[schemars(description = "The Router ID (rid) of the track/bus.")]
    rid: i32,
    #[schemars(description = "The desired gain in dB. Valid range: -400.0 to 6.0.")]
    gain_db: f32,
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
    impl<'de> _serde::Deserialize<'de> for SetTrackGainDBArgs {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
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
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "field identifier",
                    )
                }
                fn visit_u64<__E>(
                    self,
                    __value: u64,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "rid" => _serde::__private::Ok(__Field::__field0),
                        "gain_db" => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"rid" => _serde::__private::Ok(__Field::__field0),
                        b"gain_db" => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
            }
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
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
                marker: _serde::__private::PhantomData<SetTrackGainDBArgs>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = SetTrackGainDBArgs;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "struct SetTrackGainDBArgs",
                    )
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<
                        i32,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct SetTrackGainDBArgs with 2 elements",
                                ),
                            );
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<
                        f32,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct SetTrackGainDBArgs with 2 elements",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(SetTrackGainDBArgs {
                        rid: __field0,
                        gain_db: __field1,
                    })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<i32> = _serde::__private::None;
                    let mut __field1: _serde::__private::Option<f32> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                        __Field,
                    >(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("rid"),
                                    );
                                }
                                __field0 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<i32>(&mut __map)?,
                                );
                            }
                            __Field::__field1 => {
                                if _serde::__private::Option::is_some(&__field1) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "gain_db",
                                        ),
                                    );
                                }
                                __field1 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<f32>(&mut __map)?,
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
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("rid")?
                        }
                    };
                    let __field1 = match __field1 {
                        _serde::__private::Some(__field1) => __field1,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("gain_db")?
                        }
                    };
                    _serde::__private::Ok(SetTrackGainDBArgs {
                        rid: __field0,
                        gain_db: __field1,
                    })
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["rid", "gain_db"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "SetTrackGainDBArgs",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<SetTrackGainDBArgs>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
const _: () = {
    #[automatically_derived]
    #[allow(unused_braces)]
    impl schemars::JsonSchema for SetTrackGainDBArgs {
        fn schema_name() -> std::string::String {
            "SetTrackGainDBArgs".to_owned()
        }
        fn schema_id() -> std::borrow::Cow<'static, str> {
            std::borrow::Cow::Borrowed("ardour_mcp_server::SetTrackGainDBArgs")
        }
        fn json_schema(
            generator: &mut schemars::gen::SchemaGenerator,
        ) -> schemars::schema::Schema {
            {
                let mut schema_object = schemars::schema::SchemaObject {
                    instance_type: Some(schemars::schema::InstanceType::Object.into()),
                    ..Default::default()
                };
                let object_validation = schema_object.object();
                {
                    schemars::_private::insert_object_property::<
                        i32,
                    >(
                        object_validation,
                        "rid",
                        false,
                        false,
                        schemars::_private::metadata::add_description(
                            generator.subschema_for::<i32>(),
                            "The Router ID (rid) of the track/bus.",
                        ),
                    );
                }
                {
                    schemars::_private::insert_object_property::<
                        f32,
                    >(
                        object_validation,
                        "gain_db",
                        false,
                        false,
                        schemars::_private::metadata::add_description(
                            generator.subschema_for::<f32>(),
                            "The desired gain in dB. Valid range: -400.0 to 6.0.",
                        ),
                    );
                }
                schemars::schema::Schema::Object(schema_object)
            }
        }
    }
};
#[automatically_derived]
impl ::core::fmt::Debug for SetTrackGainDBArgs {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "SetTrackGainDBArgs",
            "rid",
            &self.rid,
            "gain_db",
            &&self.gain_db,
        )
    }
}
struct SetTrackTrimAbsArgs {
    #[schemars(description = "The Router ID (rid) of the track/bus.")]
    rid: i32,
    #[schemars(description = "The desired absolute trim. Valid range: 0.1 to 10.0.")]
    trim_abs: f32,
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
    impl<'de> _serde::Deserialize<'de> for SetTrackTrimAbsArgs {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
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
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "field identifier",
                    )
                }
                fn visit_u64<__E>(
                    self,
                    __value: u64,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "rid" => _serde::__private::Ok(__Field::__field0),
                        "trim_abs" => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"rid" => _serde::__private::Ok(__Field::__field0),
                        b"trim_abs" => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
            }
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
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
                marker: _serde::__private::PhantomData<SetTrackTrimAbsArgs>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = SetTrackTrimAbsArgs;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "struct SetTrackTrimAbsArgs",
                    )
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<
                        i32,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct SetTrackTrimAbsArgs with 2 elements",
                                ),
                            );
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<
                        f32,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct SetTrackTrimAbsArgs with 2 elements",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(SetTrackTrimAbsArgs {
                        rid: __field0,
                        trim_abs: __field1,
                    })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<i32> = _serde::__private::None;
                    let mut __field1: _serde::__private::Option<f32> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                        __Field,
                    >(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("rid"),
                                    );
                                }
                                __field0 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<i32>(&mut __map)?,
                                );
                            }
                            __Field::__field1 => {
                                if _serde::__private::Option::is_some(&__field1) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "trim_abs",
                                        ),
                                    );
                                }
                                __field1 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<f32>(&mut __map)?,
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
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("rid")?
                        }
                    };
                    let __field1 = match __field1 {
                        _serde::__private::Some(__field1) => __field1,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("trim_abs")?
                        }
                    };
                    _serde::__private::Ok(SetTrackTrimAbsArgs {
                        rid: __field0,
                        trim_abs: __field1,
                    })
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["rid", "trim_abs"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "SetTrackTrimAbsArgs",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<SetTrackTrimAbsArgs>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
const _: () = {
    #[automatically_derived]
    #[allow(unused_braces)]
    impl schemars::JsonSchema for SetTrackTrimAbsArgs {
        fn schema_name() -> std::string::String {
            "SetTrackTrimAbsArgs".to_owned()
        }
        fn schema_id() -> std::borrow::Cow<'static, str> {
            std::borrow::Cow::Borrowed("ardour_mcp_server::SetTrackTrimAbsArgs")
        }
        fn json_schema(
            generator: &mut schemars::gen::SchemaGenerator,
        ) -> schemars::schema::Schema {
            {
                let mut schema_object = schemars::schema::SchemaObject {
                    instance_type: Some(schemars::schema::InstanceType::Object.into()),
                    ..Default::default()
                };
                let object_validation = schema_object.object();
                {
                    schemars::_private::insert_object_property::<
                        i32,
                    >(
                        object_validation,
                        "rid",
                        false,
                        false,
                        schemars::_private::metadata::add_description(
                            generator.subschema_for::<i32>(),
                            "The Router ID (rid) of the track/bus.",
                        ),
                    );
                }
                {
                    schemars::_private::insert_object_property::<
                        f32,
                    >(
                        object_validation,
                        "trim_abs",
                        false,
                        false,
                        schemars::_private::metadata::add_description(
                            generator.subschema_for::<f32>(),
                            "The desired absolute trim. Valid range: 0.1 to 10.0.",
                        ),
                    );
                }
                schemars::schema::Schema::Object(schema_object)
            }
        }
    }
};
#[automatically_derived]
impl ::core::fmt::Debug for SetTrackTrimAbsArgs {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "SetTrackTrimAbsArgs",
            "rid",
            &self.rid,
            "trim_abs",
            &&self.trim_abs,
        )
    }
}
struct SetTrackTrimDBArgs {
    #[schemars(description = "The Router ID (rid) of the track/bus.")]
    rid: i32,
    #[schemars(description = "The desired trim in dB. Valid range: -20.0 to 20.0.")]
    trim_db: f32,
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
    impl<'de> _serde::Deserialize<'de> for SetTrackTrimDBArgs {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
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
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "field identifier",
                    )
                }
                fn visit_u64<__E>(
                    self,
                    __value: u64,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "rid" => _serde::__private::Ok(__Field::__field0),
                        "trim_db" => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"rid" => _serde::__private::Ok(__Field::__field0),
                        b"trim_db" => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
            }
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
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
                marker: _serde::__private::PhantomData<SetTrackTrimDBArgs>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = SetTrackTrimDBArgs;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "struct SetTrackTrimDBArgs",
                    )
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<
                        i32,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct SetTrackTrimDBArgs with 2 elements",
                                ),
                            );
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<
                        f32,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct SetTrackTrimDBArgs with 2 elements",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(SetTrackTrimDBArgs {
                        rid: __field0,
                        trim_db: __field1,
                    })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<i32> = _serde::__private::None;
                    let mut __field1: _serde::__private::Option<f32> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                        __Field,
                    >(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("rid"),
                                    );
                                }
                                __field0 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<i32>(&mut __map)?,
                                );
                            }
                            __Field::__field1 => {
                                if _serde::__private::Option::is_some(&__field1) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "trim_db",
                                        ),
                                    );
                                }
                                __field1 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<f32>(&mut __map)?,
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
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("rid")?
                        }
                    };
                    let __field1 = match __field1 {
                        _serde::__private::Some(__field1) => __field1,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("trim_db")?
                        }
                    };
                    _serde::__private::Ok(SetTrackTrimDBArgs {
                        rid: __field0,
                        trim_db: __field1,
                    })
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["rid", "trim_db"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "SetTrackTrimDBArgs",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<SetTrackTrimDBArgs>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
const _: () = {
    #[automatically_derived]
    #[allow(unused_braces)]
    impl schemars::JsonSchema for SetTrackTrimDBArgs {
        fn schema_name() -> std::string::String {
            "SetTrackTrimDBArgs".to_owned()
        }
        fn schema_id() -> std::borrow::Cow<'static, str> {
            std::borrow::Cow::Borrowed("ardour_mcp_server::SetTrackTrimDBArgs")
        }
        fn json_schema(
            generator: &mut schemars::gen::SchemaGenerator,
        ) -> schemars::schema::Schema {
            {
                let mut schema_object = schemars::schema::SchemaObject {
                    instance_type: Some(schemars::schema::InstanceType::Object.into()),
                    ..Default::default()
                };
                let object_validation = schema_object.object();
                {
                    schemars::_private::insert_object_property::<
                        i32,
                    >(
                        object_validation,
                        "rid",
                        false,
                        false,
                        schemars::_private::metadata::add_description(
                            generator.subschema_for::<i32>(),
                            "The Router ID (rid) of the track/bus.",
                        ),
                    );
                }
                {
                    schemars::_private::insert_object_property::<
                        f32,
                    >(
                        object_validation,
                        "trim_db",
                        false,
                        false,
                        schemars::_private::metadata::add_description(
                            generator.subschema_for::<f32>(),
                            "The desired trim in dB. Valid range: -20.0 to 20.0.",
                        ),
                    );
                }
                schemars::schema::Schema::Object(schema_object)
            }
        }
    }
};
#[automatically_derived]
impl ::core::fmt::Debug for SetTrackTrimDBArgs {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "SetTrackTrimDBArgs",
            "rid",
            &self.rid,
            "trim_db",
            &&self.trim_db,
        )
    }
}
struct AccessActionArgs {
    #[schemars(
        description = "The name of the Ardour menu action to execute (e.g., 'Editor/zoom-to-session')."
    )]
    action_name: String,
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
    impl<'de> _serde::Deserialize<'de> for AccessActionArgs {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
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
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "field identifier",
                    )
                }
                fn visit_u64<__E>(
                    self,
                    __value: u64,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "action_name" => _serde::__private::Ok(__Field::__field0),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"action_name" => _serde::__private::Ok(__Field::__field0),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
            }
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
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
                marker: _serde::__private::PhantomData<AccessActionArgs>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = AccessActionArgs;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "struct AccessActionArgs",
                    )
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<
                        String,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct AccessActionArgs with 1 element",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(AccessActionArgs {
                        action_name: __field0,
                    })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                        __Field,
                    >(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "action_name",
                                        ),
                                    );
                                }
                                __field0 = _serde::__private::Some(
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
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("action_name")?
                        }
                    };
                    _serde::__private::Ok(AccessActionArgs {
                        action_name: __field0,
                    })
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["action_name"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "AccessActionArgs",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<AccessActionArgs>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
const _: () = {
    #[automatically_derived]
    #[allow(unused_braces)]
    impl schemars::JsonSchema for AccessActionArgs {
        fn schema_name() -> std::string::String {
            "AccessActionArgs".to_owned()
        }
        fn schema_id() -> std::borrow::Cow<'static, str> {
            std::borrow::Cow::Borrowed("ardour_mcp_server::AccessActionArgs")
        }
        fn json_schema(
            generator: &mut schemars::gen::SchemaGenerator,
        ) -> schemars::schema::Schema {
            {
                let mut schema_object = schemars::schema::SchemaObject {
                    instance_type: Some(schemars::schema::InstanceType::Object.into()),
                    ..Default::default()
                };
                let object_validation = schema_object.object();
                {
                    schemars::_private::insert_object_property::<
                        String,
                    >(
                        object_validation,
                        "action_name",
                        false,
                        false,
                        schemars::_private::metadata::add_description(
                            generator.subschema_for::<String>(),
                            "The name of the Ardour menu action to execute (e.g., 'Editor/zoom-to-session').",
                        ),
                    );
                }
                schemars::schema::Schema::Object(schema_object)
            }
        }
    }
};
#[automatically_derived]
impl ::core::fmt::Debug for AccessActionArgs {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(
            f,
            "AccessActionArgs",
            "action_name",
            &&self.action_name,
        )
    }
}
impl ArdourService {
    pub fn new() -> Result<Self> {
        {
            use ::tracing::__macro_support::Callsite as _;
            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                static META: ::tracing::Metadata<'static> = {
                    ::tracing_core::metadata::Metadata::new(
                        "event src/main.rs:163",
                        "ardour_mcp_server",
                        ::tracing::Level::INFO,
                        ::tracing_core::__macro_support::Option::Some("src/main.rs"),
                        ::tracing_core::__macro_support::Option::Some(163u32),
                        ::tracing_core::__macro_support::Option::Some(
                            "ardour_mcp_server",
                        ),
                        ::tracing_core::field::FieldSet::new(
                            &["message"],
                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                        ),
                        ::tracing::metadata::Kind::EVENT,
                    )
                };
                ::tracing::callsite::DefaultCallsite::new(&META)
            };
            let enabled = ::tracing::Level::INFO
                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                && ::tracing::Level::INFO
                    <= ::tracing::level_filters::LevelFilter::current()
                && {
                    let interest = __CALLSITE.interest();
                    !interest.is_never()
                        && ::tracing::__macro_support::__is_enabled(
                            __CALLSITE.metadata(),
                            interest,
                        )
                };
            if enabled {
                (|value_set: ::tracing::field::ValueSet| {
                    let meta = __CALLSITE.metadata();
                    ::tracing::Event::dispatch(meta, &value_set);
                })({
                    #[allow(unused_imports)]
                    use ::tracing::field::{debug, display, Value};
                    let mut iter = __CALLSITE.metadata().fields().iter();
                    __CALLSITE
                        .metadata()
                        .fields()
                        .value_set(
                            &[
                                (
                                    &::tracing::__macro_support::Iterator::next(&mut iter)
                                        .expect("FieldSet corrupted (this is a bug)"),
                                    ::tracing::__macro_support::Option::Some(
                                        &format_args!(
                                            "Attempting to create OSC sender for Ardour at {0}",
                                            ARDOUR_OSC_TARGET_ADDR,
                                        ) as &dyn Value,
                                    ),
                                ),
                            ],
                        )
                });
            } else {
            }
        };
        let sender = osc::sender()
            .map_err(|e| ::anyhow::Error::msg(
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!("Failed to create OSC sender builder: {0}", e),
                    )
                }),
            ))?
            .connect(ARDOUR_OSC_TARGET_ADDR)
            .map_err(|e| ::anyhow::Error::msg(
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!(
                            "Failed to prepare OSC sender for {0}: {1}",
                            ARDOUR_OSC_TARGET_ADDR,
                            e,
                        ),
                    )
                }),
            ))?;
        {
            use ::tracing::__macro_support::Callsite as _;
            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                static META: ::tracing::Metadata<'static> = {
                    ::tracing_core::metadata::Metadata::new(
                        "event src/main.rs:168",
                        "ardour_mcp_server",
                        ::tracing::Level::INFO,
                        ::tracing_core::__macro_support::Option::Some("src/main.rs"),
                        ::tracing_core::__macro_support::Option::Some(168u32),
                        ::tracing_core::__macro_support::Option::Some(
                            "ardour_mcp_server",
                        ),
                        ::tracing_core::field::FieldSet::new(
                            &["message"],
                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                        ),
                        ::tracing::metadata::Kind::EVENT,
                    )
                };
                ::tracing::callsite::DefaultCallsite::new(&META)
            };
            let enabled = ::tracing::Level::INFO
                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                && ::tracing::Level::INFO
                    <= ::tracing::level_filters::LevelFilter::current()
                && {
                    let interest = __CALLSITE.interest();
                    !interest.is_never()
                        && ::tracing::__macro_support::__is_enabled(
                            __CALLSITE.metadata(),
                            interest,
                        )
                };
            if enabled {
                (|value_set: ::tracing::field::ValueSet| {
                    let meta = __CALLSITE.metadata();
                    ::tracing::Event::dispatch(meta, &value_set);
                })({
                    #[allow(unused_imports)]
                    use ::tracing::field::{debug, display, Value};
                    let mut iter = __CALLSITE.metadata().fields().iter();
                    __CALLSITE
                        .metadata()
                        .fields()
                        .value_set(
                            &[
                                (
                                    &::tracing::__macro_support::Iterator::next(&mut iter)
                                        .expect("FieldSet corrupted (this is a bug)"),
                                    ::tracing::__macro_support::Option::Some(
                                        &format_args!(
                                            "OSC sender created and connected to Ardour at {0}",
                                            ARDOUR_OSC_TARGET_ADDR,
                                        ) as &dyn Value,
                                    ),
                                ),
                            ],
                        )
                });
            } else {
            }
        };
        Ok(Self {
            osc_sender: Arc::new(Mutex::new(sender)),
            ardour_state: Arc::new(Mutex::new(ArdourState::new())),
        })
    }
    async fn send_osc_setup_to_ardour(&self) -> Result<()> {
        {
            use ::tracing::__macro_support::Callsite as _;
            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                static META: ::tracing::Metadata<'static> = {
                    ::tracing_core::metadata::Metadata::new(
                        "event src/main.rs:176",
                        "ardour_mcp_server",
                        ::tracing::Level::INFO,
                        ::tracing_core::__macro_support::Option::Some("src/main.rs"),
                        ::tracing_core::__macro_support::Option::Some(176u32),
                        ::tracing_core::__macro_support::Option::Some(
                            "ardour_mcp_server",
                        ),
                        ::tracing_core::field::FieldSet::new(
                            &["message"],
                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                        ),
                        ::tracing::metadata::Kind::EVENT,
                    )
                };
                ::tracing::callsite::DefaultCallsite::new(&META)
            };
            let enabled = ::tracing::Level::INFO
                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                && ::tracing::Level::INFO
                    <= ::tracing::level_filters::LevelFilter::current()
                && {
                    let interest = __CALLSITE.interest();
                    !interest.is_never()
                        && ::tracing::__macro_support::__is_enabled(
                            __CALLSITE.metadata(),
                            interest,
                        )
                };
            if enabled {
                (|value_set: ::tracing::field::ValueSet| {
                    let meta = __CALLSITE.metadata();
                    ::tracing::Event::dispatch(meta, &value_set);
                })({
                    #[allow(unused_imports)]
                    use ::tracing::field::{debug, display, Value};
                    let mut iter = __CALLSITE.metadata().fields().iter();
                    __CALLSITE
                        .metadata()
                        .fields()
                        .value_set(
                            &[
                                (
                                    &::tracing::__macro_support::Iterator::next(&mut iter)
                                        .expect("FieldSet corrupted (this is a bug)"),
                                    ::tracing::__macro_support::Option::Some(
                                        &format_args!(
                                            "Sending /set_surface to Ardour to enable OSC feedback.",
                                        ) as &dyn Value,
                                    ),
                                ),
                            ],
                        )
                });
            } else {
            }
        };
        let parts: Vec<&str> = MCP_SERVER_OSC_LISTEN_ADDR.split(':').collect();
        if parts.len() != 2 {
            return Err(
                ::anyhow::Error::msg(
                    ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "Invalid MCP_SERVER_OSC_LISTEN_ADDR format: {0}",
                                MCP_SERVER_OSC_LISTEN_ADDR,
                            ),
                        )
                    }),
                ),
            );
        }
        let feedback_port_num: i32 = parts[1]
            .parse()
            .map_err(|e| ::anyhow::Error::msg(
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!(
                            "Failed to parse port from MCP_SERVER_OSC_LISTEN_ADDR: {0}",
                            e,
                        ),
                    )
                }),
            ))?;
        {
            use ::tracing::__macro_support::Callsite as _;
            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                static META: ::tracing::Metadata<'static> = {
                    ::tracing_core::metadata::Metadata::new(
                        "event src/main.rs:186",
                        "ardour_mcp_server",
                        ::tracing::Level::INFO,
                        ::tracing_core::__macro_support::Option::Some("src/main.rs"),
                        ::tracing_core::__macro_support::Option::Some(186u32),
                        ::tracing_core::__macro_support::Option::Some(
                            "ardour_mcp_server",
                        ),
                        ::tracing_core::field::FieldSet::new(
                            &["message"],
                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                        ),
                        ::tracing::metadata::Kind::EVENT,
                    )
                };
                ::tracing::callsite::DefaultCallsite::new(&META)
            };
            let enabled = ::tracing::Level::INFO
                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                && ::tracing::Level::INFO
                    <= ::tracing::level_filters::LevelFilter::current()
                && {
                    let interest = __CALLSITE.interest();
                    !interest.is_never()
                        && ::tracing::__macro_support::__is_enabled(
                            __CALLSITE.metadata(),
                            interest,
                        )
                };
            if enabled {
                (|value_set: ::tracing::field::ValueSet| {
                    let meta = __CALLSITE.metadata();
                    ::tracing::Event::dispatch(meta, &value_set);
                })({
                    #[allow(unused_imports)]
                    use ::tracing::field::{debug, display, Value};
                    let mut iter = __CALLSITE.metadata().fields().iter();
                    __CALLSITE
                        .metadata()
                        .fields()
                        .value_set(
                            &[
                                (
                                    &::tracing::__macro_support::Iterator::next(&mut iter)
                                        .expect("FieldSet corrupted (this is a bug)"),
                                    ::tracing::__macro_support::Option::Some(
                                        &format_args!(
                                            "Targeting feedback port: {0}",
                                            feedback_port_num,
                                        ) as &dyn Value,
                                    ),
                                ),
                            ],
                        )
                });
            } else {
            }
        };
        let osc_args = <[_]>::into_vec(
            ::alloc::boxed::box_new([
                osc::Type::Int(0),
                osc::Type::Int(159),
                osc::Type::Int(7),
                osc::Type::Int(0),
                osc::Type::Int(0),
                osc::Type::Int(0),
                osc::Type::Int(feedback_port_num),
                osc::Type::Int(0),
                osc::Type::Int(0),
            ]),
        );
        match self.send_osc_message("/set_surface", Some(osc_args)).await {
            Ok(_) => {
                {
                    use ::tracing::__macro_support::Callsite as _;
                    static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "event src/main.rs:206",
                                "ardour_mcp_server",
                                ::tracing::Level::INFO,
                                ::tracing_core::__macro_support::Option::Some(
                                    "src/main.rs",
                                ),
                                ::tracing_core::__macro_support::Option::Some(206u32),
                                ::tracing_core::__macro_support::Option::Some(
                                    "ardour_mcp_server",
                                ),
                                ::tracing_core::field::FieldSet::new(
                                    &["message"],
                                    ::tracing_core::callsite::Identifier(&__CALLSITE),
                                ),
                                ::tracing::metadata::Kind::EVENT,
                            )
                        };
                        ::tracing::callsite::DefaultCallsite::new(&META)
                    };
                    let enabled = ::tracing::Level::INFO
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && ::tracing::Level::INFO
                            <= ::tracing::level_filters::LevelFilter::current()
                        && {
                            let interest = __CALLSITE.interest();
                            !interest.is_never()
                                && ::tracing::__macro_support::__is_enabled(
                                    __CALLSITE.metadata(),
                                    interest,
                                )
                        };
                    if enabled {
                        (|value_set: ::tracing::field::ValueSet| {
                            let meta = __CALLSITE.metadata();
                            ::tracing::Event::dispatch(meta, &value_set);
                        })({
                            #[allow(unused_imports)]
                            use ::tracing::field::{debug, display, Value};
                            let mut iter = __CALLSITE.metadata().fields().iter();
                            __CALLSITE
                                .metadata()
                                .fields()
                                .value_set(
                                    &[
                                        (
                                            &::tracing::__macro_support::Iterator::next(&mut iter)
                                                .expect("FieldSet corrupted (this is a bug)"),
                                            ::tracing::__macro_support::Option::Some(
                                                &format_args!(
                                                    "/set_surface command sent successfully to Ardour.",
                                                ) as &dyn Value,
                                            ),
                                        ),
                                    ],
                                )
                        });
                    } else {
                    }
                };
                Ok(())
            }
            Err(e) => {
                let err_msg = ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!(
                            "Failed to send /set_surface OSC message to Ardour: {0}",
                            e,
                        ),
                    )
                });
                {
                    use ::tracing::__macro_support::Callsite as _;
                    static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "event src/main.rs:211",
                                "ardour_mcp_server",
                                ::tracing::Level::ERROR,
                                ::tracing_core::__macro_support::Option::Some(
                                    "src/main.rs",
                                ),
                                ::tracing_core::__macro_support::Option::Some(211u32),
                                ::tracing_core::__macro_support::Option::Some(
                                    "ardour_mcp_server",
                                ),
                                ::tracing_core::field::FieldSet::new(
                                    &["message"],
                                    ::tracing_core::callsite::Identifier(&__CALLSITE),
                                ),
                                ::tracing::metadata::Kind::EVENT,
                            )
                        };
                        ::tracing::callsite::DefaultCallsite::new(&META)
                    };
                    let enabled = ::tracing::Level::ERROR
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && ::tracing::Level::ERROR
                            <= ::tracing::level_filters::LevelFilter::current()
                        && {
                            let interest = __CALLSITE.interest();
                            !interest.is_never()
                                && ::tracing::__macro_support::__is_enabled(
                                    __CALLSITE.metadata(),
                                    interest,
                                )
                        };
                    if enabled {
                        (|value_set: ::tracing::field::ValueSet| {
                            let meta = __CALLSITE.metadata();
                            ::tracing::Event::dispatch(meta, &value_set);
                        })({
                            #[allow(unused_imports)]
                            use ::tracing::field::{debug, display, Value};
                            let mut iter = __CALLSITE.metadata().fields().iter();
                            __CALLSITE
                                .metadata()
                                .fields()
                                .value_set(
                                    &[
                                        (
                                            &::tracing::__macro_support::Iterator::next(&mut iter)
                                                .expect("FieldSet corrupted (this is a bug)"),
                                            ::tracing::__macro_support::Option::Some(
                                                &format_args!("{0}", err_msg) as &dyn Value,
                                            ),
                                        ),
                                    ],
                                )
                        });
                    } else {
                    }
                };
                Err(
                    ::anyhow::__private::must_use({
                        use ::anyhow::__private::kind::*;
                        let error = match err_msg {
                            error => (&error).anyhow_kind().new(error),
                        };
                        error
                    }),
                )
            }
        }
    }
    async fn send_osc_message(
        &self,
        address: &str,
        args: Option<Vec<osc::Type>>,
    ) -> Result<()> {
        let osc_sender_clone = Arc::clone(&self.osc_sender);
        let owned_address = address.to_string();
        tokio::task::spawn_blocking(move || {
                let sender_guard = osc_sender_clone.blocking_lock();
                let msg_args = args.unwrap_or_default();
                let msg = osc::Message {
                    addr: owned_address,
                    args: msg_args,
                };
                sender_guard
                    .send(msg)
                    .map_err(|e| ::anyhow::Error::msg(
                        ::alloc::__export::must_use({
                            ::alloc::fmt::format(
                                format_args!("Failed to send OSC message: {0}", e),
                            )
                        }),
                    ))
            })
            .await??;
        Ok(())
    }
    fn transport_play_tool_tool_attr() -> rmcp::model::Tool {
        rmcp::model::Tool {
            name: "transport_play".into(),
            description: "Starts Ardour playback.".into(),
            input_schema: rmcp::handler::server::tool::cached_schema_for_type::<
                rmcp::model::EmptyObject,
            >()
                .into(),
        }
    }
    async fn transport_play_tool_tool_call(
        context: rmcp::handler::server::tool::ToolCallContext<'_, Self>,
    ) -> std::result::Result<rmcp::model::CallToolResult, rmcp::Error> {
        use rmcp::handler::server::tool::*;
        let (__rmcp_tool_receiver, context) = <&Self>::from_tool_call_context_part(
            context,
        )?;
        Self::transport_play_tool(__rmcp_tool_receiver).await.into_call_tool_result()
    }
    async fn transport_play_tool(&self) -> Result<CallToolResult, McpError> {
        {
            use ::tracing::__macro_support::Callsite as _;
            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                static META: ::tracing::Metadata<'static> = {
                    ::tracing_core::metadata::Metadata::new(
                        "event src/main.rs:231",
                        "ardour_mcp_server",
                        ::tracing::Level::INFO,
                        ::tracing_core::__macro_support::Option::Some("src/main.rs"),
                        ::tracing_core::__macro_support::Option::Some(231u32),
                        ::tracing_core::__macro_support::Option::Some(
                            "ardour_mcp_server",
                        ),
                        ::tracing_core::field::FieldSet::new(
                            &["message"],
                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                        ),
                        ::tracing::metadata::Kind::EVENT,
                    )
                };
                ::tracing::callsite::DefaultCallsite::new(&META)
            };
            let enabled = ::tracing::Level::INFO
                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                && ::tracing::Level::INFO
                    <= ::tracing::level_filters::LevelFilter::current()
                && {
                    let interest = __CALLSITE.interest();
                    !interest.is_never()
                        && ::tracing::__macro_support::__is_enabled(
                            __CALLSITE.metadata(),
                            interest,
                        )
                };
            if enabled {
                (|value_set: ::tracing::field::ValueSet| {
                    let meta = __CALLSITE.metadata();
                    ::tracing::Event::dispatch(meta, &value_set);
                })({
                    #[allow(unused_imports)]
                    use ::tracing::field::{debug, display, Value};
                    let mut iter = __CALLSITE.metadata().fields().iter();
                    __CALLSITE
                        .metadata()
                        .fields()
                        .value_set(
                            &[
                                (
                                    &::tracing::__macro_support::Iterator::next(&mut iter)
                                        .expect("FieldSet corrupted (this is a bug)"),
                                    ::tracing::__macro_support::Option::Some(
                                        &format_args!("Executing transport_play_tool") as &dyn Value,
                                    ),
                                ),
                            ],
                        )
                });
            } else {
            }
        };
        match self.send_osc_message("/transport_play", None).await {
            Ok(_) => {
                Ok(
                    CallToolResult::success(
                        <[_]>::into_vec(
                            ::alloc::boxed::box_new([Content::text("Playback started")]),
                        ),
                    ),
                )
            }
            Err(e) => {
                Ok(
                    CallToolResult::error(
                        <[_]>::into_vec(
                            ::alloc::boxed::box_new([
                                Content::text(
                                    ::alloc::__export::must_use({
                                        ::alloc::fmt::format(format_args!("OSC send error: {0}", e))
                                    }),
                                ),
                            ]),
                        ),
                    ),
                )
            }
        }
    }
    fn transport_stop_tool_tool_attr() -> rmcp::model::Tool {
        rmcp::model::Tool {
            name: "transport_stop".into(),
            description: "Stops Ardour playback.".into(),
            input_schema: rmcp::handler::server::tool::cached_schema_for_type::<
                rmcp::model::EmptyObject,
            >()
                .into(),
        }
    }
    async fn transport_stop_tool_tool_call(
        context: rmcp::handler::server::tool::ToolCallContext<'_, Self>,
    ) -> std::result::Result<rmcp::model::CallToolResult, rmcp::Error> {
        use rmcp::handler::server::tool::*;
        let (__rmcp_tool_receiver, context) = <&Self>::from_tool_call_context_part(
            context,
        )?;
        Self::transport_stop_tool(__rmcp_tool_receiver).await.into_call_tool_result()
    }
    async fn transport_stop_tool(&self) -> Result<CallToolResult, McpError> {
        {
            use ::tracing::__macro_support::Callsite as _;
            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                static META: ::tracing::Metadata<'static> = {
                    ::tracing_core::metadata::Metadata::new(
                        "event src/main.rs:240",
                        "ardour_mcp_server",
                        ::tracing::Level::INFO,
                        ::tracing_core::__macro_support::Option::Some("src/main.rs"),
                        ::tracing_core::__macro_support::Option::Some(240u32),
                        ::tracing_core::__macro_support::Option::Some(
                            "ardour_mcp_server",
                        ),
                        ::tracing_core::field::FieldSet::new(
                            &["message"],
                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                        ),
                        ::tracing::metadata::Kind::EVENT,
                    )
                };
                ::tracing::callsite::DefaultCallsite::new(&META)
            };
            let enabled = ::tracing::Level::INFO
                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                && ::tracing::Level::INFO
                    <= ::tracing::level_filters::LevelFilter::current()
                && {
                    let interest = __CALLSITE.interest();
                    !interest.is_never()
                        && ::tracing::__macro_support::__is_enabled(
                            __CALLSITE.metadata(),
                            interest,
                        )
                };
            if enabled {
                (|value_set: ::tracing::field::ValueSet| {
                    let meta = __CALLSITE.metadata();
                    ::tracing::Event::dispatch(meta, &value_set);
                })({
                    #[allow(unused_imports)]
                    use ::tracing::field::{debug, display, Value};
                    let mut iter = __CALLSITE.metadata().fields().iter();
                    __CALLSITE
                        .metadata()
                        .fields()
                        .value_set(
                            &[
                                (
                                    &::tracing::__macro_support::Iterator::next(&mut iter)
                                        .expect("FieldSet corrupted (this is a bug)"),
                                    ::tracing::__macro_support::Option::Some(
                                        &format_args!("Executing transport_stop_tool") as &dyn Value,
                                    ),
                                ),
                            ],
                        )
                });
            } else {
            }
        };
        match self.send_osc_message("/transport_stop", None).await {
            Ok(_) => {
                Ok(
                    CallToolResult::success(
                        <[_]>::into_vec(
                            ::alloc::boxed::box_new([Content::text("Playback stopped")]),
                        ),
                    ),
                )
            }
            Err(e) => {
                Ok(
                    CallToolResult::error(
                        <[_]>::into_vec(
                            ::alloc::boxed::box_new([
                                Content::text(
                                    ::alloc::__export::must_use({
                                        ::alloc::fmt::format(format_args!("OSC send error: {0}", e))
                                    }),
                                ),
                            ]),
                        ),
                    ),
                )
            }
        }
    }
    fn goto_start_tool_tool_attr() -> rmcp::model::Tool {
        rmcp::model::Tool {
            name: "goto_start".into(),
            description: "Moves the playhead to the session start.".into(),
            input_schema: rmcp::handler::server::tool::cached_schema_for_type::<
                rmcp::model::EmptyObject,
            >()
                .into(),
        }
    }
    async fn goto_start_tool_tool_call(
        context: rmcp::handler::server::tool::ToolCallContext<'_, Self>,
    ) -> std::result::Result<rmcp::model::CallToolResult, rmcp::Error> {
        use rmcp::handler::server::tool::*;
        let (__rmcp_tool_receiver, context) = <&Self>::from_tool_call_context_part(
            context,
        )?;
        Self::goto_start_tool(__rmcp_tool_receiver).await.into_call_tool_result()
    }
    async fn goto_start_tool(&self) -> Result<CallToolResult, McpError> {
        {
            use ::tracing::__macro_support::Callsite as _;
            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                static META: ::tracing::Metadata<'static> = {
                    ::tracing_core::metadata::Metadata::new(
                        "event src/main.rs:249",
                        "ardour_mcp_server",
                        ::tracing::Level::INFO,
                        ::tracing_core::__macro_support::Option::Some("src/main.rs"),
                        ::tracing_core::__macro_support::Option::Some(249u32),
                        ::tracing_core::__macro_support::Option::Some(
                            "ardour_mcp_server",
                        ),
                        ::tracing_core::field::FieldSet::new(
                            &["message"],
                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                        ),
                        ::tracing::metadata::Kind::EVENT,
                    )
                };
                ::tracing::callsite::DefaultCallsite::new(&META)
            };
            let enabled = ::tracing::Level::INFO
                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                && ::tracing::Level::INFO
                    <= ::tracing::level_filters::LevelFilter::current()
                && {
                    let interest = __CALLSITE.interest();
                    !interest.is_never()
                        && ::tracing::__macro_support::__is_enabled(
                            __CALLSITE.metadata(),
                            interest,
                        )
                };
            if enabled {
                (|value_set: ::tracing::field::ValueSet| {
                    let meta = __CALLSITE.metadata();
                    ::tracing::Event::dispatch(meta, &value_set);
                })({
                    #[allow(unused_imports)]
                    use ::tracing::field::{debug, display, Value};
                    let mut iter = __CALLSITE.metadata().fields().iter();
                    __CALLSITE
                        .metadata()
                        .fields()
                        .value_set(
                            &[
                                (
                                    &::tracing::__macro_support::Iterator::next(&mut iter)
                                        .expect("FieldSet corrupted (this is a bug)"),
                                    ::tracing::__macro_support::Option::Some(
                                        &format_args!("Executing goto_start_tool") as &dyn Value,
                                    ),
                                ),
                            ],
                        )
                });
            } else {
            }
        };
        match self.send_osc_message("/goto_start", None).await {
            Ok(_) => {
                Ok(
                    CallToolResult::success(
                        <[_]>::into_vec(
                            ::alloc::boxed::box_new([
                                Content::text("Playhead moved to start"),
                            ]),
                        ),
                    ),
                )
            }
            Err(e) => {
                Ok(
                    CallToolResult::error(
                        <[_]>::into_vec(
                            ::alloc::boxed::box_new([
                                Content::text(
                                    ::alloc::__export::must_use({
                                        ::alloc::fmt::format(format_args!("OSC send error: {0}", e))
                                    }),
                                ),
                            ]),
                        ),
                    ),
                )
            }
        }
    }
    fn goto_end_tool_tool_attr() -> rmcp::model::Tool {
        rmcp::model::Tool {
            name: "goto_end".into(),
            description: "Moves the playhead to the session end.".into(),
            input_schema: rmcp::handler::server::tool::cached_schema_for_type::<
                rmcp::model::EmptyObject,
            >()
                .into(),
        }
    }
    async fn goto_end_tool_tool_call(
        context: rmcp::handler::server::tool::ToolCallContext<'_, Self>,
    ) -> std::result::Result<rmcp::model::CallToolResult, rmcp::Error> {
        use rmcp::handler::server::tool::*;
        let (__rmcp_tool_receiver, context) = <&Self>::from_tool_call_context_part(
            context,
        )?;
        Self::goto_end_tool(__rmcp_tool_receiver).await.into_call_tool_result()
    }
    async fn goto_end_tool(&self) -> Result<CallToolResult, McpError> {
        {
            use ::tracing::__macro_support::Callsite as _;
            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                static META: ::tracing::Metadata<'static> = {
                    ::tracing_core::metadata::Metadata::new(
                        "event src/main.rs:258",
                        "ardour_mcp_server",
                        ::tracing::Level::INFO,
                        ::tracing_core::__macro_support::Option::Some("src/main.rs"),
                        ::tracing_core::__macro_support::Option::Some(258u32),
                        ::tracing_core::__macro_support::Option::Some(
                            "ardour_mcp_server",
                        ),
                        ::tracing_core::field::FieldSet::new(
                            &["message"],
                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                        ),
                        ::tracing::metadata::Kind::EVENT,
                    )
                };
                ::tracing::callsite::DefaultCallsite::new(&META)
            };
            let enabled = ::tracing::Level::INFO
                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                && ::tracing::Level::INFO
                    <= ::tracing::level_filters::LevelFilter::current()
                && {
                    let interest = __CALLSITE.interest();
                    !interest.is_never()
                        && ::tracing::__macro_support::__is_enabled(
                            __CALLSITE.metadata(),
                            interest,
                        )
                };
            if enabled {
                (|value_set: ::tracing::field::ValueSet| {
                    let meta = __CALLSITE.metadata();
                    ::tracing::Event::dispatch(meta, &value_set);
                })({
                    #[allow(unused_imports)]
                    use ::tracing::field::{debug, display, Value};
                    let mut iter = __CALLSITE.metadata().fields().iter();
                    __CALLSITE
                        .metadata()
                        .fields()
                        .value_set(
                            &[
                                (
                                    &::tracing::__macro_support::Iterator::next(&mut iter)
                                        .expect("FieldSet corrupted (this is a bug)"),
                                    ::tracing::__macro_support::Option::Some(
                                        &format_args!("Executing goto_end_tool") as &dyn Value,
                                    ),
                                ),
                            ],
                        )
                });
            } else {
            }
        };
        match self.send_osc_message("/goto_end", None).await {
            Ok(_) => {
                Ok(
                    CallToolResult::success(
                        <[_]>::into_vec(
                            ::alloc::boxed::box_new([
                                Content::text("Playhead moved to end"),
                            ]),
                        ),
                    ),
                )
            }
            Err(e) => {
                Ok(
                    CallToolResult::error(
                        <[_]>::into_vec(
                            ::alloc::boxed::box_new([
                                Content::text(
                                    ::alloc::__export::must_use({
                                        ::alloc::fmt::format(format_args!("OSC send error: {0}", e))
                                    }),
                                ),
                            ]),
                        ),
                    ),
                )
            }
        }
    }
    fn loop_toggle_tool_tool_attr() -> rmcp::model::Tool {
        rmcp::model::Tool {
            name: "loop_toggle".into(),
            description: "Toggles loop playback mode.".into(),
            input_schema: rmcp::handler::server::tool::cached_schema_for_type::<
                rmcp::model::EmptyObject,
            >()
                .into(),
        }
    }
    async fn loop_toggle_tool_tool_call(
        context: rmcp::handler::server::tool::ToolCallContext<'_, Self>,
    ) -> std::result::Result<rmcp::model::CallToolResult, rmcp::Error> {
        use rmcp::handler::server::tool::*;
        let (__rmcp_tool_receiver, context) = <&Self>::from_tool_call_context_part(
            context,
        )?;
        Self::loop_toggle_tool(__rmcp_tool_receiver).await.into_call_tool_result()
    }
    async fn loop_toggle_tool(&self) -> Result<CallToolResult, McpError> {
        {
            use ::tracing::__macro_support::Callsite as _;
            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                static META: ::tracing::Metadata<'static> = {
                    ::tracing_core::metadata::Metadata::new(
                        "event src/main.rs:267",
                        "ardour_mcp_server",
                        ::tracing::Level::INFO,
                        ::tracing_core::__macro_support::Option::Some("src/main.rs"),
                        ::tracing_core::__macro_support::Option::Some(267u32),
                        ::tracing_core::__macro_support::Option::Some(
                            "ardour_mcp_server",
                        ),
                        ::tracing_core::field::FieldSet::new(
                            &["message"],
                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                        ),
                        ::tracing::metadata::Kind::EVENT,
                    )
                };
                ::tracing::callsite::DefaultCallsite::new(&META)
            };
            let enabled = ::tracing::Level::INFO
                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                && ::tracing::Level::INFO
                    <= ::tracing::level_filters::LevelFilter::current()
                && {
                    let interest = __CALLSITE.interest();
                    !interest.is_never()
                        && ::tracing::__macro_support::__is_enabled(
                            __CALLSITE.metadata(),
                            interest,
                        )
                };
            if enabled {
                (|value_set: ::tracing::field::ValueSet| {
                    let meta = __CALLSITE.metadata();
                    ::tracing::Event::dispatch(meta, &value_set);
                })({
                    #[allow(unused_imports)]
                    use ::tracing::field::{debug, display, Value};
                    let mut iter = __CALLSITE.metadata().fields().iter();
                    __CALLSITE
                        .metadata()
                        .fields()
                        .value_set(
                            &[
                                (
                                    &::tracing::__macro_support::Iterator::next(&mut iter)
                                        .expect("FieldSet corrupted (this is a bug)"),
                                    ::tracing::__macro_support::Option::Some(
                                        &format_args!("Executing loop_toggle_tool") as &dyn Value,
                                    ),
                                ),
                            ],
                        )
                });
            } else {
            }
        };
        match self.send_osc_message("/loop_toggle", None).await {
            Ok(_) => {
                Ok(
                    CallToolResult::success(
                        <[_]>::into_vec(
                            ::alloc::boxed::box_new([Content::text("Loop mode toggled")]),
                        ),
                    ),
                )
            }
            Err(e) => {
                Ok(
                    CallToolResult::error(
                        <[_]>::into_vec(
                            ::alloc::boxed::box_new([
                                Content::text(
                                    ::alloc::__export::must_use({
                                        ::alloc::fmt::format(format_args!("OSC send error: {0}", e))
                                    }),
                                ),
                            ]),
                        ),
                    ),
                )
            }
        }
    }
    fn undo_tool_tool_attr() -> rmcp::model::Tool {
        rmcp::model::Tool {
            name: "undo".into(),
            description: "Undoes the last action.".into(),
            input_schema: rmcp::handler::server::tool::cached_schema_for_type::<
                rmcp::model::EmptyObject,
            >()
                .into(),
        }
    }
    async fn undo_tool_tool_call(
        context: rmcp::handler::server::tool::ToolCallContext<'_, Self>,
    ) -> std::result::Result<rmcp::model::CallToolResult, rmcp::Error> {
        use rmcp::handler::server::tool::*;
        let (__rmcp_tool_receiver, context) = <&Self>::from_tool_call_context_part(
            context,
        )?;
        Self::undo_tool(__rmcp_tool_receiver).await.into_call_tool_result()
    }
    async fn undo_tool(&self) -> Result<CallToolResult, McpError> {
        {
            use ::tracing::__macro_support::Callsite as _;
            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                static META: ::tracing::Metadata<'static> = {
                    ::tracing_core::metadata::Metadata::new(
                        "event src/main.rs:276",
                        "ardour_mcp_server",
                        ::tracing::Level::INFO,
                        ::tracing_core::__macro_support::Option::Some("src/main.rs"),
                        ::tracing_core::__macro_support::Option::Some(276u32),
                        ::tracing_core::__macro_support::Option::Some(
                            "ardour_mcp_server",
                        ),
                        ::tracing_core::field::FieldSet::new(
                            &["message"],
                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                        ),
                        ::tracing::metadata::Kind::EVENT,
                    )
                };
                ::tracing::callsite::DefaultCallsite::new(&META)
            };
            let enabled = ::tracing::Level::INFO
                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                && ::tracing::Level::INFO
                    <= ::tracing::level_filters::LevelFilter::current()
                && {
                    let interest = __CALLSITE.interest();
                    !interest.is_never()
                        && ::tracing::__macro_support::__is_enabled(
                            __CALLSITE.metadata(),
                            interest,
                        )
                };
            if enabled {
                (|value_set: ::tracing::field::ValueSet| {
                    let meta = __CALLSITE.metadata();
                    ::tracing::Event::dispatch(meta, &value_set);
                })({
                    #[allow(unused_imports)]
                    use ::tracing::field::{debug, display, Value};
                    let mut iter = __CALLSITE.metadata().fields().iter();
                    __CALLSITE
                        .metadata()
                        .fields()
                        .value_set(
                            &[
                                (
                                    &::tracing::__macro_support::Iterator::next(&mut iter)
                                        .expect("FieldSet corrupted (this is a bug)"),
                                    ::tracing::__macro_support::Option::Some(
                                        &format_args!("Executing undo_tool") as &dyn Value,
                                    ),
                                ),
                            ],
                        )
                });
            } else {
            }
        };
        match self.send_osc_message("/undo", None).await {
            Ok(_) => {
                Ok(
                    CallToolResult::success(
                        <[_]>::into_vec(
                            ::alloc::boxed::box_new([
                                Content::text("Undo action performed"),
                            ]),
                        ),
                    ),
                )
            }
            Err(e) => {
                Ok(
                    CallToolResult::error(
                        <[_]>::into_vec(
                            ::alloc::boxed::box_new([
                                Content::text(
                                    ::alloc::__export::must_use({
                                        ::alloc::fmt::format(format_args!("OSC send error: {0}", e))
                                    }),
                                ),
                            ]),
                        ),
                    ),
                )
            }
        }
    }
    fn redo_tool_tool_attr() -> rmcp::model::Tool {
        rmcp::model::Tool {
            name: "redo".into(),
            description: "Redoes the last undone action.".into(),
            input_schema: rmcp::handler::server::tool::cached_schema_for_type::<
                rmcp::model::EmptyObject,
            >()
                .into(),
        }
    }
    async fn redo_tool_tool_call(
        context: rmcp::handler::server::tool::ToolCallContext<'_, Self>,
    ) -> std::result::Result<rmcp::model::CallToolResult, rmcp::Error> {
        use rmcp::handler::server::tool::*;
        let (__rmcp_tool_receiver, context) = <&Self>::from_tool_call_context_part(
            context,
        )?;
        Self::redo_tool(__rmcp_tool_receiver).await.into_call_tool_result()
    }
    async fn redo_tool(&self) -> Result<CallToolResult, McpError> {
        {
            use ::tracing::__macro_support::Callsite as _;
            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                static META: ::tracing::Metadata<'static> = {
                    ::tracing_core::metadata::Metadata::new(
                        "event src/main.rs:285",
                        "ardour_mcp_server",
                        ::tracing::Level::INFO,
                        ::tracing_core::__macro_support::Option::Some("src/main.rs"),
                        ::tracing_core::__macro_support::Option::Some(285u32),
                        ::tracing_core::__macro_support::Option::Some(
                            "ardour_mcp_server",
                        ),
                        ::tracing_core::field::FieldSet::new(
                            &["message"],
                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                        ),
                        ::tracing::metadata::Kind::EVENT,
                    )
                };
                ::tracing::callsite::DefaultCallsite::new(&META)
            };
            let enabled = ::tracing::Level::INFO
                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                && ::tracing::Level::INFO
                    <= ::tracing::level_filters::LevelFilter::current()
                && {
                    let interest = __CALLSITE.interest();
                    !interest.is_never()
                        && ::tracing::__macro_support::__is_enabled(
                            __CALLSITE.metadata(),
                            interest,
                        )
                };
            if enabled {
                (|value_set: ::tracing::field::ValueSet| {
                    let meta = __CALLSITE.metadata();
                    ::tracing::Event::dispatch(meta, &value_set);
                })({
                    #[allow(unused_imports)]
                    use ::tracing::field::{debug, display, Value};
                    let mut iter = __CALLSITE.metadata().fields().iter();
                    __CALLSITE
                        .metadata()
                        .fields()
                        .value_set(
                            &[
                                (
                                    &::tracing::__macro_support::Iterator::next(&mut iter)
                                        .expect("FieldSet corrupted (this is a bug)"),
                                    ::tracing::__macro_support::Option::Some(
                                        &format_args!("Executing redo_tool") as &dyn Value,
                                    ),
                                ),
                            ],
                        )
                });
            } else {
            }
        };
        match self.send_osc_message("/redo", None).await {
            Ok(_) => {
                Ok(
                    CallToolResult::success(
                        <[_]>::into_vec(
                            ::alloc::boxed::box_new([
                                Content::text("Redo action performed"),
                            ]),
                        ),
                    ),
                )
            }
            Err(e) => {
                Ok(
                    CallToolResult::error(
                        <[_]>::into_vec(
                            ::alloc::boxed::box_new([
                                Content::text(
                                    ::alloc::__export::must_use({
                                        ::alloc::fmt::format(format_args!("OSC send error: {0}", e))
                                    }),
                                ),
                            ]),
                        ),
                    ),
                )
            }
        }
    }
    fn toggle_punch_in_tool_tool_attr() -> rmcp::model::Tool {
        rmcp::model::Tool {
            name: "toggle_punch_in".into(),
            description: "Toggles the Punch In state.".into(),
            input_schema: rmcp::handler::server::tool::cached_schema_for_type::<
                rmcp::model::EmptyObject,
            >()
                .into(),
        }
    }
    async fn toggle_punch_in_tool_tool_call(
        context: rmcp::handler::server::tool::ToolCallContext<'_, Self>,
    ) -> std::result::Result<rmcp::model::CallToolResult, rmcp::Error> {
        use rmcp::handler::server::tool::*;
        let (__rmcp_tool_receiver, context) = <&Self>::from_tool_call_context_part(
            context,
        )?;
        Self::toggle_punch_in_tool(__rmcp_tool_receiver).await.into_call_tool_result()
    }
    async fn toggle_punch_in_tool(&self) -> Result<CallToolResult, McpError> {
        {
            use ::tracing::__macro_support::Callsite as _;
            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                static META: ::tracing::Metadata<'static> = {
                    ::tracing_core::metadata::Metadata::new(
                        "event src/main.rs:294",
                        "ardour_mcp_server",
                        ::tracing::Level::INFO,
                        ::tracing_core::__macro_support::Option::Some("src/main.rs"),
                        ::tracing_core::__macro_support::Option::Some(294u32),
                        ::tracing_core::__macro_support::Option::Some(
                            "ardour_mcp_server",
                        ),
                        ::tracing_core::field::FieldSet::new(
                            &["message"],
                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                        ),
                        ::tracing::metadata::Kind::EVENT,
                    )
                };
                ::tracing::callsite::DefaultCallsite::new(&META)
            };
            let enabled = ::tracing::Level::INFO
                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                && ::tracing::Level::INFO
                    <= ::tracing::level_filters::LevelFilter::current()
                && {
                    let interest = __CALLSITE.interest();
                    !interest.is_never()
                        && ::tracing::__macro_support::__is_enabled(
                            __CALLSITE.metadata(),
                            interest,
                        )
                };
            if enabled {
                (|value_set: ::tracing::field::ValueSet| {
                    let meta = __CALLSITE.metadata();
                    ::tracing::Event::dispatch(meta, &value_set);
                })({
                    #[allow(unused_imports)]
                    use ::tracing::field::{debug, display, Value};
                    let mut iter = __CALLSITE.metadata().fields().iter();
                    __CALLSITE
                        .metadata()
                        .fields()
                        .value_set(
                            &[
                                (
                                    &::tracing::__macro_support::Iterator::next(&mut iter)
                                        .expect("FieldSet corrupted (this is a bug)"),
                                    ::tracing::__macro_support::Option::Some(
                                        &format_args!("Executing toggle_punch_in_tool")
                                            as &dyn Value,
                                    ),
                                ),
                            ],
                        )
                });
            } else {
            }
        };
        match self.send_osc_message("/toggle_punch_in", None).await {
            Ok(_) => {
                Ok(
                    CallToolResult::success(
                        <[_]>::into_vec(
                            ::alloc::boxed::box_new([Content::text("Punch In toggled")]),
                        ),
                    ),
                )
            }
            Err(e) => {
                Ok(
                    CallToolResult::error(
                        <[_]>::into_vec(
                            ::alloc::boxed::box_new([
                                Content::text(
                                    ::alloc::__export::must_use({
                                        ::alloc::fmt::format(format_args!("OSC send error: {0}", e))
                                    }),
                                ),
                            ]),
                        ),
                    ),
                )
            }
        }
    }
    fn toggle_punch_out_tool_tool_attr() -> rmcp::model::Tool {
        rmcp::model::Tool {
            name: "toggle_punch_out".into(),
            description: "Toggles the Punch Out state.".into(),
            input_schema: rmcp::handler::server::tool::cached_schema_for_type::<
                rmcp::model::EmptyObject,
            >()
                .into(),
        }
    }
    async fn toggle_punch_out_tool_tool_call(
        context: rmcp::handler::server::tool::ToolCallContext<'_, Self>,
    ) -> std::result::Result<rmcp::model::CallToolResult, rmcp::Error> {
        use rmcp::handler::server::tool::*;
        let (__rmcp_tool_receiver, context) = <&Self>::from_tool_call_context_part(
            context,
        )?;
        Self::toggle_punch_out_tool(__rmcp_tool_receiver).await.into_call_tool_result()
    }
    async fn toggle_punch_out_tool(&self) -> Result<CallToolResult, McpError> {
        {
            use ::tracing::__macro_support::Callsite as _;
            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                static META: ::tracing::Metadata<'static> = {
                    ::tracing_core::metadata::Metadata::new(
                        "event src/main.rs:303",
                        "ardour_mcp_server",
                        ::tracing::Level::INFO,
                        ::tracing_core::__macro_support::Option::Some("src/main.rs"),
                        ::tracing_core::__macro_support::Option::Some(303u32),
                        ::tracing_core::__macro_support::Option::Some(
                            "ardour_mcp_server",
                        ),
                        ::tracing_core::field::FieldSet::new(
                            &["message"],
                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                        ),
                        ::tracing::metadata::Kind::EVENT,
                    )
                };
                ::tracing::callsite::DefaultCallsite::new(&META)
            };
            let enabled = ::tracing::Level::INFO
                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                && ::tracing::Level::INFO
                    <= ::tracing::level_filters::LevelFilter::current()
                && {
                    let interest = __CALLSITE.interest();
                    !interest.is_never()
                        && ::tracing::__macro_support::__is_enabled(
                            __CALLSITE.metadata(),
                            interest,
                        )
                };
            if enabled {
                (|value_set: ::tracing::field::ValueSet| {
                    let meta = __CALLSITE.metadata();
                    ::tracing::Event::dispatch(meta, &value_set);
                })({
                    #[allow(unused_imports)]
                    use ::tracing::field::{debug, display, Value};
                    let mut iter = __CALLSITE.metadata().fields().iter();
                    __CALLSITE
                        .metadata()
                        .fields()
                        .value_set(
                            &[
                                (
                                    &::tracing::__macro_support::Iterator::next(&mut iter)
                                        .expect("FieldSet corrupted (this is a bug)"),
                                    ::tracing::__macro_support::Option::Some(
                                        &format_args!("Executing toggle_punch_out_tool")
                                            as &dyn Value,
                                    ),
                                ),
                            ],
                        )
                });
            } else {
            }
        };
        match self.send_osc_message("/toggle_punch_out", None).await {
            Ok(_) => {
                Ok(
                    CallToolResult::success(
                        <[_]>::into_vec(
                            ::alloc::boxed::box_new([Content::text("Punch Out toggled")]),
                        ),
                    ),
                )
            }
            Err(e) => {
                Ok(
                    CallToolResult::error(
                        <[_]>::into_vec(
                            ::alloc::boxed::box_new([
                                Content::text(
                                    ::alloc::__export::must_use({
                                        ::alloc::fmt::format(format_args!("OSC send error: {0}", e))
                                    }),
                                ),
                            ]),
                        ),
                    ),
                )
            }
        }
    }
    fn rec_enable_toggle_tool_tool_attr() -> rmcp::model::Tool {
        rmcp::model::Tool {
            name: "rec_enable_toggle".into(),
            description: "Toggles the master record enable or selected track record enable."
                .into(),
            input_schema: rmcp::handler::server::tool::cached_schema_for_type::<
                rmcp::model::EmptyObject,
            >()
                .into(),
        }
    }
    async fn rec_enable_toggle_tool_tool_call(
        context: rmcp::handler::server::tool::ToolCallContext<'_, Self>,
    ) -> std::result::Result<rmcp::model::CallToolResult, rmcp::Error> {
        use rmcp::handler::server::tool::*;
        let (__rmcp_tool_receiver, context) = <&Self>::from_tool_call_context_part(
            context,
        )?;
        Self::rec_enable_toggle_tool(__rmcp_tool_receiver).await.into_call_tool_result()
    }
    async fn rec_enable_toggle_tool(&self) -> Result<CallToolResult, McpError> {
        {
            use ::tracing::__macro_support::Callsite as _;
            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                static META: ::tracing::Metadata<'static> = {
                    ::tracing_core::metadata::Metadata::new(
                        "event src/main.rs:312",
                        "ardour_mcp_server",
                        ::tracing::Level::INFO,
                        ::tracing_core::__macro_support::Option::Some("src/main.rs"),
                        ::tracing_core::__macro_support::Option::Some(312u32),
                        ::tracing_core::__macro_support::Option::Some(
                            "ardour_mcp_server",
                        ),
                        ::tracing_core::field::FieldSet::new(
                            &["message"],
                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                        ),
                        ::tracing::metadata::Kind::EVENT,
                    )
                };
                ::tracing::callsite::DefaultCallsite::new(&META)
            };
            let enabled = ::tracing::Level::INFO
                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                && ::tracing::Level::INFO
                    <= ::tracing::level_filters::LevelFilter::current()
                && {
                    let interest = __CALLSITE.interest();
                    !interest.is_never()
                        && ::tracing::__macro_support::__is_enabled(
                            __CALLSITE.metadata(),
                            interest,
                        )
                };
            if enabled {
                (|value_set: ::tracing::field::ValueSet| {
                    let meta = __CALLSITE.metadata();
                    ::tracing::Event::dispatch(meta, &value_set);
                })({
                    #[allow(unused_imports)]
                    use ::tracing::field::{debug, display, Value};
                    let mut iter = __CALLSITE.metadata().fields().iter();
                    __CALLSITE
                        .metadata()
                        .fields()
                        .value_set(
                            &[
                                (
                                    &::tracing::__macro_support::Iterator::next(&mut iter)
                                        .expect("FieldSet corrupted (this is a bug)"),
                                    ::tracing::__macro_support::Option::Some(
                                        &format_args!("Executing rec_enable_toggle_tool")
                                            as &dyn Value,
                                    ),
                                ),
                            ],
                        )
                });
            } else {
            }
        };
        match self.send_osc_message("/rec_enable_toggle", None).await {
            Ok(_) => {
                Ok(
                    CallToolResult::success(
                        <[_]>::into_vec(
                            ::alloc::boxed::box_new([
                                Content::text("Record Enable toggled"),
                            ]),
                        ),
                    ),
                )
            }
            Err(e) => {
                Ok(
                    CallToolResult::error(
                        <[_]>::into_vec(
                            ::alloc::boxed::box_new([
                                Content::text(
                                    ::alloc::__export::must_use({
                                        ::alloc::fmt::format(format_args!("OSC send error: {0}", e))
                                    }),
                                ),
                            ]),
                        ),
                    ),
                )
            }
        }
    }
    fn toggle_all_rec_enables_tool_tool_attr() -> rmcp::model::Tool {
        rmcp::model::Tool {
            name: "toggle_all_rec_enables".into(),
            description: "Toggles the record enable state for ALL tracks.".into(),
            input_schema: rmcp::handler::server::tool::cached_schema_for_type::<
                rmcp::model::EmptyObject,
            >()
                .into(),
        }
    }
    async fn toggle_all_rec_enables_tool_tool_call(
        context: rmcp::handler::server::tool::ToolCallContext<'_, Self>,
    ) -> std::result::Result<rmcp::model::CallToolResult, rmcp::Error> {
        use rmcp::handler::server::tool::*;
        let (__rmcp_tool_receiver, context) = <&Self>::from_tool_call_context_part(
            context,
        )?;
        Self::toggle_all_rec_enables_tool(__rmcp_tool_receiver)
            .await
            .into_call_tool_result()
    }
    async fn toggle_all_rec_enables_tool(&self) -> Result<CallToolResult, McpError> {
        {
            use ::tracing::__macro_support::Callsite as _;
            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                static META: ::tracing::Metadata<'static> = {
                    ::tracing_core::metadata::Metadata::new(
                        "event src/main.rs:321",
                        "ardour_mcp_server",
                        ::tracing::Level::INFO,
                        ::tracing_core::__macro_support::Option::Some("src/main.rs"),
                        ::tracing_core::__macro_support::Option::Some(321u32),
                        ::tracing_core::__macro_support::Option::Some(
                            "ardour_mcp_server",
                        ),
                        ::tracing_core::field::FieldSet::new(
                            &["message"],
                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                        ),
                        ::tracing::metadata::Kind::EVENT,
                    )
                };
                ::tracing::callsite::DefaultCallsite::new(&META)
            };
            let enabled = ::tracing::Level::INFO
                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                && ::tracing::Level::INFO
                    <= ::tracing::level_filters::LevelFilter::current()
                && {
                    let interest = __CALLSITE.interest();
                    !interest.is_never()
                        && ::tracing::__macro_support::__is_enabled(
                            __CALLSITE.metadata(),
                            interest,
                        )
                };
            if enabled {
                (|value_set: ::tracing::field::ValueSet| {
                    let meta = __CALLSITE.metadata();
                    ::tracing::Event::dispatch(meta, &value_set);
                })({
                    #[allow(unused_imports)]
                    use ::tracing::field::{debug, display, Value};
                    let mut iter = __CALLSITE.metadata().fields().iter();
                    __CALLSITE
                        .metadata()
                        .fields()
                        .value_set(
                            &[
                                (
                                    &::tracing::__macro_support::Iterator::next(&mut iter)
                                        .expect("FieldSet corrupted (this is a bug)"),
                                    ::tracing::__macro_support::Option::Some(
                                        &format_args!("Executing toggle_all_rec_enables_tool")
                                            as &dyn Value,
                                    ),
                                ),
                            ],
                        )
                });
            } else {
            }
        };
        match self.send_osc_message("/toggle_all_rec_enables", None).await {
            Ok(_) => {
                Ok(
                    CallToolResult::success(
                        <[_]>::into_vec(
                            ::alloc::boxed::box_new([
                                Content::text("All Record Enables toggled"),
                            ]),
                        ),
                    ),
                )
            }
            Err(e) => {
                Ok(
                    CallToolResult::error(
                        <[_]>::into_vec(
                            ::alloc::boxed::box_new([
                                Content::text(
                                    ::alloc::__export::must_use({
                                        ::alloc::fmt::format(format_args!("OSC send error: {0}", e))
                                    }),
                                ),
                            ]),
                        ),
                    ),
                )
            }
        }
    }
    fn ffwd_tool_tool_attr() -> rmcp::model::Tool {
        rmcp::model::Tool {
            name: "ffwd".into(),
            description: "Fast forwards the transport.".into(),
            input_schema: rmcp::handler::server::tool::cached_schema_for_type::<
                rmcp::model::EmptyObject,
            >()
                .into(),
        }
    }
    async fn ffwd_tool_tool_call(
        context: rmcp::handler::server::tool::ToolCallContext<'_, Self>,
    ) -> std::result::Result<rmcp::model::CallToolResult, rmcp::Error> {
        use rmcp::handler::server::tool::*;
        let (__rmcp_tool_receiver, context) = <&Self>::from_tool_call_context_part(
            context,
        )?;
        Self::ffwd_tool(__rmcp_tool_receiver).await.into_call_tool_result()
    }
    async fn ffwd_tool(&self) -> Result<CallToolResult, McpError> {
        {
            use ::tracing::__macro_support::Callsite as _;
            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                static META: ::tracing::Metadata<'static> = {
                    ::tracing_core::metadata::Metadata::new(
                        "event src/main.rs:330",
                        "ardour_mcp_server",
                        ::tracing::Level::INFO,
                        ::tracing_core::__macro_support::Option::Some("src/main.rs"),
                        ::tracing_core::__macro_support::Option::Some(330u32),
                        ::tracing_core::__macro_support::Option::Some(
                            "ardour_mcp_server",
                        ),
                        ::tracing_core::field::FieldSet::new(
                            &["message"],
                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                        ),
                        ::tracing::metadata::Kind::EVENT,
                    )
                };
                ::tracing::callsite::DefaultCallsite::new(&META)
            };
            let enabled = ::tracing::Level::INFO
                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                && ::tracing::Level::INFO
                    <= ::tracing::level_filters::LevelFilter::current()
                && {
                    let interest = __CALLSITE.interest();
                    !interest.is_never()
                        && ::tracing::__macro_support::__is_enabled(
                            __CALLSITE.metadata(),
                            interest,
                        )
                };
            if enabled {
                (|value_set: ::tracing::field::ValueSet| {
                    let meta = __CALLSITE.metadata();
                    ::tracing::Event::dispatch(meta, &value_set);
                })({
                    #[allow(unused_imports)]
                    use ::tracing::field::{debug, display, Value};
                    let mut iter = __CALLSITE.metadata().fields().iter();
                    __CALLSITE
                        .metadata()
                        .fields()
                        .value_set(
                            &[
                                (
                                    &::tracing::__macro_support::Iterator::next(&mut iter)
                                        .expect("FieldSet corrupted (this is a bug)"),
                                    ::tracing::__macro_support::Option::Some(
                                        &format_args!("Executing ffwd_tool") as &dyn Value,
                                    ),
                                ),
                            ],
                        )
                });
            } else {
            }
        };
        match self.send_osc_message("/ffwd", None).await {
            Ok(_) => {
                Ok(
                    CallToolResult::success(
                        <[_]>::into_vec(
                            ::alloc::boxed::box_new([
                                Content::text("Fast Forward activated"),
                            ]),
                        ),
                    ),
                )
            }
            Err(e) => {
                Ok(
                    CallToolResult::error(
                        <[_]>::into_vec(
                            ::alloc::boxed::box_new([
                                Content::text(
                                    ::alloc::__export::must_use({
                                        ::alloc::fmt::format(format_args!("OSC send error: {0}", e))
                                    }),
                                ),
                            ]),
                        ),
                    ),
                )
            }
        }
    }
    fn rewind_tool_tool_attr() -> rmcp::model::Tool {
        rmcp::model::Tool {
            name: "rewind".into(),
            description: "Rewinds the transport.".into(),
            input_schema: rmcp::handler::server::tool::cached_schema_for_type::<
                rmcp::model::EmptyObject,
            >()
                .into(),
        }
    }
    async fn rewind_tool_tool_call(
        context: rmcp::handler::server::tool::ToolCallContext<'_, Self>,
    ) -> std::result::Result<rmcp::model::CallToolResult, rmcp::Error> {
        use rmcp::handler::server::tool::*;
        let (__rmcp_tool_receiver, context) = <&Self>::from_tool_call_context_part(
            context,
        )?;
        Self::rewind_tool(__rmcp_tool_receiver).await.into_call_tool_result()
    }
    async fn rewind_tool(&self) -> Result<CallToolResult, McpError> {
        {
            use ::tracing::__macro_support::Callsite as _;
            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                static META: ::tracing::Metadata<'static> = {
                    ::tracing_core::metadata::Metadata::new(
                        "event src/main.rs:339",
                        "ardour_mcp_server",
                        ::tracing::Level::INFO,
                        ::tracing_core::__macro_support::Option::Some("src/main.rs"),
                        ::tracing_core::__macro_support::Option::Some(339u32),
                        ::tracing_core::__macro_support::Option::Some(
                            "ardour_mcp_server",
                        ),
                        ::tracing_core::field::FieldSet::new(
                            &["message"],
                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                        ),
                        ::tracing::metadata::Kind::EVENT,
                    )
                };
                ::tracing::callsite::DefaultCallsite::new(&META)
            };
            let enabled = ::tracing::Level::INFO
                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                && ::tracing::Level::INFO
                    <= ::tracing::level_filters::LevelFilter::current()
                && {
                    let interest = __CALLSITE.interest();
                    !interest.is_never()
                        && ::tracing::__macro_support::__is_enabled(
                            __CALLSITE.metadata(),
                            interest,
                        )
                };
            if enabled {
                (|value_set: ::tracing::field::ValueSet| {
                    let meta = __CALLSITE.metadata();
                    ::tracing::Event::dispatch(meta, &value_set);
                })({
                    #[allow(unused_imports)]
                    use ::tracing::field::{debug, display, Value};
                    let mut iter = __CALLSITE.metadata().fields().iter();
                    __CALLSITE
                        .metadata()
                        .fields()
                        .value_set(
                            &[
                                (
                                    &::tracing::__macro_support::Iterator::next(&mut iter)
                                        .expect("FieldSet corrupted (this is a bug)"),
                                    ::tracing::__macro_support::Option::Some(
                                        &format_args!("Executing rewind_tool") as &dyn Value,
                                    ),
                                ),
                            ],
                        )
                });
            } else {
            }
        };
        match self.send_osc_message("/rewind", None).await {
            Ok(_) => {
                Ok(
                    CallToolResult::success(
                        <[_]>::into_vec(
                            ::alloc::boxed::box_new([Content::text("Rewind activated")]),
                        ),
                    ),
                )
            }
            Err(e) => {
                Ok(
                    CallToolResult::error(
                        <[_]>::into_vec(
                            ::alloc::boxed::box_new([
                                Content::text(
                                    ::alloc::__export::must_use({
                                        ::alloc::fmt::format(format_args!("OSC send error: {0}", e))
                                    }),
                                ),
                            ]),
                        ),
                    ),
                )
            }
        }
    }
    fn add_marker_tool_tool_attr() -> rmcp::model::Tool {
        rmcp::model::Tool {
            name: "add_marker".into(),
            description: "Adds a location marker at the current playhead position."
                .into(),
            input_schema: rmcp::handler::server::tool::cached_schema_for_type::<
                rmcp::model::EmptyObject,
            >()
                .into(),
        }
    }
    async fn add_marker_tool_tool_call(
        context: rmcp::handler::server::tool::ToolCallContext<'_, Self>,
    ) -> std::result::Result<rmcp::model::CallToolResult, rmcp::Error> {
        use rmcp::handler::server::tool::*;
        let (__rmcp_tool_receiver, context) = <&Self>::from_tool_call_context_part(
            context,
        )?;
        Self::add_marker_tool(__rmcp_tool_receiver).await.into_call_tool_result()
    }
    async fn add_marker_tool(&self) -> Result<CallToolResult, McpError> {
        {
            use ::tracing::__macro_support::Callsite as _;
            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                static META: ::tracing::Metadata<'static> = {
                    ::tracing_core::metadata::Metadata::new(
                        "event src/main.rs:348",
                        "ardour_mcp_server",
                        ::tracing::Level::INFO,
                        ::tracing_core::__macro_support::Option::Some("src/main.rs"),
                        ::tracing_core::__macro_support::Option::Some(348u32),
                        ::tracing_core::__macro_support::Option::Some(
                            "ardour_mcp_server",
                        ),
                        ::tracing_core::field::FieldSet::new(
                            &["message"],
                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                        ),
                        ::tracing::metadata::Kind::EVENT,
                    )
                };
                ::tracing::callsite::DefaultCallsite::new(&META)
            };
            let enabled = ::tracing::Level::INFO
                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                && ::tracing::Level::INFO
                    <= ::tracing::level_filters::LevelFilter::current()
                && {
                    let interest = __CALLSITE.interest();
                    !interest.is_never()
                        && ::tracing::__macro_support::__is_enabled(
                            __CALLSITE.metadata(),
                            interest,
                        )
                };
            if enabled {
                (|value_set: ::tracing::field::ValueSet| {
                    let meta = __CALLSITE.metadata();
                    ::tracing::Event::dispatch(meta, &value_set);
                })({
                    #[allow(unused_imports)]
                    use ::tracing::field::{debug, display, Value};
                    let mut iter = __CALLSITE.metadata().fields().iter();
                    __CALLSITE
                        .metadata()
                        .fields()
                        .value_set(
                            &[
                                (
                                    &::tracing::__macro_support::Iterator::next(&mut iter)
                                        .expect("FieldSet corrupted (this is a bug)"),
                                    ::tracing::__macro_support::Option::Some(
                                        &format_args!("Executing add_marker_tool") as &dyn Value,
                                    ),
                                ),
                            ],
                        )
                });
            } else {
            }
        };
        match self.send_osc_message("/add_marker", None).await {
            Ok(_) => {
                Ok(
                    CallToolResult::success(
                        <[_]>::into_vec(
                            ::alloc::boxed::box_new([Content::text("Marker added")]),
                        ),
                    ),
                )
            }
            Err(e) => {
                Ok(
                    CallToolResult::error(
                        <[_]>::into_vec(
                            ::alloc::boxed::box_new([
                                Content::text(
                                    ::alloc::__export::must_use({
                                        ::alloc::fmt::format(format_args!("OSC send error: {0}", e))
                                    }),
                                ),
                            ]),
                        ),
                    ),
                )
            }
        }
    }
    fn next_marker_tool_tool_attr() -> rmcp::model::Tool {
        rmcp::model::Tool {
            name: "next_marker".into(),
            description: "Moves the playhead to the next location marker.".into(),
            input_schema: rmcp::handler::server::tool::cached_schema_for_type::<
                rmcp::model::EmptyObject,
            >()
                .into(),
        }
    }
    async fn next_marker_tool_tool_call(
        context: rmcp::handler::server::tool::ToolCallContext<'_, Self>,
    ) -> std::result::Result<rmcp::model::CallToolResult, rmcp::Error> {
        use rmcp::handler::server::tool::*;
        let (__rmcp_tool_receiver, context) = <&Self>::from_tool_call_context_part(
            context,
        )?;
        Self::next_marker_tool(__rmcp_tool_receiver).await.into_call_tool_result()
    }
    async fn next_marker_tool(&self) -> Result<CallToolResult, McpError> {
        {
            use ::tracing::__macro_support::Callsite as _;
            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                static META: ::tracing::Metadata<'static> = {
                    ::tracing_core::metadata::Metadata::new(
                        "event src/main.rs:357",
                        "ardour_mcp_server",
                        ::tracing::Level::INFO,
                        ::tracing_core::__macro_support::Option::Some("src/main.rs"),
                        ::tracing_core::__macro_support::Option::Some(357u32),
                        ::tracing_core::__macro_support::Option::Some(
                            "ardour_mcp_server",
                        ),
                        ::tracing_core::field::FieldSet::new(
                            &["message"],
                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                        ),
                        ::tracing::metadata::Kind::EVENT,
                    )
                };
                ::tracing::callsite::DefaultCallsite::new(&META)
            };
            let enabled = ::tracing::Level::INFO
                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                && ::tracing::Level::INFO
                    <= ::tracing::level_filters::LevelFilter::current()
                && {
                    let interest = __CALLSITE.interest();
                    !interest.is_never()
                        && ::tracing::__macro_support::__is_enabled(
                            __CALLSITE.metadata(),
                            interest,
                        )
                };
            if enabled {
                (|value_set: ::tracing::field::ValueSet| {
                    let meta = __CALLSITE.metadata();
                    ::tracing::Event::dispatch(meta, &value_set);
                })({
                    #[allow(unused_imports)]
                    use ::tracing::field::{debug, display, Value};
                    let mut iter = __CALLSITE.metadata().fields().iter();
                    __CALLSITE
                        .metadata()
                        .fields()
                        .value_set(
                            &[
                                (
                                    &::tracing::__macro_support::Iterator::next(&mut iter)
                                        .expect("FieldSet corrupted (this is a bug)"),
                                    ::tracing::__macro_support::Option::Some(
                                        &format_args!("Executing next_marker_tool") as &dyn Value,
                                    ),
                                ),
                            ],
                        )
                });
            } else {
            }
        };
        match self.send_osc_message("/next_marker", None).await {
            Ok(_) => {
                Ok(
                    CallToolResult::success(
                        <[_]>::into_vec(
                            ::alloc::boxed::box_new([
                                Content::text("Moved to next marker"),
                            ]),
                        ),
                    ),
                )
            }
            Err(e) => {
                Ok(
                    CallToolResult::error(
                        <[_]>::into_vec(
                            ::alloc::boxed::box_new([
                                Content::text(
                                    ::alloc::__export::must_use({
                                        ::alloc::fmt::format(format_args!("OSC send error: {0}", e))
                                    }),
                                ),
                            ]),
                        ),
                    ),
                )
            }
        }
    }
    fn prev_marker_tool_tool_attr() -> rmcp::model::Tool {
        rmcp::model::Tool {
            name: "prev_marker".into(),
            description: "Moves the playhead to the previous location marker.".into(),
            input_schema: rmcp::handler::server::tool::cached_schema_for_type::<
                rmcp::model::EmptyObject,
            >()
                .into(),
        }
    }
    async fn prev_marker_tool_tool_call(
        context: rmcp::handler::server::tool::ToolCallContext<'_, Self>,
    ) -> std::result::Result<rmcp::model::CallToolResult, rmcp::Error> {
        use rmcp::handler::server::tool::*;
        let (__rmcp_tool_receiver, context) = <&Self>::from_tool_call_context_part(
            context,
        )?;
        Self::prev_marker_tool(__rmcp_tool_receiver).await.into_call_tool_result()
    }
    async fn prev_marker_tool(&self) -> Result<CallToolResult, McpError> {
        {
            use ::tracing::__macro_support::Callsite as _;
            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                static META: ::tracing::Metadata<'static> = {
                    ::tracing_core::metadata::Metadata::new(
                        "event src/main.rs:366",
                        "ardour_mcp_server",
                        ::tracing::Level::INFO,
                        ::tracing_core::__macro_support::Option::Some("src/main.rs"),
                        ::tracing_core::__macro_support::Option::Some(366u32),
                        ::tracing_core::__macro_support::Option::Some(
                            "ardour_mcp_server",
                        ),
                        ::tracing_core::field::FieldSet::new(
                            &["message"],
                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                        ),
                        ::tracing::metadata::Kind::EVENT,
                    )
                };
                ::tracing::callsite::DefaultCallsite::new(&META)
            };
            let enabled = ::tracing::Level::INFO
                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                && ::tracing::Level::INFO
                    <= ::tracing::level_filters::LevelFilter::current()
                && {
                    let interest = __CALLSITE.interest();
                    !interest.is_never()
                        && ::tracing::__macro_support::__is_enabled(
                            __CALLSITE.metadata(),
                            interest,
                        )
                };
            if enabled {
                (|value_set: ::tracing::field::ValueSet| {
                    let meta = __CALLSITE.metadata();
                    ::tracing::Event::dispatch(meta, &value_set);
                })({
                    #[allow(unused_imports)]
                    use ::tracing::field::{debug, display, Value};
                    let mut iter = __CALLSITE.metadata().fields().iter();
                    __CALLSITE
                        .metadata()
                        .fields()
                        .value_set(
                            &[
                                (
                                    &::tracing::__macro_support::Iterator::next(&mut iter)
                                        .expect("FieldSet corrupted (this is a bug)"),
                                    ::tracing::__macro_support::Option::Some(
                                        &format_args!("Executing prev_marker_tool") as &dyn Value,
                                    ),
                                ),
                            ],
                        )
                });
            } else {
            }
        };
        match self.send_osc_message("/prev_marker", None).await {
            Ok(_) => {
                Ok(
                    CallToolResult::success(
                        <[_]>::into_vec(
                            ::alloc::boxed::box_new([
                                Content::text("Moved to previous marker"),
                            ]),
                        ),
                    ),
                )
            }
            Err(e) => {
                Ok(
                    CallToolResult::error(
                        <[_]>::into_vec(
                            ::alloc::boxed::box_new([
                                Content::text(
                                    ::alloc::__export::must_use({
                                        ::alloc::fmt::format(format_args!("OSC send error: {0}", e))
                                    }),
                                ),
                            ]),
                        ),
                    ),
                )
            }
        }
    }
    fn save_state_tool_tool_attr() -> rmcp::model::Tool {
        rmcp::model::Tool {
            name: "save_state".into(),
            description: "Saves the current session state.".into(),
            input_schema: rmcp::handler::server::tool::cached_schema_for_type::<
                rmcp::model::EmptyObject,
            >()
                .into(),
        }
    }
    async fn save_state_tool_tool_call(
        context: rmcp::handler::server::tool::ToolCallContext<'_, Self>,
    ) -> std::result::Result<rmcp::model::CallToolResult, rmcp::Error> {
        use rmcp::handler::server::tool::*;
        let (__rmcp_tool_receiver, context) = <&Self>::from_tool_call_context_part(
            context,
        )?;
        Self::save_state_tool(__rmcp_tool_receiver).await.into_call_tool_result()
    }
    async fn save_state_tool(&self) -> Result<CallToolResult, McpError> {
        {
            use ::tracing::__macro_support::Callsite as _;
            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                static META: ::tracing::Metadata<'static> = {
                    ::tracing_core::metadata::Metadata::new(
                        "event src/main.rs:375",
                        "ardour_mcp_server",
                        ::tracing::Level::INFO,
                        ::tracing_core::__macro_support::Option::Some("src/main.rs"),
                        ::tracing_core::__macro_support::Option::Some(375u32),
                        ::tracing_core::__macro_support::Option::Some(
                            "ardour_mcp_server",
                        ),
                        ::tracing_core::field::FieldSet::new(
                            &["message"],
                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                        ),
                        ::tracing::metadata::Kind::EVENT,
                    )
                };
                ::tracing::callsite::DefaultCallsite::new(&META)
            };
            let enabled = ::tracing::Level::INFO
                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                && ::tracing::Level::INFO
                    <= ::tracing::level_filters::LevelFilter::current()
                && {
                    let interest = __CALLSITE.interest();
                    !interest.is_never()
                        && ::tracing::__macro_support::__is_enabled(
                            __CALLSITE.metadata(),
                            interest,
                        )
                };
            if enabled {
                (|value_set: ::tracing::field::ValueSet| {
                    let meta = __CALLSITE.metadata();
                    ::tracing::Event::dispatch(meta, &value_set);
                })({
                    #[allow(unused_imports)]
                    use ::tracing::field::{debug, display, Value};
                    let mut iter = __CALLSITE.metadata().fields().iter();
                    __CALLSITE
                        .metadata()
                        .fields()
                        .value_set(
                            &[
                                (
                                    &::tracing::__macro_support::Iterator::next(&mut iter)
                                        .expect("FieldSet corrupted (this is a bug)"),
                                    ::tracing::__macro_support::Option::Some(
                                        &format_args!("Executing save_state_tool") as &dyn Value,
                                    ),
                                ),
                            ],
                        )
                });
            } else {
            }
        };
        match self.send_osc_message("/save_state", None).await {
            Ok(_) => {
                Ok(
                    CallToolResult::success(
                        <[_]>::into_vec(
                            ::alloc::boxed::box_new([
                                Content::text("Session state saved"),
                            ]),
                        ),
                    ),
                )
            }
            Err(e) => {
                Ok(
                    CallToolResult::error(
                        <[_]>::into_vec(
                            ::alloc::boxed::box_new([
                                Content::text(
                                    ::alloc::__export::must_use({
                                        ::alloc::fmt::format(format_args!("OSC send error: {0}", e))
                                    }),
                                ),
                            ]),
                        ),
                    ),
                )
            }
        }
    }
    fn set_track_mute_tool_tool_attr() -> rmcp::model::Tool {
        rmcp::model::Tool {
            name: "set_track_mute".into(),
            description: "Sets the mute state of a specific track.".into(),
            input_schema: rmcp::handler::server::tool::cached_schema_for_type::<
                SetTrackMuteArgs,
            >()
                .into(),
        }
    }
    async fn set_track_mute_tool_tool_call(
        context: rmcp::handler::server::tool::ToolCallContext<'_, Self>,
    ) -> std::result::Result<rmcp::model::CallToolResult, rmcp::Error> {
        use rmcp::handler::server::tool::*;
        let (__rmcp_tool_receiver, context) = <&Self>::from_tool_call_context_part(
            context,
        )?;
        let (Parameters(args), context) = <Parameters<
            SetTrackMuteArgs,
        >>::from_tool_call_context_part(context)?;
        Self::set_track_mute_tool(__rmcp_tool_receiver, args)
            .await
            .into_call_tool_result()
    }
    async fn set_track_mute_tool(
        &self,
        args: SetTrackMuteArgs,
    ) -> Result<CallToolResult, McpError> {
        {
            use ::tracing::__macro_support::Callsite as _;
            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                static META: ::tracing::Metadata<'static> = {
                    ::tracing_core::metadata::Metadata::new(
                        "event src/main.rs:388",
                        "ardour_mcp_server",
                        ::tracing::Level::INFO,
                        ::tracing_core::__macro_support::Option::Some("src/main.rs"),
                        ::tracing_core::__macro_support::Option::Some(388u32),
                        ::tracing_core::__macro_support::Option::Some(
                            "ardour_mcp_server",
                        ),
                        ::tracing_core::field::FieldSet::new(
                            &["message"],
                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                        ),
                        ::tracing::metadata::Kind::EVENT,
                    )
                };
                ::tracing::callsite::DefaultCallsite::new(&META)
            };
            let enabled = ::tracing::Level::INFO
                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                && ::tracing::Level::INFO
                    <= ::tracing::level_filters::LevelFilter::current()
                && {
                    let interest = __CALLSITE.interest();
                    !interest.is_never()
                        && ::tracing::__macro_support::__is_enabled(
                            __CALLSITE.metadata(),
                            interest,
                        )
                };
            if enabled {
                (|value_set: ::tracing::field::ValueSet| {
                    let meta = __CALLSITE.metadata();
                    ::tracing::Event::dispatch(meta, &value_set);
                })({
                    #[allow(unused_imports)]
                    use ::tracing::field::{debug, display, Value};
                    let mut iter = __CALLSITE.metadata().fields().iter();
                    __CALLSITE
                        .metadata()
                        .fields()
                        .value_set(
                            &[
                                (
                                    &::tracing::__macro_support::Iterator::next(&mut iter)
                                        .expect("FieldSet corrupted (this is a bug)"),
                                    ::tracing::__macro_support::Option::Some(
                                        &format_args!(
                                            "Executing set_track_mute_tool with args: {0:?}",
                                            args,
                                        ) as &dyn Value,
                                    ),
                                ),
                            ],
                        )
                });
            } else {
            }
        };
        let osc_mute_state = if args.mute_state { 1i32 } else { 0i32 };
        let osc_args = <[_]>::into_vec(
            ::alloc::boxed::box_new([
                osc::Type::Int(args.rid),
                osc::Type::Int(osc_mute_state),
            ]),
        );
        match self.send_osc_message("/strip/mute", Some(osc_args)).await {
            Ok(_) => {
                Ok(
                    CallToolResult::success(
                        <[_]>::into_vec(
                            ::alloc::boxed::box_new([
                                Content::text(
                                    ::alloc::__export::must_use({
                                        ::alloc::fmt::format(
                                            format_args!(
                                                "Track {0} mute state set to {1}",
                                                args.rid,
                                                args.mute_state,
                                            ),
                                        )
                                    }),
                                ),
                            ]),
                        ),
                    ),
                )
            }
            Err(e) => {
                Ok(
                    CallToolResult::error(
                        <[_]>::into_vec(
                            ::alloc::boxed::box_new([
                                Content::text(
                                    ::alloc::__export::must_use({
                                        ::alloc::fmt::format(
                                            format_args!("OSC send error for /strip/mute: {0}", e),
                                        )
                                    }),
                                ),
                            ]),
                        ),
                    ),
                )
            }
        }
    }
    fn set_transport_speed_tool_tool_attr() -> rmcp::model::Tool {
        rmcp::model::Tool {
            name: "set_transport_speed".into(),
            description: "Sets Ardour's transport speed. Valid range: -8.0 to 8.0."
                .into(),
            input_schema: rmcp::handler::server::tool::cached_schema_for_type::<
                SetTransportSpeedArgs,
            >()
                .into(),
        }
    }
    async fn set_transport_speed_tool_tool_call(
        context: rmcp::handler::server::tool::ToolCallContext<'_, Self>,
    ) -> std::result::Result<rmcp::model::CallToolResult, rmcp::Error> {
        use rmcp::handler::server::tool::*;
        let (__rmcp_tool_receiver, context) = <&Self>::from_tool_call_context_part(
            context,
        )?;
        let (Parameters(args), context) = <Parameters<
            SetTransportSpeedArgs,
        >>::from_tool_call_context_part(context)?;
        Self::set_transport_speed_tool(__rmcp_tool_receiver, args)
            .await
            .into_call_tool_result()
    }
    async fn set_transport_speed_tool(
        &self,
        args: SetTransportSpeedArgs,
    ) -> Result<CallToolResult, McpError> {
        {
            use ::tracing::__macro_support::Callsite as _;
            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                static META: ::tracing::Metadata<'static> = {
                    ::tracing_core::metadata::Metadata::new(
                        "event src/main.rs:411",
                        "ardour_mcp_server",
                        ::tracing::Level::INFO,
                        ::tracing_core::__macro_support::Option::Some("src/main.rs"),
                        ::tracing_core::__macro_support::Option::Some(411u32),
                        ::tracing_core::__macro_support::Option::Some(
                            "ardour_mcp_server",
                        ),
                        ::tracing_core::field::FieldSet::new(
                            &["message"],
                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                        ),
                        ::tracing::metadata::Kind::EVENT,
                    )
                };
                ::tracing::callsite::DefaultCallsite::new(&META)
            };
            let enabled = ::tracing::Level::INFO
                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                && ::tracing::Level::INFO
                    <= ::tracing::level_filters::LevelFilter::current()
                && {
                    let interest = __CALLSITE.interest();
                    !interest.is_never()
                        && ::tracing::__macro_support::__is_enabled(
                            __CALLSITE.metadata(),
                            interest,
                        )
                };
            if enabled {
                (|value_set: ::tracing::field::ValueSet| {
                    let meta = __CALLSITE.metadata();
                    ::tracing::Event::dispatch(meta, &value_set);
                })({
                    #[allow(unused_imports)]
                    use ::tracing::field::{debug, display, Value};
                    let mut iter = __CALLSITE.metadata().fields().iter();
                    __CALLSITE
                        .metadata()
                        .fields()
                        .value_set(
                            &[
                                (
                                    &::tracing::__macro_support::Iterator::next(&mut iter)
                                        .expect("FieldSet corrupted (this is a bug)"),
                                    ::tracing::__macro_support::Option::Some(
                                        &format_args!(
                                            "Executing set_transport_speed_tool with speed: {0}",
                                            args.speed,
                                        ) as &dyn Value,
                                    ),
                                ),
                            ],
                        )
                });
            } else {
            }
        };
        if args.speed < -8.0 || args.speed > 8.0 {
            {
                use ::tracing::__macro_support::Callsite as _;
                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                    static META: ::tracing::Metadata<'static> = {
                        ::tracing_core::metadata::Metadata::new(
                            "event src/main.rs:417",
                            "ardour_mcp_server",
                            ::tracing::Level::WARN,
                            ::tracing_core::__macro_support::Option::Some("src/main.rs"),
                            ::tracing_core::__macro_support::Option::Some(417u32),
                            ::tracing_core::__macro_support::Option::Some(
                                "ardour_mcp_server",
                            ),
                            ::tracing_core::field::FieldSet::new(
                                &["message"],
                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                            ),
                            ::tracing::metadata::Kind::EVENT,
                        )
                    };
                    ::tracing::callsite::DefaultCallsite::new(&META)
                };
                let enabled = ::tracing::Level::WARN
                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                    && ::tracing::Level::WARN
                        <= ::tracing::level_filters::LevelFilter::current()
                    && {
                        let interest = __CALLSITE.interest();
                        !interest.is_never()
                            && ::tracing::__macro_support::__is_enabled(
                                __CALLSITE.metadata(),
                                interest,
                            )
                    };
                if enabled {
                    (|value_set: ::tracing::field::ValueSet| {
                        let meta = __CALLSITE.metadata();
                        ::tracing::Event::dispatch(meta, &value_set);
                    })({
                        #[allow(unused_imports)]
                        use ::tracing::field::{debug, display, Value};
                        let mut iter = __CALLSITE.metadata().fields().iter();
                        __CALLSITE
                            .metadata()
                            .fields()
                            .value_set(
                                &[
                                    (
                                        &::tracing::__macro_support::Iterator::next(&mut iter)
                                            .expect("FieldSet corrupted (this is a bug)"),
                                        ::tracing::__macro_support::Option::Some(
                                            &format_args!(
                                                "Invalid transport speed: {0}. Must be between -8.0 and 8.0.",
                                                args.speed,
                                            ) as &dyn Value,
                                        ),
                                    ),
                                ],
                            )
                    });
                } else {
                }
            };
            return Ok(
                CallToolResult::error(
                    <[_]>::into_vec(
                        ::alloc::boxed::box_new([
                            Content::text(
                                ::alloc::__export::must_use({
                                    ::alloc::fmt::format(
                                        format_args!(
                                            "Invalid transport speed: {0}. Must be between -8.0 and 8.0.",
                                            args.speed,
                                        ),
                                    )
                                }),
                            ),
                        ]),
                    ),
                ),
            );
        }
        let osc_args = <[_]>::into_vec(
            ::alloc::boxed::box_new([osc::Type::Float(args.speed)]),
        );
        match self.send_osc_message("/set_transport_speed", Some(osc_args)).await {
            Ok(_) => {
                Ok(
                    CallToolResult::success(
                        <[_]>::into_vec(
                            ::alloc::boxed::box_new([
                                Content::text(
                                    ::alloc::__export::must_use({
                                        ::alloc::fmt::format(
                                            format_args!("Transport speed set to {0}", args.speed),
                                        )
                                    }),
                                ),
                            ]),
                        ),
                    ),
                )
            }
            Err(e) => {
                Ok(
                    CallToolResult::error(
                        <[_]>::into_vec(
                            ::alloc::boxed::box_new([
                                Content::text(
                                    ::alloc::__export::must_use({
                                        ::alloc::fmt::format(
                                            format_args!(
                                                "OSC send error for /set_transport_speed: {0}",
                                                e,
                                            ),
                                        )
                                    }),
                                ),
                            ]),
                        ),
                    ),
                )
            }
        }
    }
    fn locate_tool_tool_attr() -> rmcp::model::Tool {
        rmcp::model::Tool {
            name: "locate".into(),
            description: "Locates the playhead to a specific sample position and optionally starts playback."
                .into(),
            input_schema: rmcp::handler::server::tool::cached_schema_for_type::<
                LocateToolArgs,
            >()
                .into(),
        }
    }
    async fn locate_tool_tool_call(
        context: rmcp::handler::server::tool::ToolCallContext<'_, Self>,
    ) -> std::result::Result<rmcp::model::CallToolResult, rmcp::Error> {
        use rmcp::handler::server::tool::*;
        let (__rmcp_tool_receiver, context) = <&Self>::from_tool_call_context_part(
            context,
        )?;
        let (Parameters(args), context) = <Parameters<
            LocateToolArgs,
        >>::from_tool_call_context_part(context)?;
        Self::locate_tool(__rmcp_tool_receiver, args).await.into_call_tool_result()
    }
    async fn locate_tool(
        &self,
        args: LocateToolArgs,
    ) -> Result<CallToolResult, McpError> {
        {
            use ::tracing::__macro_support::Callsite as _;
            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                static META: ::tracing::Metadata<'static> = {
                    ::tracing_core::metadata::Metadata::new(
                        "event src/main.rs:443",
                        "ardour_mcp_server",
                        ::tracing::Level::INFO,
                        ::tracing_core::__macro_support::Option::Some("src/main.rs"),
                        ::tracing_core::__macro_support::Option::Some(443u32),
                        ::tracing_core::__macro_support::Option::Some(
                            "ardour_mcp_server",
                        ),
                        ::tracing_core::field::FieldSet::new(
                            &["message"],
                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                        ),
                        ::tracing::metadata::Kind::EVENT,
                    )
                };
                ::tracing::callsite::DefaultCallsite::new(&META)
            };
            let enabled = ::tracing::Level::INFO
                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                && ::tracing::Level::INFO
                    <= ::tracing::level_filters::LevelFilter::current()
                && {
                    let interest = __CALLSITE.interest();
                    !interest.is_never()
                        && ::tracing::__macro_support::__is_enabled(
                            __CALLSITE.metadata(),
                            interest,
                        )
                };
            if enabled {
                (|value_set: ::tracing::field::ValueSet| {
                    let meta = __CALLSITE.metadata();
                    ::tracing::Event::dispatch(meta, &value_set);
                })({
                    #[allow(unused_imports)]
                    use ::tracing::field::{debug, display, Value};
                    let mut iter = __CALLSITE.metadata().fields().iter();
                    __CALLSITE
                        .metadata()
                        .fields()
                        .value_set(
                            &[
                                (
                                    &::tracing::__macro_support::Iterator::next(&mut iter)
                                        .expect("FieldSet corrupted (this is a bug)"),
                                    ::tracing::__macro_support::Option::Some(
                                        &format_args!(
                                            "Executing locate_tool with spos: {0}, roll: {1}",
                                            args.spos,
                                            args.roll,
                                        ) as &dyn Value,
                                    ),
                                ),
                            ],
                        )
                });
            } else {
            }
        };
        let osc_args = <[_]>::into_vec(
            ::alloc::boxed::box_new([
                osc::Type::Long(args.spos),
                osc::Type::Int(args.roll),
            ]),
        );
        match self.send_osc_message("/locate", Some(osc_args)).await {
            Ok(_) => {
                Ok(
                    CallToolResult::success(
                        <[_]>::into_vec(
                            ::alloc::boxed::box_new([
                                Content::text(
                                    ::alloc::__export::must_use({
                                        ::alloc::fmt::format(
                                            format_args!(
                                                "Located to sample {0} with roll state {1}",
                                                args.spos,
                                                args.roll,
                                            ),
                                        )
                                    }),
                                ),
                            ]),
                        ),
                    ),
                )
            }
            Err(e) => {
                Ok(
                    CallToolResult::error(
                        <[_]>::into_vec(
                            ::alloc::boxed::box_new([
                                Content::text(
                                    ::alloc::__export::must_use({
                                        ::alloc::fmt::format(
                                            format_args!("OSC send error for /locate: {0}", e),
                                        )
                                    }),
                                ),
                            ]),
                        ),
                    ),
                )
            }
        }
    }
    fn set_track_solo_tool_tool_attr() -> rmcp::model::Tool {
        rmcp::model::Tool {
            name: "set_track_solo".into(),
            description: "Sets the solo state of a specific track.".into(),
            input_schema: rmcp::handler::server::tool::cached_schema_for_type::<
                SetTrackSoloArgs,
            >()
                .into(),
        }
    }
    async fn set_track_solo_tool_tool_call(
        context: rmcp::handler::server::tool::ToolCallContext<'_, Self>,
    ) -> std::result::Result<rmcp::model::CallToolResult, rmcp::Error> {
        use rmcp::handler::server::tool::*;
        let (__rmcp_tool_receiver, context) = <&Self>::from_tool_call_context_part(
            context,
        )?;
        let (Parameters(args), context) = <Parameters<
            SetTrackSoloArgs,
        >>::from_tool_call_context_part(context)?;
        Self::set_track_solo_tool(__rmcp_tool_receiver, args)
            .await
            .into_call_tool_result()
    }
    async fn set_track_solo_tool(
        &self,
        args: SetTrackSoloArgs,
    ) -> Result<CallToolResult, McpError> {
        {
            use ::tracing::__macro_support::Callsite as _;
            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                static META: ::tracing::Metadata<'static> = {
                    ::tracing_core::metadata::Metadata::new(
                        "event src/main.rs:463",
                        "ardour_mcp_server",
                        ::tracing::Level::INFO,
                        ::tracing_core::__macro_support::Option::Some("src/main.rs"),
                        ::tracing_core::__macro_support::Option::Some(463u32),
                        ::tracing_core::__macro_support::Option::Some(
                            "ardour_mcp_server",
                        ),
                        ::tracing_core::field::FieldSet::new(
                            &["message"],
                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                        ),
                        ::tracing::metadata::Kind::EVENT,
                    )
                };
                ::tracing::callsite::DefaultCallsite::new(&META)
            };
            let enabled = ::tracing::Level::INFO
                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                && ::tracing::Level::INFO
                    <= ::tracing::level_filters::LevelFilter::current()
                && {
                    let interest = __CALLSITE.interest();
                    !interest.is_never()
                        && ::tracing::__macro_support::__is_enabled(
                            __CALLSITE.metadata(),
                            interest,
                        )
                };
            if enabled {
                (|value_set: ::tracing::field::ValueSet| {
                    let meta = __CALLSITE.metadata();
                    ::tracing::Event::dispatch(meta, &value_set);
                })({
                    #[allow(unused_imports)]
                    use ::tracing::field::{debug, display, Value};
                    let mut iter = __CALLSITE.metadata().fields().iter();
                    __CALLSITE
                        .metadata()
                        .fields()
                        .value_set(
                            &[
                                (
                                    &::tracing::__macro_support::Iterator::next(&mut iter)
                                        .expect("FieldSet corrupted (this is a bug)"),
                                    ::tracing::__macro_support::Option::Some(
                                        &format_args!(
                                            "Executing set_track_solo_tool for rid: {0}, solo_state: {1}",
                                            args.rid,
                                            args.solo_st,
                                        ) as &dyn Value,
                                    ),
                                ),
                            ],
                        )
                });
            } else {
            }
        };
        if !(args.solo_st == 0 || args.solo_st == 1) {
            {
                use ::tracing::__macro_support::Callsite as _;
                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                    static META: ::tracing::Metadata<'static> = {
                        ::tracing_core::metadata::Metadata::new(
                            "event src/main.rs:466",
                            "ardour_mcp_server",
                            ::tracing::Level::WARN,
                            ::tracing_core::__macro_support::Option::Some("src/main.rs"),
                            ::tracing_core::__macro_support::Option::Some(466u32),
                            ::tracing_core::__macro_support::Option::Some(
                                "ardour_mcp_server",
                            ),
                            ::tracing_core::field::FieldSet::new(
                                &["message"],
                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                            ),
                            ::tracing::metadata::Kind::EVENT,
                        )
                    };
                    ::tracing::callsite::DefaultCallsite::new(&META)
                };
                let enabled = ::tracing::Level::WARN
                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                    && ::tracing::Level::WARN
                        <= ::tracing::level_filters::LevelFilter::current()
                    && {
                        let interest = __CALLSITE.interest();
                        !interest.is_never()
                            && ::tracing::__macro_support::__is_enabled(
                                __CALLSITE.metadata(),
                                interest,
                            )
                    };
                if enabled {
                    (|value_set: ::tracing::field::ValueSet| {
                        let meta = __CALLSITE.metadata();
                        ::tracing::Event::dispatch(meta, &value_set);
                    })({
                        #[allow(unused_imports)]
                        use ::tracing::field::{debug, display, Value};
                        let mut iter = __CALLSITE.metadata().fields().iter();
                        __CALLSITE
                            .metadata()
                            .fields()
                            .value_set(
                                &[
                                    (
                                        &::tracing::__macro_support::Iterator::next(&mut iter)
                                            .expect("FieldSet corrupted (this is a bug)"),
                                        ::tracing::__macro_support::Option::Some(
                                            &format_args!(
                                                "Invalid solo_st value: {0}. Must be 0 or 1.",
                                                args.solo_st,
                                            ) as &dyn Value,
                                        ),
                                    ),
                                ],
                            )
                    });
                } else {
                }
            };
            return Ok(
                CallToolResult::error(
                    <[_]>::into_vec(
                        ::alloc::boxed::box_new([
                            Content::text(
                                ::alloc::__export::must_use({
                                    ::alloc::fmt::format(
                                        format_args!(
                                            "Invalid solo_st value: {0}. Must be 0 (off) or 1 (on).",
                                            args.solo_st,
                                        ),
                                    )
                                }),
                            ),
                        ]),
                    ),
                ),
            );
        }
        let osc_args = <[_]>::into_vec(
            ::alloc::boxed::box_new([
                osc::Type::Int(args.rid),
                osc::Type::Int(args.solo_st),
            ]),
        );
        let address = "/strip/solo";
        match self.send_osc_message(address, Some(osc_args)).await {
            Ok(_) => {
                Ok(
                    CallToolResult::success(
                        <[_]>::into_vec(
                            ::alloc::boxed::box_new([
                                Content::text(
                                    ::alloc::__export::must_use({
                                        ::alloc::fmt::format(
                                            format_args!(
                                                "Track {0} solo state set to {1}",
                                                args.rid,
                                                args.solo_st,
                                            ),
                                        )
                                    }),
                                ),
                            ]),
                        ),
                    ),
                )
            }
            Err(e) => {
                Ok(
                    CallToolResult::error(
                        <[_]>::into_vec(
                            ::alloc::boxed::box_new([
                                Content::text(
                                    ::alloc::__export::must_use({
                                        ::alloc::fmt::format(
                                            format_args!("OSC send error for {0}: {1}", address, e),
                                        )
                                    }),
                                ),
                            ]),
                        ),
                    ),
                )
            }
        }
    }
    fn set_track_rec_enable_tool_tool_attr() -> rmcp::model::Tool {
        rmcp::model::Tool {
            name: "set_track_rec_enable".into(),
            description: "Sets the record enable state of a specific track.".into(),
            input_schema: rmcp::handler::server::tool::cached_schema_for_type::<
                SetTrackRecEnableArgs,
            >()
                .into(),
        }
    }
    async fn set_track_rec_enable_tool_tool_call(
        context: rmcp::handler::server::tool::ToolCallContext<'_, Self>,
    ) -> std::result::Result<rmcp::model::CallToolResult, rmcp::Error> {
        use rmcp::handler::server::tool::*;
        let (__rmcp_tool_receiver, context) = <&Self>::from_tool_call_context_part(
            context,
        )?;
        let (Parameters(args), context) = <Parameters<
            SetTrackRecEnableArgs,
        >>::from_tool_call_context_part(context)?;
        Self::set_track_rec_enable_tool(__rmcp_tool_receiver, args)
            .await
            .into_call_tool_result()
    }
    async fn set_track_rec_enable_tool(
        &self,
        args: SetTrackRecEnableArgs,
    ) -> Result<CallToolResult, McpError> {
        {
            use ::tracing::__macro_support::Callsite as _;
            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                static META: ::tracing::Metadata<'static> = {
                    ::tracing_core::metadata::Metadata::new(
                        "event src/main.rs:494",
                        "ardour_mcp_server",
                        ::tracing::Level::INFO,
                        ::tracing_core::__macro_support::Option::Some("src/main.rs"),
                        ::tracing_core::__macro_support::Option::Some(494u32),
                        ::tracing_core::__macro_support::Option::Some(
                            "ardour_mcp_server",
                        ),
                        ::tracing_core::field::FieldSet::new(
                            &["message"],
                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                        ),
                        ::tracing::metadata::Kind::EVENT,
                    )
                };
                ::tracing::callsite::DefaultCallsite::new(&META)
            };
            let enabled = ::tracing::Level::INFO
                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                && ::tracing::Level::INFO
                    <= ::tracing::level_filters::LevelFilter::current()
                && {
                    let interest = __CALLSITE.interest();
                    !interest.is_never()
                        && ::tracing::__macro_support::__is_enabled(
                            __CALLSITE.metadata(),
                            interest,
                        )
                };
            if enabled {
                (|value_set: ::tracing::field::ValueSet| {
                    let meta = __CALLSITE.metadata();
                    ::tracing::Event::dispatch(meta, &value_set);
                })({
                    #[allow(unused_imports)]
                    use ::tracing::field::{debug, display, Value};
                    let mut iter = __CALLSITE.metadata().fields().iter();
                    __CALLSITE
                        .metadata()
                        .fields()
                        .value_set(
                            &[
                                (
                                    &::tracing::__macro_support::Iterator::next(&mut iter)
                                        .expect("FieldSet corrupted (this is a bug)"),
                                    ::tracing::__macro_support::Option::Some(
                                        &format_args!(
                                            "Executing set_track_rec_enable_tool for rid: {0}, rec_enable_state: {1}",
                                            args.rid,
                                            args.rec_st,
                                        ) as &dyn Value,
                                    ),
                                ),
                            ],
                        )
                });
            } else {
            }
        };
        if !(args.rec_st == 0 || args.rec_st == 1) {
            {
                use ::tracing::__macro_support::Callsite as _;
                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                    static META: ::tracing::Metadata<'static> = {
                        ::tracing_core::metadata::Metadata::new(
                            "event src/main.rs:497",
                            "ardour_mcp_server",
                            ::tracing::Level::WARN,
                            ::tracing_core::__macro_support::Option::Some("src/main.rs"),
                            ::tracing_core::__macro_support::Option::Some(497u32),
                            ::tracing_core::__macro_support::Option::Some(
                                "ardour_mcp_server",
                            ),
                            ::tracing_core::field::FieldSet::new(
                                &["message"],
                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                            ),
                            ::tracing::metadata::Kind::EVENT,
                        )
                    };
                    ::tracing::callsite::DefaultCallsite::new(&META)
                };
                let enabled = ::tracing::Level::WARN
                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                    && ::tracing::Level::WARN
                        <= ::tracing::level_filters::LevelFilter::current()
                    && {
                        let interest = __CALLSITE.interest();
                        !interest.is_never()
                            && ::tracing::__macro_support::__is_enabled(
                                __CALLSITE.metadata(),
                                interest,
                            )
                    };
                if enabled {
                    (|value_set: ::tracing::field::ValueSet| {
                        let meta = __CALLSITE.metadata();
                        ::tracing::Event::dispatch(meta, &value_set);
                    })({
                        #[allow(unused_imports)]
                        use ::tracing::field::{debug, display, Value};
                        let mut iter = __CALLSITE.metadata().fields().iter();
                        __CALLSITE
                            .metadata()
                            .fields()
                            .value_set(
                                &[
                                    (
                                        &::tracing::__macro_support::Iterator::next(&mut iter)
                                            .expect("FieldSet corrupted (this is a bug)"),
                                        ::tracing::__macro_support::Option::Some(
                                            &format_args!(
                                                "Invalid rec_st value: {0}. Must be 0 or 1.",
                                                args.rec_st,
                                            ) as &dyn Value,
                                        ),
                                    ),
                                ],
                            )
                    });
                } else {
                }
            };
            return Ok(
                CallToolResult::error(
                    <[_]>::into_vec(
                        ::alloc::boxed::box_new([
                            Content::text(
                                ::alloc::__export::must_use({
                                    ::alloc::fmt::format(
                                        format_args!(
                                            "Invalid rec_st value: {0}. Must be 0 (off) or 1 (on).",
                                            args.rec_st,
                                        ),
                                    )
                                }),
                            ),
                        ]),
                    ),
                ),
            );
        }
        let osc_args = <[_]>::into_vec(
            ::alloc::boxed::box_new([
                osc::Type::Int(args.rid),
                osc::Type::Int(args.rec_st),
            ]),
        );
        let address = "/strip/recenable";
        match self.send_osc_message(address, Some(osc_args)).await {
            Ok(_) => {
                Ok(
                    CallToolResult::success(
                        <[_]>::into_vec(
                            ::alloc::boxed::box_new([
                                Content::text(
                                    ::alloc::__export::must_use({
                                        ::alloc::fmt::format(
                                            format_args!(
                                                "Track {0} record enable state set to {1}",
                                                args.rid,
                                                args.rec_st,
                                            ),
                                        )
                                    }),
                                ),
                            ]),
                        ),
                    ),
                )
            }
            Err(e) => {
                Ok(
                    CallToolResult::error(
                        <[_]>::into_vec(
                            ::alloc::boxed::box_new([
                                Content::text(
                                    ::alloc::__export::must_use({
                                        ::alloc::fmt::format(
                                            format_args!("OSC send error for {0}: {1}", address, e),
                                        )
                                    }),
                                ),
                            ]),
                        ),
                    ),
                )
            }
        }
    }
    fn set_track_gain_abs_tool_tool_attr() -> rmcp::model::Tool {
        rmcp::model::Tool {
            name: "set_track_gain_abs".into(),
            description: "Sets the absolute gain of a specific track.".into(),
            input_schema: rmcp::handler::server::tool::cached_schema_for_type::<
                SetTrackGainAbsArgs,
            >()
                .into(),
        }
    }
    async fn set_track_gain_abs_tool_tool_call(
        context: rmcp::handler::server::tool::ToolCallContext<'_, Self>,
    ) -> std::result::Result<rmcp::model::CallToolResult, rmcp::Error> {
        use rmcp::handler::server::tool::*;
        let (__rmcp_tool_receiver, context) = <&Self>::from_tool_call_context_part(
            context,
        )?;
        let (Parameters(args), context) = <Parameters<
            SetTrackGainAbsArgs,
        >>::from_tool_call_context_part(context)?;
        Self::set_track_gain_abs_tool(__rmcp_tool_receiver, args)
            .await
            .into_call_tool_result()
    }
    async fn set_track_gain_abs_tool(
        &self,
        args: SetTrackGainAbsArgs,
    ) -> Result<CallToolResult, McpError> {
        {
            use ::tracing::__macro_support::Callsite as _;
            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                static META: ::tracing::Metadata<'static> = {
                    ::tracing_core::metadata::Metadata::new(
                        "event src/main.rs:525",
                        "ardour_mcp_server",
                        ::tracing::Level::INFO,
                        ::tracing_core::__macro_support::Option::Some("src/main.rs"),
                        ::tracing_core::__macro_support::Option::Some(525u32),
                        ::tracing_core::__macro_support::Option::Some(
                            "ardour_mcp_server",
                        ),
                        ::tracing_core::field::FieldSet::new(
                            &["message"],
                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                        ),
                        ::tracing::metadata::Kind::EVENT,
                    )
                };
                ::tracing::callsite::DefaultCallsite::new(&META)
            };
            let enabled = ::tracing::Level::INFO
                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                && ::tracing::Level::INFO
                    <= ::tracing::level_filters::LevelFilter::current()
                && {
                    let interest = __CALLSITE.interest();
                    !interest.is_never()
                        && ::tracing::__macro_support::__is_enabled(
                            __CALLSITE.metadata(),
                            interest,
                        )
                };
            if enabled {
                (|value_set: ::tracing::field::ValueSet| {
                    let meta = __CALLSITE.metadata();
                    ::tracing::Event::dispatch(meta, &value_set);
                })({
                    #[allow(unused_imports)]
                    use ::tracing::field::{debug, display, Value};
                    let mut iter = __CALLSITE.metadata().fields().iter();
                    __CALLSITE
                        .metadata()
                        .fields()
                        .value_set(
                            &[
                                (
                                    &::tracing::__macro_support::Iterator::next(&mut iter)
                                        .expect("FieldSet corrupted (this is a bug)"),
                                    ::tracing::__macro_support::Option::Some(
                                        &format_args!(
                                            "Executing set_track_gain_abs_tool for rid: {0}, gain_abs: {1}",
                                            args.rid,
                                            args.gain_abs,
                                        ) as &dyn Value,
                                    ),
                                ),
                            ],
                        )
                });
            } else {
            }
        };
        if args.gain_abs < 0.0 || args.gain_abs > 2.0 {
            {
                use ::tracing::__macro_support::Callsite as _;
                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                    static META: ::tracing::Metadata<'static> = {
                        ::tracing_core::metadata::Metadata::new(
                            "event src/main.rs:529",
                            "ardour_mcp_server",
                            ::tracing::Level::WARN,
                            ::tracing_core::__macro_support::Option::Some("src/main.rs"),
                            ::tracing_core::__macro_support::Option::Some(529u32),
                            ::tracing_core::__macro_support::Option::Some(
                                "ardour_mcp_server",
                            ),
                            ::tracing_core::field::FieldSet::new(
                                &["message"],
                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                            ),
                            ::tracing::metadata::Kind::EVENT,
                        )
                    };
                    ::tracing::callsite::DefaultCallsite::new(&META)
                };
                let enabled = ::tracing::Level::WARN
                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                    && ::tracing::Level::WARN
                        <= ::tracing::level_filters::LevelFilter::current()
                    && {
                        let interest = __CALLSITE.interest();
                        !interest.is_never()
                            && ::tracing::__macro_support::__is_enabled(
                                __CALLSITE.metadata(),
                                interest,
                            )
                    };
                if enabled {
                    (|value_set: ::tracing::field::ValueSet| {
                        let meta = __CALLSITE.metadata();
                        ::tracing::Event::dispatch(meta, &value_set);
                    })({
                        #[allow(unused_imports)]
                        use ::tracing::field::{debug, display, Value};
                        let mut iter = __CALLSITE.metadata().fields().iter();
                        __CALLSITE
                            .metadata()
                            .fields()
                            .value_set(
                                &[
                                    (
                                        &::tracing::__macro_support::Iterator::next(&mut iter)
                                            .expect("FieldSet corrupted (this is a bug)"),
                                        ::tracing::__macro_support::Option::Some(
                                            &format_args!(
                                                "Invalid gain_abs value: {0}. Must be between 0.0 and 2.0 (maps to fader 0.0-1.0).",
                                                args.gain_abs,
                                            ) as &dyn Value,
                                        ),
                                    ),
                                ],
                            )
                    });
                } else {
                }
            };
            return Ok(
                CallToolResult::error(
                    <[_]>::into_vec(
                        ::alloc::boxed::box_new([
                            Content::text(
                                ::alloc::__export::must_use({
                                    ::alloc::fmt::format(
                                        format_args!(
                                            "Invalid gain_abs value: {0}. Must be between 0.0 and 2.0 (maps to fader 0.0-1.0).",
                                            args.gain_abs,
                                        ),
                                    )
                                }),
                            ),
                        ]),
                    ),
                ),
            );
        }
        let fader_position = (args.gain_abs / 2.0).clamp(0.0, 1.0);
        let osc_args = <[_]>::into_vec(
            ::alloc::boxed::box_new([
                osc::Type::Int(args.rid),
                osc::Type::Float(fader_position),
            ]),
        );
        let address = "/strip/fader";
        match self.send_osc_message(address, Some(osc_args)).await {
            Ok(_) => {
                Ok(
                    CallToolResult::success(
                        <[_]>::into_vec(
                            ::alloc::boxed::box_new([
                                Content::text(
                                    ::alloc::__export::must_use({
                                        ::alloc::fmt::format(
                                            format_args!(
                                                "Track {0} fader position set to {1} (from gain_abs {2})",
                                                args.rid,
                                                fader_position,
                                                args.gain_abs,
                                            ),
                                        )
                                    }),
                                ),
                            ]),
                        ),
                    ),
                )
            }
            Err(e) => {
                Ok(
                    CallToolResult::error(
                        <[_]>::into_vec(
                            ::alloc::boxed::box_new([
                                Content::text(
                                    ::alloc::__export::must_use({
                                        ::alloc::fmt::format(
                                            format_args!("OSC send error for {0}: {1}", address, e),
                                        )
                                    }),
                                ),
                            ]),
                        ),
                    ),
                )
            }
        }
    }
    fn set_track_gain_db_tool_tool_attr() -> rmcp::model::Tool {
        rmcp::model::Tool {
            name: "set_track_gain_db".into(),
            description: "Sets the gain of a specific track in dB.".into(),
            input_schema: rmcp::handler::server::tool::cached_schema_for_type::<
                SetTrackGainDBArgs,
            >()
                .into(),
        }
    }
    async fn set_track_gain_db_tool_tool_call(
        context: rmcp::handler::server::tool::ToolCallContext<'_, Self>,
    ) -> std::result::Result<rmcp::model::CallToolResult, rmcp::Error> {
        use rmcp::handler::server::tool::*;
        let (__rmcp_tool_receiver, context) = <&Self>::from_tool_call_context_part(
            context,
        )?;
        let (Parameters(args), context) = <Parameters<
            SetTrackGainDBArgs,
        >>::from_tool_call_context_part(context)?;
        Self::set_track_gain_db_tool(__rmcp_tool_receiver, args)
            .await
            .into_call_tool_result()
    }
    async fn set_track_gain_db_tool(
        &self,
        args: SetTrackGainDBArgs,
    ) -> Result<CallToolResult, McpError> {
        {
            use ::tracing::__macro_support::Callsite as _;
            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                static META: ::tracing::Metadata<'static> = {
                    ::tracing_core::metadata::Metadata::new(
                        "event src/main.rs:558",
                        "ardour_mcp_server",
                        ::tracing::Level::INFO,
                        ::tracing_core::__macro_support::Option::Some("src/main.rs"),
                        ::tracing_core::__macro_support::Option::Some(558u32),
                        ::tracing_core::__macro_support::Option::Some(
                            "ardour_mcp_server",
                        ),
                        ::tracing_core::field::FieldSet::new(
                            &["message"],
                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                        ),
                        ::tracing::metadata::Kind::EVENT,
                    )
                };
                ::tracing::callsite::DefaultCallsite::new(&META)
            };
            let enabled = ::tracing::Level::INFO
                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                && ::tracing::Level::INFO
                    <= ::tracing::level_filters::LevelFilter::current()
                && {
                    let interest = __CALLSITE.interest();
                    !interest.is_never()
                        && ::tracing::__macro_support::__is_enabled(
                            __CALLSITE.metadata(),
                            interest,
                        )
                };
            if enabled {
                (|value_set: ::tracing::field::ValueSet| {
                    let meta = __CALLSITE.metadata();
                    ::tracing::Event::dispatch(meta, &value_set);
                })({
                    #[allow(unused_imports)]
                    use ::tracing::field::{debug, display, Value};
                    let mut iter = __CALLSITE.metadata().fields().iter();
                    __CALLSITE
                        .metadata()
                        .fields()
                        .value_set(
                            &[
                                (
                                    &::tracing::__macro_support::Iterator::next(&mut iter)
                                        .expect("FieldSet corrupted (this is a bug)"),
                                    ::tracing::__macro_support::Option::Some(
                                        &format_args!(
                                            "Executing set_track_gain_db_tool for rid: {0}, gain_db: {1}",
                                            args.rid,
                                            args.gain_db,
                                        ) as &dyn Value,
                                    ),
                                ),
                            ],
                        )
                });
            } else {
            }
        };
        if !(args.gain_db >= -400.0 && args.gain_db <= 6.0) {
            {
                use ::tracing::__macro_support::Callsite as _;
                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                    static META: ::tracing::Metadata<'static> = {
                        ::tracing_core::metadata::Metadata::new(
                            "event src/main.rs:561",
                            "ardour_mcp_server",
                            ::tracing::Level::WARN,
                            ::tracing_core::__macro_support::Option::Some("src/main.rs"),
                            ::tracing_core::__macro_support::Option::Some(561u32),
                            ::tracing_core::__macro_support::Option::Some(
                                "ardour_mcp_server",
                            ),
                            ::tracing_core::field::FieldSet::new(
                                &["message"],
                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                            ),
                            ::tracing::metadata::Kind::EVENT,
                        )
                    };
                    ::tracing::callsite::DefaultCallsite::new(&META)
                };
                let enabled = ::tracing::Level::WARN
                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                    && ::tracing::Level::WARN
                        <= ::tracing::level_filters::LevelFilter::current()
                    && {
                        let interest = __CALLSITE.interest();
                        !interest.is_never()
                            && ::tracing::__macro_support::__is_enabled(
                                __CALLSITE.metadata(),
                                interest,
                            )
                    };
                if enabled {
                    (|value_set: ::tracing::field::ValueSet| {
                        let meta = __CALLSITE.metadata();
                        ::tracing::Event::dispatch(meta, &value_set);
                    })({
                        #[allow(unused_imports)]
                        use ::tracing::field::{debug, display, Value};
                        let mut iter = __CALLSITE.metadata().fields().iter();
                        __CALLSITE
                            .metadata()
                            .fields()
                            .value_set(
                                &[
                                    (
                                        &::tracing::__macro_support::Iterator::next(&mut iter)
                                            .expect("FieldSet corrupted (this is a bug)"),
                                        ::tracing::__macro_support::Option::Some(
                                            &format_args!(
                                                "Invalid gain_db value: {0}. Must be between -400.0 and 6.0.",
                                                args.gain_db,
                                            ) as &dyn Value,
                                        ),
                                    ),
                                ],
                            )
                    });
                } else {
                }
            };
            return Ok(
                CallToolResult::error(
                    <[_]>::into_vec(
                        ::alloc::boxed::box_new([
                            Content::text(
                                ::alloc::__export::must_use({
                                    ::alloc::fmt::format(
                                        format_args!(
                                            "Invalid gain_db value: {0}. Must be between -400.0 and 6.0.",
                                            args.gain_db,
                                        ),
                                    )
                                }),
                            ),
                        ]),
                    ),
                ),
            );
        }
        let osc_args = <[_]>::into_vec(
            ::alloc::boxed::box_new([
                osc::Type::Int(args.rid),
                osc::Type::Float(args.gain_db),
            ]),
        );
        let address = "/strip/gain";
        match self.send_osc_message(address, Some(osc_args)).await {
            Ok(_) => {
                Ok(
                    CallToolResult::success(
                        <[_]>::into_vec(
                            ::alloc::boxed::box_new([
                                Content::text(
                                    ::alloc::__export::must_use({
                                        ::alloc::fmt::format(
                                            format_args!(
                                                "Track {0} gain (dB) set to {1}",
                                                args.rid,
                                                args.gain_db,
                                            ),
                                        )
                                    }),
                                ),
                            ]),
                        ),
                    ),
                )
            }
            Err(e) => {
                Ok(
                    CallToolResult::error(
                        <[_]>::into_vec(
                            ::alloc::boxed::box_new([
                                Content::text(
                                    ::alloc::__export::must_use({
                                        ::alloc::fmt::format(
                                            format_args!("OSC send error for {0}: {1}", address, e),
                                        )
                                    }),
                                ),
                            ]),
                        ),
                    ),
                )
            }
        }
    }
    fn set_track_trim_abs_tool_tool_attr() -> rmcp::model::Tool {
        rmcp::model::Tool {
            name: "set_track_trim_abs".into(),
            description: "Sets the absolute trim of a specific track.".into(),
            input_schema: rmcp::handler::server::tool::cached_schema_for_type::<
                SetTrackTrimAbsArgs,
            >()
                .into(),
        }
    }
    async fn set_track_trim_abs_tool_tool_call(
        context: rmcp::handler::server::tool::ToolCallContext<'_, Self>,
    ) -> std::result::Result<rmcp::model::CallToolResult, rmcp::Error> {
        use rmcp::handler::server::tool::*;
        let (__rmcp_tool_receiver, context) = <&Self>::from_tool_call_context_part(
            context,
        )?;
        let (Parameters(args), context) = <Parameters<
            SetTrackTrimAbsArgs,
        >>::from_tool_call_context_part(context)?;
        Self::set_track_trim_abs_tool(__rmcp_tool_receiver, args)
            .await
            .into_call_tool_result()
    }
    async fn set_track_trim_abs_tool(
        &self,
        args: SetTrackTrimAbsArgs,
    ) -> Result<CallToolResult, McpError> {
        {
            use ::tracing::__macro_support::Callsite as _;
            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                static META: ::tracing::Metadata<'static> = {
                    ::tracing_core::metadata::Metadata::new(
                        "event src/main.rs:589",
                        "ardour_mcp_server",
                        ::tracing::Level::INFO,
                        ::tracing_core::__macro_support::Option::Some("src/main.rs"),
                        ::tracing_core::__macro_support::Option::Some(589u32),
                        ::tracing_core::__macro_support::Option::Some(
                            "ardour_mcp_server",
                        ),
                        ::tracing_core::field::FieldSet::new(
                            &["message"],
                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                        ),
                        ::tracing::metadata::Kind::EVENT,
                    )
                };
                ::tracing::callsite::DefaultCallsite::new(&META)
            };
            let enabled = ::tracing::Level::INFO
                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                && ::tracing::Level::INFO
                    <= ::tracing::level_filters::LevelFilter::current()
                && {
                    let interest = __CALLSITE.interest();
                    !interest.is_never()
                        && ::tracing::__macro_support::__is_enabled(
                            __CALLSITE.metadata(),
                            interest,
                        )
                };
            if enabled {
                (|value_set: ::tracing::field::ValueSet| {
                    let meta = __CALLSITE.metadata();
                    ::tracing::Event::dispatch(meta, &value_set);
                })({
                    #[allow(unused_imports)]
                    use ::tracing::field::{debug, display, Value};
                    let mut iter = __CALLSITE.metadata().fields().iter();
                    __CALLSITE
                        .metadata()
                        .fields()
                        .value_set(
                            &[
                                (
                                    &::tracing::__macro_support::Iterator::next(&mut iter)
                                        .expect("FieldSet corrupted (this is a bug)"),
                                    ::tracing::__macro_support::Option::Some(
                                        &format_args!(
                                            "Executing set_track_trim_abs_tool for rid: {0}, trim_abs: {1}",
                                            args.rid,
                                            args.trim_abs,
                                        ) as &dyn Value,
                                    ),
                                ),
                            ],
                        )
                });
            } else {
            }
        };
        if args.trim_abs < 0.1 || args.trim_abs > 10.0 {
            {
                use ::tracing::__macro_support::Callsite as _;
                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                    static META: ::tracing::Metadata<'static> = {
                        ::tracing_core::metadata::Metadata::new(
                            "event src/main.rs:593",
                            "ardour_mcp_server",
                            ::tracing::Level::WARN,
                            ::tracing_core::__macro_support::Option::Some("src/main.rs"),
                            ::tracing_core::__macro_support::Option::Some(593u32),
                            ::tracing_core::__macro_support::Option::Some(
                                "ardour_mcp_server",
                            ),
                            ::tracing_core::field::FieldSet::new(
                                &["message"],
                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                            ),
                            ::tracing::metadata::Kind::EVENT,
                        )
                    };
                    ::tracing::callsite::DefaultCallsite::new(&META)
                };
                let enabled = ::tracing::Level::WARN
                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                    && ::tracing::Level::WARN
                        <= ::tracing::level_filters::LevelFilter::current()
                    && {
                        let interest = __CALLSITE.interest();
                        !interest.is_never()
                            && ::tracing::__macro_support::__is_enabled(
                                __CALLSITE.metadata(),
                                interest,
                            )
                    };
                if enabled {
                    (|value_set: ::tracing::field::ValueSet| {
                        let meta = __CALLSITE.metadata();
                        ::tracing::Event::dispatch(meta, &value_set);
                    })({
                        #[allow(unused_imports)]
                        use ::tracing::field::{debug, display, Value};
                        let mut iter = __CALLSITE.metadata().fields().iter();
                        __CALLSITE
                            .metadata()
                            .fields()
                            .value_set(
                                &[
                                    (
                                        &::tracing::__macro_support::Iterator::next(&mut iter)
                                            .expect("FieldSet corrupted (this is a bug)"),
                                        ::tracing::__macro_support::Option::Some(
                                            &format_args!(
                                                "Invalid trim_abs value: {0}. Must be between 0.1 and 10.0.",
                                                args.trim_abs,
                                            ) as &dyn Value,
                                        ),
                                    ),
                                ],
                            )
                    });
                } else {
                }
            };
            return Ok(
                CallToolResult::error(
                    <[_]>::into_vec(
                        ::alloc::boxed::box_new([
                            Content::text(
                                ::alloc::__export::must_use({
                                    ::alloc::fmt::format(
                                        format_args!(
                                            "Invalid trim_abs value: {0}. Must be between 0.1 and 10.0.",
                                            args.trim_abs,
                                        ),
                                    )
                                }),
                            ),
                        ]),
                    ),
                ),
            );
        }
        let fader_position = ((args.trim_abs - 0.1) / 9.9).clamp(0.0, 1.0);
        let osc_args = <[_]>::into_vec(
            ::alloc::boxed::box_new([
                osc::Type::Int(args.rid),
                osc::Type::Float(fader_position),
            ]),
        );
        let address = "/strip/trim_fader";
        match self.send_osc_message(address, Some(osc_args)).await {
            Ok(_) => {
                Ok(
                    CallToolResult::success(
                        <[_]>::into_vec(
                            ::alloc::boxed::box_new([
                                Content::text(
                                    ::alloc::__export::must_use({
                                        ::alloc::fmt::format(
                                            format_args!(
                                                "Track {0} trim fader position set to {1} (from trim_abs {2})",
                                                args.rid,
                                                fader_position,
                                                args.trim_abs,
                                            ),
                                        )
                                    }),
                                ),
                            ]),
                        ),
                    ),
                )
            }
            Err(e) => {
                Ok(
                    CallToolResult::error(
                        <[_]>::into_vec(
                            ::alloc::boxed::box_new([
                                Content::text(
                                    ::alloc::__export::must_use({
                                        ::alloc::fmt::format(
                                            format_args!("OSC send error for {0}: {1}", address, e),
                                        )
                                    }),
                                ),
                            ]),
                        ),
                    ),
                )
            }
        }
    }
    fn set_track_trim_db_tool_tool_attr() -> rmcp::model::Tool {
        rmcp::model::Tool {
            name: "set_track_trim_db".into(),
            description: "Sets the trim of a specific track in dB.".into(),
            input_schema: rmcp::handler::server::tool::cached_schema_for_type::<
                SetTrackTrimDBArgs,
            >()
                .into(),
        }
    }
    async fn set_track_trim_db_tool_tool_call(
        context: rmcp::handler::server::tool::ToolCallContext<'_, Self>,
    ) -> std::result::Result<rmcp::model::CallToolResult, rmcp::Error> {
        use rmcp::handler::server::tool::*;
        let (__rmcp_tool_receiver, context) = <&Self>::from_tool_call_context_part(
            context,
        )?;
        let (Parameters(args), context) = <Parameters<
            SetTrackTrimDBArgs,
        >>::from_tool_call_context_part(context)?;
        Self::set_track_trim_db_tool(__rmcp_tool_receiver, args)
            .await
            .into_call_tool_result()
    }
    async fn set_track_trim_db_tool(
        &self,
        args: SetTrackTrimDBArgs,
    ) -> Result<CallToolResult, McpError> {
        {
            use ::tracing::__macro_support::Callsite as _;
            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                static META: ::tracing::Metadata<'static> = {
                    ::tracing_core::metadata::Metadata::new(
                        "event src/main.rs:622",
                        "ardour_mcp_server",
                        ::tracing::Level::INFO,
                        ::tracing_core::__macro_support::Option::Some("src/main.rs"),
                        ::tracing_core::__macro_support::Option::Some(622u32),
                        ::tracing_core::__macro_support::Option::Some(
                            "ardour_mcp_server",
                        ),
                        ::tracing_core::field::FieldSet::new(
                            &["message"],
                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                        ),
                        ::tracing::metadata::Kind::EVENT,
                    )
                };
                ::tracing::callsite::DefaultCallsite::new(&META)
            };
            let enabled = ::tracing::Level::INFO
                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                && ::tracing::Level::INFO
                    <= ::tracing::level_filters::LevelFilter::current()
                && {
                    let interest = __CALLSITE.interest();
                    !interest.is_never()
                        && ::tracing::__macro_support::__is_enabled(
                            __CALLSITE.metadata(),
                            interest,
                        )
                };
            if enabled {
                (|value_set: ::tracing::field::ValueSet| {
                    let meta = __CALLSITE.metadata();
                    ::tracing::Event::dispatch(meta, &value_set);
                })({
                    #[allow(unused_imports)]
                    use ::tracing::field::{debug, display, Value};
                    let mut iter = __CALLSITE.metadata().fields().iter();
                    __CALLSITE
                        .metadata()
                        .fields()
                        .value_set(
                            &[
                                (
                                    &::tracing::__macro_support::Iterator::next(&mut iter)
                                        .expect("FieldSet corrupted (this is a bug)"),
                                    ::tracing::__macro_support::Option::Some(
                                        &format_args!(
                                            "Executing set_track_trim_db_tool for rid: {0}, trim_db: {1}",
                                            args.rid,
                                            args.trim_db,
                                        ) as &dyn Value,
                                    ),
                                ),
                            ],
                        )
                });
            } else {
            }
        };
        if args.trim_db < -20.0 || args.trim_db > 20.0 {
            let error_msg = ::alloc::__export::must_use({
                ::alloc::fmt::format(
                    format_args!(
                        "Invalid trim_db value: {0}. Must be between -20.0 and 20.0.",
                        args.trim_db,
                    ),
                )
            });
            {
                use ::tracing::__macro_support::Callsite as _;
                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                    static META: ::tracing::Metadata<'static> = {
                        ::tracing_core::metadata::Metadata::new(
                            "event src/main.rs:629",
                            "ardour_mcp_server",
                            ::tracing::Level::WARN,
                            ::tracing_core::__macro_support::Option::Some("src/main.rs"),
                            ::tracing_core::__macro_support::Option::Some(629u32),
                            ::tracing_core::__macro_support::Option::Some(
                                "ardour_mcp_server",
                            ),
                            ::tracing_core::field::FieldSet::new(
                                &["message"],
                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                            ),
                            ::tracing::metadata::Kind::EVENT,
                        )
                    };
                    ::tracing::callsite::DefaultCallsite::new(&META)
                };
                let enabled = ::tracing::Level::WARN
                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                    && ::tracing::Level::WARN
                        <= ::tracing::level_filters::LevelFilter::current()
                    && {
                        let interest = __CALLSITE.interest();
                        !interest.is_never()
                            && ::tracing::__macro_support::__is_enabled(
                                __CALLSITE.metadata(),
                                interest,
                            )
                    };
                if enabled {
                    (|value_set: ::tracing::field::ValueSet| {
                        let meta = __CALLSITE.metadata();
                        ::tracing::Event::dispatch(meta, &value_set);
                    })({
                        #[allow(unused_imports)]
                        use ::tracing::field::{debug, display, Value};
                        let mut iter = __CALLSITE.metadata().fields().iter();
                        __CALLSITE
                            .metadata()
                            .fields()
                            .value_set(
                                &[
                                    (
                                        &::tracing::__macro_support::Iterator::next(&mut iter)
                                            .expect("FieldSet corrupted (this is a bug)"),
                                        ::tracing::__macro_support::Option::Some(
                                            &format_args!("{0}", error_msg) as &dyn Value,
                                        ),
                                    ),
                                ],
                            )
                    });
                } else {
                }
            };
            return Ok(
                CallToolResult::error(
                    <[_]>::into_vec(::alloc::boxed::box_new([Content::text(error_msg)])),
                ),
            );
        }
        let osc_addr = "/strip/trimdB";
        let osc_args = <[_]>::into_vec(
            ::alloc::boxed::box_new([
                osc::Type::Int(args.rid),
                osc::Type::Float(args.trim_db),
            ]),
        );
        match self.send_osc_message(osc_addr, Some(osc_args)).await {
            Ok(_) => {
                let success_msg = ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!(
                            "Successfully sent OSC message {0} with rid {1} and trim_db {2}",
                            osc_addr,
                            args.rid,
                            args.trim_db,
                        ),
                    )
                });
                {
                    use ::tracing::__macro_support::Callsite as _;
                    static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "event src/main.rs:642",
                                "ardour_mcp_server",
                                ::tracing::Level::INFO,
                                ::tracing_core::__macro_support::Option::Some(
                                    "src/main.rs",
                                ),
                                ::tracing_core::__macro_support::Option::Some(642u32),
                                ::tracing_core::__macro_support::Option::Some(
                                    "ardour_mcp_server",
                                ),
                                ::tracing_core::field::FieldSet::new(
                                    &["message"],
                                    ::tracing_core::callsite::Identifier(&__CALLSITE),
                                ),
                                ::tracing::metadata::Kind::EVENT,
                            )
                        };
                        ::tracing::callsite::DefaultCallsite::new(&META)
                    };
                    let enabled = ::tracing::Level::INFO
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && ::tracing::Level::INFO
                            <= ::tracing::level_filters::LevelFilter::current()
                        && {
                            let interest = __CALLSITE.interest();
                            !interest.is_never()
                                && ::tracing::__macro_support::__is_enabled(
                                    __CALLSITE.metadata(),
                                    interest,
                                )
                        };
                    if enabled {
                        (|value_set: ::tracing::field::ValueSet| {
                            let meta = __CALLSITE.metadata();
                            ::tracing::Event::dispatch(meta, &value_set);
                        })({
                            #[allow(unused_imports)]
                            use ::tracing::field::{debug, display, Value};
                            let mut iter = __CALLSITE.metadata().fields().iter();
                            __CALLSITE
                                .metadata()
                                .fields()
                                .value_set(
                                    &[
                                        (
                                            &::tracing::__macro_support::Iterator::next(&mut iter)
                                                .expect("FieldSet corrupted (this is a bug)"),
                                            ::tracing::__macro_support::Option::Some(
                                                &format_args!("{0}", success_msg) as &dyn Value,
                                            ),
                                        ),
                                    ],
                                )
                        });
                    } else {
                    }
                };
                Ok(
                    CallToolResult::success(
                        <[_]>::into_vec(
                            ::alloc::boxed::box_new([Content::text(success_msg)]),
                        ),
                    ),
                )
            }
            Err(e) => {
                let error_msg = ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!(
                            "Failed to send OSC message {0} for rid {1}: {2:?}",
                            osc_addr,
                            args.rid,
                            e,
                        ),
                    )
                });
                {
                    use ::tracing::__macro_support::Callsite as _;
                    static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "event src/main.rs:647",
                                "ardour_mcp_server",
                                ::tracing::Level::ERROR,
                                ::tracing_core::__macro_support::Option::Some(
                                    "src/main.rs",
                                ),
                                ::tracing_core::__macro_support::Option::Some(647u32),
                                ::tracing_core::__macro_support::Option::Some(
                                    "ardour_mcp_server",
                                ),
                                ::tracing_core::field::FieldSet::new(
                                    &["message"],
                                    ::tracing_core::callsite::Identifier(&__CALLSITE),
                                ),
                                ::tracing::metadata::Kind::EVENT,
                            )
                        };
                        ::tracing::callsite::DefaultCallsite::new(&META)
                    };
                    let enabled = ::tracing::Level::ERROR
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && ::tracing::Level::ERROR
                            <= ::tracing::level_filters::LevelFilter::current()
                        && {
                            let interest = __CALLSITE.interest();
                            !interest.is_never()
                                && ::tracing::__macro_support::__is_enabled(
                                    __CALLSITE.metadata(),
                                    interest,
                                )
                        };
                    if enabled {
                        (|value_set: ::tracing::field::ValueSet| {
                            let meta = __CALLSITE.metadata();
                            ::tracing::Event::dispatch(meta, &value_set);
                        })({
                            #[allow(unused_imports)]
                            use ::tracing::field::{debug, display, Value};
                            let mut iter = __CALLSITE.metadata().fields().iter();
                            __CALLSITE
                                .metadata()
                                .fields()
                                .value_set(
                                    &[
                                        (
                                            &::tracing::__macro_support::Iterator::next(&mut iter)
                                                .expect("FieldSet corrupted (this is a bug)"),
                                            ::tracing::__macro_support::Option::Some(
                                                &format_args!("{0}", error_msg) as &dyn Value,
                                            ),
                                        ),
                                    ],
                                )
                        });
                    } else {
                    }
                };
                Err(McpError::internal_error(error_msg, None))
            }
        }
    }
    fn access_action_tool_tool_attr() -> rmcp::model::Tool {
        rmcp::model::Tool {
            name: "access_action".into(),
            description: "Executes a specified Ardour menu action by its name.".into(),
            input_schema: rmcp::handler::server::tool::cached_schema_for_type::<
                AccessActionArgs,
            >()
                .into(),
        }
    }
    async fn access_action_tool_tool_call(
        context: rmcp::handler::server::tool::ToolCallContext<'_, Self>,
    ) -> std::result::Result<rmcp::model::CallToolResult, rmcp::Error> {
        use rmcp::handler::server::tool::*;
        let (__rmcp_tool_receiver, context) = <&Self>::from_tool_call_context_part(
            context,
        )?;
        let (Parameters(args), context) = <Parameters<
            AccessActionArgs,
        >>::from_tool_call_context_part(context)?;
        Self::access_action_tool(__rmcp_tool_receiver, args)
            .await
            .into_call_tool_result()
    }
    async fn access_action_tool(
        &self,
        args: AccessActionArgs,
    ) -> Result<CallToolResult, McpError> {
        {
            use ::tracing::__macro_support::Callsite as _;
            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                static META: ::tracing::Metadata<'static> = {
                    ::tracing_core::metadata::Metadata::new(
                        "event src/main.rs:659",
                        "ardour_mcp_server",
                        ::tracing::Level::INFO,
                        ::tracing_core::__macro_support::Option::Some("src/main.rs"),
                        ::tracing_core::__macro_support::Option::Some(659u32),
                        ::tracing_core::__macro_support::Option::Some(
                            "ardour_mcp_server",
                        ),
                        ::tracing_core::field::FieldSet::new(
                            &["message"],
                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                        ),
                        ::tracing::metadata::Kind::EVENT,
                    )
                };
                ::tracing::callsite::DefaultCallsite::new(&META)
            };
            let enabled = ::tracing::Level::INFO
                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                && ::tracing::Level::INFO
                    <= ::tracing::level_filters::LevelFilter::current()
                && {
                    let interest = __CALLSITE.interest();
                    !interest.is_never()
                        && ::tracing::__macro_support::__is_enabled(
                            __CALLSITE.metadata(),
                            interest,
                        )
                };
            if enabled {
                (|value_set: ::tracing::field::ValueSet| {
                    let meta = __CALLSITE.metadata();
                    ::tracing::Event::dispatch(meta, &value_set);
                })({
                    #[allow(unused_imports)]
                    use ::tracing::field::{debug, display, Value};
                    let mut iter = __CALLSITE.metadata().fields().iter();
                    __CALLSITE
                        .metadata()
                        .fields()
                        .value_set(
                            &[
                                (
                                    &::tracing::__macro_support::Iterator::next(&mut iter)
                                        .expect("FieldSet corrupted (this is a bug)"),
                                    ::tracing::__macro_support::Option::Some(
                                        &format_args!(
                                            "Executing access_action_tool for action_name: {0}",
                                            args.action_name,
                                        ) as &dyn Value,
                                    ),
                                ),
                            ],
                        )
                });
            } else {
            }
        };
        if args.action_name.is_empty() {
            let error_msg = "action_name cannot be empty.";
            {
                use ::tracing::__macro_support::Callsite as _;
                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                    static META: ::tracing::Metadata<'static> = {
                        ::tracing_core::metadata::Metadata::new(
                            "event src/main.rs:663",
                            "ardour_mcp_server",
                            ::tracing::Level::WARN,
                            ::tracing_core::__macro_support::Option::Some("src/main.rs"),
                            ::tracing_core::__macro_support::Option::Some(663u32),
                            ::tracing_core::__macro_support::Option::Some(
                                "ardour_mcp_server",
                            ),
                            ::tracing_core::field::FieldSet::new(
                                &["message"],
                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                            ),
                            ::tracing::metadata::Kind::EVENT,
                        )
                    };
                    ::tracing::callsite::DefaultCallsite::new(&META)
                };
                let enabled = ::tracing::Level::WARN
                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                    && ::tracing::Level::WARN
                        <= ::tracing::level_filters::LevelFilter::current()
                    && {
                        let interest = __CALLSITE.interest();
                        !interest.is_never()
                            && ::tracing::__macro_support::__is_enabled(
                                __CALLSITE.metadata(),
                                interest,
                            )
                    };
                if enabled {
                    (|value_set: ::tracing::field::ValueSet| {
                        let meta = __CALLSITE.metadata();
                        ::tracing::Event::dispatch(meta, &value_set);
                    })({
                        #[allow(unused_imports)]
                        use ::tracing::field::{debug, display, Value};
                        let mut iter = __CALLSITE.metadata().fields().iter();
                        __CALLSITE
                            .metadata()
                            .fields()
                            .value_set(
                                &[
                                    (
                                        &::tracing::__macro_support::Iterator::next(&mut iter)
                                            .expect("FieldSet corrupted (this is a bug)"),
                                        ::tracing::__macro_support::Option::Some(
                                            &format_args!("{0}", error_msg) as &dyn Value,
                                        ),
                                    ),
                                ],
                            )
                    });
                } else {
                }
            };
            return Ok(
                CallToolResult::error(
                    <[_]>::into_vec(
                        ::alloc::boxed::box_new([Content::text(error_msg.to_string())]),
                    ),
                ),
            );
        }
        let osc_addr = "/access_action";
        let osc_args = <[_]>::into_vec(
            ::alloc::boxed::box_new([osc::Type::String(args.action_name.clone())]),
        );
        match self.send_osc_message(osc_addr, Some(osc_args)).await {
            Ok(_) => {
                let success_msg = ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!(
                            "Successfully sent OSC message {0} with action_name \'{1}\'",
                            osc_addr,
                            args.action_name,
                        ),
                    )
                });
                {
                    use ::tracing::__macro_support::Callsite as _;
                    static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "event src/main.rs:677",
                                "ardour_mcp_server",
                                ::tracing::Level::INFO,
                                ::tracing_core::__macro_support::Option::Some(
                                    "src/main.rs",
                                ),
                                ::tracing_core::__macro_support::Option::Some(677u32),
                                ::tracing_core::__macro_support::Option::Some(
                                    "ardour_mcp_server",
                                ),
                                ::tracing_core::field::FieldSet::new(
                                    &["message"],
                                    ::tracing_core::callsite::Identifier(&__CALLSITE),
                                ),
                                ::tracing::metadata::Kind::EVENT,
                            )
                        };
                        ::tracing::callsite::DefaultCallsite::new(&META)
                    };
                    let enabled = ::tracing::Level::INFO
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && ::tracing::Level::INFO
                            <= ::tracing::level_filters::LevelFilter::current()
                        && {
                            let interest = __CALLSITE.interest();
                            !interest.is_never()
                                && ::tracing::__macro_support::__is_enabled(
                                    __CALLSITE.metadata(),
                                    interest,
                                )
                        };
                    if enabled {
                        (|value_set: ::tracing::field::ValueSet| {
                            let meta = __CALLSITE.metadata();
                            ::tracing::Event::dispatch(meta, &value_set);
                        })({
                            #[allow(unused_imports)]
                            use ::tracing::field::{debug, display, Value};
                            let mut iter = __CALLSITE.metadata().fields().iter();
                            __CALLSITE
                                .metadata()
                                .fields()
                                .value_set(
                                    &[
                                        (
                                            &::tracing::__macro_support::Iterator::next(&mut iter)
                                                .expect("FieldSet corrupted (this is a bug)"),
                                            ::tracing::__macro_support::Option::Some(
                                                &format_args!("{0}", success_msg) as &dyn Value,
                                            ),
                                        ),
                                    ],
                                )
                        });
                    } else {
                    }
                };
                Ok(
                    CallToolResult::success(
                        <[_]>::into_vec(
                            ::alloc::boxed::box_new([Content::text(success_msg)]),
                        ),
                    ),
                )
            }
            Err(e) => {
                let error_msg = ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!(
                            "Failed to send OSC message {0} for action_name \'{1}\': {2:?}",
                            osc_addr,
                            args.action_name,
                            e,
                        ),
                    )
                });
                {
                    use ::tracing::__macro_support::Callsite as _;
                    static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "event src/main.rs:685",
                                "ardour_mcp_server",
                                ::tracing::Level::ERROR,
                                ::tracing_core::__macro_support::Option::Some(
                                    "src/main.rs",
                                ),
                                ::tracing_core::__macro_support::Option::Some(685u32),
                                ::tracing_core::__macro_support::Option::Some(
                                    "ardour_mcp_server",
                                ),
                                ::tracing_core::field::FieldSet::new(
                                    &["message"],
                                    ::tracing_core::callsite::Identifier(&__CALLSITE),
                                ),
                                ::tracing::metadata::Kind::EVENT,
                            )
                        };
                        ::tracing::callsite::DefaultCallsite::new(&META)
                    };
                    let enabled = ::tracing::Level::ERROR
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && ::tracing::Level::ERROR
                            <= ::tracing::level_filters::LevelFilter::current()
                        && {
                            let interest = __CALLSITE.interest();
                            !interest.is_never()
                                && ::tracing::__macro_support::__is_enabled(
                                    __CALLSITE.metadata(),
                                    interest,
                                )
                        };
                    if enabled {
                        (|value_set: ::tracing::field::ValueSet| {
                            let meta = __CALLSITE.metadata();
                            ::tracing::Event::dispatch(meta, &value_set);
                        })({
                            #[allow(unused_imports)]
                            use ::tracing::field::{debug, display, Value};
                            let mut iter = __CALLSITE.metadata().fields().iter();
                            __CALLSITE
                                .metadata()
                                .fields()
                                .value_set(
                                    &[
                                        (
                                            &::tracing::__macro_support::Iterator::next(&mut iter)
                                                .expect("FieldSet corrupted (this is a bug)"),
                                            ::tracing::__macro_support::Option::Some(
                                                &format_args!("{0}", error_msg) as &dyn Value,
                                            ),
                                        ),
                                    ],
                                )
                        });
                    } else {
                    }
                };
                Err(McpError::internal_error(error_msg, None))
            }
        }
    }
    fn tool_box() -> &'static ::rmcp::handler::server::tool::ToolBox<ArdourService> {
        use ::rmcp::handler::server::tool::{ToolBox, ToolBoxItem};
        static TOOL_BOX: std::sync::OnceLock<ToolBox<ArdourService>> = std::sync::OnceLock::new();
        TOOL_BOX
            .get_or_init(|| {
                let mut tool_box = ToolBox::new();
                tool_box
                    .add(
                        ToolBoxItem::new(
                            ArdourService::transport_play_tool_tool_attr(),
                            |context| Box::pin(
                                ArdourService::transport_play_tool_tool_call(context),
                            ),
                        ),
                    );
                tool_box
                    .add(
                        ToolBoxItem::new(
                            ArdourService::transport_stop_tool_tool_attr(),
                            |context| Box::pin(
                                ArdourService::transport_stop_tool_tool_call(context),
                            ),
                        ),
                    );
                tool_box
                    .add(
                        ToolBoxItem::new(
                            ArdourService::goto_start_tool_tool_attr(),
                            |context| Box::pin(
                                ArdourService::goto_start_tool_tool_call(context),
                            ),
                        ),
                    );
                tool_box
                    .add(
                        ToolBoxItem::new(
                            ArdourService::goto_end_tool_tool_attr(),
                            |context| Box::pin(
                                ArdourService::goto_end_tool_tool_call(context),
                            ),
                        ),
                    );
                tool_box
                    .add(
                        ToolBoxItem::new(
                            ArdourService::loop_toggle_tool_tool_attr(),
                            |context| Box::pin(
                                ArdourService::loop_toggle_tool_tool_call(context),
                            ),
                        ),
                    );
                tool_box
                    .add(
                        ToolBoxItem::new(
                            ArdourService::undo_tool_tool_attr(),
                            |context| Box::pin(
                                ArdourService::undo_tool_tool_call(context),
                            ),
                        ),
                    );
                tool_box
                    .add(
                        ToolBoxItem::new(
                            ArdourService::redo_tool_tool_attr(),
                            |context| Box::pin(
                                ArdourService::redo_tool_tool_call(context),
                            ),
                        ),
                    );
                tool_box
                    .add(
                        ToolBoxItem::new(
                            ArdourService::toggle_punch_in_tool_tool_attr(),
                            |context| Box::pin(
                                ArdourService::toggle_punch_in_tool_tool_call(context),
                            ),
                        ),
                    );
                tool_box
                    .add(
                        ToolBoxItem::new(
                            ArdourService::toggle_punch_out_tool_tool_attr(),
                            |context| Box::pin(
                                ArdourService::toggle_punch_out_tool_tool_call(context),
                            ),
                        ),
                    );
                tool_box
                    .add(
                        ToolBoxItem::new(
                            ArdourService::rec_enable_toggle_tool_tool_attr(),
                            |context| Box::pin(
                                ArdourService::rec_enable_toggle_tool_tool_call(context),
                            ),
                        ),
                    );
                tool_box
                    .add(
                        ToolBoxItem::new(
                            ArdourService::toggle_all_rec_enables_tool_tool_attr(),
                            |context| Box::pin(
                                ArdourService::toggle_all_rec_enables_tool_tool_call(
                                    context,
                                ),
                            ),
                        ),
                    );
                tool_box
                    .add(
                        ToolBoxItem::new(
                            ArdourService::ffwd_tool_tool_attr(),
                            |context| Box::pin(
                                ArdourService::ffwd_tool_tool_call(context),
                            ),
                        ),
                    );
                tool_box
                    .add(
                        ToolBoxItem::new(
                            ArdourService::rewind_tool_tool_attr(),
                            |context| Box::pin(
                                ArdourService::rewind_tool_tool_call(context),
                            ),
                        ),
                    );
                tool_box
                    .add(
                        ToolBoxItem::new(
                            ArdourService::add_marker_tool_tool_attr(),
                            |context| Box::pin(
                                ArdourService::add_marker_tool_tool_call(context),
                            ),
                        ),
                    );
                tool_box
                    .add(
                        ToolBoxItem::new(
                            ArdourService::next_marker_tool_tool_attr(),
                            |context| Box::pin(
                                ArdourService::next_marker_tool_tool_call(context),
                            ),
                        ),
                    );
                tool_box
                    .add(
                        ToolBoxItem::new(
                            ArdourService::prev_marker_tool_tool_attr(),
                            |context| Box::pin(
                                ArdourService::prev_marker_tool_tool_call(context),
                            ),
                        ),
                    );
                tool_box
                    .add(
                        ToolBoxItem::new(
                            ArdourService::save_state_tool_tool_attr(),
                            |context| Box::pin(
                                ArdourService::save_state_tool_tool_call(context),
                            ),
                        ),
                    );
                tool_box
                    .add(
                        ToolBoxItem::new(
                            ArdourService::set_track_mute_tool_tool_attr(),
                            |context| Box::pin(
                                ArdourService::set_track_mute_tool_tool_call(context),
                            ),
                        ),
                    );
                tool_box
                    .add(
                        ToolBoxItem::new(
                            ArdourService::set_transport_speed_tool_tool_attr(),
                            |context| Box::pin(
                                ArdourService::set_transport_speed_tool_tool_call(context),
                            ),
                        ),
                    );
                tool_box
                    .add(
                        ToolBoxItem::new(
                            ArdourService::locate_tool_tool_attr(),
                            |context| Box::pin(
                                ArdourService::locate_tool_tool_call(context),
                            ),
                        ),
                    );
                tool_box
                    .add(
                        ToolBoxItem::new(
                            ArdourService::set_track_solo_tool_tool_attr(),
                            |context| Box::pin(
                                ArdourService::set_track_solo_tool_tool_call(context),
                            ),
                        ),
                    );
                tool_box
                    .add(
                        ToolBoxItem::new(
                            ArdourService::set_track_rec_enable_tool_tool_attr(),
                            |context| Box::pin(
                                ArdourService::set_track_rec_enable_tool_tool_call(context),
                            ),
                        ),
                    );
                tool_box
                    .add(
                        ToolBoxItem::new(
                            ArdourService::set_track_gain_abs_tool_tool_attr(),
                            |context| Box::pin(
                                ArdourService::set_track_gain_abs_tool_tool_call(context),
                            ),
                        ),
                    );
                tool_box
                    .add(
                        ToolBoxItem::new(
                            ArdourService::set_track_gain_db_tool_tool_attr(),
                            |context| Box::pin(
                                ArdourService::set_track_gain_db_tool_tool_call(context),
                            ),
                        ),
                    );
                tool_box
                    .add(
                        ToolBoxItem::new(
                            ArdourService::set_track_trim_abs_tool_tool_attr(),
                            |context| Box::pin(
                                ArdourService::set_track_trim_abs_tool_tool_call(context),
                            ),
                        ),
                    );
                tool_box
                    .add(
                        ToolBoxItem::new(
                            ArdourService::set_track_trim_db_tool_tool_attr(),
                            |context| Box::pin(
                                ArdourService::set_track_trim_db_tool_tool_call(context),
                            ),
                        ),
                    );
                tool_box
                    .add(
                        ToolBoxItem::new(
                            ArdourService::access_action_tool_tool_attr(),
                            |context| Box::pin(
                                ArdourService::access_action_tool_tool_call(context),
                            ),
                        ),
                    );
                tool_box
            })
    }
}
impl ServerHandler for ArdourService {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            server_info: Implementation {
                name: "ardour-mcp-server".to_string(),
                version: "0.1.0".to_string(),
            },
            capabilities: ServerCapabilities {
                tools: Some(ToolsCapability {
                    list_changed: Some(false),
                }),
                resources: Some(ResourcesCapability {
                    subscribe: Some(false),
                    list_changed: Some(false),
                }),
                prompts: None,
                experimental: None,
                logging: None,
            },
            instructions: Some("Ardour MCP server for OSC control.".to_string()),
        }
    }
    async fn list_resources(
        &self,
        _request: PaginatedRequestParam,
        _ctx: RequestContext<RoleServer>,
    ) -> Result<ListResourcesResult, McpError> {
        let mut raw_playback_state_resource = RawResource::new(
            "ardour:/state/playback",
            "Ardour Playback State",
        );
        raw_playback_state_resource.description = Some(
            "Current playback state of Ardour (e.g., Playing, Stopped, Unknown)."
                .to_string(),
        );
        let playback_state_resource: Resource = raw_playback_state_resource
            .no_annotation();
        let mut raw_transport_frame_resource = RawResource::new(
            "ardour:/state/transport_frame",
            "Ardour Transport Frame Position",
        );
        raw_transport_frame_resource.description = Some(
            "Current playhead position in samples. Returns 'Unknown' if not yet reported by Ardour."
                .to_string(),
        );
        let transport_frame_resource: Resource = raw_transport_frame_resource
            .no_annotation();
        let all_resources = <[_]>::into_vec(
            ::alloc::boxed::box_new([playback_state_resource, transport_frame_resource]),
        );
        {
            use ::tracing::__macro_support::Callsite as _;
            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                static META: ::tracing::Metadata<'static> = {
                    ::tracing_core::metadata::Metadata::new(
                        "event src/main.rs:742",
                        "ardour_mcp_server",
                        ::tracing::Level::DEBUG,
                        ::tracing_core::__macro_support::Option::Some("src/main.rs"),
                        ::tracing_core::__macro_support::Option::Some(742u32),
                        ::tracing_core::__macro_support::Option::Some(
                            "ardour_mcp_server",
                        ),
                        ::tracing_core::field::FieldSet::new(
                            &["message"],
                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                        ),
                        ::tracing::metadata::Kind::EVENT,
                    )
                };
                ::tracing::callsite::DefaultCallsite::new(&META)
            };
            let enabled = ::tracing::Level::DEBUG
                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                && ::tracing::Level::DEBUG
                    <= ::tracing::level_filters::LevelFilter::current()
                && {
                    let interest = __CALLSITE.interest();
                    !interest.is_never()
                        && ::tracing::__macro_support::__is_enabled(
                            __CALLSITE.metadata(),
                            interest,
                        )
                };
            if enabled {
                (|value_set: ::tracing::field::ValueSet| {
                    let meta = __CALLSITE.metadata();
                    ::tracing::Event::dispatch(meta, &value_set);
                })({
                    #[allow(unused_imports)]
                    use ::tracing::field::{debug, display, Value};
                    let mut iter = __CALLSITE.metadata().fields().iter();
                    __CALLSITE
                        .metadata()
                        .fields()
                        .value_set(
                            &[
                                (
                                    &::tracing::__macro_support::Iterator::next(&mut iter)
                                        .expect("FieldSet corrupted (this is a bug)"),
                                    ::tracing::__macro_support::Option::Some(
                                        &format_args!(
                                            "Listing resources. Count: {0}, Content: {1:?}",
                                            all_resources.len(),
                                            all_resources,
                                        ) as &dyn Value,
                                    ),
                                ),
                            ],
                        )
                });
            } else {
            }
        };
        Ok(ListResourcesResult {
            resources: all_resources,
            next_cursor: None,
        })
    }
    async fn read_resource(
        &self,
        request: ReadResourceRequestParam,
        _ctx: RequestContext<RoleServer>,
    ) -> Result<ReadResourceResult, McpError> {
        let resource_uri = request.uri.as_str();
        match resource_uri {
            "ardour:/state/playback" => {
                let state = self.ardour_state.lock().await;
                let status_str = match state.playback_status {
                    PlaybackStatus::Playing => "Playing",
                    PlaybackStatus::Stopped => "Stopped",
                    PlaybackStatus::Unknown => "Unknown",
                };
                Ok(ReadResourceResult {
                    contents: <[_]>::into_vec(
                        ::alloc::boxed::box_new([
                            rmcp::model::ResourceContents::TextResourceContents {
                                uri: resource_uri.to_string(),
                                mime_type: Some("text/plain".to_string()),
                                text: status_str.to_string(),
                            },
                        ]),
                    ),
                })
            }
            "ardour:/strip/list" => {
                let state = self.ardour_state.lock().await;
                let valid_strips: Vec<&TrackInfo> = state
                    .strip_list
                    .iter()
                    .filter(|ti| ti.id != 0)
                    .collect();
                match serde_json::to_string_pretty(&valid_strips) {
                    Ok(json_response) => {
                        Ok(ReadResourceResult {
                            contents: <[_]>::into_vec(
                                ::alloc::boxed::box_new([
                                    rmcp::model::ResourceContents::TextResourceContents {
                                        uri: resource_uri.to_string(),
                                        mime_type: Some("application/json".to_string()),
                                        text: json_response,
                                    },
                                ]),
                            ),
                        })
                    }
                    Err(e) => {
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event src/main.rs:785",
                                        "ardour_mcp_server",
                                        ::tracing::Level::ERROR,
                                        ::tracing_core::__macro_support::Option::Some(
                                            "src/main.rs",
                                        ),
                                        ::tracing_core::__macro_support::Option::Some(785u32),
                                        ::tracing_core::__macro_support::Option::Some(
                                            "ardour_mcp_server",
                                        ),
                                        ::tracing_core::field::FieldSet::new(
                                            &["message"],
                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::EVENT,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let enabled = ::tracing::Level::ERROR
                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && ::tracing::Level::ERROR
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    let interest = __CALLSITE.interest();
                                    !interest.is_never()
                                        && ::tracing::__macro_support::__is_enabled(
                                            __CALLSITE.metadata(),
                                            interest,
                                        )
                                };
                            if enabled {
                                (|value_set: ::tracing::field::ValueSet| {
                                    let meta = __CALLSITE.metadata();
                                    ::tracing::Event::dispatch(meta, &value_set);
                                })({
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    let mut iter = __CALLSITE.metadata().fields().iter();
                                    __CALLSITE
                                        .metadata()
                                        .fields()
                                        .value_set(
                                            &[
                                                (
                                                    &::tracing::__macro_support::Iterator::next(&mut iter)
                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                    ::tracing::__macro_support::Option::Some(
                                                        &format_args!("Failed to serialize strip list: {0}", e)
                                                            as &dyn Value,
                                                    ),
                                                ),
                                            ],
                                        )
                                });
                            } else {
                            }
                        };
                        Err(
                            McpError::internal_error(
                                ::alloc::__export::must_use({
                                    ::alloc::fmt::format(
                                        format_args!("Failed to serialize strip list: {0}", e),
                                    )
                                }),
                                None,
                            ),
                        )
                    }
                }
            }
            "ardour:/state/transport_frame" => {
                let state = self.ardour_state.lock().await;
                let frame_str = match state.transport_frame {
                    Some(frame) => frame.to_string(),
                    None => "Unknown".to_string(),
                };
                Ok(ReadResourceResult {
                    contents: <[_]>::into_vec(
                        ::alloc::boxed::box_new([
                            rmcp::model::ResourceContents::TextResourceContents {
                                uri: resource_uri.to_string(),
                                mime_type: Some("text/plain".to_string()),
                                text: frame_str,
                            },
                        ]),
                    ),
                })
            }
            "ardour:/action/list" => {
                let placeholder_actions = ::serde_json::Value::Array(
                    <[_]>::into_vec(
                        ::alloc::boxed::box_new([
                            ::serde_json::Value::Object({
                                let mut object = ::serde_json::Map::new();
                                let _ = object
                                    .insert(
                                        ("name").into(),
                                        ::serde_json::to_value(&"Session/Save").unwrap(),
                                    );
                                let _ = object
                                    .insert(
                                        ("description").into(),
                                        ::serde_json::to_value(&"Saves the current session.")
                                            .unwrap(),
                                    );
                                object
                            }),
                            ::serde_json::Value::Object({
                                let mut object = ::serde_json::Map::new();
                                let _ = object
                                    .insert(
                                        ("name").into(),
                                        ::serde_json::to_value(&"Editor/zoom-to-session").unwrap(),
                                    );
                                let _ = object
                                    .insert(
                                        ("description").into(),
                                        ::serde_json::to_value(&"Zooms to fit the entire session.")
                                            .unwrap(),
                                    );
                                object
                            }),
                            ::serde_json::Value::Object({
                                let mut object = ::serde_json::Map::new();
                                let _ = object
                                    .insert(
                                        ("name").into(),
                                        ::serde_json::to_value(&"Transport/Loop").unwrap(),
                                    );
                                let _ = object
                                    .insert(
                                        ("description").into(),
                                        ::serde_json::to_value(&"Toggles loop playback.").unwrap(),
                                    );
                                object
                            }),
                        ]),
                    ),
                );
                Ok(ReadResourceResult {
                    contents: <[_]>::into_vec(
                        ::alloc::boxed::box_new([
                            rmcp::model::ResourceContents::TextResourceContents {
                                uri: resource_uri.to_string(),
                                mime_type: Some("application/json".to_string()),
                                text: placeholder_actions.to_string(),
                            },
                        ]),
                    ),
                })
            }
            _ => {
                Err(
                    McpError::resource_not_found(
                        ::alloc::__export::must_use({
                            ::alloc::fmt::format(
                                format_args!(
                                    "Resource URI \'{0}\' not found.",
                                    resource_uri,
                                ),
                            )
                        }),
                        Some(
                            ::serde_json::Value::Object({
                                let mut object = ::serde_json::Map::new();
                                let _ = object
                                    .insert(
                                        ("uri").into(),
                                        ::serde_json::to_value(&resource_uri).unwrap(),
                                    );
                                object
                            }),
                        ),
                    ),
                )
            }
        }
    }
    async fn list_prompts(
        &self,
        _request: PaginatedRequestParam,
        _ctx: RequestContext<RoleServer>,
    ) -> Result<ListPromptsResult, McpError> {
        Ok(ListPromptsResult {
            prompts: ::alloc::vec::Vec::new(),
            next_cursor: None,
        })
    }
    async fn get_prompt(
        &self,
        _req: GetPromptRequestParam,
        _ctx: RequestContext<RoleServer>,
    ) -> Result<GetPromptResult, McpError> {
        Err(McpError::invalid_params("Prompt not found", None))
    }
    async fn list_tools(
        &self,
        _: ::rmcp::model::PaginatedRequestParam,
        _: ::rmcp::service::RequestContext<::rmcp::service::RoleServer>,
    ) -> Result<::rmcp::model::ListToolsResult, ::rmcp::Error> {
        Ok(::rmcp::model::ListToolsResult {
            next_cursor: None,
            tools: Self::tool_box().list(),
        })
    }
    async fn call_tool(
        &self,
        call_tool_request_param: ::rmcp::model::CallToolRequestParam,
        context: ::rmcp::service::RequestContext<::rmcp::service::RoleServer>,
    ) -> Result<::rmcp::model::CallToolResult, ::rmcp::Error> {
        let context = ::rmcp::handler::server::tool::ToolCallContext::new(
            self,
            call_tool_request_param,
            context,
        );
        Self::tool_box().call(context).await
    }
}
fn main() -> Result<()> {
    let body = async {
        let log_dir = Path::new("logs");
        if !log_dir.exists() {
            std::fs::create_dir_all(log_dir)?;
        }
        let log_file_path = log_dir.join("ardour_mcp_server.log");
        let log_file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(log_file_path)?;
        let stderr_writer = std::io::stderr.with_max_level(tracing::Level::INFO);
        let file_writer = log_file.with_max_level(tracing::Level::DEBUG);
        let combined_writer = stderr_writer.and(file_writer);
        tracing_subscriber::fmt()
            .with_env_filter(
                tracing_subscriber::EnvFilter::from_default_env()
                    .add_directive(tracing::Level::DEBUG.into()),
            )
            .with_writer(combined_writer)
            .with_ansi(true)
            .init();
        {
            use ::tracing::__macro_support::Callsite as _;
            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                static META: ::tracing::Metadata<'static> = {
                    ::tracing_core::metadata::Metadata::new(
                        "event src/main.rs:877",
                        "ardour_mcp_server",
                        ::tracing::Level::INFO,
                        ::tracing_core::__macro_support::Option::Some("src/main.rs"),
                        ::tracing_core::__macro_support::Option::Some(877u32),
                        ::tracing_core::__macro_support::Option::Some(
                            "ardour_mcp_server",
                        ),
                        ::tracing_core::field::FieldSet::new(
                            &["message"],
                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                        ),
                        ::tracing::metadata::Kind::EVENT,
                    )
                };
                ::tracing::callsite::DefaultCallsite::new(&META)
            };
            let enabled = ::tracing::Level::INFO
                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                && ::tracing::Level::INFO
                    <= ::tracing::level_filters::LevelFilter::current()
                && {
                    let interest = __CALLSITE.interest();
                    !interest.is_never()
                        && ::tracing::__macro_support::__is_enabled(
                            __CALLSITE.metadata(),
                            interest,
                        )
                };
            if enabled {
                (|value_set: ::tracing::field::ValueSet| {
                    let meta = __CALLSITE.metadata();
                    ::tracing::Event::dispatch(meta, &value_set);
                })({
                    #[allow(unused_imports)]
                    use ::tracing::field::{debug, display, Value};
                    let mut iter = __CALLSITE.metadata().fields().iter();
                    __CALLSITE
                        .metadata()
                        .fields()
                        .value_set(
                            &[
                                (
                                    &::tracing::__macro_support::Iterator::next(&mut iter)
                                        .expect("FieldSet corrupted (this is a bug)"),
                                    ::tracing::__macro_support::Option::Some(
                                        &format_args!(
                                            "\n======================================================================\nNEW SERVER RUN: {0}\n======================================================================",
                                            chrono::Local::now().to_rfc2822(),
                                        ) as &dyn Value,
                                    ),
                                ),
                            ],
                        )
                });
            } else {
            }
        };
        let ardour_service = ArdourService::new()?;
        if let Err(e) = ardour_service.send_osc_setup_to_ardour().await {
            {
                use ::tracing::__macro_support::Callsite as _;
                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                    static META: ::tracing::Metadata<'static> = {
                        ::tracing_core::metadata::Metadata::new(
                            "event src/main.rs:883",
                            "ardour_mcp_server",
                            ::tracing::Level::WARN,
                            ::tracing_core::__macro_support::Option::Some("src/main.rs"),
                            ::tracing_core::__macro_support::Option::Some(883u32),
                            ::tracing_core::__macro_support::Option::Some(
                                "ardour_mcp_server",
                            ),
                            ::tracing_core::field::FieldSet::new(
                                &["message"],
                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                            ),
                            ::tracing::metadata::Kind::EVENT,
                        )
                    };
                    ::tracing::callsite::DefaultCallsite::new(&META)
                };
                let enabled = ::tracing::Level::WARN
                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                    && ::tracing::Level::WARN
                        <= ::tracing::level_filters::LevelFilter::current()
                    && {
                        let interest = __CALLSITE.interest();
                        !interest.is_never()
                            && ::tracing::__macro_support::__is_enabled(
                                __CALLSITE.metadata(),
                                interest,
                            )
                    };
                if enabled {
                    (|value_set: ::tracing::field::ValueSet| {
                        let meta = __CALLSITE.metadata();
                        ::tracing::Event::dispatch(meta, &value_set);
                    })({
                        #[allow(unused_imports)]
                        use ::tracing::field::{debug, display, Value};
                        let mut iter = __CALLSITE.metadata().fields().iter();
                        __CALLSITE
                            .metadata()
                            .fields()
                            .value_set(
                                &[
                                    (
                                        &::tracing::__macro_support::Iterator::next(&mut iter)
                                            .expect("FieldSet corrupted (this is a bug)"),
                                        ::tracing::__macro_support::Option::Some(
                                            &format_args!(
                                                "Could not send initial OSC setup to Ardour: {0}. Feedback might not work.",
                                                e,
                                            ) as &dyn Value,
                                        ),
                                    ),
                                ],
                            )
                    });
                } else {
                }
            };
        }
        let ardour_state_clone = Arc::clone(&ardour_service.ardour_state);
        let server_process = ardour_service
            .serve(stdio())
            .await
            .inspect_err(|e| {
                {
                    use ::tracing::__macro_support::Callsite as _;
                    static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "event src/main.rs:890",
                                "ardour_mcp_server",
                                ::tracing::Level::ERROR,
                                ::tracing_core::__macro_support::Option::Some(
                                    "src/main.rs",
                                ),
                                ::tracing_core::__macro_support::Option::Some(890u32),
                                ::tracing_core::__macro_support::Option::Some(
                                    "ardour_mcp_server",
                                ),
                                ::tracing_core::field::FieldSet::new(
                                    &["message"],
                                    ::tracing_core::callsite::Identifier(&__CALLSITE),
                                ),
                                ::tracing::metadata::Kind::EVENT,
                            )
                        };
                        ::tracing::callsite::DefaultCallsite::new(&META)
                    };
                    let enabled = ::tracing::Level::ERROR
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && ::tracing::Level::ERROR
                            <= ::tracing::level_filters::LevelFilter::current()
                        && {
                            let interest = __CALLSITE.interest();
                            !interest.is_never()
                                && ::tracing::__macro_support::__is_enabled(
                                    __CALLSITE.metadata(),
                                    interest,
                                )
                        };
                    if enabled {
                        (|value_set: ::tracing::field::ValueSet| {
                            let meta = __CALLSITE.metadata();
                            ::tracing::Event::dispatch(meta, &value_set);
                        })({
                            #[allow(unused_imports)]
                            use ::tracing::field::{debug, display, Value};
                            let mut iter = __CALLSITE.metadata().fields().iter();
                            __CALLSITE
                                .metadata()
                                .fields()
                                .value_set(
                                    &[
                                        (
                                            &::tracing::__macro_support::Iterator::next(&mut iter)
                                                .expect("FieldSet corrupted (this is a bug)"),
                                            ::tracing::__macro_support::Option::Some(
                                                &format_args!("MCP Server serving error: {0:?}", e)
                                                    as &dyn Value,
                                            ),
                                        ),
                                    ],
                                )
                        });
                    } else {
                    }
                };
            })?;
        {
            use ::tracing::__macro_support::Callsite as _;
            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                static META: ::tracing::Metadata<'static> = {
                    ::tracing_core::metadata::Metadata::new(
                        "event src/main.rs:893",
                        "ardour_mcp_server",
                        ::tracing::Level::INFO,
                        ::tracing_core::__macro_support::Option::Some("src/main.rs"),
                        ::tracing_core::__macro_support::Option::Some(893u32),
                        ::tracing_core::__macro_support::Option::Some(
                            "ardour_mcp_server",
                        ),
                        ::tracing_core::field::FieldSet::new(
                            &["message"],
                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                        ),
                        ::tracing::metadata::Kind::EVENT,
                    )
                };
                ::tracing::callsite::DefaultCallsite::new(&META)
            };
            let enabled = ::tracing::Level::INFO
                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                && ::tracing::Level::INFO
                    <= ::tracing::level_filters::LevelFilter::current()
                && {
                    let interest = __CALLSITE.interest();
                    !interest.is_never()
                        && ::tracing::__macro_support::__is_enabled(
                            __CALLSITE.metadata(),
                            interest,
                        )
                };
            if enabled {
                (|value_set: ::tracing::field::ValueSet| {
                    let meta = __CALLSITE.metadata();
                    ::tracing::Event::dispatch(meta, &value_set);
                })({
                    #[allow(unused_imports)]
                    use ::tracing::field::{debug, display, Value};
                    let mut iter = __CALLSITE.metadata().fields().iter();
                    __CALLSITE
                        .metadata()
                        .fields()
                        .value_set(
                            &[
                                (
                                    &::tracing::__macro_support::Iterator::next(&mut iter)
                                        .expect("FieldSet corrupted (this is a bug)"),
                                    ::tracing::__macro_support::Option::Some(
                                        &format_args!(
                                            "Ardour MCP server started and waiting for connections...",
                                        ) as &dyn Value,
                                    ),
                                ),
                            ],
                        )
                });
            } else {
            }
        };
        let _osc_listener_handle = tokio::spawn(async move {
            if let Err(e) = listen_ardour_osc_events(ardour_state_clone).await {
                {
                    use ::tracing::__macro_support::Callsite as _;
                    static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "event src/main.rs:897",
                                "ardour_mcp_server",
                                ::tracing::Level::ERROR,
                                ::tracing_core::__macro_support::Option::Some(
                                    "src/main.rs",
                                ),
                                ::tracing_core::__macro_support::Option::Some(897u32),
                                ::tracing_core::__macro_support::Option::Some(
                                    "ardour_mcp_server",
                                ),
                                ::tracing_core::field::FieldSet::new(
                                    &["message"],
                                    ::tracing_core::callsite::Identifier(&__CALLSITE),
                                ),
                                ::tracing::metadata::Kind::EVENT,
                            )
                        };
                        ::tracing::callsite::DefaultCallsite::new(&META)
                    };
                    let enabled = ::tracing::Level::ERROR
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && ::tracing::Level::ERROR
                            <= ::tracing::level_filters::LevelFilter::current()
                        && {
                            let interest = __CALLSITE.interest();
                            !interest.is_never()
                                && ::tracing::__macro_support::__is_enabled(
                                    __CALLSITE.metadata(),
                                    interest,
                                )
                        };
                    if enabled {
                        (|value_set: ::tracing::field::ValueSet| {
                            let meta = __CALLSITE.metadata();
                            ::tracing::Event::dispatch(meta, &value_set);
                        })({
                            #[allow(unused_imports)]
                            use ::tracing::field::{debug, display, Value};
                            let mut iter = __CALLSITE.metadata().fields().iter();
                            __CALLSITE
                                .metadata()
                                .fields()
                                .value_set(
                                    &[
                                        (
                                            &::tracing::__macro_support::Iterator::next(&mut iter)
                                                .expect("FieldSet corrupted (this is a bug)"),
                                            ::tracing::__macro_support::Option::Some(
                                                &format_args!("OSC listener task failed: {0:?}", e)
                                                    as &dyn Value,
                                            ),
                                        ),
                                    ],
                                )
                        });
                    } else {
                    }
                };
            }
        });
        server_process.waiting().await?;
        {
            use ::tracing::__macro_support::Callsite as _;
            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                static META: ::tracing::Metadata<'static> = {
                    ::tracing_core::metadata::Metadata::new(
                        "event src/main.rs:903",
                        "ardour_mcp_server",
                        ::tracing::Level::INFO,
                        ::tracing_core::__macro_support::Option::Some("src/main.rs"),
                        ::tracing_core::__macro_support::Option::Some(903u32),
                        ::tracing_core::__macro_support::Option::Some(
                            "ardour_mcp_server",
                        ),
                        ::tracing_core::field::FieldSet::new(
                            &["message"],
                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                        ),
                        ::tracing::metadata::Kind::EVENT,
                    )
                };
                ::tracing::callsite::DefaultCallsite::new(&META)
            };
            let enabled = ::tracing::Level::INFO
                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                && ::tracing::Level::INFO
                    <= ::tracing::level_filters::LevelFilter::current()
                && {
                    let interest = __CALLSITE.interest();
                    !interest.is_never()
                        && ::tracing::__macro_support::__is_enabled(
                            __CALLSITE.metadata(),
                            interest,
                        )
                };
            if enabled {
                (|value_set: ::tracing::field::ValueSet| {
                    let meta = __CALLSITE.metadata();
                    ::tracing::Event::dispatch(meta, &value_set);
                })({
                    #[allow(unused_imports)]
                    use ::tracing::field::{debug, display, Value};
                    let mut iter = __CALLSITE.metadata().fields().iter();
                    __CALLSITE
                        .metadata()
                        .fields()
                        .value_set(
                            &[
                                (
                                    &::tracing::__macro_support::Iterator::next(&mut iter)
                                        .expect("FieldSet corrupted (this is a bug)"),
                                    ::tracing::__macro_support::Option::Some(
                                        &format_args!("Ardour MCP server stopped.") as &dyn Value,
                                    ),
                                ),
                            ],
                        )
                });
            } else {
            }
        };
        Ok(())
    };
    #[allow(
        clippy::expect_used,
        clippy::diverging_sub_expression,
        clippy::needless_return
    )]
    {
        return tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed building the Runtime")
            .block_on(body);
    }
}
async fn listen_ardour_osc_events(state: Arc<Mutex<ArdourState>>) -> Result<()> {
    {
        use ::tracing::__macro_support::Callsite as _;
        static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
            static META: ::tracing::Metadata<'static> = {
                ::tracing_core::metadata::Metadata::new(
                    "event src/main.rs:909",
                    "ardour_mcp_server",
                    ::tracing::Level::INFO,
                    ::tracing_core::__macro_support::Option::Some("src/main.rs"),
                    ::tracing_core::__macro_support::Option::Some(909u32),
                    ::tracing_core::__macro_support::Option::Some("ardour_mcp_server"),
                    ::tracing_core::field::FieldSet::new(
                        &["message"],
                        ::tracing_core::callsite::Identifier(&__CALLSITE),
                    ),
                    ::tracing::metadata::Kind::EVENT,
                )
            };
            ::tracing::callsite::DefaultCallsite::new(&META)
        };
        let enabled = ::tracing::Level::INFO
            <= ::tracing::level_filters::STATIC_MAX_LEVEL
            && ::tracing::Level::INFO <= ::tracing::level_filters::LevelFilter::current()
            && {
                let interest = __CALLSITE.interest();
                !interest.is_never()
                    && ::tracing::__macro_support::__is_enabled(
                        __CALLSITE.metadata(),
                        interest,
                    )
            };
        if enabled {
            (|value_set: ::tracing::field::ValueSet| {
                let meta = __CALLSITE.metadata();
                ::tracing::Event::dispatch(meta, &value_set);
            })({
                #[allow(unused_imports)]
                use ::tracing::field::{debug, display, Value};
                let mut iter = __CALLSITE.metadata().fields().iter();
                __CALLSITE
                    .metadata()
                    .fields()
                    .value_set(
                        &[
                            (
                                &::tracing::__macro_support::Iterator::next(&mut iter)
                                    .expect("FieldSet corrupted (this is a bug)"),
                                ::tracing::__macro_support::Option::Some(
                                    &format_args!(
                                        "Starting OSC listener for Ardour events on {0}",
                                        MCP_SERVER_OSC_LISTEN_ADDR,
                                    ) as &dyn Value,
                                ),
                            ),
                        ],
                    )
            });
        } else {
        }
    };
    let listen_socket = tokio::net::UdpSocket::bind(MCP_SERVER_OSC_LISTEN_ADDR)
        .await
        .map_err(|e| ::anyhow::Error::msg(
            ::alloc::__export::must_use({
                ::alloc::fmt::format(
                    format_args!(
                        "Failed to bind Tokio UDP socket for OSC on {0}: {1}",
                        MCP_SERVER_OSC_LISTEN_ADDR,
                        e,
                    ),
                )
            }),
        ))?;
    {
        use ::tracing::__macro_support::Callsite as _;
        static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
            static META: ::tracing::Metadata<'static> = {
                ::tracing_core::metadata::Metadata::new(
                    "event src/main.rs:918",
                    "ardour_mcp_server",
                    ::tracing::Level::INFO,
                    ::tracing_core::__macro_support::Option::Some("src/main.rs"),
                    ::tracing_core::__macro_support::Option::Some(918u32),
                    ::tracing_core::__macro_support::Option::Some("ardour_mcp_server"),
                    ::tracing_core::field::FieldSet::new(
                        &["message"],
                        ::tracing_core::callsite::Identifier(&__CALLSITE),
                    ),
                    ::tracing::metadata::Kind::EVENT,
                )
            };
            ::tracing::callsite::DefaultCallsite::new(&META)
        };
        let enabled = ::tracing::Level::INFO
            <= ::tracing::level_filters::STATIC_MAX_LEVEL
            && ::tracing::Level::INFO <= ::tracing::level_filters::LevelFilter::current()
            && {
                let interest = __CALLSITE.interest();
                !interest.is_never()
                    && ::tracing::__macro_support::__is_enabled(
                        __CALLSITE.metadata(),
                        interest,
                    )
            };
        if enabled {
            (|value_set: ::tracing::field::ValueSet| {
                let meta = __CALLSITE.metadata();
                ::tracing::Event::dispatch(meta, &value_set);
            })({
                #[allow(unused_imports)]
                use ::tracing::field::{debug, display, Value};
                let mut iter = __CALLSITE.metadata().fields().iter();
                __CALLSITE
                    .metadata()
                    .fields()
                    .value_set(
                        &[
                            (
                                &::tracing::__macro_support::Iterator::next(&mut iter)
                                    .expect("FieldSet corrupted (this is a bug)"),
                                ::tracing::__macro_support::Option::Some(
                                    &format_args!(
                                        "Tokio UDP socket for OSC bound to {0}",
                                        MCP_SERVER_OSC_LISTEN_ADDR,
                                    ) as &dyn Value,
                                ),
                            ),
                        ],
                    )
            });
        } else {
        }
    };
    let mut buf = [0u8; osc::recv::DEFAULT_MTU];
    loop {
        match listen_socket.recv_from(&mut buf).await {
            Ok((size, peer_addr)) => {
                let packet = osc::decode(&buf[..size])
                    .map_err(|e| ::anyhow::Error::msg(
                        ::alloc::__export::must_use({
                            ::alloc::fmt::format(
                                format_args!("OSC decode error from {0}: {1}", peer_addr, e),
                            )
                        }),
                    ))?;
                handle_osc_packet(packet, peer_addr, Arc::clone(&state)).await;
            }
            Err(e) => {
                {
                    use ::tracing::__macro_support::Callsite as _;
                    static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "event src/main.rs:931",
                                "ardour_mcp_server",
                                ::tracing::Level::ERROR,
                                ::tracing_core::__macro_support::Option::Some(
                                    "src/main.rs",
                                ),
                                ::tracing_core::__macro_support::Option::Some(931u32),
                                ::tracing_core::__macro_support::Option::Some(
                                    "ardour_mcp_server",
                                ),
                                ::tracing_core::field::FieldSet::new(
                                    &["message"],
                                    ::tracing_core::callsite::Identifier(&__CALLSITE),
                                ),
                                ::tracing::metadata::Kind::EVENT,
                            )
                        };
                        ::tracing::callsite::DefaultCallsite::new(&META)
                    };
                    let enabled = ::tracing::Level::ERROR
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && ::tracing::Level::ERROR
                            <= ::tracing::level_filters::LevelFilter::current()
                        && {
                            let interest = __CALLSITE.interest();
                            !interest.is_never()
                                && ::tracing::__macro_support::__is_enabled(
                                    __CALLSITE.metadata(),
                                    interest,
                                )
                        };
                    if enabled {
                        (|value_set: ::tracing::field::ValueSet| {
                            let meta = __CALLSITE.metadata();
                            ::tracing::Event::dispatch(meta, &value_set);
                        })({
                            #[allow(unused_imports)]
                            use ::tracing::field::{debug, display, Value};
                            let mut iter = __CALLSITE.metadata().fields().iter();
                            __CALLSITE
                                .metadata()
                                .fields()
                                .value_set(
                                    &[
                                        (
                                            &::tracing::__macro_support::Iterator::next(&mut iter)
                                                .expect("FieldSet corrupted (this is a bug)"),
                                            ::tracing::__macro_support::Option::Some(
                                                &format_args!(
                                                    "OSC recv_from error: {0}. Listener might stop.",
                                                    e,
                                                ) as &dyn Value,
                                            ),
                                        ),
                                    ],
                                )
                        });
                    } else {
                    }
                };
                break Err(
                    ::anyhow::Error::msg(
                        ::alloc::__export::must_use({
                            ::alloc::fmt::format(
                                format_args!("OSC recv_from error: {0}", e),
                            )
                        }),
                    ),
                );
            }
        }
    }
}
async fn handle_osc_packet(
    packet: osc::Packet,
    peer_addr: std::net::SocketAddr,
    state: Arc<Mutex<ArdourState>>,
) {
    match packet {
        osc::Packet::Message(msg) => {
            {
                use ::tracing::__macro_support::Callsite as _;
                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                    static META: ::tracing::Metadata<'static> = {
                        ::tracing_core::metadata::Metadata::new(
                            "event src/main.rs:943",
                            "ardour_mcp_server",
                            ::tracing::Level::DEBUG,
                            ::tracing_core::__macro_support::Option::Some("src/main.rs"),
                            ::tracing_core::__macro_support::Option::Some(943u32),
                            ::tracing_core::__macro_support::Option::Some(
                                "ardour_mcp_server",
                            ),
                            ::tracing_core::field::FieldSet::new(
                                &["message"],
                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                            ),
                            ::tracing::metadata::Kind::EVENT,
                        )
                    };
                    ::tracing::callsite::DefaultCallsite::new(&META)
                };
                let enabled = ::tracing::Level::DEBUG
                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                    && ::tracing::Level::DEBUG
                        <= ::tracing::level_filters::LevelFilter::current()
                    && {
                        let interest = __CALLSITE.interest();
                        !interest.is_never()
                            && ::tracing::__macro_support::__is_enabled(
                                __CALLSITE.metadata(),
                                interest,
                            )
                    };
                if enabled {
                    (|value_set: ::tracing::field::ValueSet| {
                        let meta = __CALLSITE.metadata();
                        ::tracing::Event::dispatch(meta, &value_set);
                    })({
                        #[allow(unused_imports)]
                        use ::tracing::field::{debug, display, Value};
                        let mut iter = __CALLSITE.metadata().fields().iter();
                        __CALLSITE
                            .metadata()
                            .fields()
                            .value_set(
                                &[
                                    (
                                        &::tracing::__macro_support::Iterator::next(&mut iter)
                                            .expect("FieldSet corrupted (this is a bug)"),
                                        ::tracing::__macro_support::Option::Some(
                                            &format_args!(
                                                "Received OSC message from {0}: {1} {2:?}",
                                                peer_addr,
                                                msg.addr,
                                                msg.args,
                                            ) as &dyn Value,
                                        ),
                                    ),
                                ],
                            )
                    });
                } else {
                }
            };
            if msg.addr.starts_with("/strip/name/") {
                let parts: Vec<&str> = msg.addr.split('/').collect();
                if parts.len() == 4 {
                    if let Ok(ssid) = parts[3].parse::<i32>() {
                        if ssid > 0 {
                            if let Some(osc::Type::String(name)) = msg.args.get(0) {
                                {
                                    use ::tracing::__macro_support::Callsite as _;
                                    static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                        static META: ::tracing::Metadata<'static> = {
                                            ::tracing_core::metadata::Metadata::new(
                                                "event src/main.rs:952",
                                                "ardour_mcp_server",
                                                ::tracing::Level::INFO,
                                                ::tracing_core::__macro_support::Option::Some(
                                                    "src/main.rs",
                                                ),
                                                ::tracing_core::__macro_support::Option::Some(952u32),
                                                ::tracing_core::__macro_support::Option::Some(
                                                    "ardour_mcp_server",
                                                ),
                                                ::tracing_core::field::FieldSet::new(
                                                    &["message"],
                                                    ::tracing_core::callsite::Identifier(&__CALLSITE),
                                                ),
                                                ::tracing::metadata::Kind::EVENT,
                                            )
                                        };
                                        ::tracing::callsite::DefaultCallsite::new(&META)
                                    };
                                    let enabled = ::tracing::Level::INFO
                                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                        && ::tracing::Level::INFO
                                            <= ::tracing::level_filters::LevelFilter::current()
                                        && {
                                            let interest = __CALLSITE.interest();
                                            !interest.is_never()
                                                && ::tracing::__macro_support::__is_enabled(
                                                    __CALLSITE.metadata(),
                                                    interest,
                                                )
                                        };
                                    if enabled {
                                        (|value_set: ::tracing::field::ValueSet| {
                                            let meta = __CALLSITE.metadata();
                                            ::tracing::Event::dispatch(meta, &value_set);
                                        })({
                                            #[allow(unused_imports)]
                                            use ::tracing::field::{debug, display, Value};
                                            let mut iter = __CALLSITE.metadata().fields().iter();
                                            __CALLSITE
                                                .metadata()
                                                .fields()
                                                .value_set(
                                                    &[
                                                        (
                                                            &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                            ::tracing::__macro_support::Option::Some(
                                                                &format_args!(
                                                                    "Ardour feedback: /strip/name/{0} -> {1}",
                                                                    ssid,
                                                                    name,
                                                                ) as &dyn Value,
                                                            ),
                                                        ),
                                                    ],
                                                )
                                        });
                                    } else {
                                    }
                                };
                                let mut current_state = state.lock().await;
                                let vec_idx = (ssid - 1) as usize;
                                if vec_idx >= current_state.strip_list.len() {
                                    current_state
                                        .strip_list
                                        .resize_with(
                                            vec_idx + 1,
                                            || TrackInfo {
                                                id: 0,
                                                name: String::new(),
                                                track_type: "unknown".to_string(),
                                            },
                                        );
                                }
                                let strip_info = &mut current_state.strip_list[vec_idx];
                                strip_info.id = ssid;
                                strip_info.name = name.clone();
                                {
                                    use ::tracing::__macro_support::Callsite as _;
                                    static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                        static META: ::tracing::Metadata<'static> = {
                                            ::tracing_core::metadata::Metadata::new(
                                                "event src/main.rs:970",
                                                "ardour_mcp_server",
                                                ::tracing::Level::DEBUG,
                                                ::tracing_core::__macro_support::Option::Some(
                                                    "src/main.rs",
                                                ),
                                                ::tracing_core::__macro_support::Option::Some(970u32),
                                                ::tracing_core::__macro_support::Option::Some(
                                                    "ardour_mcp_server",
                                                ),
                                                ::tracing_core::field::FieldSet::new(
                                                    &["message"],
                                                    ::tracing_core::callsite::Identifier(&__CALLSITE),
                                                ),
                                                ::tracing::metadata::Kind::EVENT,
                                            )
                                        };
                                        ::tracing::callsite::DefaultCallsite::new(&META)
                                    };
                                    let enabled = ::tracing::Level::DEBUG
                                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                        && ::tracing::Level::DEBUG
                                            <= ::tracing::level_filters::LevelFilter::current()
                                        && {
                                            let interest = __CALLSITE.interest();
                                            !interest.is_never()
                                                && ::tracing::__macro_support::__is_enabled(
                                                    __CALLSITE.metadata(),
                                                    interest,
                                                )
                                        };
                                    if enabled {
                                        (|value_set: ::tracing::field::ValueSet| {
                                            let meta = __CALLSITE.metadata();
                                            ::tracing::Event::dispatch(meta, &value_set);
                                        })({
                                            #[allow(unused_imports)]
                                            use ::tracing::field::{debug, display, Value};
                                            let mut iter = __CALLSITE.metadata().fields().iter();
                                            __CALLSITE
                                                .metadata()
                                                .fields()
                                                .value_set(
                                                    &[
                                                        (
                                                            &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                            ::tracing::__macro_support::Option::Some(
                                                                &format_args!(
                                                                    "Updated strip_list: SSID {0}, Name \'{1}\', Type \'{2}\'",
                                                                    strip_info.id,
                                                                    strip_info.name,
                                                                    strip_info.track_type,
                                                                ) as &dyn Value,
                                                            ),
                                                        ),
                                                    ],
                                                )
                                        });
                                    } else {
                                    }
                                };
                            }
                        }
                    }
                }
            } else if msg.addr.starts_with("/strip/type/") {
                let parts: Vec<&str> = msg.addr.split('/').collect();
                if parts.len() == 4 {
                    if let Ok(ssid) = parts[3].parse::<i32>() {
                        if ssid > 0 {
                            if let Some(type_arg) = msg.args.get(0) {
                                let type_str = match type_arg {
                                    osc::Type::String(s) => s.clone(),
                                    osc::Type::Int(i) => {
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(format_args!("type_id_{0}", i))
                                        })
                                    }
                                    _ => "unknown_type_format".to_string(),
                                };
                                {
                                    use ::tracing::__macro_support::Callsite as _;
                                    static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                        static META: ::tracing::Metadata<'static> = {
                                            ::tracing_core::metadata::Metadata::new(
                                                "event src/main.rs:988",
                                                "ardour_mcp_server",
                                                ::tracing::Level::INFO,
                                                ::tracing_core::__macro_support::Option::Some(
                                                    "src/main.rs",
                                                ),
                                                ::tracing_core::__macro_support::Option::Some(988u32),
                                                ::tracing_core::__macro_support::Option::Some(
                                                    "ardour_mcp_server",
                                                ),
                                                ::tracing_core::field::FieldSet::new(
                                                    &["message"],
                                                    ::tracing_core::callsite::Identifier(&__CALLSITE),
                                                ),
                                                ::tracing::metadata::Kind::EVENT,
                                            )
                                        };
                                        ::tracing::callsite::DefaultCallsite::new(&META)
                                    };
                                    let enabled = ::tracing::Level::INFO
                                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                        && ::tracing::Level::INFO
                                            <= ::tracing::level_filters::LevelFilter::current()
                                        && {
                                            let interest = __CALLSITE.interest();
                                            !interest.is_never()
                                                && ::tracing::__macro_support::__is_enabled(
                                                    __CALLSITE.metadata(),
                                                    interest,
                                                )
                                        };
                                    if enabled {
                                        (|value_set: ::tracing::field::ValueSet| {
                                            let meta = __CALLSITE.metadata();
                                            ::tracing::Event::dispatch(meta, &value_set);
                                        })({
                                            #[allow(unused_imports)]
                                            use ::tracing::field::{debug, display, Value};
                                            let mut iter = __CALLSITE.metadata().fields().iter();
                                            __CALLSITE
                                                .metadata()
                                                .fields()
                                                .value_set(
                                                    &[
                                                        (
                                                            &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                            ::tracing::__macro_support::Option::Some(
                                                                &format_args!(
                                                                    "Ardour feedback: /strip/type/{0} -> {1}",
                                                                    ssid,
                                                                    type_str,
                                                                ) as &dyn Value,
                                                            ),
                                                        ),
                                                    ],
                                                )
                                        });
                                    } else {
                                    }
                                };
                                let mut current_state = state.lock().await;
                                let vec_idx = (ssid - 1) as usize;
                                if vec_idx < current_state.strip_list.len() {
                                    let strip_info = &mut current_state.strip_list[vec_idx];
                                    if strip_info.id == ssid {
                                        strip_info.track_type = type_str;
                                        {
                                            use ::tracing::__macro_support::Callsite as _;
                                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                                static META: ::tracing::Metadata<'static> = {
                                                    ::tracing_core::metadata::Metadata::new(
                                                        "event src/main.rs:997",
                                                        "ardour_mcp_server",
                                                        ::tracing::Level::DEBUG,
                                                        ::tracing_core::__macro_support::Option::Some(
                                                            "src/main.rs",
                                                        ),
                                                        ::tracing_core::__macro_support::Option::Some(997u32),
                                                        ::tracing_core::__macro_support::Option::Some(
                                                            "ardour_mcp_server",
                                                        ),
                                                        ::tracing_core::field::FieldSet::new(
                                                            &["message"],
                                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                                        ),
                                                        ::tracing::metadata::Kind::EVENT,
                                                    )
                                                };
                                                ::tracing::callsite::DefaultCallsite::new(&META)
                                            };
                                            let enabled = ::tracing::Level::DEBUG
                                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                                && ::tracing::Level::DEBUG
                                                    <= ::tracing::level_filters::LevelFilter::current()
                                                && {
                                                    let interest = __CALLSITE.interest();
                                                    !interest.is_never()
                                                        && ::tracing::__macro_support::__is_enabled(
                                                            __CALLSITE.metadata(),
                                                            interest,
                                                        )
                                                };
                                            if enabled {
                                                (|value_set: ::tracing::field::ValueSet| {
                                                    let meta = __CALLSITE.metadata();
                                                    ::tracing::Event::dispatch(meta, &value_set);
                                                })({
                                                    #[allow(unused_imports)]
                                                    use ::tracing::field::{debug, display, Value};
                                                    let mut iter = __CALLSITE.metadata().fields().iter();
                                                    __CALLSITE
                                                        .metadata()
                                                        .fields()
                                                        .value_set(
                                                            &[
                                                                (
                                                                    &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                                    ::tracing::__macro_support::Option::Some(
                                                                        &format_args!(
                                                                            "Updated strip_list: SSID {0}, Name \'{1}\', Type \'{2}\'",
                                                                            strip_info.id,
                                                                            strip_info.name,
                                                                            strip_info.track_type,
                                                                        ) as &dyn Value,
                                                                    ),
                                                                ),
                                                            ],
                                                        )
                                                });
                                            } else {
                                            }
                                        };
                                    } else {
                                        {
                                            use ::tracing::__macro_support::Callsite as _;
                                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                                static META: ::tracing::Metadata<'static> = {
                                                    ::tracing_core::metadata::Metadata::new(
                                                        "event src/main.rs:999",
                                                        "ardour_mcp_server",
                                                        ::tracing::Level::WARN,
                                                        ::tracing_core::__macro_support::Option::Some(
                                                            "src/main.rs",
                                                        ),
                                                        ::tracing_core::__macro_support::Option::Some(999u32),
                                                        ::tracing_core::__macro_support::Option::Some(
                                                            "ardour_mcp_server",
                                                        ),
                                                        ::tracing_core::field::FieldSet::new(
                                                            &["message"],
                                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                                        ),
                                                        ::tracing::metadata::Kind::EVENT,
                                                    )
                                                };
                                                ::tracing::callsite::DefaultCallsite::new(&META)
                                            };
                                            let enabled = ::tracing::Level::WARN
                                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                                && ::tracing::Level::WARN
                                                    <= ::tracing::level_filters::LevelFilter::current()
                                                && {
                                                    let interest = __CALLSITE.interest();
                                                    !interest.is_never()
                                                        && ::tracing::__macro_support::__is_enabled(
                                                            __CALLSITE.metadata(),
                                                            interest,
                                                        )
                                                };
                                            if enabled {
                                                (|value_set: ::tracing::field::ValueSet| {
                                                    let meta = __CALLSITE.metadata();
                                                    ::tracing::Event::dispatch(meta, &value_set);
                                                })({
                                                    #[allow(unused_imports)]
                                                    use ::tracing::field::{debug, display, Value};
                                                    let mut iter = __CALLSITE.metadata().fields().iter();
                                                    __CALLSITE
                                                        .metadata()
                                                        .fields()
                                                        .value_set(
                                                            &[
                                                                (
                                                                    &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                                    ::tracing::__macro_support::Option::Some(
                                                                        &format_args!(
                                                                            "Received /strip/type/{0} but strip_list[{1}] has id {2}, expected {3}. Type not updated.",
                                                                            ssid,
                                                                            vec_idx,
                                                                            strip_info.id,
                                                                            ssid,
                                                                        ) as &dyn Value,
                                                                    ),
                                                                ),
                                                            ],
                                                        )
                                                });
                                            } else {
                                            }
                                        };
                                    }
                                } else {
                                    {
                                        use ::tracing::__macro_support::Callsite as _;
                                        static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                            static META: ::tracing::Metadata<'static> = {
                                                ::tracing_core::metadata::Metadata::new(
                                                    "event src/main.rs:1002",
                                                    "ardour_mcp_server",
                                                    ::tracing::Level::WARN,
                                                    ::tracing_core::__macro_support::Option::Some(
                                                        "src/main.rs",
                                                    ),
                                                    ::tracing_core::__macro_support::Option::Some(1002u32),
                                                    ::tracing_core::__macro_support::Option::Some(
                                                        "ardour_mcp_server",
                                                    ),
                                                    ::tracing_core::field::FieldSet::new(
                                                        &["message"],
                                                        ::tracing_core::callsite::Identifier(&__CALLSITE),
                                                    ),
                                                    ::tracing::metadata::Kind::EVENT,
                                                )
                                            };
                                            ::tracing::callsite::DefaultCallsite::new(&META)
                                        };
                                        let enabled = ::tracing::Level::WARN
                                            <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                            && ::tracing::Level::WARN
                                                <= ::tracing::level_filters::LevelFilter::current()
                                            && {
                                                let interest = __CALLSITE.interest();
                                                !interest.is_never()
                                                    && ::tracing::__macro_support::__is_enabled(
                                                        __CALLSITE.metadata(),
                                                        interest,
                                                    )
                                            };
                                        if enabled {
                                            (|value_set: ::tracing::field::ValueSet| {
                                                let meta = __CALLSITE.metadata();
                                                ::tracing::Event::dispatch(meta, &value_set);
                                            })({
                                                #[allow(unused_imports)]
                                                use ::tracing::field::{debug, display, Value};
                                                let mut iter = __CALLSITE.metadata().fields().iter();
                                                __CALLSITE
                                                    .metadata()
                                                    .fields()
                                                    .value_set(
                                                        &[
                                                            (
                                                                &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                    .expect("FieldSet corrupted (this is a bug)"),
                                                                ::tracing::__macro_support::Option::Some(
                                                                    &format_args!(
                                                                        "Received /strip/type/{0} but strip {1} is out of bounds for current strip_list (len {2}). Type not updated.",
                                                                        ssid,
                                                                        ssid,
                                                                        current_state.strip_list.len(),
                                                                    ) as &dyn Value,
                                                                ),
                                                            ),
                                                        ],
                                                    )
                                            });
                                        } else {
                                        }
                                    };
                                }
                            }
                        }
                    }
                }
            } else if msg.addr == "/transport_state" {
                if msg.args.len() == 2 {
                    let transport_state_val = msg
                        .args
                        .get(0)
                        .and_then(|arg| {
                            if let osc::Type::Int(s) = arg { Some(*s) } else { None }
                        });
                    let speed = msg
                        .args
                        .get(1)
                        .and_then(|arg| {
                            if let osc::Type::Float(s) = arg { Some(*s) } else { None }
                        });
                    if let (Some(ts_val), Some(s)) = (transport_state_val, speed) {
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event src/main.rs:1020",
                                        "ardour_mcp_server",
                                        ::tracing::Level::INFO,
                                        ::tracing_core::__macro_support::Option::Some(
                                            "src/main.rs",
                                        ),
                                        ::tracing_core::__macro_support::Option::Some(1020u32),
                                        ::tracing_core::__macro_support::Option::Some(
                                            "ardour_mcp_server",
                                        ),
                                        ::tracing_core::field::FieldSet::new(
                                            &["message"],
                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::EVENT,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let enabled = ::tracing::Level::INFO
                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && ::tracing::Level::INFO
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    let interest = __CALLSITE.interest();
                                    !interest.is_never()
                                        && ::tracing::__macro_support::__is_enabled(
                                            __CALLSITE.metadata(),
                                            interest,
                                        )
                                };
                            if enabled {
                                (|value_set: ::tracing::field::ValueSet| {
                                    let meta = __CALLSITE.metadata();
                                    ::tracing::Event::dispatch(meta, &value_set);
                                })({
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    let mut iter = __CALLSITE.metadata().fields().iter();
                                    __CALLSITE
                                        .metadata()
                                        .fields()
                                        .value_set(
                                            &[
                                                (
                                                    &::tracing::__macro_support::Iterator::next(&mut iter)
                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                    ::tracing::__macro_support::Option::Some(
                                                        &format_args!(
                                                            "Ardour feedback: /transport_state state: {0}, speed: {1}",
                                                            ts_val,
                                                            s,
                                                        ) as &dyn Value,
                                                    ),
                                                ),
                                            ],
                                        )
                                });
                            } else {
                            }
                        };
                        let mut current_state_guard = state.lock().await;
                        match ts_val {
                            0 => {
                                current_state_guard.playback_status = PlaybackStatus::Stopped;
                                {
                                    use ::tracing::__macro_support::Callsite as _;
                                    static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                        static META: ::tracing::Metadata<'static> = {
                                            ::tracing_core::metadata::Metadata::new(
                                                "event src/main.rs:1025",
                                                "ardour_mcp_server",
                                                ::tracing::Level::INFO,
                                                ::tracing_core::__macro_support::Option::Some(
                                                    "src/main.rs",
                                                ),
                                                ::tracing_core::__macro_support::Option::Some(1025u32),
                                                ::tracing_core::__macro_support::Option::Some(
                                                    "ardour_mcp_server",
                                                ),
                                                ::tracing_core::field::FieldSet::new(
                                                    &["message"],
                                                    ::tracing_core::callsite::Identifier(&__CALLSITE),
                                                ),
                                                ::tracing::metadata::Kind::EVENT,
                                            )
                                        };
                                        ::tracing::callsite::DefaultCallsite::new(&META)
                                    };
                                    let enabled = ::tracing::Level::INFO
                                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                        && ::tracing::Level::INFO
                                            <= ::tracing::level_filters::LevelFilter::current()
                                        && {
                                            let interest = __CALLSITE.interest();
                                            !interest.is_never()
                                                && ::tracing::__macro_support::__is_enabled(
                                                    __CALLSITE.metadata(),
                                                    interest,
                                                )
                                        };
                                    if enabled {
                                        (|value_set: ::tracing::field::ValueSet| {
                                            let meta = __CALLSITE.metadata();
                                            ::tracing::Event::dispatch(meta, &value_set);
                                        })({
                                            #[allow(unused_imports)]
                                            use ::tracing::field::{debug, display, Value};
                                            let mut iter = __CALLSITE.metadata().fields().iter();
                                            __CALLSITE
                                                .metadata()
                                                .fields()
                                                .value_set(
                                                    &[
                                                        (
                                                            &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                            ::tracing::__macro_support::Option::Some(
                                                                &format_args!(
                                                                    "Playback status updated to Stopped via /transport_state",
                                                                ) as &dyn Value,
                                                            ),
                                                        ),
                                                    ],
                                                )
                                        });
                                    } else {
                                    }
                                };
                            }
                            1 => {
                                current_state_guard.playback_status = PlaybackStatus::Playing;
                                {
                                    use ::tracing::__macro_support::Callsite as _;
                                    static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                        static META: ::tracing::Metadata<'static> = {
                                            ::tracing_core::metadata::Metadata::new(
                                                "event src/main.rs:1029",
                                                "ardour_mcp_server",
                                                ::tracing::Level::INFO,
                                                ::tracing_core::__macro_support::Option::Some(
                                                    "src/main.rs",
                                                ),
                                                ::tracing_core::__macro_support::Option::Some(1029u32),
                                                ::tracing_core::__macro_support::Option::Some(
                                                    "ardour_mcp_server",
                                                ),
                                                ::tracing_core::field::FieldSet::new(
                                                    &["message"],
                                                    ::tracing_core::callsite::Identifier(&__CALLSITE),
                                                ),
                                                ::tracing::metadata::Kind::EVENT,
                                            )
                                        };
                                        ::tracing::callsite::DefaultCallsite::new(&META)
                                    };
                                    let enabled = ::tracing::Level::INFO
                                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                        && ::tracing::Level::INFO
                                            <= ::tracing::level_filters::LevelFilter::current()
                                        && {
                                            let interest = __CALLSITE.interest();
                                            !interest.is_never()
                                                && ::tracing::__macro_support::__is_enabled(
                                                    __CALLSITE.metadata(),
                                                    interest,
                                                )
                                        };
                                    if enabled {
                                        (|value_set: ::tracing::field::ValueSet| {
                                            let meta = __CALLSITE.metadata();
                                            ::tracing::Event::dispatch(meta, &value_set);
                                        })({
                                            #[allow(unused_imports)]
                                            use ::tracing::field::{debug, display, Value};
                                            let mut iter = __CALLSITE.metadata().fields().iter();
                                            __CALLSITE
                                                .metadata()
                                                .fields()
                                                .value_set(
                                                    &[
                                                        (
                                                            &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                            ::tracing::__macro_support::Option::Some(
                                                                &format_args!(
                                                                    "Playback status updated to Playing (Rolling) via /transport_state",
                                                                ) as &dyn Value,
                                                            ),
                                                        ),
                                                    ],
                                                )
                                        });
                                    } else {
                                    }
                                };
                            }
                            2 => {
                                current_state_guard.playback_status = PlaybackStatus::Playing;
                                {
                                    use ::tracing::__macro_support::Callsite as _;
                                    static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                        static META: ::tracing::Metadata<'static> = {
                                            ::tracing_core::metadata::Metadata::new(
                                                "event src/main.rs:1033",
                                                "ardour_mcp_server",
                                                ::tracing::Level::INFO,
                                                ::tracing_core::__macro_support::Option::Some(
                                                    "src/main.rs",
                                                ),
                                                ::tracing_core::__macro_support::Option::Some(1033u32),
                                                ::tracing_core::__macro_support::Option::Some(
                                                    "ardour_mcp_server",
                                                ),
                                                ::tracing_core::field::FieldSet::new(
                                                    &["message"],
                                                    ::tracing_core::callsite::Identifier(&__CALLSITE),
                                                ),
                                                ::tracing::metadata::Kind::EVENT,
                                            )
                                        };
                                        ::tracing::callsite::DefaultCallsite::new(&META)
                                    };
                                    let enabled = ::tracing::Level::INFO
                                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                        && ::tracing::Level::INFO
                                            <= ::tracing::level_filters::LevelFilter::current()
                                        && {
                                            let interest = __CALLSITE.interest();
                                            !interest.is_never()
                                                && ::tracing::__macro_support::__is_enabled(
                                                    __CALLSITE.metadata(),
                                                    interest,
                                                )
                                        };
                                    if enabled {
                                        (|value_set: ::tracing::field::ValueSet| {
                                            let meta = __CALLSITE.metadata();
                                            ::tracing::Event::dispatch(meta, &value_set);
                                        })({
                                            #[allow(unused_imports)]
                                            use ::tracing::field::{debug, display, Value};
                                            let mut iter = __CALLSITE.metadata().fields().iter();
                                            __CALLSITE
                                                .metadata()
                                                .fields()
                                                .value_set(
                                                    &[
                                                        (
                                                            &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                            ::tracing::__macro_support::Option::Some(
                                                                &format_args!(
                                                                    "Playback status updated to Playing (Looping) via /transport_state",
                                                                ) as &dyn Value,
                                                            ),
                                                        ),
                                                    ],
                                                )
                                        });
                                    } else {
                                    }
                                };
                            }
                            _ => {
                                {
                                    use ::tracing::__macro_support::Callsite as _;
                                    static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                        static META: ::tracing::Metadata<'static> = {
                                            ::tracing_core::metadata::Metadata::new(
                                                "event src/main.rs:1036",
                                                "ardour_mcp_server",
                                                ::tracing::Level::WARN,
                                                ::tracing_core::__macro_support::Option::Some(
                                                    "src/main.rs",
                                                ),
                                                ::tracing_core::__macro_support::Option::Some(1036u32),
                                                ::tracing_core::__macro_support::Option::Some(
                                                    "ardour_mcp_server",
                                                ),
                                                ::tracing_core::field::FieldSet::new(
                                                    &["message"],
                                                    ::tracing_core::callsite::Identifier(&__CALLSITE),
                                                ),
                                                ::tracing::metadata::Kind::EVENT,
                                            )
                                        };
                                        ::tracing::callsite::DefaultCallsite::new(&META)
                                    };
                                    let enabled = ::tracing::Level::WARN
                                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                        && ::tracing::Level::WARN
                                            <= ::tracing::level_filters::LevelFilter::current()
                                        && {
                                            let interest = __CALLSITE.interest();
                                            !interest.is_never()
                                                && ::tracing::__macro_support::__is_enabled(
                                                    __CALLSITE.metadata(),
                                                    interest,
                                                )
                                        };
                                    if enabled {
                                        (|value_set: ::tracing::field::ValueSet| {
                                            let meta = __CALLSITE.metadata();
                                            ::tracing::Event::dispatch(meta, &value_set);
                                        })({
                                            #[allow(unused_imports)]
                                            use ::tracing::field::{debug, display, Value};
                                            let mut iter = __CALLSITE.metadata().fields().iter();
                                            __CALLSITE
                                                .metadata()
                                                .fields()
                                                .value_set(
                                                    &[
                                                        (
                                                            &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                            ::tracing::__macro_support::Option::Some(
                                                                &format_args!(
                                                                    "Received /transport_state with unknown state value: {0}",
                                                                    ts_val,
                                                                ) as &dyn Value,
                                                            ),
                                                        ),
                                                    ],
                                                )
                                        });
                                    } else {
                                    }
                                };
                            }
                        }
                    } else {
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event src/main.rs:1040",
                                        "ardour_mcp_server",
                                        ::tracing::Level::WARN,
                                        ::tracing_core::__macro_support::Option::Some(
                                            "src/main.rs",
                                        ),
                                        ::tracing_core::__macro_support::Option::Some(1040u32),
                                        ::tracing_core::__macro_support::Option::Some(
                                            "ardour_mcp_server",
                                        ),
                                        ::tracing_core::field::FieldSet::new(
                                            &["message"],
                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::EVENT,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let enabled = ::tracing::Level::WARN
                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && ::tracing::Level::WARN
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    let interest = __CALLSITE.interest();
                                    !interest.is_never()
                                        && ::tracing::__macro_support::__is_enabled(
                                            __CALLSITE.metadata(),
                                            interest,
                                        )
                                };
                            if enabled {
                                (|value_set: ::tracing::field::ValueSet| {
                                    let meta = __CALLSITE.metadata();
                                    ::tracing::Event::dispatch(meta, &value_set);
                                })({
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    let mut iter = __CALLSITE.metadata().fields().iter();
                                    __CALLSITE
                                        .metadata()
                                        .fields()
                                        .value_set(
                                            &[
                                                (
                                                    &::tracing::__macro_support::Iterator::next(&mut iter)
                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                    ::tracing::__macro_support::Option::Some(
                                                        &format_args!(
                                                            "Received /transport_state with unexpected argument types: {0:?}. Expected Int, Float.",
                                                            msg.args,
                                                        ) as &dyn Value,
                                                    ),
                                                ),
                                            ],
                                        )
                                });
                            } else {
                            }
                        };
                    }
                } else {
                    {
                        use ::tracing::__macro_support::Callsite as _;
                        static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "event src/main.rs:1043",
                                    "ardour_mcp_server",
                                    ::tracing::Level::WARN,
                                    ::tracing_core::__macro_support::Option::Some(
                                        "src/main.rs",
                                    ),
                                    ::tracing_core::__macro_support::Option::Some(1043u32),
                                    ::tracing_core::__macro_support::Option::Some(
                                        "ardour_mcp_server",
                                    ),
                                    ::tracing_core::field::FieldSet::new(
                                        &["message"],
                                        ::tracing_core::callsite::Identifier(&__CALLSITE),
                                    ),
                                    ::tracing::metadata::Kind::EVENT,
                                )
                            };
                            ::tracing::callsite::DefaultCallsite::new(&META)
                        };
                        let enabled = ::tracing::Level::WARN
                            <= ::tracing::level_filters::STATIC_MAX_LEVEL
                            && ::tracing::Level::WARN
                                <= ::tracing::level_filters::LevelFilter::current()
                            && {
                                let interest = __CALLSITE.interest();
                                !interest.is_never()
                                    && ::tracing::__macro_support::__is_enabled(
                                        __CALLSITE.metadata(),
                                        interest,
                                    )
                            };
                        if enabled {
                            (|value_set: ::tracing::field::ValueSet| {
                                let meta = __CALLSITE.metadata();
                                ::tracing::Event::dispatch(meta, &value_set);
                            })({
                                #[allow(unused_imports)]
                                use ::tracing::field::{debug, display, Value};
                                let mut iter = __CALLSITE.metadata().fields().iter();
                                __CALLSITE
                                    .metadata()
                                    .fields()
                                    .value_set(
                                        &[
                                            (
                                                &::tracing::__macro_support::Iterator::next(&mut iter)
                                                    .expect("FieldSet corrupted (this is a bug)"),
                                                ::tracing::__macro_support::Option::Some(
                                                    &format_args!(
                                                        "Received /transport_state with incorrect number of arguments: {0}. Expected 2.",
                                                        msg.args.len(),
                                                    ) as &dyn Value,
                                                ),
                                            ),
                                        ],
                                    )
                            });
                        } else {
                        }
                    };
                }
            } else if msg.addr == "/transport_frame" {
                if msg.args.len() == 1 {
                    if let Some(osc::Type::Long(frame)) = msg.args.get(0) {
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event src/main.rs:1050",
                                        "ardour_mcp_server",
                                        ::tracing::Level::INFO,
                                        ::tracing_core::__macro_support::Option::Some(
                                            "src/main.rs",
                                        ),
                                        ::tracing_core::__macro_support::Option::Some(1050u32),
                                        ::tracing_core::__macro_support::Option::Some(
                                            "ardour_mcp_server",
                                        ),
                                        ::tracing_core::field::FieldSet::new(
                                            &["message"],
                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::EVENT,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let enabled = ::tracing::Level::INFO
                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && ::tracing::Level::INFO
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    let interest = __CALLSITE.interest();
                                    !interest.is_never()
                                        && ::tracing::__macro_support::__is_enabled(
                                            __CALLSITE.metadata(),
                                            interest,
                                        )
                                };
                            if enabled {
                                (|value_set: ::tracing::field::ValueSet| {
                                    let meta = __CALLSITE.metadata();
                                    ::tracing::Event::dispatch(meta, &value_set);
                                })({
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    let mut iter = __CALLSITE.metadata().fields().iter();
                                    __CALLSITE
                                        .metadata()
                                        .fields()
                                        .value_set(
                                            &[
                                                (
                                                    &::tracing::__macro_support::Iterator::next(&mut iter)
                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                    ::tracing::__macro_support::Option::Some(
                                                        &format_args!(
                                                            "Ardour feedback: /transport_frame -> {0}",
                                                            frame,
                                                        ) as &dyn Value,
                                                    ),
                                                ),
                                            ],
                                        )
                                });
                            } else {
                            }
                        };
                        let mut current_state_guard = state.lock().await;
                        current_state_guard.transport_frame = Some(*frame);
                    } else {
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event src/main.rs:1054",
                                        "ardour_mcp_server",
                                        ::tracing::Level::WARN,
                                        ::tracing_core::__macro_support::Option::Some(
                                            "src/main.rs",
                                        ),
                                        ::tracing_core::__macro_support::Option::Some(1054u32),
                                        ::tracing_core::__macro_support::Option::Some(
                                            "ardour_mcp_server",
                                        ),
                                        ::tracing_core::field::FieldSet::new(
                                            &["message"],
                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::EVENT,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let enabled = ::tracing::Level::WARN
                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && ::tracing::Level::WARN
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    let interest = __CALLSITE.interest();
                                    !interest.is_never()
                                        && ::tracing::__macro_support::__is_enabled(
                                            __CALLSITE.metadata(),
                                            interest,
                                        )
                                };
                            if enabled {
                                (|value_set: ::tracing::field::ValueSet| {
                                    let meta = __CALLSITE.metadata();
                                    ::tracing::Event::dispatch(meta, &value_set);
                                })({
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    let mut iter = __CALLSITE.metadata().fields().iter();
                                    __CALLSITE
                                        .metadata()
                                        .fields()
                                        .value_set(
                                            &[
                                                (
                                                    &::tracing::__macro_support::Iterator::next(&mut iter)
                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                    ::tracing::__macro_support::Option::Some(
                                                        &format_args!(
                                                            "Received /transport_frame with unexpected argument type: {0:?}. Expected Long.",
                                                            msg.args.get(0),
                                                        ) as &dyn Value,
                                                    ),
                                                ),
                                            ],
                                        )
                                });
                            } else {
                            }
                        };
                    }
                } else {
                    {
                        use ::tracing::__macro_support::Callsite as _;
                        static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "event src/main.rs:1057",
                                    "ardour_mcp_server",
                                    ::tracing::Level::WARN,
                                    ::tracing_core::__macro_support::Option::Some(
                                        "src/main.rs",
                                    ),
                                    ::tracing_core::__macro_support::Option::Some(1057u32),
                                    ::tracing_core::__macro_support::Option::Some(
                                        "ardour_mcp_server",
                                    ),
                                    ::tracing_core::field::FieldSet::new(
                                        &["message"],
                                        ::tracing_core::callsite::Identifier(&__CALLSITE),
                                    ),
                                    ::tracing::metadata::Kind::EVENT,
                                )
                            };
                            ::tracing::callsite::DefaultCallsite::new(&META)
                        };
                        let enabled = ::tracing::Level::WARN
                            <= ::tracing::level_filters::STATIC_MAX_LEVEL
                            && ::tracing::Level::WARN
                                <= ::tracing::level_filters::LevelFilter::current()
                            && {
                                let interest = __CALLSITE.interest();
                                !interest.is_never()
                                    && ::tracing::__macro_support::__is_enabled(
                                        __CALLSITE.metadata(),
                                        interest,
                                    )
                            };
                        if enabled {
                            (|value_set: ::tracing::field::ValueSet| {
                                let meta = __CALLSITE.metadata();
                                ::tracing::Event::dispatch(meta, &value_set);
                            })({
                                #[allow(unused_imports)]
                                use ::tracing::field::{debug, display, Value};
                                let mut iter = __CALLSITE.metadata().fields().iter();
                                __CALLSITE
                                    .metadata()
                                    .fields()
                                    .value_set(
                                        &[
                                            (
                                                &::tracing::__macro_support::Iterator::next(&mut iter)
                                                    .expect("FieldSet corrupted (this is a bug)"),
                                                ::tracing::__macro_support::Option::Some(
                                                    &format_args!(
                                                        "Received /transport_frame with incorrect number of arguments: {0}. Expected 1.",
                                                        msg.args.len(),
                                                    ) as &dyn Value,
                                                ),
                                            ),
                                        ],
                                    )
                            });
                        } else {
                        }
                    };
                }
            } else if msg.addr.as_str() == "/strip/play" {
                let mut current_state = state.lock().await;
                if let Some(osc::Type::Int(is_playing_val)) = msg.args.get(0) {
                    if *is_playing_val == 1 {
                        current_state.playback_status = PlaybackStatus::Playing;
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event src/main.rs:1066",
                                        "ardour_mcp_server",
                                        ::tracing::Level::INFO,
                                        ::tracing_core::__macro_support::Option::Some(
                                            "src/main.rs",
                                        ),
                                        ::tracing_core::__macro_support::Option::Some(1066u32),
                                        ::tracing_core::__macro_support::Option::Some(
                                            "ardour_mcp_server",
                                        ),
                                        ::tracing_core::field::FieldSet::new(
                                            &["message"],
                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::EVENT,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let enabled = ::tracing::Level::INFO
                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && ::tracing::Level::INFO
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    let interest = __CALLSITE.interest();
                                    !interest.is_never()
                                        && ::tracing::__macro_support::__is_enabled(
                                            __CALLSITE.metadata(),
                                            interest,
                                        )
                                };
                            if enabled {
                                (|value_set: ::tracing::field::ValueSet| {
                                    let meta = __CALLSITE.metadata();
                                    ::tracing::Event::dispatch(meta, &value_set);
                                })({
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    let mut iter = __CALLSITE.metadata().fields().iter();
                                    __CALLSITE
                                        .metadata()
                                        .fields()
                                        .value_set(
                                            &[
                                                (
                                                    &::tracing::__macro_support::Iterator::next(&mut iter)
                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                    ::tracing::__macro_support::Option::Some(
                                                        &format_args!(
                                                            "Ardour feedback via /strip/play: Playback Started (state=1)",
                                                        ) as &dyn Value,
                                                    ),
                                                ),
                                            ],
                                        )
                                });
                            } else {
                            }
                        };
                    } else if *is_playing_val == 0 {
                        current_state.playback_status = PlaybackStatus::Stopped;
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event src/main.rs:1069",
                                        "ardour_mcp_server",
                                        ::tracing::Level::INFO,
                                        ::tracing_core::__macro_support::Option::Some(
                                            "src/main.rs",
                                        ),
                                        ::tracing_core::__macro_support::Option::Some(1069u32),
                                        ::tracing_core::__macro_support::Option::Some(
                                            "ardour_mcp_server",
                                        ),
                                        ::tracing_core::field::FieldSet::new(
                                            &["message"],
                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::EVENT,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let enabled = ::tracing::Level::INFO
                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && ::tracing::Level::INFO
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    let interest = __CALLSITE.interest();
                                    !interest.is_never()
                                        && ::tracing::__macro_support::__is_enabled(
                                            __CALLSITE.metadata(),
                                            interest,
                                        )
                                };
                            if enabled {
                                (|value_set: ::tracing::field::ValueSet| {
                                    let meta = __CALLSITE.metadata();
                                    ::tracing::Event::dispatch(meta, &value_set);
                                })({
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    let mut iter = __CALLSITE.metadata().fields().iter();
                                    __CALLSITE
                                        .metadata()
                                        .fields()
                                        .value_set(
                                            &[
                                                (
                                                    &::tracing::__macro_support::Iterator::next(&mut iter)
                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                    ::tracing::__macro_support::Option::Some(
                                                        &format_args!(
                                                            "Ardour feedback via /strip/play: Playback Stopped (state=0)",
                                                        ) as &dyn Value,
                                                    ),
                                                ),
                                            ],
                                        )
                                });
                            } else {
                            }
                        };
                    } else {
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event src/main.rs:1071",
                                        "ardour_mcp_server",
                                        ::tracing::Level::WARN,
                                        ::tracing_core::__macro_support::Option::Some(
                                            "src/main.rs",
                                        ),
                                        ::tracing_core::__macro_support::Option::Some(1071u32),
                                        ::tracing_core::__macro_support::Option::Some(
                                            "ardour_mcp_server",
                                        ),
                                        ::tracing_core::field::FieldSet::new(
                                            &["message"],
                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::EVENT,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let enabled = ::tracing::Level::WARN
                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && ::tracing::Level::WARN
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    let interest = __CALLSITE.interest();
                                    !interest.is_never()
                                        && ::tracing::__macro_support::__is_enabled(
                                            __CALLSITE.metadata(),
                                            interest,
                                        )
                                };
                            if enabled {
                                (|value_set: ::tracing::field::ValueSet| {
                                    let meta = __CALLSITE.metadata();
                                    ::tracing::Event::dispatch(meta, &value_set);
                                })({
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    let mut iter = __CALLSITE.metadata().fields().iter();
                                    __CALLSITE
                                        .metadata()
                                        .fields()
                                        .value_set(
                                            &[
                                                (
                                                    &::tracing::__macro_support::Iterator::next(&mut iter)
                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                    ::tracing::__macro_support::Option::Some(
                                                        &format_args!(
                                                            "Ardour feedback via /strip/play: Received with unexpected integer state: {0}. Ignoring for playback status.",
                                                            is_playing_val,
                                                        ) as &dyn Value,
                                                    ),
                                                ),
                                            ],
                                        )
                                });
                            } else {
                            }
                        };
                    }
                } else {
                    {
                        use ::tracing::__macro_support::Callsite as _;
                        static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "event src/main.rs:1077",
                                    "ardour_mcp_server",
                                    ::tracing::Level::WARN,
                                    ::tracing_core::__macro_support::Option::Some(
                                        "src/main.rs",
                                    ),
                                    ::tracing_core::__macro_support::Option::Some(1077u32),
                                    ::tracing_core::__macro_support::Option::Some(
                                        "ardour_mcp_server",
                                    ),
                                    ::tracing_core::field::FieldSet::new(
                                        &["message"],
                                        ::tracing_core::callsite::Identifier(&__CALLSITE),
                                    ),
                                    ::tracing::metadata::Kind::EVENT,
                                )
                            };
                            ::tracing::callsite::DefaultCallsite::new(&META)
                        };
                        let enabled = ::tracing::Level::WARN
                            <= ::tracing::level_filters::STATIC_MAX_LEVEL
                            && ::tracing::Level::WARN
                                <= ::tracing::level_filters::LevelFilter::current()
                            && {
                                let interest = __CALLSITE.interest();
                                !interest.is_never()
                                    && ::tracing::__macro_support::__is_enabled(
                                        __CALLSITE.metadata(),
                                        interest,
                                    )
                            };
                        if enabled {
                            (|value_set: ::tracing::field::ValueSet| {
                                let meta = __CALLSITE.metadata();
                                ::tracing::Event::dispatch(meta, &value_set);
                            })({
                                #[allow(unused_imports)]
                                use ::tracing::field::{debug, display, Value};
                                let mut iter = __CALLSITE.metadata().fields().iter();
                                __CALLSITE
                                    .metadata()
                                    .fields()
                                    .value_set(
                                        &[
                                            (
                                                &::tracing::__macro_support::Iterator::next(&mut iter)
                                                    .expect("FieldSet corrupted (this is a bug)"),
                                                ::tracing::__macro_support::Option::Some(
                                                    &format_args!(
                                                        "Ardour feedback via /strip/play: Received without expected integer argument. Args: {0:?}. Ignoring for playback status.",
                                                        msg.args,
                                                    ) as &dyn Value,
                                                ),
                                            ),
                                        ],
                                    )
                            });
                        } else {
                        }
                    };
                }
            } else if msg.addr.starts_with("/strip/") {
                {
                    use ::tracing::__macro_support::Callsite as _;
                    static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "event src/main.rs:1085",
                                "ardour_mcp_server",
                                ::tracing::Level::DEBUG,
                                ::tracing_core::__macro_support::Option::Some(
                                    "src/main.rs",
                                ),
                                ::tracing_core::__macro_support::Option::Some(1085u32),
                                ::tracing_core::__macro_support::Option::Some(
                                    "ardour_mcp_server",
                                ),
                                ::tracing_core::field::FieldSet::new(
                                    &["message"],
                                    ::tracing_core::callsite::Identifier(&__CALLSITE),
                                ),
                                ::tracing::metadata::Kind::EVENT,
                            )
                        };
                        ::tracing::callsite::DefaultCallsite::new(&META)
                    };
                    let enabled = ::tracing::Level::DEBUG
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && ::tracing::Level::DEBUG
                            <= ::tracing::level_filters::LevelFilter::current()
                        && {
                            let interest = __CALLSITE.interest();
                            !interest.is_never()
                                && ::tracing::__macro_support::__is_enabled(
                                    __CALLSITE.metadata(),
                                    interest,
                                )
                        };
                    if enabled {
                        (|value_set: ::tracing::field::ValueSet| {
                            let meta = __CALLSITE.metadata();
                            ::tracing::Event::dispatch(meta, &value_set);
                        })({
                            #[allow(unused_imports)]
                            use ::tracing::field::{debug, display, Value};
                            let mut iter = __CALLSITE.metadata().fields().iter();
                            __CALLSITE
                                .metadata()
                                .fields()
                                .value_set(
                                    &[
                                        (
                                            &::tracing::__macro_support::Iterator::next(&mut iter)
                                                .expect("FieldSet corrupted (this is a bug)"),
                                            ::tracing::__macro_support::Option::Some(
                                                &format_args!(
                                                    "Received other Ardour /strip/ feedback: {0} {1:?}",
                                                    msg.addr,
                                                    msg.args,
                                                ) as &dyn Value,
                                            ),
                                        ),
                                    ],
                                )
                        });
                    } else {
                    }
                };
            }
        }
        osc::Packet::Bundle(bundle) => {
            {
                use ::tracing::__macro_support::Callsite as _;
                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                    static META: ::tracing::Metadata<'static> = {
                        ::tracing_core::metadata::Metadata::new(
                            "event src/main.rs:1093",
                            "ardour_mcp_server",
                            ::tracing::Level::DEBUG,
                            ::tracing_core::__macro_support::Option::Some("src/main.rs"),
                            ::tracing_core::__macro_support::Option::Some(1093u32),
                            ::tracing_core::__macro_support::Option::Some(
                                "ardour_mcp_server",
                            ),
                            ::tracing_core::field::FieldSet::new(
                                &["message"],
                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                            ),
                            ::tracing::metadata::Kind::EVENT,
                        )
                    };
                    ::tracing::callsite::DefaultCallsite::new(&META)
                };
                let enabled = ::tracing::Level::DEBUG
                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                    && ::tracing::Level::DEBUG
                        <= ::tracing::level_filters::LevelFilter::current()
                    && {
                        let interest = __CALLSITE.interest();
                        !interest.is_never()
                            && ::tracing::__macro_support::__is_enabled(
                                __CALLSITE.metadata(),
                                interest,
                            )
                    };
                if enabled {
                    (|value_set: ::tracing::field::ValueSet| {
                        let meta = __CALLSITE.metadata();
                        ::tracing::Event::dispatch(meta, &value_set);
                    })({
                        #[allow(unused_imports)]
                        use ::tracing::field::{debug, display, Value};
                        let mut iter = __CALLSITE.metadata().fields().iter();
                        __CALLSITE
                            .metadata()
                            .fields()
                            .value_set(
                                &[
                                    (
                                        &::tracing::__macro_support::Iterator::next(&mut iter)
                                            .expect("FieldSet corrupted (this is a bug)"),
                                        ::tracing::__macro_support::Option::Some(
                                            &format_args!("Received OSC bundle from {0}:", peer_addr)
                                                as &dyn Value,
                                        ),
                                    ),
                                ],
                            )
                    });
                } else {
                }
            };
            for p_in_bundle in bundle.content {
                Box::pin(
                        handle_osc_packet(
                            p_in_bundle.into(),
                            peer_addr,
                            Arc::clone(&state),
                        ),
                    )
                    .await;
            }
        }
    }
}
