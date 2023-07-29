///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MidPrivacySettingsConfig {
    ///
    #[prost(message, optional, tag = "1")]
    pub recommend_to_known: ::core::option::Option<super::super::v1::BoolValue>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrivacySettingsConfig {
    ///
    #[prost(message, optional, tag = "1")]
    pub ad_recommand_store: ::core::option::Option<super::super::v1::BoolValue>,
    /// 传感器权限
    #[prost(message, optional, tag = "2")]
    pub sensor_access: ::core::option::Option<super::super::v1::BoolValue>,
}
