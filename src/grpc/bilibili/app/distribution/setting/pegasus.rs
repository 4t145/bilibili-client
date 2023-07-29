///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedModeValue {
    ///
    #[prost(message, optional, tag = "1")]
    pub value: ::core::option::Option<super::super::v1::Int64Value>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PegasusAutoPlay {
    ///
    #[prost(message, optional, tag = "1")]
    pub single: ::core::option::Option<super::super::v1::Int64Value>,
    ///
    #[prost(message, optional, tag = "2")]
    pub double: ::core::option::Option<super::super::v1::Int64Value>,
    ///
    #[prost(message, optional, tag = "3")]
    pub single_affected_by_server_side: ::core::option::Option<
        super::super::v1::BoolValue,
    >,
    ///
    #[prost(message, optional, tag = "4")]
    pub double_affected_by_server_side: ::core::option::Option<
        super::super::v1::BoolValue,
    >,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PegasusColumnValue {
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
pub struct PegasusDeviceConfig {
    ///
    #[prost(message, optional, tag = "1")]
    pub column: ::core::option::Option<PegasusColumnValue>,
    ///
    #[prost(message, optional, tag = "2")]
    pub mode: ::core::option::Option<FeedModeValue>,
    ///
    #[prost(message, optional, tag = "3")]
    pub auto_play: ::core::option::Option<PegasusAutoPlay>,
}
