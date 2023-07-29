///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NightSettingsConfig {
    ///
    #[prost(message, optional, tag = "1")]
    pub is_night_follow_system: ::core::option::Option<super::super::v1::BoolValue>,
}
