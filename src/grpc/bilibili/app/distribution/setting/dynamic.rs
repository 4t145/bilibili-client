///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynamicAutoPlay {
    ///
    #[prost(message, optional, tag = "1")]
    pub value: ::core::option::Option<super::super::v1::Int64Value>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynamicDeviceConfig {
    ///
    #[prost(message, optional, tag = "1")]
    pub auto_play: ::core::option::Option<DynamicAutoPlay>,
}
