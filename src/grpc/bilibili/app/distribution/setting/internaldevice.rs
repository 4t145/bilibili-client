///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InternalDeviceConfig {
    /// 首次启动时间
    #[prost(message, optional, tag = "1")]
    pub fts: ::core::option::Option<super::super::v1::Int64Value>,
}
