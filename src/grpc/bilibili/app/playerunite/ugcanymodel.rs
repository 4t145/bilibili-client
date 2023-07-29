#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ButtonStyle {
    #[prost(string, tag = "1")]
    pub text: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub text_color: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub bg_color: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub jump_link: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayLimit {
    #[prost(enumeration = "PlayLimitCode", tag = "1")]
    pub code: i32,
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub sub_message: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub button: ::core::option::Option<ButtonStyle>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UgcAnyModel {
    #[prost(message, optional, tag = "1")]
    pub play_limit: ::core::option::Option<PlayLimit>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PlayLimitCode {
    PlcUnknown = 0,
    PlcNotpayed = 1,
}
impl PlayLimitCode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PlayLimitCode::PlcUnknown => "PLC_UNKNOWN",
            PlayLimitCode::PlcNotpayed => "PLC_NOTPAYED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PLC_UNKNOWN" => Some(Self::PlcUnknown),
            "PLC_NOTPAYED" => Some(Self::PlcNotpayed),
            _ => None,
        }
    }
}
