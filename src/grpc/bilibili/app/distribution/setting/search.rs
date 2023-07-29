///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchAutoPlay {
    ///
    #[prost(message, optional, tag = "1")]
    pub value: ::core::option::Option<super::super::v1::Int64Value>,
    ///
    #[prost(message, optional, tag = "2")]
    pub affected_by_server_side: ::core::option::Option<super::super::v1::BoolValue>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchDeviceConfig {
    ///
    #[prost(message, optional, tag = "1")]
    pub auto_play: ::core::option::Option<SearchAutoPlay>,
}
